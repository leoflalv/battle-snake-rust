use std::collections::HashSet;

use crate::domain::{Battlesnake, Board, Coord, Direction};

pub fn get_valid_moves(board: &Board, you: &Battlesnake) -> HashSet<Direction> {
    let head_x = you.head.x;
    let head_y = you.head.y;

    let movement_array = vec![
        Direction::Up(Coord::new(head_x, head_y + 1)),
        Direction::Right(Coord::new(head_x + 1, head_y)),
        Direction::Down(Coord::new(head_x, head_y - 1)),
        Direction::Left(Coord::new(head_x - 1, head_y)),
    ]; // All the available x, y moves

    movement_array
        .into_iter()
        .filter(|dir| is_valid_move(board, you, dir.get_coord()))
        .collect()
}

fn is_valid_move(board: &Board, you: &Battlesnake, next_movement: &Coord) -> bool {
    is_inside_bounds(board, next_movement)
        && is_not_own_body(you, next_movement)
        && is_not_an_enemy(board, next_movement)
}

fn is_inside_bounds(board: &Board, next_movement: &Coord) -> bool {
    next_movement.x >= 0
        && (next_movement.x as u32) < board.width
        && next_movement.y >= 0
        && (next_movement.y as u32) < (board.height)
}

fn is_not_own_body(you: &Battlesnake, next_movement: &Coord) -> bool {
    you.body[..you.body.len() - 1]
        .iter()
        .position(|x| x == next_movement)
        .is_none()
}

fn is_not_an_enemy(board: &Board, next_movement: &Coord) -> bool {
    for enemy in &board.snakes {
        let is_an_enemy = enemy.body[..enemy.body.len() - 1]
            .iter()
            .position(|x| x == next_movement)
            .is_some();

        if is_an_enemy {
            return false;
        }
    }

    true
}

