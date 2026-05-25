mod api;

#[macro_use] extern crate rocket;

use sqlx::{Pool, Sqlite};
use sqlx::types::chrono;

// TODO: create multiple modules to better strutcure the project
#[derive(sqlx::FromRow, Debug)]
struct Image {
    id: i32,
    puzzle_id: i32,
    image_path: String,
    display_order: i8,
}

#[derive(sqlx::FromRow, Debug)]
struct Country {
    id: i32,
    name: String,
}

#[derive(sqlx::FromRow, Debug)]
struct User {
    id: String,
    username: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
struct Puzzle {
    id: String,
    //TODO: Check if NaiveDate actually works here or I need to switch to DateTime
    puzzle_date: chrono::NaiveDate,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

async fn connect_to_db() -> Pool<Sqlite> {
    let db_url = "DATABASE_URL";
    let panic_message = &format!("{} must be set", db_url);

    let file_name = std::env::var(db_url).expect(panic_message);
    let options = sqlx::sqlite::SqliteConnectOptions::new().filename(file_name).create_if_missing(false);

    //TODO: Handle the bad connection instead of panicking
    sqlx::sqlite::SqlitePool::connect_with(options).await.unwrap()
}

async fn fetch_images(connection: Pool<Sqlite>) -> Vec<Image> {
    let images:Vec<Image> = sqlx::query_as("SELECT * FROM images").fetch_all(&connection).await.unwrap();
    println!("{:?}", images);
    images
}


#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let connection = connect_to_db().await;
    let _images = fetch_images(connection).await;

    rocket::build().mount("/", routes![index])
}
