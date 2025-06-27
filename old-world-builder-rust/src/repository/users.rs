use serde::Serialize;
use sqlx::{Pool, Postgres};
use sqlx::error::Error;
use sqlx::Row;
use sqlx::Column;

use crate::types::users;

#[cfg(test)]
mod tests {
    use sqlx::{postgres::PgPoolOptions, Executor};

    use super::*;

    #[actix_rt::test]
    async fn create_user() {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable")
            .await.unwrap();

        pool.execute("TRUNCATE users").await.expect("unable to truncate users");

        let repo = UserRepo::new(pool);

        let new_user = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: String::from("nick@mail.com"),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.unwrap();

        assert_ne!(new_user.id, 0);
        assert_eq!(new_user.email, "nick@mail.com");

        let err: Error = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: String::from("nick@mail.com"),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.unwrap_err();

        assert_eq!(err.to_string(), "error returned from database: duplicate key value violates unique constraint \"users_email_key\"");
    }
}

pub struct UserRepo {
    pool: Pool<Postgres>,
}

impl UserRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepo{pool: pool}
    }

    async fn create(&self, new_user: users::create_user) -> Result<users::user, Error> {
        let row = sqlx::query(
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4) RETURNING id, created_at, updated_at")
            .bind(new_user.first_name.clone())
            .bind(new_user.last_name.clone())
            .bind(new_user.email.clone())
            .bind(new_user.password.clone())
            .fetch_one(&self.pool).await?;

        let mut new_user = users::user{
            id: 0,
            first_name: String::from(new_user.first_name),
            last_name: String::from(new_user.last_name),
            email: String::from(new_user.email),
            password: String::from(new_user.password),
        };

        for col in row.columns() {
            match col.name() {
                "id" => {
                    new_user.id = row.try_get(col.name()).unwrap();
                }
                "created_at" => {

                }
                "updated_at" => {

                }
                _ => {

                }
            }
        }
        
        Ok(new_user)
    }
}