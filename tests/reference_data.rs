// Reference data from https://www.chessprogramming.org/Perft_Results
// Kiwipete depth 2 divide:

/*
a2a3: 43
a2a4: 44
b2b3: 42
c2c3: 42
c2c4: 42
d5d6: 41
d5e6: 46
e4e5: ???  <-- MISSING!
g2g3: 42
g2g4: 42
g2h3: 43
*/

// We need to verify if e4e5 should exist. Looking at position:
// r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1
// 
// Rank 5: ...PN... means pawn on D5, Knight on E5
// Rank 4: .p..P... means black pawn b4, White Pawn E4
//
// So E4 pawn cannot move to E5 (occupied by own knight)
// But maybe there are other moves we're missing?
