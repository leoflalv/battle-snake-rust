use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::Value;

use crate::domain::GameState;
use crate::logic;

#[get("/")]
pub fn handle_index() -> Json<Value> {
    Json(logic::info())
}

#[post("/start", format = "json", data = "<start_req>")]
pub fn handle_start(start_req: Json<GameState>) -> Status {
    logic::start(
        &start_req.game,
        &start_req.turn,
        &start_req.board,
        &start_req.you,
    );

    Status::Ok
}

#[post("/move", format = "json", data = "<move_req>")]
pub fn handle_move(move_req: Json<GameState>) -> Json<Value> {
    let response = logic::get_move(
        &move_req.game,
        &move_req.turn,
        &move_req.board,
        &move_req.you,
    );

    Json(response)
}

#[post("/end", format = "json", data = "<end_req>")]
pub fn handle_end(end_req: Json<GameState>) -> Status {
    logic::end(&end_req.game, &end_req.turn, &end_req.board, &end_req.you);

    Status::Ok
}
