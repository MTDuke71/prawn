// Board extensions for move generation support

use crate::board::{Board, Color, Piece, PieceType, Square};
use crate::moves::Move;

/// Extensions to Board for move execution
pub trait BoardExt {
    fn make_move_complete(&mut self, m: Move);
}

impl BoardExt for Board {
    /// Make a move with full handling of special cases
    fn make_move_complete(&mut self, m: Move) {
        let from = m.from();
        let to = m.to();
        let color = self.side_to_move();

        // Clear en passant square by default (will be set if double pawn push)
        self.set_en_passant_square(None);

        // Update castling rights based on piece movement
        if let Some(piece) = self.piece_at(from) {
            match piece.piece_type() {
                PieceType::King => {
                    // King move: clear both castling rights for this color
                    match color {
                        Color::White => self.clear_castling_rights(0b0011),
                        Color::Black => self.clear_castling_rights(0b1100),
                    }
                }
                PieceType::Rook => {
                    // Rook move: clear castling right for that side
                    match (color, from) {
                        (Color::White, Square::H1) => self.clear_castling_rights(0b0001),
                        (Color::White, Square::A1) => self.clear_castling_rights(0b0010),
                        (Color::Black, Square::H8) => self.clear_castling_rights(0b0100),
                        (Color::Black, Square::A8) => self.clear_castling_rights(0b1000),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Get the piece being moved
        if let Some(piece) = self.piece_at(from) {
            // Remove piece from source square
            self.clear_piece(from);

            // Handle captures (including clearing castling rights if rook captured)
            if m.is_capture() && !matches!(m.move_type(), crate::moves::MoveType::EnPassant) {
                // If capturing a rook on its starting square, clear opponent's castling rights
                if let Some(captured_piece) = self.piece_at(to) {
                    if captured_piece.piece_type() == PieceType::Rook {
                        match to {
                            Square::H1 => self.clear_castling_rights(0b0001),
                            Square::A1 => self.clear_castling_rights(0b0010),
                            Square::H8 => self.clear_castling_rights(0b0100),
                            Square::A8 => self.clear_castling_rights(0b1000),
                            _ => {}
                        }
                    }
                }
                self.clear_piece(to);
            }

            // Handle special moves
            match m.move_type() {
                crate::moves::MoveType::DoublePawnPush => {
                    // Set en passant square
                    let ep_square = match color {
                        Color::White => Square::from_index(from.index() as u8 + 8).unwrap(),
                        Color::Black => Square::from_index(from.index() as u8 - 8).unwrap(),
                    };
                    self.set_en_passant_square(Some(ep_square));
                    self.set_piece(to, piece);
                }
                crate::moves::MoveType::Promotion => {
                    if let Some(promo) = m.promotion() {
                        let promo_piece = Piece::from_type_and_color(promo, color);
                        self.set_piece(to, promo_piece);
                    }
                }
                crate::moves::MoveType::CapturePromotion => {
                    if let Some(promo) = m.promotion() {
                        let promo_piece = Piece::from_type_and_color(promo, color);
                        self.set_piece(to, promo_piece);
                    }
                }
                crate::moves::MoveType::EnPassant => {
                    // Remove captured pawn
                    let captured_square = match color {
                        Color::White => Square::from_index((to.rank() - 1) * 8 + to.file()).unwrap(),
                        Color::Black => Square::from_index((to.rank() + 1) * 8 + to.file()).unwrap(),
                    };
                    self.clear_piece(captured_square);
                    self.set_piece(to, piece);
                }
                crate::moves::MoveType::KingsideCastle => {
                    // Move king
                    self.set_piece(to, piece);
                    // Move rook
                    match color {
                        Color::White => {
                            self.clear_piece(Square::H1);
                            self.set_piece(Square::F1, Piece::WhiteRook);
                        }
                        Color::Black => {
                            self.clear_piece(Square::H8);
                            self.set_piece(Square::F8, Piece::BlackRook);
                        }
                    }
                }
                crate::moves::MoveType::QueensideCastle => {
                    // Move king
                    self.set_piece(to, piece);
                    // Move rook
                    match color {
                        Color::White => {
                            self.clear_piece(Square::A1);
                            self.set_piece(Square::D1, Piece::WhiteRook);
                        }
                        Color::Black => {
                            self.clear_piece(Square::A8);
                            self.set_piece(Square::D8, Piece::BlackRook);
                        }
                    }
                }
                _ => {
                    // Normal move
                    self.set_piece(to, piece);
                }
            }
        }

        // Update side to move
        self.swap_side();
    }
}
