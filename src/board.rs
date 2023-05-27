use std::fmt::{self, Display};

use crate::field::{ChessError, Color, Field, FieldType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    state: [Field; 64],
    active_player: Color,
    white_can_large_castle: bool,
    white_can_small_castle: bool,
    black_can_large_castle: bool,
    black_can_small_castle: bool,
    en_passant_pos: (i32, i32),
}

impl Board {
    pub fn new() -> Board {
        Board {
            state: [Field::new(); 64],
            active_player: Color::White,
            white_can_large_castle: true,
            white_can_small_castle: true,
            black_can_large_castle: true,
            black_can_small_castle: true,
            en_passant_pos: (0, 0),
        }
    }

    pub fn is_king_attacked(
        &mut self,
        color: Color,
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    ) -> bool {
        let mut result = false;
        let old_state = self.state;
        self.finalize_move(from_x, from_y, to_x, to_y);
        let king = self.get_king(color);
        match king {
            Some(k) => {
                for field in self
                    .state
                    .into_iter()
                    .filter(|c| c.get_color() == color.enemy_color())
                {
                    match self.validate_move(
                        field.get_x(),
                        field.get_y(),
                        k.get_x(),
                        k.get_y(),
                        field.get_color(),
                    ) {
                        Ok(_) => {
                            //king can be attacked by any figure other than pawns moving straigt
                            if !(field.get_type() == FieldType::Pawn && field.get_y() == k.get_y())
                            {
                                result = true;
                            }
                        }
                        Err(_) => {
                            //special case: pawns can threaten a king even when self.validate_move fails
                            if field.get_type() == FieldType::Pawn
                                && (field.get_y() == k.get_y() + 1
                                    || field.get_y() == k.get_y() - 1)
                            {
                                if (field.get_color() == Color::White
                                    && field.get_x() == k.get_x() - 1)
                                    || (field.get_color() == Color::Black
                                        && field.get_x() == k.get_x() + 1)
                                {
                                    result = true;
                                }
                            }
                        }
                    }
                }
            }
            None => {}
        }
        //return to previous state
        self.state = old_state;
        result
    }

    fn get_king(&self, king_color: Color) -> Option<Field> {
        self.state
            .into_iter()
            .filter(|x| x.get_type() == FieldType::King && x.get_color() == king_color)
            .last()
    }

