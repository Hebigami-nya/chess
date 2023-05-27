use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    King,
    Queen,
    Pawn,
    Bishop,
    Tower,
    Knight,
    None,
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldType::King => write!(f, "King"),
            FieldType::Queen => write!(f, "Queen"),
            FieldType::Bishop => write!(f, "Bishop"),
            FieldType::Knight => write!(f, "Knight"),
            FieldType::Tower => write!(f, "Tower"),
            FieldType::Pawn => write!(f, "Pawn"),
            FieldType::None => write!(f, "None"),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
    None,
}

impl Color {
    pub fn enemy_color(self) -> Color {
        if self == Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
            Color::None => write!(f, "None"),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field {
    color: Color,
    figure_type: FieldType,
    x: i32,
    y: i32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChessError {
    InvalidMoveSyntax(String),
    MoveOutsideOfBoard,
    InvalidFigureMove(FieldType),
    SameStartAndTargetPos(i32, i32),
    EmptyStartingField(i32, i32),
    FigureWrongColor(FieldType),
    FigureHasNoColor,
    FieldAlreadyOwned(i32, i32),
    PawnMoveBlocked(i32, i32),
    PawnCantTakeEmptyField,
    MoveBlockedByEnemyFigure(i32, i32),
    CastlingBlockedByFigure(i32, i32),
    CastlingNoPossibleAlreadyMoved,
    CastlingNotPossibleWhenKingUnderAttack(i32, i32),
    CantMoveFromToAsKingWillBeUnderAttack(i32, i32, i32, i32),
}

impl fmt::Display for ChessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChessError::InvalidMoveSyntax(c) => write!(f, "Error: Invalid move syntax: {}", c),
            ChessError::MoveOutsideOfBoard => write!(f, "Error: Move outside of board"),
            ChessError::InvalidFigureMove(c) => write!(f, "Error: {} can't move this way", c),
            ChessError::SameStartAndTargetPos(x, y) => write!(
                f,
                "Error: Can't move figure {}{} to itself.",
                Field::y_to_letter(y),
                x
            ),
            ChessError::EmptyStartingField(x, y) => write!(
                f,
                "Error: Empty starting field {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::FigureWrongColor(c) => write!(f, "Error: Figure {} is the wrong color", c),
            ChessError::FigureHasNoColor => write!(f, "Error: Figure has no color"), //Debug Error
            ChessError::FieldAlreadyOwned(x, y) => write!(
                f,
                "Error: You already have a figure on {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::PawnMoveBlocked(x, y) => write!(
                f,
                "Error: Can't move pawn forward because a figure is alreay on the field {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::PawnCantTakeEmptyField => {
                write!(f, "Error: Pawn can't take an empty field")
            }
            ChessError::MoveBlockedByEnemyFigure(x, y) => write!(
                f,
                "Error: Move blocked by enemy figure at {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::CastlingNoPossibleAlreadyMoved => write!(
                f,
                "Error: Castling not possible since figures have already moved"
            ),
            ChessError::CastlingBlockedByFigure(x, y) => write!(
                f,
                "Error: Castling not possible since a figure is blocking it at {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::CastlingNotPossibleWhenKingUnderAttack(x, y) => write!(
                f,
                "Error: Castling not possible because King is under attack at {}{}",
                Field::y_to_letter(y),
                x
            ),
            ChessError::CantMoveFromToAsKingWillBeUnderAttack(from_x, from_y, to_x, to_y,) => write!(
                f,
                "Error: Moving from {}{} to {}{} is not possible because King would be under attack",
                Field::y_to_letter(from_y),
                from_x,
                Field::y_to_letter(to_y),
                to_x
            ),
        }
    }
}

impl Field {
    pub fn new() -> Field {
        Field {
            color: Color::None,
            figure_type: FieldType::None,
            x: 0,
            y: 0,
        }
    }
    pub fn validate_move(self, x: i32, y: i32) -> Result<(), ChessError> {
        //basic move checks only (without board state)
        //can't move out of board
        if (x < 0) || (y < 0) || (x > 8) || (y > 8) {
            return Err(ChessError::MoveOutsideOfBoard);
        }

        //can't move to same position
        if (self.x == x) && (self.y == y) {
            return Err(ChessError::SameStartAndTargetPos(x, y));
        }

        match self.figure_type {
            FieldType::King => {
                //basic king move check
                if (self.x - x > 1) || (x - self.x > 1) || (self.y - y > 1) || (y - self.y > 1) {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
            }
            FieldType::Queen => {
                //basic tower or basic bishop move check
                if ((self.x - self.y != x - y) && (self.x + self.y != x + y))
                    && ((self.x != x) && (self.y != y))
                {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
            }
            FieldType::Bishop => {
                //basic bishop move check TODO: needs better check
                if (self.x - self.y != x - y) && (self.x + self.y != x + y) {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
            }
            FieldType::Tower => {
                //basic tower move check
                if (self.x != x) && (self.y != y) {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
            }
            FieldType::Knight => {
                //basic knight move
                if (self.x + 2 == x) || (self.x - 2 == x) {
                    if (self.y - y != 1) && (y - self.y != 1) {
                        return Err(ChessError::InvalidFigureMove(self.figure_type));
                    }
                } else if (self.y + 2 == y) || (self.y - 2 == y) {
                    if (self.x - x != 1) && (x - self.x != 1) {
                        return Err(ChessError::InvalidFigureMove(self.figure_type));
                    }
                } else {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
            }
            FieldType::Pawn => {
                //can't move more than one y
                if (self.y - y > 1) || (y - self.y > 1) {
                    return Err(ChessError::InvalidFigureMove(self.figure_type));
                }
                match self.color {
                    Color::White => {
                        //on pawn starting position
                        if self.x == 2 {
                            //can't move by 2 and change y
                            if (self.y != y) && (x - self.x > 1) {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                            //can't move by more than 2
                            if x - self.x > 2 {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                        } else {
                            //any other position
                            if x - self.x > 1 {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                        }
                    }
                    Color::Black => {
                        //on pawn starting position
                        if self.x == 7 {
                            //can't move by 2 and change y
                            if (self.y != y) && (x - self.x > 1) {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                            //can't move by more than 2
                            if self.x - x > 2 {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                        } else {
                            //any other postion
                            if self.x - x > 1 {
                                return Err(ChessError::InvalidFigureMove(self.figure_type));
                            }
                        }
                    }
                    Color::None => {
                        //empty field
                        return Err(ChessError::FigureHasNoColor);
                    }
                }
            }
            FieldType::None => {
                return Err(ChessError::EmptyStartingField(self.get_x(), self.get_y()));
            }
        }

        Ok(())
    }

    pub fn finalize_move(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_color(&self) -> Color {
        self.color
    }
    pub fn get_type(&self) -> FieldType {
        self.figure_type
    }
    pub fn set_empty_with_pos(&mut self, x: i32, y: i32) {
        self.color = Color::None;
        self.figure_type = FieldType::None;
        self.x = x;
        self.y = y;
    }
    pub fn set_figure(&mut self, figure_type: FieldType, color: Color) {
        self.figure_type = figure_type;
        self.color = color;
    }
    pub fn y_to_letter(y: &i32) -> char {
        match y {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => ' ',
        }
    }

    pub fn check_promote_pawn(&mut self) {
        if self.figure_type == FieldType::Pawn {
            if (self.color == Color::White && self.x == 8)
                || (self.color == Color::Black && self.x == 1)
            {
                let mut user_input = String::from("");
                println!(
                    "{} Pawn at {}{} can be promoted to queen, bishop, knight or tower (default: queen):",
                    self.color, Field::y_to_letter(&self.y), self.x
                );
                match std::io::stdin().read_line(&mut user_input) {
                    Ok(_) => match &user_input.to_lowercase().trim()[..] {
                        "bishop" => self.figure_type = FieldType::Bishop,
                        "tower" => self.figure_type = FieldType::Tower,
                        "knight" => self.figure_type = FieldType::Knight,
                        _ => self.figure_type = FieldType::Queen,
                    },
                    Err(_) => self.figure_type = FieldType::Queen,
                }

                println!(
                    "{} Pawn has been promoted to {}",
                    self.color, self.figure_type
                );
            }
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.color {
            Color::White => match self.figure_type {
                FieldType::King => write!(f, "♔"),
                FieldType::Queen => write!(f, "♕"),
                FieldType::Bishop => write!(f, "♗"),
                FieldType::Knight => write!(f, "♘"),
                FieldType::Tower => write!(f, "♖"),
                FieldType::Pawn => write!(f, "♙"),
                FieldType::None => write!(f, " "),
            },
            Color::Black => match self.figure_type {
                FieldType::King => write!(f, "♚"),
                FieldType::Queen => write!(f, "♛"),
                FieldType::Bishop => write!(f, "♝"),
                FieldType::Knight => write!(f, "♞"),
                FieldType::Tower => write!(f, "♜"),
                FieldType::Pawn => write!(f, "♟︎"),
                FieldType::None => write!(f, " "),
            },
            Color::None => write!(f, " "),
        }
    }
}
