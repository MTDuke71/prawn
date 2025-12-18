use prawn::board::{Square};

#[test]
fn verify_square_indices() {
    println!("E1 index: {}, rank: {}", Square::E1.index(), Square::E1.rank());
    println!("E4 index: {}, rank: {}", Square::E4.index(), Square::E4.rank());
    println!("E5 index: {}, rank: {}", Square::E5.index(), Square::E5.rank());
    println!("E8 index: {}, rank: {}", Square::E8.index(), Square::E8.rank());
    
    // The file should line up vertically
    // E-file squares: E1 (rank 0), E2 (rank 1), ..., E8 (rank 7)
    assert_eq!(Square::E1.file(), Square::E4.file());
    assert_eq!(Square::E1.file(), Square::E5.file());
    assert_eq!(Square::E1.file(), Square::E8.file());
}
