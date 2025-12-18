// Magic Bitboard Implementation
// High-performance sliding piece attack generation

use crate::board::Square;

/// Magic bitboard structure for one square
#[derive(Clone, Copy)]
pub struct Magic {
    pub mask: u64,      // Relevant occupancy mask
    pub magic: u64,     // Magic number
    pub shift: u8,      // Shift amount (64 - bit_count)
    pub offset: u32,    // Offset into attacks array
}

/// Magic bitboard tables for bishops and rooks
pub struct MagicTable {
    pub bishop_magics: [Magic; 64],
    pub rook_magics: [Magic; 64],
    pub bishop_attacks: Vec<u64>,
    pub rook_attacks: Vec<u64>,
}

impl MagicTable {
    /// Initialize magic bitboard tables
    pub fn new() -> Self {
        let mut table = MagicTable {
            bishop_magics: [Magic {
                mask: 0,
                magic: 0,
                shift: 0,
                offset: 0,
            }; 64],
            rook_magics: [Magic {
                mask: 0,
                magic: 0,
                shift: 0,
                offset: 0,
            }; 64],
            bishop_attacks: Vec::new(),
            rook_attacks: Vec::new(),
        };

        table.init_bishop_magics();
        table.init_rook_magics();
        table
    }

    /// Initialize bishop magic numbers and attack tables
    /// Initialize bishop magic numbers and attack tables
    fn init_bishop_magics(&mut self) {
        let mut offset = 0u32;

        for square in 0..64 {
            let sq = Square::from_index(square).unwrap();
            let mask = bishop_mask(sq);
            let bit_count = mask.count_ones();
            let table_size = 1 << bit_count;

            self.bishop_magics[square as usize] = Magic {
                mask,
                magic: BISHOP_MAGICS[square as usize],
                shift: (64 - bit_count) as u8,
                offset,
            };

            // Pre-allocate space for this square's attack table
            let start_idx = self.bishop_attacks.len();
            self.bishop_attacks.resize(start_idx + table_size as usize, 0);

            // Generate all occupancy variations and place them at correct index
            for i in 0..table_size {
                let occupancy = index_to_occupancy(i, mask);
                let attack = bishop_attacks_slow(sq, occupancy);
                
                // Calculate magic index
                let magic_index = ((occupancy.wrapping_mul(BISHOP_MAGICS[square as usize])) 
                    >> (64 - bit_count)) as usize;
                self.bishop_attacks[offset as usize + magic_index] = attack;
            }

            offset += table_size;
        }
    }

    /// Initialize rook magic numbers and attack tables
    fn init_rook_magics(&mut self) {
        let mut offset = 0u32;

        for square in 0..64 {
            let sq = Square::from_index(square).unwrap();
            let mask = rook_mask(sq);
            let bit_count = mask.count_ones();
            let table_size = 1 << bit_count;

            self.rook_magics[square as usize] = Magic {
                mask,
                magic: ROOK_MAGICS[square as usize],
                shift: (64 - bit_count) as u8,
                offset,
            };

            // Pre-allocate space for this square's attack table
            let start_idx = self.rook_attacks.len();
            self.rook_attacks.resize(start_idx + table_size as usize, 0);

            // Generate all occupancy variations and place them at correct index
            for i in 0..table_size {
                let occupancy = index_to_occupancy(i, mask);
                let attack = rook_attacks_slow(sq, occupancy);
                
                // Calculate magic index
                let magic_index = ((occupancy.wrapping_mul(ROOK_MAGICS[square as usize])) 
                    >> (64 - bit_count)) as usize;
                self.rook_attacks[offset as usize + magic_index] = attack;
            }

            offset += table_size;
        }
    }

    /// Get bishop attacks using magic bitboards (O(1))
    #[inline(always)]
    pub fn bishop_attacks(&self, square: Square, occupancy: u64) -> u64 {
        let magic = &self.bishop_magics[square.index()];
        let relevant_occ = occupancy & magic.mask;
        let index = ((relevant_occ.wrapping_mul(magic.magic)) >> magic.shift) as usize;
        self.bishop_attacks[magic.offset as usize + index]
    }

    /// Get rook attacks using magic bitboards (O(1))
    #[inline(always)]
    pub fn rook_attacks(&self, square: Square, occupancy: u64) -> u64 {
        let magic = &self.rook_magics[square.index()];
        let relevant_occ = occupancy & magic.mask;
        let index = ((relevant_occ.wrapping_mul(magic.magic)) >> magic.shift) as usize;
        self.rook_attacks[magic.offset as usize + index]
    }