    pub fn validate_move(
        &self,
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
        color: Color,
    ) -> Result<(), ChessError> {
        let figure = self.get(from_x, from_y);

        //not the current player's figure
        if figure.get_color() != color {
            return Err(ChessError::FigureWrongColor(figure.get_type()));
        }
        //empty board validation
        figure.validate_move(to_x, to_y)?;

        match figure.get_type() {
            FieldType::King => {
                let target_field = self.get(to_x, to_y);
                if target_field.get_color() == figure.get_color() {
                    return Err(ChessError::FieldAlreadyOwned(to_x, to_y));
                }
            }
            FieldType::Queen => {
                //queen moves like a tower
                if (figure.get_x() == to_x) || (figure.get_y() == to_y) {
                    if figure.get_x() != to_x {
                        let step_x = if figure.get_x() < to_x { 1 } else { -1 };
                        let steps = if figure.get_x() - to_x > 0 {
                            figure.get_x() - to_x + 1
                        } else {
                            to_x - figure.get_x() + 1
                        };
                        for n in 1..steps {
                            let next_field = self.get(figure.get_x() + step_x * n, to_y);
                            if next_field.get_type() != FieldType::None {
                                //blocked by owned figure
                                if next_field.get_color() == figure.get_color() {
                                    return Err(ChessError::FieldAlreadyOwned(
                                        next_field.get_x(),
                                        next_field.get_y(),
                                    ));
                                }
                                //blocked by enemy figure but not the target
                                if next_field.get_x() != to_x {
                                    return Err(ChessError::MoveBlockedByEnemyFigure(
                                        next_field.get_x(),
                                        to_y,
                                    ));
                                }
                            }
                        }
                    } else {
                        let step_y = if figure.get_y() < to_y { 1 } else { -1 };
                        let steps = if figure.get_y() - to_y > 0 {
                            figure.get_y() - to_y + 1
                        } else {
                            to_y - figure.get_y() + 1
                        };
                        for n in 1..steps {
                            let next_field = self.get(to_x, figure.get_y() + step_y * n);
                            if next_field.get_type() != FieldType::None {
                                //blocked by owned figure
                                if next_field.get_color() == figure.get_color() {
                                    return Err(ChessError::FieldAlreadyOwned(
                                        next_field.get_x(),
                                        next_field.get_y(),
                                    ));
                                }
                                //blocked by enemy figure but not the target
                                if next_field.get_y() != to_y {
                                    return Err(ChessError::MoveBlockedByEnemyFigure(
                                        to_x,
                                        next_field.get_y(),
                                    ));
                                }
                            }
                        }
                    }
                } else {
                    //queen moves like a bishop
                    let step_x = if figure.get_x() < to_x { 1 } else { -1 };
                    let step_y = if figure.get_y() < to_y { 1 } else { -1 };
                    let steps = if figure.get_x() - to_x > 0 {
                        figure.get_x() - to_x + 1
                    } else {
                        to_x - figure.get_x() + 1
                    };
                    for n in 1..steps {
                        let next_field =
                            self.get(figure.get_x() + step_x * n, figure.get_y() + step_y * n);
                        if next_field.get_type() != FieldType::None {
                            //blocked by owned figure
                            if next_field.get_color() == figure.get_color() {
                                return Err(ChessError::FieldAlreadyOwned(
                                    next_field.get_x(),
                                    next_field.get_y(),
                                ));
                            }
                            //blocked by enemy figure but not the target
                            if next_field.get_x() != to_x {
                                return Err(ChessError::MoveBlockedByEnemyFigure(
                                    next_field.get_x(),
                                    next_field.get_y(),
                                ));
                            }
                        }
                    }
                }
            }
            FieldType::Bishop => {
                let step_x = if figure.get_x() < to_x { 1 } else { -1 };
                let step_y = if figure.get_y() < to_y { 1 } else { -1 };
                let steps = if figure.get_x() - to_x > 0 {
                    figure.get_x() - to_x + 1
                } else {
                    to_x - figure.get_x() + 1
                };
                for n in 1..steps {
                    let next_field =
                        self.get(figure.get_x() + step_x * n, figure.get_y() + step_y * n);
                    if next_field.get_type() != FieldType::None {
                        //blocked by owned figure
                        if next_field.get_color() == figure.get_color() {
                            return Err(ChessError::FieldAlreadyOwned(
                                next_field.get_x(),
                                next_field.get_y(),
                            ));
                        }
                        //blocked by enemy figure but not the target
                        if next_field.get_x() != to_x {
                            return Err(ChessError::MoveBlockedByEnemyFigure(
                                next_field.get_x(),
                                next_field.get_y(),
                            ));
                        }
                    }
                }
            }
            FieldType::Knight => {
                let next_field = self.get(to_x, to_y);
                //field already owned
                if next_field.get_color() == figure.get_color() {
                    return Err(ChessError::FieldAlreadyOwned(
                        next_field.get_x(),
                        next_field.get_y(),
                    ));
                }
            }
            FieldType::Tower => {
                if figure.get_x() != to_x {
                    let step_x = if figure.get_x() < to_x { 1 } else { -1 };
                    let steps = if figure.get_x() - to_x > 0 {
                        figure.get_x() - to_x + 1
                    } else {
                        to_x - figure.get_x() + 1
                    };
                    for n in 1..steps {
                        let next_field = self.get(figure.get_x() + step_x * n, to_y);
                        if next_field.get_type() != FieldType::None {
                            //blocked by owned figure
                            if next_field.get_color() == figure.get_color() {
                                return Err(ChessError::FieldAlreadyOwned(
                                    next_field.get_x(),
                                    next_field.get_y(),
                                ));
                            }
                            //blocked by enemy figure but not the target
                            if next_field.get_x() != to_x {
                                return Err(ChessError::MoveBlockedByEnemyFigure(
                                    next_field.get_x(),
                                    to_y,
                                ));
                            }
                        }
                    }
                } else {
                    let step_y = if figure.get_y() < to_y { 1 } else { -1 };
                    let steps = if figure.get_y() - to_y > 0 {
                        figure.get_y() - to_y + 1
                    } else {
                        to_y - figure.get_y() + 1
                    };
                    for n in 1..steps {
                        let next_field = self.get(to_x, figure.get_y() + step_y * n);
                        if next_field.get_type() != FieldType::None {
                            //blocked by owned figure
                            if next_field.get_color() == figure.get_color() {
                                return Err(ChessError::FieldAlreadyOwned(
                                    next_field.get_x(),
                                    next_field.get_y(),
                                ));
                            }
                            //blocked by enemy figure but not the target
                            if next_field.get_y() != to_y {
                                return Err(ChessError::MoveBlockedByEnemyFigure(
                                    to_x,
                                    next_field.get_y(),
                                ));
                            }
                        }
                    }
                }
            }
            FieldType::Pawn => {
                //pawn doesn't take another figure
                if figure.get_y() == to_y {
                    //target free
                    if self.get(to_x, to_y).get_type() != FieldType::None {
                        return Err(ChessError::PawnMoveBlocked(to_x, to_y));
                    }
                    //white pawn move by 2
                    if to_x - figure.get_x() == 2 {
                        if self.get(to_x - 1, to_y).get_type() != FieldType::None {
                            return Err(ChessError::PawnMoveBlocked(to_x - 1, to_y));
                        }
                    }
                    //black pawn moveby 2
                    if figure.get_x() - to_x == 2 {
                        if self.get(to_x + 1, to_y).get_type() != FieldType::None {
                            return Err(ChessError::PawnMoveBlocked(to_x + 1, to_y));
                        }
                    }
                } else {
                    //pawn takes another figure
                    let target_field = self.get(to_x, to_y);
                    //pawn can't take empty field
                    if target_field.get_type() == FieldType::None {
                        //unless it's en passant
                        if (target_field.get_x(), target_field.get_y()) != self.en_passant_pos {
                            return Err(ChessError::PawnCantTakeEmptyField);
                        }
                    }
                    //field already owned
                    if target_field.get_color() == figure.get_color() {
                        return Err(ChessError::FieldAlreadyOwned(
                            figure.get_x(),
                            figure.get_y(),
                        ));
                    }
                }
            }
            FieldType::None => {
                //entry in state found but no figure there
                return Err(ChessError::EmptyStartingField(from_x, from_y));
            }
        }
        Ok(())
    }

