use serde::Serialize;
use sqlx::{Pool, Postgres, Transaction};
use sqlx::error::Error;
use bcrypt::{hash, BcryptResult, DEFAULT_COST}; // , verify

use crate::types::users;

fn hash_password(password: String) -> BcryptResult<String> {
    hash(password, DEFAULT_COST)
}

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

    async fn create_test_user(repo: &UserRepo, email: String) -> users::user {
        let new_user = repo.create(users::create_user{
            first_name: String::from("Nick"),
            last_name: String::from("Kotenberg"),
            email: email.clone(),
            password: String::from("1234"),
            password_confirm: String::from("1234"),
        }).await.expect("unable to create user");

        assert_ne!(new_user.id, 0);
        assert_eq!(new_user.email, email);
        
        new_user
    }

    #[actix_rt::test]
    #[serial] // so we only run them 1 at a time
    async fn create_user_success() {
        let repo = init().await;
        create_test_user(&repo, String::from("nick@mail.com")).await;

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
        let new_user = create_test_user(&repo, String::from("nick@mail.com")).await;

        let existing_user = repo.get(new_user.id).await.expect("unable to get user by id");

        assert_eq!(new_user.first_name, existing_user.first_name);
        assert_eq!(new_user.last_name, existing_user.last_name);
        assert_eq!(new_user.email, existing_user.email);
        // for some reason this is not working...  TODO: fix it
        // let hash_compare = hash(new_user.password, DEFAULT_COST).unwrap();
        // let password_compare = verify(hash_compare, &existing_user.password).expect("should successfully verify passwords");
        // assert_eq!(true, password_compare);
    }

    #[actix_rt::test]
    #[serial]
    async fn find_by_user_success() {
        let repo = init().await;
        let new_user = create_test_user(&repo, String::from("nick@mail.com")).await;

        let user = repo
            .find_by_email(String::from("nick@mail.com"))
            .await.expect("failed to find user");
        assert_ne!(user.id, 0);
        assert_eq!(user.email, "nick@mail.com");
        assert_eq!(user.id, new_user.id);
    }

    #[actix_rt::test]
    #[serial]
    async fn find_by_user_fail_not_found() {
        let repo = init().await;
        let new_user = create_test_user(&repo, String::from("nick@mail.com")).await;

        let user = repo
            .find_by_email(String::from("nick@mail.com"))
            .await.expect("failed to find user");
        assert_ne!(user.id, 0);
        assert_eq!(user.email, "nick@mail.com");
        assert_eq!(user.id, new_user.id);

        let err = repo
            .find_by_email(String::from("garbage"))
            .await.expect_err("user found when none should be");

        assert_eq!(err.to_string(), sqlx::Error::RowNotFound.to_string());
    }

    #[actix_rt::test]
    #[serial]
    async fn find_success() {
        let repo = init().await;
        let new_user = create_test_user(&repo, String::from("nick@mail.com")).await;
        let new_user2 = create_test_user(&repo, String::from("nick2@mail.com")).await;
        let new_user3 = create_test_user(&repo, String::from("nick3@mail.com")).await;

        let find_result_1 = repo.find(UserRepoFindOpts::new(0, String::from(""))).await.expect("unable to get users");
        assert_eq!(find_result_1.total, 3);
        assert_eq!(find_result_1.users.get(0).unwrap().email, new_user.email);
        assert_eq!(find_result_1.users.get(1).unwrap().email, new_user2.email);
        assert_eq!(find_result_1.users.get(2).unwrap().email, new_user3.email);

        let find_result_2 = repo.find(UserRepoFindOpts{
            limit: 50, offset: 2,
            id: 0, email: String::from(""),
        }).await.expect("unable to get users 2");
        assert_eq!(find_result_2.total, 3);
        assert_eq!(find_result_2.users.len(), 1);
        assert_eq!(find_result_2.users.get(0).unwrap().email, new_user3.email);

        let find_result_3 = repo.find(
            UserRepoFindOpts::new(0, new_user2.email.clone())
        ).await.expect("unable to get users 3");
        assert_eq!(find_result_3.total, 1);
        assert_eq!(find_result_3.users.len(), 1);
        assert_eq!(find_result_3.users.get(0).unwrap().email, new_user2.email);
    }

    #[actix_rt::test]
    #[serial]
    async fn update_success() {
        let repo = init().await;
        let new_user = create_test_user(&repo, String::from("nick@mail.com")).await;

        let old_first_name = new_user.first_name.clone();
        let old_last_name = new_user.last_name.clone();

        let new_first_name = String::from("new first name");
        let new_last_name = String::from("new last name");

        let usr_update = users::update_user{
            first_name: new_first_name.clone(),
            last_name: new_last_name.clone(),
        };

        let existing_user = repo.update(new_user.id, usr_update).await.expect("unable to update user");

        // ID check
        assert_eq!(new_user.id, existing_user.id);

        // First Name
        assert_eq!(existing_user.first_name.clone(), new_first_name);
        assert_ne!(existing_user.first_name.clone(), old_first_name);

        // Last Name
        assert_eq!(existing_user.last_name.clone(), new_last_name);
        assert_ne!(existing_user.last_name.clone(), old_last_name);
    }

    #[actix_rt::test]
    #[serial]
    async fn update_fail_not_found() {
        let repo = init().await;

        let usr_update = users::update_user{
            first_name: String::from("new first name"),
            last_name: String::from("new last name"),
        };

        let err = repo.update(999999, usr_update).await.expect_err("able to update user not found");
        assert_eq!(err.to_string(), sqlx::Error::RowNotFound.to_string());
    }
}

