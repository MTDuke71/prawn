// Mission 1: Board Representation - Unit Tests
// Following TDD approach: tests written BEFORE implementation

use prawn::board::{BitboardOps, Board, Color, Piece, PieceType, Square};

// ============================================================================
// REQ-1: 8x8 Board Representation with Bitboards
// ============================================================================

#[test]
fn req1_empty_board_initialization() {
    // Test: Empty board should have all bitboards set to 0
    let board = Board::new();

    // Verify all piece bitboards are empty
    for color in [Color::White, Color::Black] {
        for piece_type in [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ] {
            assert_eq!(
                board.piece_bitboard(color, piece_type),
                0,
                "Empty board should have no {:?} {:?}",
                color,
                piece_type
            );
        }
    }

    // Verify occupancy bitboards
    assert_eq!(
        board.occupancy(Color::White),
        0,
        "White occupancy should be 0"
    );
    assert_eq!(
        board.occupancy(Color::Black),
        0,
        "Black occupancy should be 0"
    );
    assert_eq!(board.all_occupancy(), 0, "All occupancy should be 0");
}

#[test]
fn req1_starting_position() {
    // Test: Default board should have standard chess starting position
    let board = Board::default();

    // White pawns on rank 2 (squares 8-15)
    let white_pawns = board.piece_bitboard(Color::White, PieceType::Pawn);
    assert_eq!(
        white_pawns, 0x000000000000FF00,
        "White pawns should be on rank 2"
    );

    // Black pawns on rank 7 (squares 48-55)
    let black_pawns = board.piece_bitboard(Color::Black, PieceType::Pawn);
    assert_eq!(
        black_pawns, 0x00FF000000000000,
        "Black pawns should be on rank 7"
    );

    // White rooks on A1 and H1
    let white_rooks = board.piece_bitboard(Color::White, PieceType::Rook);
    assert_eq!(white_rooks, 0x0000000000000081, "White rooks on A1 and H1");

    // Black king on E8 (square 60)
    let black_king = board.piece_bitboard(Color::Black, PieceType::King);
    assert_eq!(black_king, 0x1000000000000000, "Black king on E8");

    // Verify total occupancy
    assert_eq!(
        board.all_occupancy().count_ones(),
        32,
        "Starting position should have 32 pieces"
    );
}

#[test]
fn req1_piece_placement() {
    // Test: Placing and removing pieces on specific squares
    let mut board = Board::new();

    // Place white pawn on E4 (square 28)
    board.set_piece(Square::E4, Piece::WhitePawn);
    assert_eq!(
        board.piece_at(Square::E4),
        Some(Piece::WhitePawn),
        "Should have white pawn on E4"
    );

    // Place black knight on F6 (square 45)
    board.set_piece(Square::F6, Piece::BlackKnight);
    assert_eq!(
        board.piece_at(Square::F6),
        Some(Piece::BlackKnight),
        "Should have black knight on F6"
    );

    // Verify occupancy
    assert_eq!(
        board.all_occupancy().count_ones(),
        2,
        "Should have 2 pieces"
    );

    // Remove piece from E4
    board.clear_piece(Square::E4);
    assert_eq!(
        board.piece_at(Square::E4),
        None,
        "E4 should be empty after clearing"
    );
    assert_eq!(
        board.all_occupancy().count_ones(),
        1,
        "Should have 1 piece left"
    );
}

// ============================================================================
// REQ-2: Bitboard Operations
// ============================================================================

#[test]
fn req2_bitboard_operations() {
    // Test: Basic bitboard manipulation functions

    // Test set_bit
    let mut bb: u64 = 0;
    bb = BitboardOps::set_bit(bb, Square::A1);
    assert_eq!(bb, 0x0000000000000001, "A1 bit should be set");

    bb = BitboardOps::set_bit(bb, Square::H8);
    assert_eq!(bb, 0x8000000000000001, "A1 and H8 bits should be set");

    // Test get_bit
    assert!(BitboardOps::get_bit(bb, Square::A1), "A1 should be set");
    assert!(BitboardOps::get_bit(bb, Square::H8), "H8 should be set");
    assert!(
        !BitboardOps::get_bit(bb, Square::E4),
        "E4 should not be set"
    );

    // Test clear_bit
    bb = BitboardOps::clear_bit(bb, Square::A1);
    assert_eq!(bb, 0x8000000000000000, "Only H8 should remain");
    assert!(
        !BitboardOps::get_bit(bb, Square::A1),
        "A1 should be cleared"
    );

    // Test count_bits
    bb = 0x00FF000000000000; // 8 bits set
    assert_eq!(BitboardOps::count_bits(bb), 8, "Should count 8 bits");

    // Test pop_bit
    let (new_bb, square) = BitboardOps::pop_bit(bb);
    assert!(square.is_some(), "Should pop a square");
    assert_eq!(
        BitboardOps::count_bits(new_bb),
        7,
        "Should have 7 bits after pop"
    );
}

