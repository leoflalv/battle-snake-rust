use std::{collections::VecDeque, usize};

use crate::domain::{Battlesnake, Board, Coord, Direction};

use super::move_validator::get_valid_moves;

#[derive(Debug, Clone)]
struct State {
    body: Vec<Coord>,
    head: Coord,
}

impl State {
    fn new(body: Vec<Coord>) -> Self {
        State {
            body: body.clone(),
            head: body[0],
        }
    }
}

#[derive(Debug, Clone)]
struct Step {
    dir: Direction,
    state: State,
}

impl Step {
    fn new(dir: Direction, state: State) -> Self {
        Step { dir, state }
    }

    // Move the battlesnake
    fn walk(&self, next_step: Direction) -> Self {
        let mut new_body: Vec<Coord> = vec![*next_step.get_coord()];
        new_body.extend_from_slice(&self.state.body);
        new_body.pop();

        Step::new(next_step, State::new(new_body))
    }
}

pub fn get_next_step(board: &Board, you: &Battlesnake, options: &Vec<Direction>) -> Direction {
    // Get the initial values for the queue
    let mut queue: VecDeque<Step> = options
        .iter()
        .map(|d| Step::new(*d, State::new(you.body.clone())).walk(*d))
        .collect();

    // Initializing path matrix, that it's gonna be used as
    // visited, and to track the path.
    let mut path: Vec<Vec<Option<Coord>>> =
        vec![vec![None; board.width as usize]; board.height as usize];
    path[you.head.y as usize][you.head.x as usize] = Some(Coord::new(-1, -1));

    // Mark as visited the initial valid movements.
    for step in queue.iter() {
        let Coord { x: new_x, y: new_y } = step.dir.get_coord();
        let (new_x, new_y) = (*new_x as usize, *new_y as usize);

        if path[new_y][new_x].is_none() {
            path[new_y][new_x] = Some(Coord::new(you.head.x, you.head.y));
        }
    }

    // To store the coord of the found food.
    let mut last_coord = *options[0].get_coord();

    // Bfs
    while let Some(step) = queue.pop_front() {
        let Coord { x, y } = step.dir.get_coord();
        let (x, y) = (*x as usize, *y as usize);

        let new_step = Coord::new(x as i32, y as i32);

        if is_food(&new_step, &board.food) {
            last_coord = new_step;
            break;
        }

        let new_snake = Battlesnake {
            head: step.state.head,
            body: step.state.body.clone(),
            ..you.clone()
        };

        // Get the new possible steps.
        let new_steps: Vec<_> = get_valid_moves(board, &new_snake)
            .into_iter()
            .map(|dir| step.walk(dir))
            .collect();

        // Interate the new possible steps and add then if the
        // node it's not visited.
        for step in new_steps {
            let Coord { x: new_x, y: new_y } = step.dir.get_coord();
            let (new_x, new_y) = (*new_x as usize, *new_y as usize);

            if path[new_y][new_x].is_none() {
                path[new_y][new_x] = Some(Coord::new(x as i32, y as i32));
                queue.push_back(step);
            }
        }
    }

    // Get the coord of the initial option tracking the path
    // of the found food in the path matrix.
    let dir_coord = track_path(&path, &last_coord);

    // Search for which initial option has the same coord as the
    // tracked one.
    *options
        .into_iter()
        .find(|dir| *dir.get_coord() == dir_coord)
        .unwrap()
}

fn is_food(position: &Coord, food: &[Coord]) -> bool {
    food.contains(position)
}

fn track_path(path: &Vec<Vec<Option<Coord>>>, last_coord: &Coord) -> Coord {
    let mut prev_coord = *last_coord;
    let mut current_coord = path[last_coord.y as usize][last_coord.x as usize].unwrap();

    loop {
        let Coord { x, y } = current_coord;
        let (x, y) = (x as usize, y as usize);

        if path[y][x] == Some(Coord::new(-1, -1)) {
            return prev_coord;
        }

        prev_coord = current_coord;
        current_coord = path[y][x].unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::move_validator;

    use super::*;

    #[test]
    fn get_correct_next_step() {
        let enemy = Battlesnake {
            id: String::from("enemy"),
            name: String::from("enemy"),
            health: 20,
            length: 4,
            body: vec![
                Coord::new(6, 3),
                Coord::new(5, 3),
                Coord::new(4, 3),
                Coord::new(3, 3),
                Coord::new(2, 3),
            ],
            head: Coord::new(6, 3),
            latency: String::from("test"),
            shout: None,
        };

        let board = Board {
            height: 10,
            width: 10,
            food: vec![Coord::new(5, 2)],
            snakes: vec![enemy],
            hazards: vec![],
        };

        let battlesnake = Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 20,
            length: 4,
            body: vec![
                Coord::new(5, 4),
                Coord::new(5, 5),
                Coord::new(4, 5),
                Coord::new(4, 5),
            ],
            head: Coord::new(5, 4),
            latency: String::from("test"),
            shout: None,
        };

        let valid_moves = move_validator::get_valid_moves(&board, &battlesnake);
        let safe_moves = valid_moves.into_iter().collect::<Vec<_>>();

        let next_step = get_next_step(&board, &battlesnake, &safe_moves);

        assert_eq!(next_step, Direction::Right(Coord::new(6, 4)))
    }
}