#[derive(Clone)]
pub struct UserRepo {
    pool: Pool<Postgres>,
}

#[derive(Debug)]
pub struct UserRepoFindOpts {
    pub limit: i64,
    pub offset: i64,
    pub id: i64,
    pub email: String,
}

impl UserRepoFindOpts {
    pub fn new(id: i64, email: String) -> Self {
        UserRepoFindOpts{
            limit: 25,
            offset: 0,
            id: id,
            email: email,
        }
    }
}

#[derive(Serialize)]
pub struct UserRepoFindResult {
    users: Vec<users::user>,
    total: i64,
}

impl UserRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        UserRepo{pool: pool}
    }

    pub async fn create(&self, new_user: users::create_user) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.create_tx(&mut tx, new_user).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn create_tx(&self, tx: &mut Transaction<'static, Postgres>, new_user: users::create_user) -> Result<users::user, Error> {
        if new_user.password == String::from("") {
            return Err(sqlx::Error::InvalidArgument(String::from("password required")));
        }

        if new_user.password != new_user.password_confirm {
            return Err(sqlx::Error::InvalidArgument(String::from("password must match password confirm")));
        }

        let row = sqlx::query_as!(users::user,
            "INSERT INTO users (first_name, last_name, email, password)
            VALUES ($1, $2, $3, $4) RETURNING *"
                ,new_user.first_name.clone()
                ,new_user.last_name.clone()
                ,new_user.email.clone()
                ,hash_password(new_user.password.clone()).unwrap())
            .fetch_one(&mut **tx).await?;
        
        Ok(row)
    }

    pub async fn get(&self, id: i64) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.get_tx(&mut tx, id).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn get_tx(&self, tx: &mut Transaction<'static, Postgres>, id: i64) -> Result<users::user, Error> {
        if id < 1 {
            return Err(sqlx::Error::RowNotFound)
        }
        
        let row = sqlx::query_as!(users::user,
            "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&mut **tx).await?;

        Ok(row)
    }

    pub async fn find_by_email(&self, email: String) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.find_by_email_tx(&mut tx, email).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn find_by_email_tx(&self, tx: &mut Transaction<'static, Postgres>, email: String) -> Result<users::user, Error> {
        let row = sqlx::query_as!(users::user, 
                "SELECT * FROM users WHERE email = $1", email)
                .fetch_one(&mut **tx).await?;

        Ok(row)
    }

    pub async fn find(&self, mut opts: UserRepoFindOpts) -> Result<UserRepoFindResult, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.find_tx(&mut tx, opts).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn find_tx(&self, tx: &mut Transaction<'static, Postgres>, mut opts: UserRepoFindOpts) -> Result<UserRepoFindResult, Error> {
        if opts.limit < 1 {
            opts.limit = 25
        }
        
        if opts.limit > 500 {
            opts.limit = 500
        }

        let rows = sqlx::query_as!(users::user, r#"
            SELECT 
                * 
            
            FROM users 

            WHERE (users.id = $3 OR $3 = 0)
            AND (users.email = $4 OR $4 = '')
            
            ORDER BY users.id
            
            LIMIT $1 OFFSET $2
        "#, opts.limit, opts.offset, opts.id, opts.email)
            .fetch_all(&mut **tx)
            .await?;

        let count = sqlx::query_scalar!(r#"
            SELECT 
                COUNT(*) 

            FROM users

            WHERE (users.id = $1 OR $1 = 0)
            AND (users.email = $2 OR $2 = '')
        "#, opts.id, opts.email).fetch_one(&mut **tx).await?;

        let result:UserRepoFindResult = UserRepoFindResult {
            users: rows,
            total: count.unwrap_or_default(),
        };
        return Ok(result)
    }

    pub async fn update(&self, id: i64, usr: users::update_user) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.update_tx(&mut tx, id, usr).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn update_tx(&self, tx: &mut Transaction<'static, Postgres>, id: i64, usr: users::update_user) -> Result<users::user, Error> {
        let res = sqlx::query!(r#"
            UPDATE users SET
                first_name = $1
                ,last_name = $2
            WHERE id = $3
        "#, usr.first_name, usr.last_name, id.clone())
            .execute(&mut **tx).await?;

        self.get_tx(tx, id).await
    }

    pub async fn update_from(&self, usr: users::user) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.update_from_tx(&mut tx, usr).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn update_from_tx(&self, tx: &mut Transaction<'static, Postgres>, usr: users::user) -> Result<users::user, Error> {
        let res = sqlx::query!(r#"
            UPDATE users SET
                first_name = $1
                ,last_name = $2
            WHERE id = $3
        "#, usr.first_name, usr.last_name, usr.id.clone()).execute(&mut **tx).await?;

        self.get_tx(tx, usr.id).await
    }

    pub async fn update_password(&self, id: i64, password: String, password_confirm: String) -> Result<users::user, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.update_password_tx(&mut tx, id, password, password_confirm).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        let _ = tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn update_password_tx(&self, tx: &mut Transaction<'static, Postgres>, 
        id: i64, password: String, password_confirm: String) -> Result<users::user, Error> {
        if password == String::from("") {
            return Err(sqlx::Error::InvalidArgument(String::from("password required")));
        }

        if password != password_confirm {
            return Err(sqlx::Error::InvalidArgument(String::from("password must match password confirm")));
        }

        let res = sqlx::query!(r#"
            UPDATE users SET
                password = $1
            WHERE id = $2
        "#, hash_password(password).unwrap(), id.clone()).execute(&mut **tx).await?;

        self.get_tx(tx, id).await
    }
}