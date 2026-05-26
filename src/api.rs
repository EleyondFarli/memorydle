use crate::Puzzle;

#[get("/puzzle/<date>")]
fn get_puzzle(date: &str) -> () {
    //TODO: fetch puzzle from DB
}

#[get("/user/<user>/<password>")]
fn get_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[post("/user/<user>/<password>")]
fn create_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[delete("/user/<user>/<password>")]
fn delete_user(user: &str, password: &str) -> () {
    //TODO: SQL query
}

#[put("/user/<user>/<password>")]
fn update_user(user: &str, password: &str) {
    //TODO: SQL query
}