    fn get(&self, x: i32, y: i32) -> Field {
        match self
            .state
            .into_iter()
            .filter(|c| (c.get_x() == x) && (c.get_y() == y))
            .last()
        {
            Some(c) => c,
            None => Field::new(),
        }
    }
    pub fn finalize_move(
        &mut self,
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    ) -> Option<String> {
        //for en passant cleanup we need to know the figure type early
        let moved_figure = self.get(from_x, from_y);
        let last_enpassant_pos = self.en_passant_pos;
        let mut response: Option<String> = None;
        for field in self.state.as_mut() {
            //previous en passant
            if (to_x, to_y) == last_enpassant_pos && moved_figure.get_type() == FieldType::Pawn {
                if field.get_color() != moved_figure.get_color()
                    && field.get_type() == FieldType::Pawn
                {
                    if field.get_color() == Color::White
                        && (field.get_x(), field.get_y()) == (to_x + 1, to_y)
                    {
                        field.set_empty_with_pos(field.get_x(), field.get_y());
                    }
                    if field.get_color() == Color::Black
                        && (field.get_x(), field.get_y()) == (to_x - 1, to_y)
                    {
                        field.set_empty_with_pos(field.get_x(), field.get_y());
                    }
                }
            }

            //cleanup taken figure
            if field.get_x() == to_x && field.get_y() == to_y {
                field.set_empty_with_pos(from_x, from_y);
            } else if field.get_x() == from_x && field.get_y() == from_y {
                //moved figure
                field.finalize_move(to_x, to_y);

                field.check_promote_pawn();

                //disable castling on King move
                if field.get_type() == FieldType::King {
                    if field.get_color() == Color::White {
                        self.white_can_large_castle = false;
                        self.white_can_small_castle = false;
                    }
                    if field.get_color() == Color::Black {
                        self.black_can_large_castle = false;
                        self.black_can_small_castle = false;
                    }
                }
                //disable castling on Tower move
                if field.get_type() == FieldType::Tower {
                    match (from_x, from_y) {
                        (1, 8) => self.white_can_small_castle = false,
                        (1, 1) => self.white_can_large_castle = false,
                        (8, 8) => self.black_can_small_castle = false,
                        (8, 1) => self.black_can_large_castle = false,
                        _ => {}
                    }
                }

                //new en passant
                self.en_passant_pos = (0, 0);
                if field.get_type() == FieldType::Pawn {
                    if to_x - from_x == 2 {
                        //white pawn moved by 2
                        self.en_passant_pos = (3, field.get_y())
                    } else if from_x - to_x == 2 {
                        //black pawn moved by 2
                        self.en_passant_pos = (6, field.get_y())
                    }
                }

                response = Some(format!(
                    "{} moved {} from {}{} to {}{}",
                    self.active_player,
                    field.get_type(),
                    Field::y_to_letter(&from_y),
                    from_x,
                    Field::y_to_letter(&field.get_y()),
                    field.get_x()
                ));
            }
        }

        response
    }

