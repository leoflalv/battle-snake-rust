use rocket::serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    id: String,
    ruleset: HashMap<String, Value>,
    timeout: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: u32,
    pub width: u32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up(Coord),
    Down(Coord),
    Right(Coord),
    Left(Coord),
}

impl Direction {
    pub fn as_str(&self) -> &str {
        match self {
            Direction::Up(_) => "up",
            Direction::Down(_) => "down",
            Direction::Right(_) => "right",
            Direction::Left(_) => "left",
        }
    }

    pub fn get_coord(&self) -> &Coord {
        match self {
            Direction::Up(coord)
            | Direction::Down(coord)
            | Direction::Right(coord)
            | Direction::Left(coord) => coord,
        }
    }
}
