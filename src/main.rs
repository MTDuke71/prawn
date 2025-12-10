use prawn::board::Board;

static NAME: &str = "prawn 0.1";

fn main() {
    println!("{} - Chess Engine", NAME);

    // Demo: Display starting position
    let board = Board::default();
    println!("\nStarting position:");
    println!("{}", board);
    println!("FEN: {}", board.to_fen());
}
