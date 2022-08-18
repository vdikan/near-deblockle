use crate::direction::GameCubeDirection;
use crate::game::{Game, GameCube, GamePhase};
use crate::player::GamePlayerIndex;
use crate::position::GameCubePosition;

impl Game {
    /// Helper fn that creates a default game setup.
    pub fn game_setup(num_cubes: Option<usize>) -> Self {
        let num_cubes = num_cubes.unwrap_or(4);
        match num_cubes {
            1 => Self {
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
            },
            2 => Self {
                phase: GamePhase::Roll,
                active_player: 1,
                board: vec![
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 3, y: 7 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 5, y: 7 },
                        direction: GameCubeDirection {
                            up: 4,
                            front: 1,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 3, y: 1 },
                        direction: GameCubeDirection {
                            up: 5,
                            front: 3,
                            right: 1,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 5, y: 1 },
                        direction: GameCubeDirection {
                            up: 4,
                            front: 1,
                            right: 2,
                        },
                    },
                ],
            },
            3 => Self {
                phase: GamePhase::Roll,
                active_player: 1,
                board: vec![
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 3, y: 7 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 5, y: 7 },
                        direction: GameCubeDirection {
                            up: 4,
                            front: 1,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 4, y: 5 },
                        direction: GameCubeDirection {
                            up: 4,
                            front: 1,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 3, y: 1 },
                        direction: GameCubeDirection {
                            up: 5,
                            front: 3,
                            right: 1,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 5, y: 1 },
                        direction: GameCubeDirection {
                            up: 4,
                            front: 1,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 4, y: 3 },
                        direction: GameCubeDirection {
                            up: 6,
                            front: 4,
                            right: 2,
                        },
                    },
                ],
            },
            _ => Self {
                phase: GamePhase::Roll,
                active_player: 1,
                board: vec![
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 3, y: 5 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 5, y: 5 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 3, y: 7 },
                        direction: GameCubeDirection {
                            up: 1,
                            front: 3,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 5, y: 7 },
                        direction: GameCubeDirection {
                            up: 3,
                            front: 6,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 3, y: 1 },
                        direction: GameCubeDirection {
                            up: 6,
                            front: 4,
                            right: 2,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 5, y: 1 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 3, y: 3 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 5, y: 3 },
                        direction: GameCubeDirection {
                            up: 5,
                            front: 3,
                            right: 1,
                        },
                    },
                ],
            },
        }
    }
}