    pub fn castling(
        &mut self,
        castle_type_str: &str,
        check_only: bool,
    ) -> Result<bool, ChessError> {
        let x = if self.active_player == Color::White {
            1
        } else {
            8
        };
        match castle_type_str.to_lowercase().trim() {
            "o-o" => {
                if (x == 1 && !self.white_can_small_castle)
                    || (x == 8 && !self.black_can_small_castle)
                {
                    return Err(ChessError::CastlingNoPossibleAlreadyMoved);
                }
                //figures blocking the path
                for y in 6..8 {
                    if self.get(x, y).get_type() != FieldType::None {
                        return Err(ChessError::CastlingBlockedByFigure(x, y));
                    }
                }
                //castling isn't allowed when king is attacked
                for y in 5..7 {
                    if self.is_king_attacked(self.active_player, x, 5, x, y) {
                        return Err(ChessError::CastlingNotPossibleWhenKingUnderAttack(x, y));
                    }
                }
                if !check_only {
                    self.finalize_move(x, 5, x, 7);
                    self.finalize_move(x, 8, x, 6);
                    println!("{} King side castling: Moved King from {}{} to {}{} and Tower from {}{} to {}{}", self.active_player, Field::y_to_letter(&5), x, Field::y_to_letter(&7), x, Field::y_to_letter(&8), x, Field::y_to_letter(&6), x)
                }
            }
            "o-o-o" => {
                if (x == 1 && !self.white_can_large_castle)
                    || (x == 8 && !self.black_can_large_castle)
                {
                    return Err(ChessError::CastlingNoPossibleAlreadyMoved);
                }
                //figures blocking the path
                for y in 2..5 {
                    if self.get(x, y).get_type() != FieldType::None {
                        return Err(ChessError::CastlingBlockedByFigure(x, y));
                    }
                }
                //castling isn't allowed when king is attacked
                for y in 3..6 {
                    if self.is_king_attacked(self.active_player, x, 5, x, y) {
                        return Err(ChessError::CastlingNotPossibleWhenKingUnderAttack(x, y));
                    }
                }

                if !check_only {
                    self.finalize_move(x, 5, x, 3);
                    self.finalize_move(x, 1, x, 4);
                    println!("{} Queen side castling: Moved King from {}{} to {}{} and Tower from {}{} to {}{}", self.active_player, Field::y_to_letter(&5), x, Field::y_to_letter(&3), x, Field::y_to_letter(&1), x, Field::y_to_letter(&4), x)
                }
            }
            _ => {
                return Ok(false);
            } //failure
        }

        Ok(true) //success
    }

