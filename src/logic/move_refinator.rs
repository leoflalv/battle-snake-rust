use crate::domain::{Battlesnake, Board, Coord, Direction};

pub fn recommend_move<'a>(
    options: &'a Vec<Direction>,
    you: &Battlesnake,
    board: &Board,
) -> Option<&'a Direction> {
    let movement_array = vec![(0, 1), (1, 0), (0, -1), (-1, 0)]; // All the available x, y moves

    for option in options {
        let Coord { x, y } = option.get_coord();

        for movement in movement_array.iter() {
            let possible_enemy = Coord::new(movement.0 + x, movement.1 + y);
            let enemy = get_snake(&possible_enemy, board);

            if let Some(snake) = enemy {
                if you.length > snake.length {
                    return Some(option);
                }
            }
        }
    }

    None
}

pub fn refined_movements(options: &Vec<Direction>, board: &Board) -> Vec<Direction> {
    options
        .iter()
        .filter(|opt| avoid_loser_hits(opt.get_coord(), board))
        .cloned()
        .collect()
}

fn avoid_loser_hits(next_movement: &Coord, board: &Board) -> bool {
    let movement_array = vec![(0, 1), (1, 0), (0, -1), (-1, 0)]; // All the available x, y moves

    for movement in movement_array {
        let possible_enemy = Coord::new(movement.0 + next_movement.x, movement.1 + next_movement.y);
        let enemy = get_snake(&possible_enemy, board);

        if enemy.is_some() {
            return false;
        }
    }

    true
}

fn get_snake<'a>(point: &Coord, board: &'a Board) -> Option<&'a Battlesnake> {
    for enemy in &board.snakes {
        let is_an_enemy = enemy.body[..enemy.body.len() - 1]
            .iter()
            .position(|x| x == point)
            .is_some();

        if is_an_enemy {
            return Some(enemy);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_mock_data(
        enemy_body: &Vec<Coord>,
        your_body: &Vec<Coord>,
    ) -> (Battlesnake, Board, Battlesnake) {
        let enemy = Battlesnake {
            id: String::from("enemy"),
            name: String::from("enemy"),
            health: 20,
            length: enemy_body.len() as i32,
            body: enemy_body.to_vec(),
            head: enemy_body[0],
            latency: String::from("test"),
            shout: None,
        };

        let board = Board {
            height: 10,
            width: 10,
            food: vec![Coord::new(10, 10)],
            snakes: vec![enemy.clone()],
            hazards: vec![],
        };

        let battlesnake = Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 20,
            length: your_body.len() as i32,
            body: your_body.to_vec(),
            head: your_body[0],
            latency: String::from("test"),
            shout: None,
        };

        (enemy, board, battlesnake)
    }

    #[test]
    fn found_possible_hit() {
        let (_, board, _) = get_mock_data(
            &vec![Coord::new(6, 3), Coord::new(5, 3), Coord::new(4, 3)],
            &vec![Coord::new(8, 3), Coord::new(9, 3)],
        );

        let next_step = Coord::new(7, 3);

        let response = avoid_loser_hits(&next_step, &board);

        assert_eq!(response, false)
    }

    #[test]
    fn not_possible_hit() {
        let (_, board, _) = get_mock_data(
            &vec![Coord::new(5, 3), Coord::new(4, 3), Coord::new(3, 3)],
            &vec![Coord::new(8, 3), Coord::new(9, 3)],
        );

        let next_step = Coord::new(7, 3);

        let response = avoid_loser_hits(&next_step, &board);

        assert_eq!(response, true);
    }

    #[test]
    fn recommend() {
        let (_, board, you) = get_mock_data(
            &vec![Coord::new(5, 3), Coord::new(4, 3)],
            &vec![Coord::new(7, 3), Coord::new(8, 3), Coord::new(9, 3)],
        );

        let options = vec![
            Direction::Up(Coord::new(7, 4)),
            Direction::Down(Coord::new(7, 2)),
            Direction::Left(Coord::new(6, 3)),
        ];

        let response = recommend_move(&options, &you, &board);

        assert_eq!(response, Some(&Direction::Left(Coord::new(6, 3))));
    }

    #[test]
    fn not_recommend() {
        let (_, board, you) = get_mock_data(
            &vec![Coord::new(6, 3), Coord::new(5, 3), Coord::new(4, 3)],
            &vec![Coord::new(8, 3), Coord::new(9, 3)],
        );

        let options = vec![
            Direction::Up(Coord::new(8, 4)),
            Direction::Down(Coord::new(8, 2)),
            Direction::Left(Coord::new(7, 2)),
        ];

        let response = recommend_move(&options, &you, &board);

        assert_eq!(response, None);
    }
}
