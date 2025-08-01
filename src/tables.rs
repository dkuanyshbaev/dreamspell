//////////////////////////////////////////
// Dreamspell magic tables
//////////////////////////////////////////
pub const MONTH_TABLE: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 13, 44, 74];
pub const YEAR_TABLE: [u32; 52] = [
    232, 77, 182, 27, 132, 237, 82, 187, 32, 137, 242, 87, 192, 37, 142, 247, 92, 197, 42, 147,
    252, 97, 202, 47, 152, 257, 102, 207, 52, 157, 2, 107, 212, 57, 162, 7, 112, 217, 62, 167, 12,
    117, 222, 67, 172, 17, 122, 227, 72, 177, 22, 127,
];
pub const ARCHETYPE_TABLE: [(u32, u32); 260] = [
    (1, 1),
    (2, 14),
    (3, 7),
    (4, 20),
    (5, 13),
    (6, 6),
    (7, 15),
    (8, 12),
    (9, 5),
    (10, 18),
    (11, 11),
    (12, 4),
    (13, 17),
    (14, 14),
    (15, 7),
    (16, 20),
    (17, 13),
    (18, 6),
    (19, 19),
    (20, 12),
    (1, 5),
    (2, 18),
    (3, 11),
    (4, 4),
    (5, 17),
    (6, 10),
    (7, 7),
    (8, 20),
    (9, 13),
    (10, 6),
    (11, 19),
    (12, 12),
    (13, 5),
    (14, 18),
    (15, 11),
    (16, 4),
    (17, 17),
    (18, 10),
    (19, 3),
    (20, 20),
    (1, 13),
    (2, 6),
    (3, 19),
    (4, 12),
    (5, 5),
    (6, 18),
    (7, 11),
    (8, 4),
    (9, 17),
    (10, 10),
    (11, 3),
    (12, 16),
    (13, 13),
    (14, 6),
    (15, 19),
    (16, 12),
    (17, 5),
    (18, 18),
    (19, 11),
    (20, 4),
    (1, 17),
    (2, 10),
    (3, 3),
    (4, 16),
    (5, 9),
    (6, 6),
    (7, 19),
    (8, 12),
    (9, 5),
    (10, 18),
    (11, 11),
    (12, 4),
    (13, 17),
    (14, 10),
    (15, 3),
    (16, 16),
    (17, 9),
    (18, 2),
    (19, 19),
    (20, 12),
    (1, 5),
    (2, 18),
    (3, 11),
    (4, 4),
    (5, 17),
    (6, 10),
    (7, 3),
    (8, 16),
    (9, 9),
    (10, 2),
    (11, 15),
    (12, 12),
    (13, 5),
    (14, 18),
    (15, 11),
    (16, 4),
    (17, 17),
    (18, 10),
    (19, 3),
    (20, 16),
    (1, 9),
    (2, 2),
    (3, 15),
    (4, 8),
    (5, 5),
    (6, 18),
    (7, 11),
    (8, 4),
    (9, 17),
    (10, 10),
    (11, 3),
    (12, 16),
    (13, 9),
    (14, 2),
    (15, 15),
    (16, 8),
    (17, 1),
    (18, 18),
    (19, 11),
    (20, 4),
    (1, 17),
    (2, 10),
    (3, 3),
    (4, 16),
    (5, 9),
    (6, 2),
    (7, 15),
    (8, 8),
    (9, 1),
    (10, 14),
    (11, 11),
    (12, 4),
    (13, 17),
    (14, 10),
    (15, 3),
    (16, 16),
    (17, 9),
    (18, 2),
    (19, 15),
    (20, 8),
    (1, 1),
    (2, 14),
    (3, 7),
    (4, 4),
    (5, 17),
    (6, 10),
    (7, 3),
    (8, 16),
    (9, 9),
    (10, 2),
    (11, 15),
    (12, 8),
    (13, 1),
    (14, 14),
    (15, 7),
    (16, 20),
    (17, 17),
    (18, 10),
    (19, 3),
    (20, 16),
    (1, 9),
    (2, 2),
    (3, 15),
    (4, 8),
    (5, 1),
    (6, 14),
    (7, 7),
    (8, 20),
    (9, 13),
    (10, 10),
    (11, 3),
    (12, 16),
    (13, 9),
    (14, 2),
    (15, 15),
    (16, 8),
    (17, 1),
    (18, 14),
    (19, 7),
    (20, 20),
    (1, 13),
    (2, 6),
    (3, 3),
    (4, 16),
    (5, 9),
    (6, 2),
    (7, 15),
    (8, 8),
    (9, 1),
    (10, 14),
    (11, 7),
    (12, 20),
    (13, 13),
    (14, 6),
    (15, 19),
    (16, 16),
    (17, 9),
    (18, 2),
    (19, 15),
    (20, 8),
    (1, 1),
    (2, 14),
    (3, 7),
    (4, 20),
    (5, 13),
    (6, 6),
    (7, 19),
    (8, 12),
    (9, 9),
    (10, 2),
    (11, 15),
    (12, 8),
    (13, 1),
    (14, 14),
    (15, 7),
    (16, 20),
    (17, 13),
    (18, 6),
    (19, 19),
    (20, 12),
    (1, 5),
    (2, 2),
    (3, 15),
    (4, 8),
    (5, 1),
    (6, 14),
    (7, 7),
    (8, 20),
    (9, 13),
    (10, 6),
    (11, 19),
    (12, 12),
    (13, 5),
    (14, 18),
    (15, 15),
    (16, 8),
    (17, 1),
    (18, 14),
    (19, 7),
    (20, 20),
    (1, 13),
    (2, 6),
    (3, 19),
    (4, 12),
    (5, 5),
    (6, 18),
    (7, 11),
    (8, 8),
    (9, 1),
    (10, 14),
    (11, 7),
    (12, 20),
    (13, 13),
    (14, 6),
    (15, 19),
    (16, 12),
    (17, 5),
    (18, 18),
    (19, 11),
    (20, 4),
];
