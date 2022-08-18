use crate::direction::{get_roll_direction, Direction, GameCubeDirection};
use crate::face::{opposite_face, GameCubeFace};
use crate::move_pattern::*;
use crate::player::{win_position, GamePlayerIndex};
use crate::position::GameCubePosition;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;
use near_sdk::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameCube {
    pub player: GamePlayerIndex,
    pub position: GameCubePosition,
    pub direction: GameCubeDirection,
}

#[derive(
    Copy, Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize,
)]
#[serde(crate = "near_sdk::serde")]
pub enum GamePhase {
    Roll = 1,
    Hop = 2,
    End = 3,
}

/*
impl From<i8> for GamePhase {
    fn from(orig: i8) -> Self {
        match orig {
            1i8 => GamePhase::Roll,
            2i8 => GamePhase::Hop,
            3i8 => GamePhase::End,
            _ => panic!("Possible game phases in range 1-3! (requested: {})", orig),
        }
    }
}
*/

/// Deblockle Game struct
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Game {
    pub phase: GamePhase,
    pub active_player: GamePlayerIndex,
    pub board: Vec<GameCube>,
}

impl Game {
    fn cubes_for_player(&self, player: &GamePlayerIndex) -> Vec<GameCube> {
        self.board
            .clone()
            .into_iter()
            .filter(|cube| cube.player == *player)
            .collect()
    }

    fn check_winner(&self) -> Option<GamePlayerIndex> {
        let p1 = 1;
        let p2 = 2;

        if self.cubes_for_player(&p1).is_empty() {
            Some(p1)
        } else if self.cubes_for_player(&p2).is_empty() {
            Some(p2)
        } else {
            None
        }
    }

    /// Returns clone of the cube, therefore &game
    pub fn get_cube_at(&self, pos: GameCubePosition) -> Option<GameCube> {
        let board = self.board.clone();
        for cube in board {
            if cube.position == pos {
                return Some(cube);
            }
        }
        None
    }

    /// Returns reference to the cube, therefore &mut game
    fn take_cube_at(&mut self, pos: GameCubePosition) -> Option<&mut GameCube> {
        // let board = game.board.clone();
        let board = &mut self.board;
        for cube in board {
            if cube.position == pos {
                return Some(cube);
            }
        }
        None
    }

    fn remove_cube_at(&mut self, pos: GameCubePosition) {
        let index = self.board.iter().position(|cube| cube.position == pos);
        if let Some(..) = index {
            self.board.remove(index.unwrap());
        }
    }

    fn move_cube(
        &mut self,
        from: GameCubePosition,
        to: GameCubePosition,
        direction: GameCubeDirection,
    ) {
        let opt_cube = self.take_cube_at(from);
        if let Some(..) = opt_cube {
            let mut cube: &mut GameCube = opt_cube.unwrap();
            cube.position = to;
            cube.direction = direction;
        }
    }

    fn filter_blocked(&mut self, positions: Vec<GameCubePosition>) -> Vec<GameCubePosition> {
        positions
            .into_iter()
            .filter(|p| self.take_cube_at(*p).is_none())
            .collect()
    }

    fn avail_moves_pattern(&mut self, pattern: Vec<(i8, i8)>) -> Vec<GameCubePosition> {
        self.filter_blocked(coord_pattern_to_positions(pattern))
    }

