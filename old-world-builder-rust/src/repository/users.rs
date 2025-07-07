use sqlx::{Pool, Postgres};
use sqlx::error::Error;
use sqlx::Row;
use sqlx::Column;

use crate::types::users;

#[cfg(test)]
mod tests {
    use std::env;

    use sqlx::{postgres::PgPoolOptions, Executor};

    use super::*;

    #[actix_rt::test]
    async fn create_user_success() {
        let mut db_url = env::var("OLD_WORLD_BUILDER_RUST_DB_URL").expect("OLD_WORLD_BUILDER_RUST_DB_URL env is required to run tests");
        db_url = String::from("postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await.expect("unable to connect to database with url");

        pool.execute("TRUNCATE users").await.expect("unable to truncate users");

        let repo = UserRepo::new(pool);

        let new_user = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: String::from("nick@mail.com"),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.expect("unable to create user");

        assert_ne!(new_user.id, 0);
        assert_eq!(new_user.email, "nick@mail.com");
        println!("{:#?}", new_user);

        let err: Error = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: String::from("nick@mail.com"),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.expect_err("creating the same user should fail");

        assert_eq!(err.to_string(), "error returned from database: duplicate key value violates unique constraint \"users_email_key\"");
    }

    async fn get_user_success() {
        let mut db_url = env::var("OLD_WORLD_BUILDER_RUST_DB_URL").expect("OLD_WORLD_BUILDER_RUST_DB_URL env is required to run tests");
        db_url = String::from("postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await.expect("unable to connect to database with url");

        pool.execute("TRUNCATE users").await.expect("unable to truncate users");

        let repo = UserRepo::new(pool);

        let new_user = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: String::from("nick@mail.com"),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.expect("unable to create user");

        assert_ne!(new_user.id, 0);
        assert_eq!(new_user.email, "nick@mail.com");
        println!("{:#?}", new_user);

        let existing_user = repo.get(new_user.id).await.expect("unable to get user by id");

        assert_eq!(new_user.first_name, existing_user.first_name);
        assert_eq!(new_user.last_name, existing_user.last_name);
        assert_eq!(new_user.email, existing_user.email);
        assert_eq!(new_user.password, existing_user.password); // TODO: compare it's hash instead... shouldn't be storing raw strings
    }
}

pub struct UserRepo {
    pool: Pool<Postgres>,
}

impl UserRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepo{pool: pool}
    }

    pub async fn create(&self, new_user: users::create_user) -> Result<users::user, Error> {
        // TODO: add validation
        // if new_user.password == String::from("") {
        //     return Err(Error::from(String::from("non-empty password required")));
        // }

        // if new_user.password != new_user.password_confirm {
        //     return Err("password must match password confirm");
        // }

        let row = sqlx::query(
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4) RETURNING id, created_at, updated_at")
            .bind(new_user.first_name.clone())
            .bind(new_user.last_name.clone())
            .bind(new_user.email.clone())
            .bind(new_user.password.clone())
            .fetch_one(&self.pool).await?;

        let mut new_user = users::user::new();

        for col in row.columns() {
            match col.name() {
                "id" => {
                    new_user.id = row.try_get(col.name()).unwrap();
                }
                "created_at" => {
                    new_user.created_at = row.try_get(col.name()).unwrap();
                }
                "updated_at" => {
                    new_user.updated_at = row.try_get(col.name()).unwrap();
                }
                _ => {
                    println!("unknown field found for user create '{}'", col.name());
                }
            }
        }
        
        Ok(new_user)
    }

    pub async fn get(&self, id: i64) -> Result<users::user, Error> {
        let row: sqlx::postgres::PgRow = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool).await?;

        let mut existing_user = users::user::new();

        for col in row.columns() {
            match col.name() {
                "id" => {
                    existing_user.id = row.try_get(col.name()).unwrap();
                }
                "first_name" => {
                    existing_user.first_name = row.try_get(col.name()).unwrap();
                }
                "last_name" => {
                    existing_user.last_name = row.try_get(col.name()).unwrap();
                }
                "email" => {
                    existing_user.email = row.try_get(col.name()).unwrap();
                }
                "password" => {
                    existing_user.password = row.try_get(col.name()).unwrap();
                }
                "created_at" => {
                    existing_user.created_at = row.try_get(col.name()).unwrap();
                }
                "updated_at" => {
                    existing_user.updated_at = row.try_get(col.name()).unwrap();
                }
                _ => {
                    println!("unknown field found for user get '{}'", col.name());
                }
            }
        }

        Ok(existing_user)
    }
}