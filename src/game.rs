use crate::board::Board;
use crate::field::{ChessError, Color};
use std::collections::VecDeque;

pub struct Game {
    history: VecDeque<Board>,
    board: Board,
    turn: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            history: VecDeque::new(),
            board: Board::new(),
            turn: 0,
        }
    }

    pub fn concede(&mut self) -> Result<(), std::io::Error> {
        println!(
            "Are you sure you want to concede ({})? (yes, no)",
            self.board.active_player()
        );

        let mut user_input = String::from("");
        std::io::stdin().read_line(&mut user_input)?;
        if &user_input == "yes" {
            if self.board.active_player() == Color::White {
                println!("Player White conceded!");
                println!("Player Black wins the game after {} turns!", self.turn);
                println!("");
            } else {
                println!("Player White conceded!");
                println!("Player Black wins the game after {} turns!", self.turn);
                println!("");
            }
        } else {
            println!("Cancelled...")
        }

        Ok(())
    }

    pub fn game_loop(&mut self) -> Result<(), std::io::Error> {
        let mut user_input = String::from("");
        self.history.clear();
        self.board.set_board_start();
        println!("{}", self.board);
        loop {
            user_input.clear();
            std::io::stdin().read_line(&mut user_input)?;
            if user_input.starts_with("exit") {
                break;
            }
            if user_input.starts_with("concede") {
                self.concede()?;
                self.history.clear();
                self.board.set_board_start();
                continue;
            }
            if user_input.starts_with("undo") {
                self.undo();
                continue;
            }
            match self.eval(&user_input) {
                Ok(_) => {
                    println!("{}", self.board)
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
        Ok(())
    }

    fn eval(&mut self, input: &String) -> Result<(), ChessError> {
        for entry in input.split("\n") {
            if entry == "" {
                continue;
            }
            if self.board.castling(entry, true)? {
                self.history.push_back(self.board);
                self.board.castling(entry, false)?;
                self.turn += 1;

                self.board
                    .set_active_player(self.board.active_player().enemy_color());

                return Ok(());
            };

            let next_move = entry.trim().split_once(" ");
            match next_move {
                Some(c) => {
                    if c.0.len() != 2 || c.1.len() != 2 {
                        return Err(ChessError::InvalidMoveSyntax(entry.to_string()));
                    }
                    let mut start_pos = c.0.chars();
                    let from_y = match start_pos.next() {
                        Some(c) => match c.to_digit(10) {
                            Some(n) => n as i32,
                            None => match c {
                                'a' => 1,
                                'b' => 2,
                                'c' => 3,
                                'd' => 4,
                                'e' => 5,
                                'f' => 6,
                                'g' => 7,
                                'h' => 8,
                                _ => 0,
                            },
                        },
                        None => 0,
                    };
                    let from_x = match start_pos.next() {
                        Some(c) => match c.to_digit(10) {
                            Some(n) => n as i32,
                            None => match c {
                                'a' => 1,
                                'b' => 2,
                                'c' => 3,
                                'd' => 4,
                                'e' => 5,
                                'f' => 6,
                                'g' => 7,
                                'h' => 8,
                                _ => 0,
                            },
                        },
                        None => 0,
                    };

                    let mut target_pos = c.1.chars();
                    let to_y = match target_pos.next() {
                        Some(c) => match c.to_digit(10) {
                            Some(n) => n as i32,
                            None => match c {
                                'a' => 1,
                                'b' => 2,
                                'c' => 3,
                                'd' => 4,
                                'e' => 5,
                                'f' => 6,
                                'g' => 7,
                                'h' => 8,
                                _ => 0,
                            },
                        },
                        None => 0,
                    };
                    let to_x = match target_pos.next() {
                        Some(c) => match c.to_digit(10) {
                            Some(n) => n as i32,
                            None => match c {
                                'a' => 1,
                                'b' => 2,
                                'c' => 3,
                                'd' => 4,
                                'e' => 5,
                                'f' => 6,
                                'g' => 7,
                                'h' => 8,
                                _ => 0,
                            },
                        },
                        None => 0,
                    };
                    if (from_x * from_y * to_x * to_y == 0)
                        || from_x > 8
                        || from_y > 8
                        || to_x > 8
                        || to_y > 8
                    {
                        return Err(ChessError::MoveOutsideOfBoard);
                    }
                    self.board.validate_move(
                        from_x,
                        from_y,
                        to_x,
                        to_y,
                        self.board.active_player(),
                    )?;
                    if self.board.is_king_attacked(
                        self.board.active_player(),
                        from_x,
                        from_y,
                        to_x,
                        to_y,
                    ) {
                        return Err(ChessError::CantMoveFromToAsKingWillBeUnderAttack(
                            from_x, from_y, to_x, to_y,
                        ));
                    } else {
                        self.history.push_back(self.board);
                        match self.board.finalize_move(from_x, from_y, to_x, to_y) {
                            Some(c) => {
                                println!("{}", c)
                            }
                            None => {}
                        }
                    }
                }
                None => return Err(ChessError::InvalidMoveSyntax(entry.to_string())),
            }
            self.turn += 1;

            self.board
                .set_active_player(self.board.active_player().enemy_color());
        }
        Ok(())
    }

    pub fn undo(&mut self) {
        match self.history.pop_back() {
            Some(c) => {
                self.board = c;
                println!(
                    "Last move was undone... now is {}'s turn",
                    self.board.active_player()
                );
                println!("{}", self.board);
                self.turn -= 1;
            }
            None => {}
        }
    }
    #[allow(dead_code)]
    pub fn test_helper(input: String) -> Result<(), ChessError> {
        let mut game = Game::new();
        game.history.clear();
        game.board.set_board_start();
        game.eval(&input)
    }
}
