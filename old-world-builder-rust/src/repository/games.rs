use crate::types::stats;
use sqlx::error::Error;

#[derive(Debug)]
pub struct GameRepoFindOpts {
    pub limit: i64,
    pub offset: i64,
    pub id: i64,
}

impl GameRepoFindOpts {
    pub fn new(id: i64, email: String) -> Self {
        GameRepoFindOpts{
            limit: 25,
            offset: 0,
            id,
        }
    }
}

#[derive(Serialize)]
pub struct GameRepoFindResult {
    games: Vec<games::Game>,
    total: i64,
}

#[derive(Clone)]
pub struct GamesRepo {
    pool: Pool<Postgres>,
}

impl GamesRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        GamesRepo{pool}
    }

    pub async fn create(&self, name: String) -> Result<games::Game, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.create_tx(&mut tx, name).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn create_tx(&self, tx: &mut Transaction<'static, Postgres>, name: String) -> Result<games::Game, Error> {
        let row = sqlx::query_as!(games::Game, "
            INSERT INTO games (name) VALUES ($1) RETURNING *
        ", name).fetch_one(&mut **tx).await?;

        Ok(row)
    }

    pub async fn get(&self, id: i64) -> Result<games::Game, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.get_tx(&mut tx, id).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn get_tx(&self, tx: &mut Transaction<'static, Postgres>, id: i64) -> Result<games::Game, Error> {
        if id < 1 {
            return Err(sqlx::Error::RowNotFound)
        }
        
        let row = sqlx::query_as!(games::Game,
            "SELECT * FROM games WHERE id = $1", id)
            .fetch_one(&mut **tx).await?;

        Ok(row)
    }

    pub async fn find(&self, opts: GameRepoFindOpts) -> Result<GameRepoFindResult, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.find_tx(&mut tx, opts).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn find_tx(&self, tx: &mut Transaction<'static, Postgres>, mut opts: GameRepoFindOpts) -> Result<GameRepoFindResult, Error> {
        if opts.limit < 1 {
            opts.limit = 25
        }
        
        if opts.limit > 500 {
            opts.limit = 500
        }

        let rows = sqlx::query_as!(games::Game, r#"
            SELECT 
                * 
            
            FROM games

            WHERE (games.id = $3 OR $3 = 0)
            
            ORDER BY games.id
            
            LIMIT $1 OFFSET $2
        "#, opts.limit, opts.offset, opts.id, opts.email)
            .fetch_all(&mut **tx)
            .await?;

        let count = sqlx::query_scalar!(r#"
            SELECT 
                COUNT(*) 

            FROM games

            WHERE (games.id = $1 OR $1 = 0)
        "#, opts.id).fetch_one(&mut **tx).await?;

        let result:GameRepoFindResult = GameRepoFindResult {
            games: rows,
            total: count.unwrap_or_default(),
        };

        Ok(result)
    }

    pub async fn update(&self, id: i64, name: String) -> Result<games::Game, Error> {
        let mut tx = self.pool.begin().await?;

        let res = self.update_tx(&mut tx, id, name).await;
        if res.is_err() {
            tx.rollback().await?;
            return Err(res.err().unwrap());
        }

        tx.commit().await?;

        Ok(res.unwrap())
    }

    pub async fn update_tx(&self, tx: &mut Transaction<'static, Postgres>, id: i64, name: String) -> Result<games::Game, Error> {
        sqlx::query!(r#"
            UPDATE users SET
                name = $1
            WHERE id = $2
        "#, name, id.clone())
            .execute(&mut **tx).await?;

        self.get_tx(tx, id).await
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use serial_test::serial;
    use sqlx::{postgres::PgPoolOptions, Executor};

    use super::*;

    async fn init() -> GameRepo {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL env is required to run tests");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url.as_str())
            .await.expect("unable to connect to database with url");

        pool.execute("TRUNCATE games").await.expect("unable to truncate games");

        GameRepo::new(pool)
    }

    async fn create_test_game(repo: &GameRepo, name: String) -> games::Game {
        let new_game = repo.create(name).await.expect("unable to create a game");

        assert_ne!(new_game.id, 0);
        assert_eq!(new_game.name, name);
        
        new_game
    }

    #[actix_rt::test]
    #[serial] // so we only run them 1 at a time
    async fn CreateGame_success() {
        let repo = init().await;
        let new_game = create_test_game(&repo, String::from("test game")).await;

        let err: Error = repo.create(new_game.name).await.expect_err("creating the same game should fail");

        assert_eq!(err.to_string(), "error returned from database: duplicate key value violates unique constraint \"games_email_key\"");
    }
}