// Complete move generation implementation
// REQ-1 through REQ-11: All chess move generation with legal move filtering

use crate::attacks::{pawn_pushes, AttackTables};
use crate::board::{BitboardOps, Board, Color, PieceType, Square};
use crate::board_ext::BoardExt;
use crate::magic::MagicTable;
use crate::moves::{Move, MoveList};

/// Move generator with pre-computed tables
pub struct MoveGenerator {
    magic_table: MagicTable,
    attack_tables: AttackTables,
}

impl MoveGenerator {
    /// Create new move generator (initializes all tables)
    pub fn new() -> Self {
        MoveGenerator {
            magic_table: MagicTable::new(),
            attack_tables: AttackTables::new(),
        }
    }

    /// Generate all legal moves for the current position
    /// REQ-8: Filters out moves that leave king in check
    pub fn generate_legal_moves(&self, board: &Board) -> MoveList {
        let mut moves = self.generate_pseudo_legal_moves(board);
        self.filter_illegal_moves(board, &mut moves);
        moves
    }

    /// Generate all pseudo-legal moves (may leave king in check)
    pub fn generate_pseudo_legal_moves(&self, board: &Board) -> MoveList {
        let mut moves = MoveList::new();
        let color = board.side_to_move();

        // Generate moves for each piece type
        self.generate_pawn_moves(board, color, &mut moves);
        self.generate_knight_moves(board, color, &mut moves);
        self.generate_bishop_moves(board, color, &mut moves);
        self.generate_rook_moves(board, color, &mut moves);
        self.generate_queen_moves(board, color, &mut moves);
        self.generate_king_moves(board, color, &mut moves);
        self.generate_castling_moves(board, color, &mut moves);

        moves
    }

    /// REQ-1: Generate pawn moves
    fn generate_pawn_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let pawns = board.piece_bitboard(color, PieceType::Pawn);
        let opponents = board.occupancy(color.opponent());
        let all_occupancy = board.all_occupancy();

        let mut bb = pawns;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                // Pawn pushes (forward moves)
                let pushes = pawn_pushes(square, color, all_occupancy);
                self.add_pawn_push_moves(square, pushes, color, moves);

                // Pawn captures
                let attacks = self.attack_tables.pawn_attacks(square, color);
                let captures = attacks & opponents;
                self.add_pawn_capture_moves(board, square, captures, color, moves);