    pub fn try_make_move(&mut self, from: GameCubePosition, to: GameCubePosition) -> String {
        let active_player_ind = self.active_player;
        let other_player_ind = 3 - active_player_ind;
        let game_phase = self.phase;
        let opt_cube = self.take_cube_at(from).copied();
        if let Some(cube) = opt_cube {
            if active_player_ind == cube.player {
                if to == win_position(other_player_ind) {
                    format!(
                        "Illegal move for player {} to win position of player {}",
                        active_player_ind, other_player_ind
                    )
                } else {
                    match game_phase {
                        GamePhase::Roll => {
                            let avail_moves =
                                self.avail_moves_pattern(t_move_pattern(from.x, from.y));
                            if avail_moves.contains(&to) {
                                let dir_after = cube
                                    .direction
                                    .direction_after_roll(get_roll_direction(from, to));
                                if dir_after.up == 1 && !(to == win_position(active_player_ind)) {
                                    // do not allow this turn (winning to no-win position)
                                    format!("This Roll move is not allowed! (will be facing Star up in no-win position for player {})", active_player_ind)
                                } else if dir_after.up != 1
                                    && (to == win_position(active_player_ind)
                                        || to == win_position(other_player_ind))
                                {
                                    "This Roll move is not allowed! (rolling into win position for either player)".to_string()
                                } else {
                                    //roll cube
                                    self.move_cube(from, to, dir_after);
                                    if dir_after.up == 1 {
                                        // remove cube from winning position
                                        self.remove_cube_at(to);
                                        let winnerp = self.check_winner();
                                        if winnerp.is_some() {
                                            // if we have a winner
                                            // end the game
                                            // the active player is the winner
                                            self.phase = GamePhase::End;
                                            format!(
                                                "Player {} scores and wins the game!!!",
                                                active_player_ind
                                            )
                                        } else {
                                            self.active_player = other_player_ind;
                                            format!(
                                                "Player {} scores! Roll phase for player {}.",
                                                active_player_ind, other_player_ind
                                            )
                                        }
                                    } else if dir_after.up == 6 {
                                        // Rolled to stop. Leave phase as Roll for another Player.
                                        self.active_player = other_player_ind;
                                        format!(
                                    "Player {} Rolls to stop. Roll phase for another player {}.",
                                    active_player_ind, other_player_ind
                                )
                                    } else {
                                        // switch phase to Hop for the same active player
                                        self.phase = GamePhase::Hop;
                                        format!("Player {} Rolled from ({},{}) to position ({},{}). Next: Hop of the same player {}.",
                                        active_player_ind, from.x, from.y, to.x, to.y, active_player_ind)
                                    }
                                }
                            } else {
                                format!(
                                    "Illegal Roll from ({},{}) to position ({},{}) for player {}.",
                                    from.x, from.y, to.x, to.y, active_player_ind
                                )
                            }
                        }
                        GamePhase::Hop => {
                            let face: GameCubeFace = cube.direction.up.into();
                            let avail_moves = match face {
                                GameCubeFace::THop => {
                                    self.avail_moves_pattern(t_move_pattern(from.x, from.y))
                                }
                                GameCubeFace::XHop => {
                                    self.avail_moves_pattern(x_move_pattern(from.x, from.y))
                                }
                                GameCubeFace::Slide => {
                                    self.avail_moves_pattern(l_move_pattern(from.x, from.y))
                                }
                                GameCubeFace::Hoops => {
                                    self.avail_moves_pattern(h_move_pattern(from.x, from.y))
                                }
                                _ => vec![],
                            };
                            if avail_moves.contains(&to) {
                                self.move_cube(from, to, cube.direction);
                                self.phase = GamePhase::Roll;
                                self.active_player = other_player_ind;
                                format!("Player {} Hopped from ({},{}) to position ({},{}). Next: Roll of the other player {}.",
                                active_player_ind, from.x, from.y, to.x, to.y, other_player_ind)
                            } else {
                                format!(
                                    "Illegal Hop from ({},{}) to position ({},{}) for player {}.",
                                    from.x, from.y, to.x, to.y, active_player_ind
                                )
                            }
                        }
                        GamePhase::End => {
                            // game is finished; this does nothing
                            format!(
                                "The game ended with player {} the winner. No turns allowed.",
                                active_player_ind
                            )
                        }
                    }
                }
            } else {
                format!(
                    "Illegal move attempt with cube not belonging to player {}.",
                    active_player_ind
                )
            }
        } else {
            ("Illegal move attempt from empty space.").to_string()
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    // use near_sdk::test_utils::VMContextBuilder;
    // use near_sdk::MockedBlockchain;
    // use near_sdk::{testing_env, VMContext};

    // fn get_context(is_view: bool) -> VMContext {
    //     VMContextBuilder::new().is_view(is_view).build()
    // }

    fn make_test_game_1_1() -> Game {
        Game {
            phase: GamePhase::Roll,
            active_player: 1,
            board: vec![
                GameCube {
                    player: 1,
                    position: GameCubePosition { x: 4, y: 7 },
                    direction: GameCubeDirection {
                        up: 2,
                        front: 3,
                        right: 6,
                    },
                },
                GameCube {
                    player: 2,
                    position: GameCubePosition { x: 4, y: 1 },
                    direction: GameCubeDirection {
                        up: 5,
                        front: 3,
                        right: 1,
                    },
                },
            ],
        }
    }

    #[test]
    fn test_faces_complements() {
        let face = GameCubeFace::Slide;
        assert_eq!(GameCubeFace::Hoops, opposite_face(face));
        assert_eq!(GameCubeFace::Star, opposite_face(GameCubeFace::Stop));
        assert_eq!(GameCubeFace::XHop, opposite_face(GameCubeFace::THop));
    }

    #[test]
    fn test_direction_validation() {
        let d = GameCubeDirection::from(1, 3, 2);
        assert!(d.is_some());
        assert!(GameCubeDirection::from(2, 2, 3).is_none());
        assert!(GameCubeDirection::from(7, 1, 2).is_none());
        assert!(GameCubeDirection::from(3, 6, 4).is_none());
    }

    #[test]
    fn test_direction_after_roll() {
        let init_d = GameCubeDirection::from(1, 3, 2).unwrap();
        let forward_d = init_d.direction_after_roll(Direction::Forward);
        assert_eq!(forward_d, GameCubeDirection::from(4, 1, 2).unwrap());

        let left_d = init_d.direction_after_roll(Direction::Left);
        assert_eq!(left_d, GameCubeDirection::from(2, 3, 6).unwrap());

        let back_d = init_d.direction_after_roll(Direction::Backward);
        assert_eq!(back_d, GameCubeDirection::from(3, 6, 2).unwrap());

        let right_d = init_d.direction_after_roll(Direction::Right);
        assert_eq!(right_d, GameCubeDirection::from(5, 3, 1).unwrap());
    }

    #[test]
    fn test_print_layout() {
        let init_d = GameCubeDirection::from(1, 3, 2).unwrap();
        println!("{}", init_d.format_layout());
    }

    #[test]
    fn test_game_cube_at_position() {
        let mut game = make_test_game_1_1();
        let cube = game.take_cube_at(GameCubePosition { x: 2, y: 2 });
        assert!(cube.is_none());
        assert!(game.take_cube_at(GameCubePosition { x: 4, y: 7 }).is_some());
        assert!(game.take_cube_at(GameCubePosition { x: 4, y: 1 }).is_some());
        assert!(game.take_cube_at(GameCubePosition { x: 3, y: 5 }).is_none());
    }

    #[test]
    fn test_remove_cube() {
        let mut game = make_test_game_1_1();
        game.remove_cube_at(GameCubePosition { x: 4, y: 7 });
        assert!(game.take_cube_at(GameCubePosition { x: 4, y: 7 }).is_none());
        println!("{}", game.format_board());
    }

    #[test]
    fn test_move_cube() {
        let mut game = make_test_game_1_1();
        let cube = *game.take_cube_at(GameCubePosition { x: 4, y: 7 }).unwrap();
        game.move_cube(
            cube.position,
            GameCubePosition { x: 3, y: 3 },
            cube.direction,
        );
        assert!(game.take_cube_at(GameCubePosition { x: 4, y: 7 }).is_none());
        assert!(game.take_cube_at(GameCubePosition { x: 3, y: 3 }).is_some());
        println!("{}", game.format_board());
    }

    #[test]
    fn test_moves() {
        let mut game = make_test_game_1_1();
        println!("{}", game.format_board());
        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 4, y: 1 },
                GameCubePosition { x: 3, y: 1 }
            )
        );
        println!("{}", game.format_board());
        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 4, y: 7 },
                GameCubePosition { x: 3, y: 7 }
            )
        );
        println!("{}", game.format_board());

        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 4, y: 1 },
                GameCubePosition { x: 5, y: 1 }
            )
        );
        println!("{}", game.format_board());

        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 3, y: 7 },
                GameCubePosition { x: 3, y: 6 }
            )
        );
        println!("{}", game.format_board());

        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 3, y: 6 },
                GameCubePosition { x: 4, y: 4 }
            )
        );
        println!("{}", game.format_board());

        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 5, y: 1 },
                GameCubePosition { x: 5, y: 2 }
            )
        );
        println!("{}", game.format_board());

        println!(
            "{}",
            game.try_make_move(
                GameCubePosition { x: 5, y: 2 },
                GameCubePosition { x: 5, y: 6 }
            )
        );
        println!("{}", game.format_board());

        let from = GameCubePosition { x: 4, y: 4 };
        let to = GameCubePosition { x: 5, y: 4 };
        println!("{}", game.try_make_move(from, to));
        println!("{}", game.format_board());
        println!(
            "{}",
            game.take_cube_at(to).unwrap().direction.format_layout()
        );

        let from = GameCubePosition { x: 5, y: 4 };
        let to = GameCubePosition { x: 4, y: 3 };
        println!("{}", game.try_make_move(from, to));
        println!("{}", game.format_board());
        println!(
            "{}",
            game.take_cube_at(to).unwrap().direction.format_layout()
        );

        let from = GameCubePosition { x: 5, y: 6 };
        let to = GameCubePosition { x: 5, y: 5 };
        println!("{}", game.try_make_move(from, to));
        println!("{}", game.format_board());
        println!(
            "{}",
            game.take_cube_at(to).unwrap().direction.format_layout()
        );

        let from = GameCubePosition { x: 5, y: 5 };
        let to = GameCubePosition { x: 4, y: 5 };
        println!("{}", game.try_make_move(from, to));
        println!("{}", game.format_board());

        let from = GameCubePosition { x: 4, y: 3 };
        let to = GameCubePosition { x: 4, y: 2 };
        println!("{}", game.try_make_move(from, to));
        println!("{}", game.format_board());
    }

    #[test]
    fn test_move_pattern() {
        let mut game = make_test_game_1_1();
        println!("{:?}", game.avail_moves_pattern(t_move_pattern(3, 7)));
        println!("{:?}", game.avail_moves_pattern(x_move_pattern(5, 2)));
        println!("{:?}", game.avail_moves_pattern(h_move_pattern(1, 1)));
        println!("{:?}", game.avail_moves_pattern(l_move_pattern(4, 4)));
        println!("{}", game.format_board());
    }

    #[test]
    fn test_print_board() {
        let game = make_test_game_1_1();
        println!("{}", game.format_board());
    }

    #[test]
    fn test_game_init() {
        let game = Game {
            phase: GamePhase::Roll,
            active_player: 1,
            board: vec![
                GameCube {
                    player: 1,
                    position: GameCubePosition::from(2, 2).unwrap(),
                    direction: GameCubeDirection::from(1, 3, 2).unwrap(),
                },
                GameCube {
                    player: 2,
                    position: GameCubePosition { x: 6, y: 5 },
                    direction: GameCubeDirection {
                        up: 5,
                        front: 4,
                        right: 6,
                    },
                },
            ],
        };

        let p2_cubes = game.cubes_for_player(&2);

        println!("{:?}", p2_cubes);
    }

    #[test]
    fn test_game_winner() {
        let game = Game {
            phase: GamePhase::Roll,
            active_player: 1,
            board: vec![
                GameCube {
                    player: 1,
                    position: GameCubePosition { x: 2, y: 2 },
                    direction: GameCubeDirection {
                        up: 1,
                        front: 3,
                        right: 2,
                    },
                },
                GameCube {
                    player: 2,
                    position: GameCubePosition { x: 6, y: 5 },
                    direction: GameCubeDirection {
                        up: 5,
                        front: 4,
                        right: 6,
                    },
                },
            ],
        };
        let winnerp = game.check_winner();
        assert!(winnerp.is_none());

        let game = Game {
            phase: GamePhase::Roll,
            active_player: 1,
            board: vec![GameCube {
                player: 1,
                position: GameCubePosition { x: 2, y: 2 },
                direction: GameCubeDirection {
                    up: 1,
                    front: 3,
                    right: 2,
                },
            }],
        };
        let winnerp = game.check_winner();
        assert!(winnerp.is_some());
        assert_eq!(winnerp.unwrap(), 2);

        let game = Game {
            phase: GamePhase::Roll,
            active_player: 1,
            board: vec![GameCube {
                player: 2,
                position: GameCubePosition { x: 5, y: 5 },
                direction: GameCubeDirection {
                    up: 5,
                    front: 4,
                    right: 6,
                },
            }],
        };
        let winnerp = game.check_winner();
        assert!(winnerp.is_some());
        assert_eq!(winnerp.unwrap(), 1);
    }
}