fn is_not_a_hazard(board: &Board, next_movement: &Coord) -> bool {
    board
        .hazards
        .iter()
        .position(|x| x == next_movement)
        .is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        body: Vec<Coord>,
        head: Coord,
        next_move: Coord,
        snakes: Vec<Battlesnake>,
        hazards: Vec<Coord>,
    }

    impl TestCase {
        fn new(
            body: Vec<Coord>,
            head: Coord,
            next_move: Coord,
            snakes: Vec<Battlesnake>,
            hazards: Vec<Coord>,
        ) -> Self {
            TestCase {
                body,
                head,
                next_move,
                snakes,
                hazards,
            }
        }
    }

    fn setup_game(
        body: &Vec<Coord>,
        head: Coord,
        snakes: Vec<Battlesnake>,
        hazards: &Vec<Coord>,
    ) -> (Board, Battlesnake) {
        let board = Board {
            height: 5,
            width: 5,
            food: vec![],
            snakes,
            hazards: hazards.to_vec(),
        };

        let battlesnake = Battlesnake {
            id: String::from("test"),
            name: String::from("test"),
            health: 20,
            length: (body.len() + 1) as i32,
            body: body.to_vec(),
            head,
            latency: String::from("test"),
            shout: None,
        };

        (board, battlesnake)
    }

    mod is_inside_bounds_tests {
        use super::*;

        #[test]
        fn not_inside_bound() {
            let test_cases = vec![
                // Go left outside bounds
                TestCase::new(
                    vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)],
                    Coord::new(0, 0),
                    Coord::new(-1, 0),
                    vec![],
                    vec![],
                ),
                // Go top outside bounds
                TestCase::new(
                    vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)],
                    Coord::new(0, 0),
                    Coord::new(0, -1),
                    vec![],
                    vec![],
                ),
                // Go right outside bounds
                TestCase::new(
                    vec![Coord::new(4, 4), Coord::new(4, 3), Coord::new(4, 2)],
                    Coord::new(4, 4),
                    Coord::new(5, 4),
                    vec![],
                    vec![],
                ),
                // Go down outside bounds
                TestCase::new(
                    vec![Coord::new(4, 4), Coord::new(4, 3), Coord::new(4, 2)],
                    Coord::new(4, 4),
                    Coord::new(4, 5),
                    vec![],
                    vec![],
                ),
            ];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_inside_bounds(&board, &test_case.next_move);
                assert_eq!(false, is_valid);
            }
        }

        #[test]
        fn inside_bound() {
            let test_cases = vec![
                TestCase::new(
                    vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)],
                    Coord::new(0, 0),
                    Coord::new(0, 1),
                    vec![],
                    vec![],
                ),
                TestCase::new(
                    vec![Coord::new(4, 4), Coord::new(4, 3), Coord::new(4, 2)],
                    Coord::new(4, 4),
                    Coord::new(3, 4),
                    vec![],
                    vec![],
                ),
            ];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_inside_bounds(&board, &test_case.next_move);
                assert_eq!(true, is_valid);
            }
        }
    }

    mod is_not_own_body_tests {
        use super::*;

        #[test]
        fn crash_with_body() {
            let test_cases = vec![
                TestCase::new(
                    vec![Coord::new(3, 0), Coord::new(2, 0), Coord::new(1, 0)],
                    Coord::new(3, 0),
                    Coord::new(2, 0),
                    vec![],
                    vec![],
                ),
                TestCase::new(
                    vec![
                        Coord::new(2, 2),
                        Coord::new(3, 2),
                        Coord::new(3, 1),
                        Coord::new(2, 1),
                        Coord::new(1, 1),
                    ],
                    Coord::new(2, 2),
                    Coord::new(2, 1),
                    vec![],
                    vec![],
                ),
            ];

            for test_case in test_cases.iter() {
                let (_, battlesnake) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_own_body(&battlesnake, &test_case.next_move);
                assert_eq!(false, is_valid);
            }
        }

        #[test]
        fn not_crash_with_body() {
            let test_cases = vec![TestCase::new(
                vec![
                    Coord::new(1, 2),
                    Coord::new(2, 2),
                    Coord::new(3, 2),
                    Coord::new(3, 1),
                    Coord::new(2, 1),
                    Coord::new(1, 1),
                ],
                Coord::new(1, 2),
                Coord::new(1, 1),
                vec![],
                vec![],
            )];

            for test_case in test_cases.iter() {
                let (_, battlesnake) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_own_body(&battlesnake, &test_case.next_move);
                assert_eq!(true, is_valid);
            }
        }
    }

    mod is_not_an_enemy_test {
        use super::*;

        #[test]
        fn crash_with_enemy() {
            let test_cases = vec![TestCase::new(
                vec![Coord::new(1, 2), Coord::new(2, 2)],
                Coord::new(1, 2),
                Coord::new(1, 1),
                vec![Battlesnake {
                    id: String::from("enemy_id"),
                    name: String::from("enemy"),
                    health: 10,
                    body: vec![Coord::new(1, 1), Coord::new(2, 1)],
                    head: Coord::new(1, 1),
                    length: 2,
                    latency: String::from(""),
                    shout: None,
                }],
                vec![],
            )];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_an_enemy(&board, &test_case.next_move);
                assert_eq!(false, is_valid);
            }
        }

        #[test]
        fn not_crash_with_enemy() {
            let test_cases = vec![TestCase::new(
                vec![Coord::new(1, 2), Coord::new(2, 2)],
                Coord::new(1, 2),
                Coord::new(1, 1),
                vec![Battlesnake {
                    id: String::from("enemy_id"),
                    name: String::from("enemy"),
                    health: 10,
                    body: vec![Coord::new(0, 1), Coord::new(1, 1)],
                    head: Coord::new(0, 1),
                    length: 2,
                    latency: String::from(""),
                    shout: None,
                }],
                vec![],
            )];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_an_enemy(&board, &test_case.next_move);
                assert_eq!(true, is_valid);
            }
        }
    }

    mod is_not_a_hazard_test {
        use super::*;

        #[test]
        fn crash_with_hazard() {
            let test_cases = vec![TestCase::new(
                vec![Coord::new(1, 2), Coord::new(2, 2)],
                Coord::new(1, 2),
                Coord::new(1, 1),
                vec![],
                vec![Coord::new(1, 1)],
            )];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_a_hazard(&board, &test_case.next_move);
                assert_eq!(false, is_valid);
            }
        }

        #[test]
        fn no_crash_with_hazard() {
            let test_cases = vec![TestCase::new(
                vec![Coord::new(1, 2), Coord::new(2, 2)],
                Coord::new(1, 2),
                Coord::new(1, 3),
                vec![],
                vec![Coord::new(1, 1)],
            )];

            for test_case in test_cases.iter() {
                let (board, _) = setup_game(
                    &test_case.body,
                    test_case.head,
                    test_case.snakes.clone(),
                    &test_case.hazards,
                );

                let is_valid = is_not_a_hazard(&board, &test_case.next_move);
                assert_eq!(true, is_valid);
            }
        }
    }

    mod get_valid_moves_tests {
        use super::*;

        #[test]
        fn should_get_moves() {
            let body = vec![
                Coord::new(2, 2),
                Coord::new(3, 2),
                Coord::new(3, 1),
                Coord::new(2, 1),
                Coord::new(1, 1),
            ];
            let head = Coord::new(2, 2);
            let (board, battlesnake) = setup_game(&body, head, vec![], &vec![]);
            let valid_moves = get_valid_moves(&board, &battlesnake);
            let correct_answer: HashSet<Direction> = vec![
                Direction::Left(Coord::new(1, 2)),
                Direction::Up(Coord::new(2, 3)),
            ]
            .into_iter()
            .collect();

            assert_eq!(valid_moves, correct_answer);
        }
    }
}
