mod db_logic;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    db_logic::create_connection().unwrap();
    rocket::build().mount("/", routes![index])
}