    /// Get queen attacks (combination of bishop and rook)
    #[inline(always)]
    pub fn queen_attacks(&self, square: Square, occupancy: u64) -> u64 {
        self.bishop_attacks(square, occupancy) | self.rook_attacks(square, occupancy)
    }
}

/// Generate bishop occupancy mask (excludes edges)
fn bishop_mask(square: Square) -> u64 {
    let mut mask = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // North-East
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 7 && f < 7 {
        mask |= 1u64 << (r * 8 + f);
        r += 1;
        f += 1;
    }

    // North-West
    r = rank + 1;
    f = file - 1;
    while r < 7 && f > 0 {
        mask |= 1u64 << (r * 8 + f);
        r += 1;
        f -= 1;
    }

    // South-East
    r = rank - 1;
    f = file + 1;
    while r > 0 && f < 7 {
        mask |= 1u64 << (r * 8 + f);
        r -= 1;
        f += 1;
    }

    // South-West
    r = rank - 1;
    f = file - 1;
    while r > 0 && f > 0 {
        mask |= 1u64 << (r * 8 + f);
        r -= 1;
        f -= 1;
    }

    mask
}

/// Generate rook occupancy mask (excludes edges)
fn rook_mask(square: Square) -> u64 {
    let mut mask = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // North
    for r in (rank + 1)..7 {
        mask |= 1u64 << (r * 8 + file);
    }

    // South
    for r in 1..rank {
        mask |= 1u64 << (r * 8 + file);
    }

    // East
    for f in (file + 1)..7 {
        mask |= 1u64 << (rank * 8 + f);
    }

    // West
    for f in 1..file {
        mask |= 1u64 << (rank * 8 + f);
    }

    mask
}

