use crate::Puzzle;

#[get("/puzzle/<date>/<order>")]
fn get_puzzle(date: &str, order: i32) -> Puzzle {
    //TODO: fetch puzzle from DB
}

#[get("/user/<user>/<password>")]
fn get_user(user: &str, password: &str) {
    //TODO: insert
}

#[post("/user/<user>/<password>")]
fn create_user(user: &str, password: &str) {
    //TODO: insert
}

#[delete("/user/<user>/<password>")]
fn delete_user(user: &str, password: &str) {
    //TODO: insert
}

#[put("/user/<user>/<password>")]
fn update_user(user: &str, password: &str) {
    //TODO: insert
}
