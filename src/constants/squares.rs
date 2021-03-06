pub const FILE_A:u8 = 0;
pub const FILE_B:u8 = 1;
pub const FILE_C:u8 = 2;
pub const FILE_D:u8 = 3;
pub const FILE_E:u8 = 4;
pub const FILE_F:u8 = 5;
pub const FILE_G:u8 = 6;
pub const FILE_H:u8 = 7;
pub const FILE_NONE:u8 = 8;

pub const FILE_SQUARES:[u8; 120] = [
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
];

pub const RANK_1:u8 = 0;
pub const RANK_2:u8 = 1;
pub const RANK_3:u8 = 2;
pub const RANK_4:u8 = 3;
pub const RANK_5:u8 = 4;
pub const RANK_6:u8 = 5;
pub const RANK_7:u8 = 6;
pub const RANK_8:u8 = 7;
pub const RANK_NONE:u8 = 8;

pub const RANK_SQUARES:[u8; 120] = [
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, RANK_1, OFFBOARD,
    OFFBOARD, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, RANK_2, OFFBOARD,
    OFFBOARD, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, RANK_3, OFFBOARD,
    OFFBOARD, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, RANK_4, OFFBOARD,
    OFFBOARD, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, RANK_5, OFFBOARD,
    OFFBOARD, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, RANK_6, OFFBOARD,
    OFFBOARD, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, RANK_7, OFFBOARD,
    OFFBOARD, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, RANK_8, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
    OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD, OFFBOARD,
];

pub const A1:u8 = 21;
pub const A2:u8 = 31;
pub const A3:u8 = 41;
pub const A4:u8 = 51;
pub const B1:u8 = 22;
pub const B2:u8 = 32;
pub const B3:u8 = 42;
pub const B4:u8 = 52;
pub const C1:u8 = 23;
pub const C2:u8 = 33;
pub const C3:u8 = 43;
pub const C4:u8 = 53;
pub const D1:u8 = 24;
pub const D2:u8 = 34;
pub const D3:u8 = 44;
pub const D4:u8 = 54;
pub const E1:u8 = 25;
pub const E2:u8 = 35;
pub const E3:u8 = 45;
pub const E4:u8 = 55;
pub const F1:u8 = 26;
pub const F2:u8 = 36;
pub const F3:u8 = 46;
pub const F4:u8 = 56;
pub const G1:u8 = 27;
pub const G2:u8 = 37;
pub const G3:u8 = 47;
pub const G4:u8 = 57;
pub const H1:u8 = 28;
pub const H2:u8 = 38;
pub const H3:u8 = 48;
pub const H4:u8 = 58;

pub const A5:u8 = 61;
pub const A6:u8 = 71;
pub const A7:u8 = 81;
pub const A8:u8 = 91;
pub const B5:u8 = 62;
pub const B6:u8 = 72;
pub const B7:u8 = 82;
pub const B8:u8 = 92;
pub const C5:u8 = 63;
pub const C6:u8 = 73;
pub const C7:u8 = 83;
pub const C8:u8 = 93;
pub const D5:u8 = 64;
pub const D6:u8 = 74;
pub const D7:u8 = 84;
pub const D8:u8 = 94;
pub const E5:u8 = 65;
pub const E6:u8 = 75;
pub const E7:u8 = 85;
pub const E8:u8 = 95;
pub const F5:u8 = 66;
pub const F6:u8 = 76;
pub const F7:u8 = 86;
pub const F8:u8 = 96;
pub const G5:u8 = 67;
pub const G6:u8 = 77;
pub const G7:u8 = 87;
pub const G8:u8 = 97;
pub const H5:u8 = 68;
pub const H6:u8 = 78;
pub const H7:u8 = 88;
pub const H8:u8 = 98;

pub const NO_SQ:u8 = 99;
pub const OFFBOARD:u8 = 100;
