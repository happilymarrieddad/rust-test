// use sqlx::{postgres::PgPoolOptions, Executor};
// use oldworldbuilderrust::repository::users::UserRepo;

// use super::*;

// #[actix_rt::test]
// async fn create_user() {
//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect("postgres://postgres:postgres@localhost:5432/oldworld-test?connect_timeout=180&sslmode=disable")
//         .await.unwrap();

//     pool.execute("TRUNCATE users").await.expect("unable to truncate users");

//     let repo = UserRepo::new(pool);

//     let new_user = repo.create(users::create_user{
//         first_name: String::from("Nick"),
//         last_name: String::from("Kotenberg"),
//         email: String::from("nick@mail.com"),
//         password: String::from("1234"),
//         password_confirm: String::from("1234"),
//     }).await.unwrap();

//     assert_ne!(new_user.id, 0);
//     assert_eq!(new_user.email, "nick@mail.com");

//     let err: Error = repo.create(users::create_user{
//         first_name: String::from("Nick"),
//         last_name: String::from("Kotenberg"),
//         email: String::from("nick@mail.com"),
//         password: String::from("1234"),
//         password_confirm: String::from("1234"),
//     }).await.unwrap_err();

//     assert_eq!(err.to_string(), "error returned from database: duplicate key value violates unique constraint \"users_email_key\"");
// }