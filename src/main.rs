#[macro_use]
extern crate rocket;

use log::info;
use rocket::fairing::AdHoc;
use std::env;

use battle_snake_rust::handlers::{handle_end, handle_index, handle_move, handle_start};

// API and Response Objects
// See https://docs.battlesnake.com/api

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    env_logger::init();

    // Lots of web hosting services expect you to bind to the port specified by the `PORT`
    // environment variable. However, Rocket looks at the `ROCKET_PORT` environment variable.
    // If we find a value for `PORT`, we set `ROCKET_PORT` to that value.
    if let Ok(port) = env::var("PORT") {
        env::set_var("ROCKET_PORT", &port);
    }

    // We default to 'info' level logging. But if the `RUST_LOG` environment variable is set,
    // we keep that value instead.
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    info!("Starting Battlesnake Server...");

    let rocket = rocket::build()
        .attach(AdHoc::on_response("Server ID Middleware", |_, res| {
            Box::pin(async move {
                res.set_raw_header("Server", "battlesnake/github/starter-snake-rust");
            })
        }))
        .mount(
            "/",
            routes![handle_index, handle_start, handle_move, handle_end],
        );

    Ok(rocket.into())
}
