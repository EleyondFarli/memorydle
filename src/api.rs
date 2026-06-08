use crate::{connect_to_db, Puzzle};
use chrono::Local;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::State;
use sqlx::{Pool, Sqlite};

#[get("/puzzle/today")]
pub async fn get_puzzle(db: &State<Pool<Sqlite>>) -> Json<Puzzle> {
    let now = Local::now().date_naive();
    let daily_puzzle: Option<Puzzle> = dbg!(
        sqlx::query_as("SELECT * FROM puzzles WHERE puzzle_date = $1")
            .bind(now)
            .fetch_optional(db.inner())
            .await
            .unwrap());

    Json(daily_puzzle.unwrap())
}

#[get("/user/<user>/<password>")]
pub fn get_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[post("/user/<user>/<password>")]
pub fn create_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[delete("/user/<user>/<password>")]
pub fn delete_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[put("/user/<user>/<password>")]
pub fn update_user(user: &str, password: &str) {
    //TODO: SQL query
}

#[catch(default)]
pub fn error_page(req: &Request) -> status::BadRequest<String> {
    status::BadRequest(format!("Sorry, {} does not exist.", req.uri()))
}