#[test]
fn req2_file_rank_masks() {
    // Test: File and rank mask constants

    // File A mask (all A-file squares: A1-A8)
    assert_eq!(BitboardOps::FILE_A, 0x0101010101010101, "File A mask");
    assert_eq!(BitboardOps::FILE_H, 0x8080808080808080, "File H mask");

    // Rank 1 mask (all rank 1 squares: A1-H1)
    assert_eq!(BitboardOps::RANK_1, 0x00000000000000FF, "Rank 1 mask");
    assert_eq!(BitboardOps::RANK_8, 0xFF00000000000000, "Rank 8 mask");

    // Verify E4 is on E-file and rank 4
    let e4_bb = BitboardOps::square_to_bitboard(Square::E4);
    assert_ne!(e4_bb & BitboardOps::FILE_E, 0, "E4 should be on E-file");
    assert_ne!(e4_bb & BitboardOps::RANK_4, 0, "E4 should be on rank 4");
}

// ============================================================================
// REQ-3: Piece Type Encoding
// ============================================================================

#[test]
fn req3_piece_type_encoding() {
    // Test: Piece type and piece enum conversions

    // Test Piece to PieceType and Color extraction
    assert_eq!(Piece::WhitePawn.piece_type(), PieceType::Pawn);
    assert_eq!(Piece::WhitePawn.color(), Color::White);

    assert_eq!(Piece::BlackKnight.piece_type(), PieceType::Knight);
    assert_eq!(Piece::BlackKnight.color(), Color::Black);

    assert_eq!(Piece::WhiteKing.piece_type(), PieceType::King);
    assert_eq!(Piece::WhiteKing.color(), Color::White);

    // Test Piece construction from type and color
    assert_eq!(
        Piece::from_type_and_color(PieceType::Rook, Color::White),
        Piece::WhiteRook
    );
    assert_eq!(
        Piece::from_type_and_color(PieceType::Queen, Color::Black),
        Piece::BlackQueen
    );
}

// ============================================================================
// REQ-4: Color Operations
// ============================================================================

#[test]
fn req4_color_operations() {
    // Test: Color enum and opponent method

    assert_eq!(Color::White.opponent(), Color::Black);
    assert_eq!(Color::Black.opponent(), Color::White);

    // Test index conversion for array indexing
    assert_eq!(Color::White as usize, 0);
    assert_eq!(Color::Black as usize, 1);
}

// ============================================================================
// REQ-5: FEN Parsing and Generation
// ============================================================================

#[test]
fn req5_fen_parsing() {
    // Test: Parse FEN string into Board

    // Starting position
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let board = Board::from_fen(fen).expect("Should parse starting position FEN");

    assert_eq!(
        board.all_occupancy().count_ones(),
        32,
        "Should have 32 pieces"
    );
    assert_eq!(board.side_to_move(), Color::White, "White to move");
    assert!(
        board.can_castle_kingside(Color::White),
        "White can castle kingside"
    );

    // Empty board
    let empty_fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let empty_board = Board::from_fen(empty_fen).expect("Should parse empty board FEN");
    assert_eq!(empty_board.all_occupancy(), 0, "Should be empty");

    // Custom position (e4 e5 opening)
    let custom_fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2";
    let custom_board = Board::from_fen(custom_fen).expect("Should parse custom FEN");
    assert_eq!(
        custom_board.piece_at(Square::E4),
        Some(Piece::WhitePawn),
        "White pawn on E4"
    );
    assert_eq!(
        custom_board.piece_at(Square::E5),
        Some(Piece::BlackPawn),
        "Black pawn on E5"
    );
}

#[test]
fn req5_fen_generation() {
    // Test: Generate FEN string from Board

    let board = Board::default();
    let fen = board.to_fen();
    assert_eq!(
        fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "Should generate starting position FEN"
    );

    // Test round-trip
    let parsed = Board::from_fen(&fen).expect("Should parse generated FEN");
    assert_eq!(parsed.to_fen(), fen, "FEN should round-trip correctly");
}

// ============================================================================
// REQ-6: Board Display
// ============================================================================

#[test]
fn req6_board_display() {
    // Test: Board display formatting

    let board = Board::default();
    let display = format!("{}", board);

    // Should contain rank numbers
    assert!(display.contains("8"), "Display should show rank 8");
    assert!(display.contains("1"), "Display should show rank 1");

    // Should contain file letters
    assert!(display.contains("a"), "Display should show file a");
    assert!(display.contains("h"), "Display should show file h");

    // Should contain Unicode pieces (or ASCII fallback)
    assert!(
        display.contains("♜") || display.contains("r"),
        "Display should show rook symbol"
    );
    assert!(
        display.contains("♟") || display.contains("p"),
        "Display should show pawn symbol"
    );
}
