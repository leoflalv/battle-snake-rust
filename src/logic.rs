mod food_finder;
mod move_refinator;
mod move_validator;

use log::info;
use serde_json::{json, Value};

use crate::{
    domain::{Battlesnake, Board, Game},
    logic::food_finder::get_next_step,
};

// info is called when you create your Battlesnake on play.battlesnake.com
// and controls your Battlesnake's appearance
// TIP: If you open your Battlesnake URL in a browser you should see this data
pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "leonardo fleitas alvarez",
        "color": "#222a34", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

// start is called when your Battlesnake begins a game
pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

// end is called when your Battlesnake finishes a game
pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

// move is called on every turn and returns your next move
// Valid moves are "up", "down", "left", or "right"
// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &i32, board: &Board, you: &Battlesnake) -> Value {
    let valid_moves = move_validator::get_valid_moves(board, you);

    if valid_moves.is_empty() {
        return json!({ "move": "up" });
    }

    // Are there any safe moves left?
    let safe_moves = valid_moves.into_iter().collect::<Vec<_>>();

    let recommended = move_refinator::recommend_move(&safe_moves, you, board);

    if let Some(direction) = recommended {
        info!("MOVE {}: {}", turn, direction.as_str());
        return json!({ "move": direction.as_str() });
    }

    let refined_moves = move_refinator::refined_movements(&safe_moves, &board, &you);

    let options = if refined_moves.len() > 0 {
        refined_moves
    } else {
        safe_moves
    };

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;
    let next_move = get_next_step(board, you, &options);
    let chosen = next_move.as_str();

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
