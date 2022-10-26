#[macro_use] extern crate rocket;
use std::io;

use rocket::tokio::task::spawn_blocking;

use diesel_lib::*;

#[get("/name?<name>")]
fn index(name: &str) -> String {
    format!("Hello {name}")
}

#[get("/user/get?<id>")]
fn take_user_by_id(id: i32) -> String {
    let user = get_user_id(id);
    println!("{:?}", &user);
    format!("Username: {}", user.login)
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| vec![20, 18, 19]).await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;


    Ok(vec)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .mount("/hello", routes![blocking_task])
        .mount("/api", routes![take_user_by_id])
        .launch()
        .await?;

    Ok(())
}