    pub fn set_board_empty(&mut self) {
        let mut curr_x = 1;
        let mut curr_y = 1;

        self.active_player = Color::White;
        self.white_can_small_castle = true;
        self.white_can_large_castle = true;
        self.black_can_small_castle = true;
        self.black_can_large_castle = true;
        self.en_passant_pos = (0, 0);

        for field in &mut self.state {
            field.set_empty_with_pos(curr_x, curr_y);
            if curr_y > 7 {
                curr_y = 1;
                curr_x += 1;
            } else {
                curr_y += 1;
            }
        }
    }

    pub fn set_board_start(&mut self) {
        self.set_board_empty();
        for field in &mut self.state {
            if field.get_x() == 1 {
                match field.get_y() {
                    1 => {
                        field.set_figure(FieldType::Tower, Color::White);
                    }
                    2 => {
                        field.set_figure(FieldType::Knight, Color::White);
                    }
                    3 => {
                        field.set_figure(FieldType::Bishop, Color::White);
                    }
                    4 => {
                        field.set_figure(FieldType::Queen, Color::White);
                    }
                    5 => {
                        field.set_figure(FieldType::King, Color::White);
                    }
                    6 => {
                        field.set_figure(FieldType::Bishop, Color::White);
                    }
                    7 => {
                        field.set_figure(FieldType::Knight, Color::White);
                    }
                    8 => {
                        field.set_figure(FieldType::Tower, Color::White);
                    }
                    _ => {}
                }
            }
            if field.get_x() == 2 {
                field.set_figure(FieldType::Pawn, Color::White);
            }
            if field.get_x() == 7 {
                field.set_figure(FieldType::Pawn, Color::Black);
            }
            if field.get_x() == 8 {
                match field.get_y() {
                    1 => {
                        field.set_figure(FieldType::Tower, Color::Black);
                    }
                    2 => {
                        field.set_figure(FieldType::Knight, Color::Black);
                    }
                    3 => {
                        field.set_figure(FieldType::Bishop, Color::Black);
                    }
                    4 => {
                        field.set_figure(FieldType::Queen, Color::Black);
                    }
                    5 => {
                        field.set_figure(FieldType::King, Color::Black);
                    }
                    6 => {
                        field.set_figure(FieldType::Bishop, Color::Black);
                    }
                    7 => {
                        field.set_figure(FieldType::Knight, Color::Black);
                    }
                    8 => {
                        field.set_figure(FieldType::Tower, Color::Black);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn active_player(&self) -> Color {
        self.active_player
    }

    pub fn set_active_player(&mut self, active_player: Color) {
        self.active_player = active_player;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    _______________________  ",)?;
        for i in (1..9).rev() {
            writeln!(
                f,
                "{} | {}  {}  {}  {}  {}  {}  {}  {} |",
                i,
                self.get(i, 1),
                self.get(i, 2),
                self.get(i, 3),
                self.get(i, 4),
                self.get(i, 5),
                self.get(i, 6),
                self.get(i, 7),
                self.get(i, 8)
            )?;
        }
        writeln!(f, "    _______________________  ",)?;
        writeln!(f, "    a  b  c  d  e  f  g  h   ",)?;

        Ok(())
    }
}