/// Generate bishop attacks (slow version for initialization)
fn bishop_attacks_slow(square: Square, occupancy: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // North-East
    let mut r = rank + 1;
    let mut f = file + 1;
    while r < 8 && f < 8 {
        let bit = 1u64 << (r * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
        r += 1;
        f += 1;
    }

    // North-West
    r = rank + 1;
    f = file - 1;
    while r < 8 && f >= 0 {
        let bit = 1u64 << (r * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
        r += 1;
        f -= 1;
    }

    // South-East
    r = rank - 1;
    f = file + 1;
    while r >= 0 && f < 8 {
        let bit = 1u64 << (r * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
        r -= 1;
        f += 1;
    }

    // South-West
    r = rank - 1;
    f = file - 1;
    while r >= 0 && f >= 0 {
        let bit = 1u64 << (r * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
        r -= 1;
        f -= 1;
    }

    attacks
}

/// Generate rook attacks (slow version for initialization)
fn rook_attacks_slow(square: Square, occupancy: u64) -> u64 {
    let mut attacks = 0u64;
    let rank = square.rank() as i8;
    let file = square.file() as i8;

    // North
    for r in (rank + 1)..8 {
        let bit = 1u64 << (r * 8 + file);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
    }

    // South
    for r in (0..rank).rev() {
        let bit = 1u64 << (r * 8 + file);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
    }

    // East
    for f in (file + 1)..8 {
        let bit = 1u64 << (rank * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
    }

    // West
    for f in (0..file).rev() {
        let bit = 1u64 << (rank * 8 + f);
        attacks |= bit;
        if (occupancy & bit) != 0 {
            break;
        }
    }

    attacks
}

/// Convert index to occupancy pattern based on mask
fn index_to_occupancy(index: u32, mut mask: u64) -> u64 {
    let mut occupancy = 0u64;
    let mut bit_index = 0u32;

    while mask != 0 {
        let lsb = mask.trailing_zeros();
        mask &= mask - 1; // Clear LSB

        if (index & (1 << bit_index)) != 0 {
            occupancy |= 1u64 << lsb;
        }
        bit_index += 1;
    }

    occupancy
}

// Pre-computed magic numbers for bishops (found via magic number search algorithm)
const BISHOP_MAGICS: [u64; 64] = [
    0x89a1121896040240, 0x2004844802002010, 0x2068080051921000, 0x62880a0220200808,
    0x4042004000000, 0x100822020200011, 0xc00444222012000a, 0x28808801216001,
    0x400492088408100, 0x201c401040c0084, 0x840800910a0010, 0x82080240060,
    0x2000840504006000, 0x30010c4108405004, 0x1008005410080802, 0x8144042209100900,
    0x208081020014400, 0x4800201208ca00, 0xf18140408012008, 0x1004002802102001,
    0x841000820080811, 0x40200200a42008, 0x800054042000, 0x88010400410c9000,
    0x520040470104290, 0x1004040051500081, 0x2002081833080021, 0x400c00c010142,
    0x941408200c002000, 0x658810000806011, 0x188071040440a00, 0x4800404002011c00,
    0x104442040404200, 0x511080202091021, 0x4022401120400, 0x80c0040400080120,
    0x8040010040820802, 0x480810700020090, 0x102008e00040242, 0x809005202050100,
    0x8002024220104080, 0x431008804142000, 0x19001802081400, 0x200014208040080,
    0x3308082008200100, 0x41010500040c020, 0x4012020c04210308, 0x208220a202004080,
    0x111040120082000, 0x6803040141280a00, 0x2101004202410000, 0x8200000041108022,
    0x21082088000, 0x2410204010040, 0x40100400809000, 0x822088220820214,
    0x40808090012004, 0x910224040218c9, 0x402814422015008, 0x90014004842410,
    0x1000042304105, 0x10008830412a00, 0x2520081090008908, 0x40102000a0a60140,
];

// Pre-computed magic numbers for rooks
const ROOK_MAGICS: [u64; 64] = [
    0xa8002c000108020, 0x6c00049b0002001, 0x100200010090040, 0x2480041000800801,
    0x280028004000800, 0x900410008040022, 0x280020001001080, 0x2880002041000080,
    0xa000800080400034, 0x4808020004000, 0x2290802004801000, 0x411000d00100020,
    0x402800800040080, 0xb000401004208, 0x2409000100040200, 0x1002100004082,
    0x22878001e24000, 0x1090810021004010, 0x801030040200012, 0x500808008001000,
    0xa08018014000880, 0x8000808004000200, 0x201008080010200, 0x801020000441091,
    0x800080204005, 0x1040200040100048, 0x120200402082, 0xd14880480100080,
    0x12040280080080, 0x100040080020080, 0x9020010080800200, 0x813241200148449,
    0x491604001800080, 0x100401000402001, 0x4820010021001040, 0x400402202000812,
    0x209009005000802, 0x810800601800400, 0x4301083214000150, 0x204026458e001401,
    0x40204000808000, 0x8001008040010020, 0x8410820820420010, 0x1003001000090020,
    0x804040008008080, 0x12000810020004, 0x1000100200040208, 0x430000a044020001,
    0x280009023410300, 0xe0100040002240, 0x200100401700, 0x2244100408008080,
    0x8000400801980, 0x2000810040200, 0x8010100228810400, 0x2000009044210200,
    0x4080008040102101, 0x40002080411d01, 0x2005524060000901, 0x502001008400422,
    0x489a000810200402, 0x1004400080a13, 0x4000011008020084, 0x26002114058042,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_table_initialization() {
        let table = MagicTable::new();

        // Verify tables are populated
        assert!(table.bishop_attacks.len() > 0);
        assert!(table.rook_attacks.len() > 0);
    }

    #[test]
    fn test_bishop_attacks_empty_board() {
        let table = MagicTable::new();
        let e4 = Square::E4;
        let attacks = table.bishop_attacks(e4, 0);

        // Bishop on E4 on empty board should attack 13 squares
        assert_eq!(attacks.count_ones(), 13);
    }

    #[test]
    fn test_rook_attacks_empty_board() {
        let table = MagicTable::new();
        let e4 = Square::E4;
        let attacks = table.rook_attacks(e4, 0);

        // Rook on E4 on empty board should attack 14 squares
        assert_eq!(attacks.count_ones(), 14);
    }

    #[test]
    fn test_bishop_attacks_blocked() {
        let table = MagicTable::new();
        let e4 = Square::E4;

        // Block diagonal with piece on G6
        let occupancy = 1u64 << Square::G6.index();
        let attacks = table.bishop_attacks(e4, occupancy);

        // Should not attack H7 (blocked by G6)
        assert_eq!((attacks & (1u64 << Square::H7.index())), 0);
        // Should attack G6 (can capture)
        assert_ne!((attacks & (1u64 << Square::G6.index())), 0);
    }

    #[test]
    fn test_rook_attacks_blocked() {
        let table = MagicTable::new();
        let e4 = Square::E4;

        // Block file with piece on E6
        let occupancy = 1u64 << Square::E6.index();
        let attacks = table.rook_attacks(e4, occupancy);

        // Should not attack E7, E8 (blocked by E6)
        assert_eq!((attacks & (1u64 << Square::E7.index())), 0);
        assert_eq!((attacks & (1u64 << Square::E8.index())), 0);
        // Should attack E6 (can capture)
        assert_ne!((attacks & (1u64 << Square::E6.index())), 0);
    }
}
