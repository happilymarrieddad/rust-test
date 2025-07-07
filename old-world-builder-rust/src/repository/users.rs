use sqlx::{Pool, Postgres};
use sqlx::error::Error;
use sqlx::Row;
use sqlx::Column;

use crate::types::users;

#[cfg(test)]
mod tests {
    use std::env;

    use serial_test::serial;
    use sqlx::{postgres::PgPoolOptions, Executor};

    use super::*;

    async fn init() -> UserRepo {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env is required to run tests");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await.expect("unable to connect to database with url");

        pool.execute("TRUNCATE users").await.expect("unable to truncate users");

        UserRepo::new(pool)
    }

    #[actix_rt::test]
    #[serial] // so we only run them 1 at a time
    async fn create_user_success() {
        let repo = init().await;

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

    #[actix_rt::test]
    #[serial]
    async fn get_user_success() {
        let repo = init().await;

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

pub struct UserRepoFindBy {
    id: i64,
    email: String,
}

impl UserRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepo{pool: pool}
    }

    pub async fn find_by(&self, opts: UserRepoFindBy) -> Result<users::user, Error> {
        // if opts.id.is_none() && opts.email.is_none() {
        //     return Err(sqlx::Error::InvalidArgument(String::from("either id or email is required")));
        // }

        let result = sqlx::query_as!(
                users::user, 
                "SELECT * FROM users WHERE id = $1 OR email = $2", 
                opts.id, opts.email)
            .fetch_optional(&self.pool)
            .await?;

        // if result.is_some() {
        //    return Ok(result.unwrap())
        // }

        //Err(sqlx::Error::RowNotFound)
        Ok(result.unwrap())
    }

    pub async fn create(&self, new_user: users::create_user) -> Result<users::user, Error> {
        // TODO: add validation
        // if new_user.password == String::from("") {
        //     return Err(Error::from(String::from("non-empty password required")));
        // }

        // if new_user.password != new_user.password_confirm {
        //     return Err("password must match password confirm");
        // }

        let row = sqlx::query_as!(users::user,
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4) RETURNING *"
                ,new_user.first_name.clone()
                ,new_user.last_name.clone()
                ,new_user.email.clone()
                ,new_user.password.clone())
            .fetch_one(&self.pool).await?;
        
        Ok(row)
    }

    pub async fn get(&self, id: i64) -> Result<users::user, Error> {
        let row = sqlx::query_as!(users::user,
            "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&self.pool).await?;

        Ok(row)
    }
}