                // En passant
                if let Some(ep_square) = board.en_passant_square() {
                    if (attacks & (1u64 << ep_square.index())) != 0 {
                        moves.push(Move::new_en_passant(square, ep_square));
                    }
                }
            }
        }
    }

    fn add_pawn_push_moves(
        &self,
        from: Square,
        mut pushes: u64,
        color: Color,
        moves: &mut MoveList,
    ) {
        while pushes != 0 {
            let (new_bb, to_opt) = BitboardOps::pop_bit(pushes);
            pushes = new_bb;

            if let Some(to) = to_opt {
                // Check for promotion
                let promotion_rank = if color == Color::White { 7 } else { 0 };
                if to.rank() == promotion_rank {
                    // Add all promotion moves
                    moves.push(Move::new_promotion(from, to, PieceType::Queen));
                    moves.push(Move::new_promotion(from, to, PieceType::Rook));
                    moves.push(Move::new_promotion(from, to, PieceType::Bishop));
                    moves.push(Move::new_promotion(from, to, PieceType::Knight));
                } else {
                    // Check for double pawn push
                    let starting_rank = if color == Color::White { 1 } else { 6 };
                    if from.rank() == starting_rank && to.rank() == (starting_rank as i8 + if color == Color::White { 2 } else { -2 }) as u8 {
                        moves.push(Move::new_double_pawn_push(from, to));
                    } else {
                        moves.push(Move::new_quiet(from, to));
                    }
                }
            }
        }
    }

    fn add_pawn_capture_moves(
        &self,
        board: &Board,
        from: Square,
        mut captures: u64,
        color: Color,
        moves: &mut MoveList,
    ) {
        while captures != 0 {
            let (new_bb, to_opt) = BitboardOps::pop_bit(captures);
            captures = new_bb;

            if let Some(to) = to_opt {
                if let Some(captured) = board.piece_at(to) {
                    // Check for promotion
                    let promotion_rank = if color == Color::White { 7 } else { 0 };
                    if to.rank() == promotion_rank {
                        moves.push(Move::new_capture_promotion(from, to, captured, PieceType::Queen));
                        moves.push(Move::new_capture_promotion(from, to, captured, PieceType::Rook));
                        moves.push(Move::new_capture_promotion(from, to, captured, PieceType::Bishop));
                        moves.push(Move::new_capture_promotion(from, to, captured, PieceType::Knight));
                    } else {
                        moves.push(Move::new_capture(from, to, captured));
                    }
                }
            }
        }
    }

    /// REQ-2: Generate knight moves
    fn generate_knight_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let knights = board.piece_bitboard(color, PieceType::Knight);
        let own_pieces = board.occupancy(color);

        let mut bb = knights;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                let attacks = self.attack_tables.knight_attacks(square);
                let valid_moves = attacks & !own_pieces;
                self.add_moves(board, square, valid_moves, moves);
            }
        }
    }

    /// REQ-3: Generate bishop moves using magic bitboards
    fn generate_bishop_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let bishops = board.piece_bitboard(color, PieceType::Bishop);
        let own_pieces = board.occupancy(color);
        let all_occupancy = board.all_occupancy();

        let mut bb = bishops;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                let attacks = self.magic_table.bishop_attacks(square, all_occupancy);
                let valid_moves = attacks & !own_pieces;
                self.add_moves(board, square, valid_moves, moves);
            }
        }
    }

    /// REQ-4: Generate rook moves using magic bitboards
    fn generate_rook_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let rooks = board.piece_bitboard(color, PieceType::Rook);
        let own_pieces = board.occupancy(color);
        let all_occupancy = board.all_occupancy();

        let mut bb = rooks;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                let attacks = self.magic_table.rook_attacks(square, all_occupancy);
                let valid_moves = attacks & !own_pieces;
                self.add_moves(board, square, valid_moves, moves);
            }
        }
    }

    /// REQ-5: Generate queen moves
    fn generate_queen_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let queens = board.piece_bitboard(color, PieceType::Queen);
        let own_pieces = board.occupancy(color);
        let all_occupancy = board.all_occupancy();

        let mut bb = queens;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                let attacks = self.magic_table.queen_attacks(square, all_occupancy);
                let valid_moves = attacks & !own_pieces;
                self.add_moves(board, square, valid_moves, moves);
            }
        }
    }

    /// REQ-6: Generate king moves
    fn generate_king_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        let kings = board.piece_bitboard(color, PieceType::King);
        let own_pieces = board.occupancy(color);

        let mut bb = kings;
        while bb != 0 {
            let (new_bb, square_opt) = BitboardOps::pop_bit(bb);
            bb = new_bb;

            if let Some(square) = square_opt {
                let attacks = self.attack_tables.king_attacks(square);
                let valid_moves = attacks & !own_pieces;
                self.add_moves(board, square, valid_moves, moves);
            }
        }
    }

    /// REQ-7: Generate castling moves
    fn generate_castling_moves(&self, board: &Board, color: Color, moves: &mut MoveList) {
        // Cannot castle if in check
        if self.is_in_check(board, color) {
            return;
        }

        let all_occupancy = board.all_occupancy();

        match color {
            Color::White => {
                // Kingside castle
                if board.can_castle_kingside(Color::White) {
                    // Squares between king and rook must be empty
                    let between = (1u64 << Square::F1.index()) | (1u64 << Square::G1.index());
                    if (all_occupancy & between) == 0 {
                        // King cannot move through check
                        if !self.is_square_attacked(board, Square::F1, Color::Black) {
                            moves.push(Move::new_kingside_castle(Square::E1, Square::G1));
                        }
                    }
                }

                // Queenside castle
                if board.can_castle_queenside(Color::White) {
                    let between = (1u64 << Square::B1.index())
                        | (1u64 << Square::C1.index())
                        | (1u64 << Square::D1.index());
                    if (all_occupancy & between) == 0 {
                        if !self.is_square_attacked(board, Square::D1, Color::Black) {
                            moves.push(Move::new_queenside_castle(Square::E1, Square::C1));
                        }
                    }
                }
            }
            Color::Black => {
                // Kingside castle
                if board.can_castle_kingside(Color::Black) {
                    let between = (1u64 << Square::F8.index()) | (1u64 << Square::G8.index());
                    if (all_occupancy & between) == 0 {
                        if !self.is_square_attacked(board, Square::F8, Color::White) {
                            moves.push(Move::new_kingside_castle(Square::E8, Square::G8));
                        }
                    }
                }

                // Queenside castle
                if board.can_castle_queenside(Color::Black) {
                    let between = (1u64 << Square::B8.index())
                        | (1u64 << Square::C8.index())
                        | (1u64 << Square::D8.index());
                    if (all_occupancy & between) == 0 {
                        if !self.is_square_attacked(board, Square::D8, Color::White) {
                            moves.push(Move::new_queenside_castle(Square::E8, Square::C8));
                        }
                    }
                }
            }
        }
    }

    /// Helper: Add moves from a bitboard of target squares
    fn add_moves(&self, board: &Board, from: Square, mut targets: u64, moves: &mut MoveList) {
        while targets != 0 {
            let (new_bb, to_opt) = BitboardOps::pop_bit(targets);
            targets = new_bb;

            if let Some(to) = to_opt {
                if let Some(captured) = board.piece_at(to) {
                    moves.push(Move::new_capture(from, to, captured));
                } else {
                    moves.push(Move::new_quiet(from, to));
                }
            }
        }
    }

    /// REQ-9: Check if a square is attacked by a given color
    pub fn is_square_attacked(&self, board: &Board, square: Square, by_color: Color) -> bool {
        let all_occupancy = board.all_occupancy();

        // Check pawn attacks
        let pawn_attacks = self.attack_tables.pawn_attacks(square, by_color.opponent());
        if (pawn_attacks & board.piece_bitboard(by_color, PieceType::Pawn)) != 0 {
            return true;
        }

        // Check knight attacks
        let knight_attacks = self.attack_tables.knight_attacks(square);
        if (knight_attacks & board.piece_bitboard(by_color, PieceType::Knight)) != 0 {
            return true;
        }

        // Check bishop/queen diagonal attacks
        let bishop_attacks = self.magic_table.bishop_attacks(square, all_occupancy);
        if (bishop_attacks & (board.piece_bitboard(by_color, PieceType::Bishop) | board.piece_bitboard(by_color, PieceType::Queen))) != 0 {
            return true;
        }

        // Check rook/queen orthogonal attacks
        let rook_attacks = self.magic_table.rook_attacks(square, all_occupancy);
        if (rook_attacks & (board.piece_bitboard(by_color, PieceType::Rook) | board.piece_bitboard(by_color, PieceType::Queen))) != 0 {
            return true;
        }

        // Check king attacks
        let king_attacks = self.attack_tables.king_attacks(square);
        if (king_attacks & board.piece_bitboard(by_color, PieceType::King)) != 0 {
            return true;
        }

        false
    }

    /// REQ-9: Check if current side to move is in check
    pub fn is_in_check(&self, board: &Board, color: Color) -> bool {
        let king_bb = board.piece_bitboard(color, PieceType::King);
        if king_bb == 0 {
            return false; // No king (shouldn't happen in valid position)
        }

        let king_square = Square::from_index(king_bb.trailing_zeros() as u8).unwrap();
        self.is_square_attacked(board, king_square, color.opponent())
    }

    /// REQ-8: Filter illegal moves (that leave king in check)
    fn filter_illegal_moves(&self, board: &Board, moves: &mut MoveList) {
        let original_moves: Vec<Move> = moves.moves().to_vec();
        moves.clear();

        for m in original_moves {
            if self.is_legal_move(board, m) {
                moves.push(m);
            }
        }
    }

    /// Check if a move is legal (doesn't leave own king in check)
    fn is_legal_move(&self, board: &Board, m: Move) -> bool {
        let original_side = board.side_to_move();
        let mut test_board = board.clone();
        test_board.make_move_complete(m);
        // After making the move, check if our king (original side) is under attack
        !self.is_in_check(&test_board, original_side)
    }

    /// REQ-10: Check if current position is checkmate
    pub fn is_checkmate(&self, board: &Board) -> bool {
        let color = board.side_to_move();
        self.is_in_check(board, color) && self.generate_legal_moves(board).is_empty()
    }

    /// REQ-11: Check if current position is stalemate
    pub fn is_stalemate(&self, board: &Board) -> bool {
        let color = board.side_to_move();
        !self.is_in_check(board, color) && self.generate_legal_moves(board).is_empty()
    }
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}
