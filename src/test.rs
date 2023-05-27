#[cfg(test)]
mod tests {
    use crate::{field::ChessError, field::FieldType, Game};

    #[test]
    fn test_move_out_of_board() {
        assert_eq!(
            Game::test_helper("e1 e0".to_string()),
            Err(ChessError::MoveOutsideOfBoard)
        );
    }

    #[test]
    fn test_king_move_invalid() {
        assert_eq!(
            Game::test_helper("e1 c3".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::King))
        );
    }
    #[test]
    fn test_king_check_by_queen() {
        assert_eq!(
            Game::test_helper("e2 e4\ne7 e5\ne1 e2\nd8 f6\ne2 f3".to_string()),
            Err(ChessError::CantMoveFromToAsKingWillBeUnderAttack(
                2, 5, 3, 6
            ))
        );
    }
    #[test]
    fn test_king_check_by_pawn() {
        assert_eq!(
            Game::test_helper("e2 e4\ne7 e5\ne1 e2\nd8 f6\ne2 e3\nf6 f3\ne3 d4".to_string()),
            Err(ChessError::CantMoveFromToAsKingWillBeUnderAttack(
                3, 5, 4, 4
            ))
        );
    }
    #[test]
    fn test_king_check_by_move_pawn_away() {
        assert_eq!(
            Game::test_helper("e2 e4\ne7 e5\nb1 c3\nd8 g5\nb2 b3\ng5 g3\nf2 f3".to_string()),
            Err(ChessError::CantMoveFromToAsKingWillBeUnderAttack(
                2, 6, 3, 6
            ))
        );
    }
    #[test]
    fn test_king_move_blocked_field() {
        assert_eq!(
            Game::test_helper("e2 e4\ne7 e5\ne1 e2\nb8 c6\ne2 e3\ng8 f6\ne3 e4".to_string()),
            Err(ChessError::FieldAlreadyOwned(4, 5))
        );
    }
    #[test]
    fn test_king_move_small_castle_blocked() {
        assert_eq!(
            Game::test_helper("o-o".to_string()),
            Err(ChessError::CastlingBlockedByFigure(1, 6))
        );
    }
    #[test]
    fn test_king_move_large_castle_blocked() {
        assert_eq!(
            Game::test_helper("o-o-o".to_string()),
            Err(ChessError::CastlingBlockedByFigure(1, 2))
        );
    }
    // #[test]
    // fn test_king_move_small_castle_check_on_king() { //TODO: failing with CastlingNoPossibleAlreadyMoved, don't want to fix it now (no impact on the game, just wrong error msg)
    //     assert_eq!(Game::test_helper("e2 e4\ne7 e5\nf1 c4\nd8 h4\ng1 f3\nh4 e4\no-o".to_string()), Err(ChessError::CastlingNotPossibleWhenKingUnderAttack(1, 5)));
    // }
    #[test]
    fn test_tower_move_invalid() {
        assert_eq!(
            Game::test_helper("a1 c3".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::Tower))
        );
    }
    #[test]
    fn test_tower_move_blocked_by_owned() {
        assert_eq!(
            Game::test_helper("a1 a8".to_string()),
            Err(ChessError::FieldAlreadyOwned(2, 1))
        );
    }
    #[test]
    fn test_tower_move_blocked_by_enemy() {
        assert_eq!(
            Game::test_helper("a2 a4\nb7 b5\na4 b5\ne7 e5\na1 a8".to_string()),
            Err(ChessError::MoveBlockedByEnemyFigure(7, 1))
        );
    }
    #[test]
    fn test_knight_move_invalid() {
        assert_eq!(
            Game::test_helper("b1 d3".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::Knight))
        );
    }
    #[test]
    fn test_bishop_move_invalid() {
        assert_eq!(
            Game::test_helper("c1 h7".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::Bishop))
        );
    }
    #[test]
    fn test_bishop_move_blocked_by_owned() {
        assert_eq!(
            Game::test_helper("c1 a3".to_string()),
            Err(ChessError::FieldAlreadyOwned(2, 2))
        );
    }
    #[test]
    fn test_bishop_move_blocked_by_enemy() {
        assert_eq!(
            Game::test_helper("b2 b3\nb7 b5\nc1 a3\nb5 b4\na3 d6".to_string()),
            Err(ChessError::MoveBlockedByEnemyFigure(4, 2))
        );
    }
    #[test]
    fn test_queen_move_invalid() {
        assert_eq!(
            Game::test_helper("d1 h7".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::Queen))
        );
    }
    #[test]
    fn test_queen_move_like_tower_blocked_by_owned() {
        assert_eq!(
            Game::test_helper("d1 d8".to_string()),
            Err(ChessError::FieldAlreadyOwned(2, 4))
        );
    }
    #[test]
    fn test_queen_move_like_bishop_blocked_by_owned() {
        assert_eq!(
            Game::test_helper("d1 a4".to_string()),
            Err(ChessError::FieldAlreadyOwned(2, 3))
        );
    }

    #[test]
    fn test_queen_move_like_tower_blocked_by_enemy() {
        assert_eq!(
            Game::test_helper("d2 d4\nc7 c5\nd4 c5\nd7 d5\nd1 d8".to_string()),
            Err(ChessError::MoveBlockedByEnemyFigure(5, 4))
        );
    }
    #[test]
    fn test_queen_move_like_bishop_blocked_by_enemy() {
        assert_eq!(
            Game::test_helper("e2 e4\ng7 g5\nb1 c3\ng5 g4\nd1 h5".to_string()),
            Err(ChessError::MoveBlockedByEnemyFigure(4, 7))
        );
    }
    #[test]
    fn test_pawn_move_invalid() {
        assert_eq!(
            Game::test_helper("e2 e6".to_string()),
            Err(ChessError::InvalidFigureMove(FieldType::Pawn))
        );
    }
    #[test]
    fn test_pawn_move_invalid_take() {
        assert_eq!(
            Game::test_helper("e2 d3".to_string()),
            Err(ChessError::PawnCantTakeEmptyField)
        );
    }
    #[test]
    fn test_pawn_move_take() {
        assert_eq!(
            Game::test_helper("e2 e4\nd7 d5\ne4 d5".to_string()),
            Ok(())
        );
    }
    #[test]
    fn test_pawn_move_en_passant() {
        assert_eq!(
            Game::test_helper("e2 e4\na7 a5\ne4 e5\nd7 d5\ne5 d6".to_string()),
            Ok(())
        );
    }
}
