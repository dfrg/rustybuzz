#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use super::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE};

static _indic_syllable_machine_actions: [i8; 44] = [
    0, 1, 0, 1, 1, 1, 2, 1, 6, 1, 7, 1, 8, 1, 9, 1, 10, 1, 11, 1, 12, 1, 13, 1, 14, 1, 15, 1, 16,
    1, 17, 1, 18, 2, 2, 3, 2, 2, 4, 2, 2, 5, 0, 0,
];
static _indic_syllable_machine_key_offsets: [i16; 140] = [
    0, 1, 7, 12, 17, 18, 24, 31, 37, 38, 43, 48, 49, 55, 62, 69, 76, 77, 82, 87, 88, 94, 100, 107,
    108, 113, 118, 119, 125, 131, 136, 137, 155, 165, 174, 182, 188, 192, 195, 196, 198, 205, 211,
    217, 225, 232, 238, 242, 249, 253, 258, 262, 271, 281, 291, 300, 308, 314, 324, 333, 341, 347,
    350, 351, 353, 360, 366, 374, 381, 387, 391, 398, 402, 406, 411, 415, 424, 434, 440, 449, 458,
    466, 472, 482, 488, 491, 492, 494, 501, 507, 515, 522, 528, 532, 541, 548, 552, 556, 561, 565,
    575, 582, 588, 598, 607, 615, 621, 631, 637, 640, 641, 643, 650, 656, 664, 671, 677, 681, 690,
    697, 701, 705, 710, 714, 729, 739, 753, 761, 765, 769, 770, 772, 782, 787, 791, 794, 795, 797,
    0, 0,
];
static _indic_syllable_machine_trans_keys: [u8; 802] = [
    8, 4, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6, 13, 4, 7, 8, 13, 5, 6, 4, 7, 8, 12, 13,
    5, 6, 4, 7, 8, 13, 5, 6, 8, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6, 13, 4, 7, 8, 13, 5, 6, 4, 7, 8, 12,
    13, 5, 6, 4, 7, 8, 12, 13, 5, 6, 4, 7, 8, 12, 13, 5, 6, 8, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6, 13,
    4, 7, 8, 13, 5, 6, 4, 7, 8, 13, 5, 6, 4, 7, 8, 12, 13, 5, 6, 8, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6,
    13, 4, 7, 8, 13, 5, 6, 4, 7, 8, 13, 5, 6, 7, 8, 13, 5, 6, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13,
    14, 15, 16, 17, 18, 10, 11, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 3, 4, 7, 8, 9, 13, 16, 5, 6, 4, 7,
    8, 9, 13, 16, 5, 6, 1, 5, 6, 8, 9, 15, 8, 9, 5, 6, 5, 8, 9, 9, 5, 9, 1, 3, 8, 9, 15, 5, 6, 1,
    8, 9, 15, 5, 6, 1, 5, 6, 8, 9, 15, 3, 4, 7, 8, 9, 13, 5, 6, 4, 7, 8, 9, 13, 5, 6, 7, 8, 9, 13,
    5, 6, 5, 8, 9, 13, 4, 7, 8, 9, 13, 5, 6, 5, 6, 8, 9, 3, 8, 9, 5, 6, 5, 6, 8, 9, 3, 4, 7, 8, 9,
    13, 16, 5, 6, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 3, 4, 5, 6, 7,
    8, 9, 13, 16, 4, 5, 6, 7, 8, 9, 13, 16, 1, 5, 6, 8, 9, 15, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 3,
    4, 7, 8, 9, 13, 16, 5, 6, 4, 7, 8, 9, 13, 16, 5, 6, 1, 5, 6, 8, 9, 15, 5, 8, 9, 9, 5, 9, 1, 3,
    8, 9, 15, 5, 6, 1, 8, 9, 15, 5, 6, 3, 4, 7, 8, 9, 13, 5, 6, 4, 7, 8, 9, 13, 5, 6, 7, 8, 9, 13,
    5, 6, 5, 8, 9, 13, 4, 7, 8, 9, 13, 5, 6, 5, 6, 8, 9, 8, 9, 5, 6, 3, 8, 9, 5, 6, 5, 6, 8, 9, 3,
    4, 7, 8, 9, 13, 16, 5, 6, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 4, 7, 8, 13, 5, 6, 3, 4, 5, 6, 7, 8,
    9, 13, 16, 3, 4, 7, 8, 9, 13, 16, 5, 6, 4, 7, 8, 9, 13, 16, 5, 6, 1, 5, 6, 8, 9, 15, 3, 4, 5,
    6, 7, 8, 9, 12, 13, 16, 1, 5, 6, 8, 9, 15, 5, 8, 9, 9, 5, 9, 1, 3, 8, 9, 15, 5, 6, 1, 8, 9, 15,
    5, 6, 3, 4, 7, 8, 9, 13, 5, 6, 4, 7, 8, 9, 13, 5, 6, 7, 8, 9, 13, 5, 6, 5, 8, 9, 13, 3, 4, 7,
    8, 9, 13, 16, 5, 6, 4, 7, 8, 9, 13, 5, 6, 5, 6, 8, 9, 8, 9, 5, 6, 3, 8, 9, 5, 6, 5, 6, 8, 9, 3,
    4, 5, 6, 7, 8, 9, 12, 13, 16, 4, 7, 8, 12, 13, 5, 6, 4, 7, 8, 13, 5, 6, 3, 4, 5, 6, 7, 8, 9,
    12, 13, 16, 3, 4, 7, 8, 9, 13, 16, 5, 6, 4, 7, 8, 9, 13, 16, 5, 6, 1, 5, 6, 8, 9, 15, 3, 4, 5,
    6, 7, 8, 9, 12, 13, 16, 1, 5, 6, 8, 9, 15, 5, 8, 9, 9, 5, 9, 1, 3, 8, 9, 15, 5, 6, 1, 8, 9, 15,
    5, 6, 3, 4, 7, 8, 9, 13, 5, 6, 4, 7, 8, 9, 13, 5, 6, 7, 8, 9, 13, 5, 6, 5, 8, 9, 13, 3, 4, 7,
    8, 9, 13, 16, 5, 6, 4, 7, 8, 9, 13, 5, 6, 5, 6, 8, 9, 8, 9, 5, 6, 3, 8, 9, 5, 6, 5, 6, 8, 9, 1,
    2, 3, 4, 5, 6, 7, 8, 9, 12, 13, 15, 16, 10, 11, 3, 4, 5, 6, 7, 8, 9, 12, 13, 16, 1, 2, 3, 4, 5,
    6, 7, 8, 9, 11, 12, 13, 15, 16, 4, 7, 8, 9, 12, 13, 5, 6, 5, 8, 9, 13, 5, 8, 9, 13, 9, 5, 9, 1,
    3, 4, 7, 8, 9, 13, 15, 5, 6, 3, 8, 9, 5, 6, 8, 9, 5, 6, 5, 8, 9, 9, 5, 9, 1, 10, 15, 0, 0,
];
static _indic_syllable_machine_single_lengths: [i8; 140] = [
    1, 4, 3, 3, 1, 4, 5, 4, 1, 3, 3, 1, 4, 5, 5, 5, 1, 3, 3, 1, 4, 4, 5, 1, 3, 3, 1, 4, 4, 3, 1,
    16, 10, 7, 6, 6, 2, 3, 1, 2, 5, 4, 6, 6, 5, 4, 4, 5, 4, 3, 4, 7, 10, 10, 9, 8, 6, 10, 7, 6, 6,
    3, 1, 2, 5, 4, 6, 5, 4, 4, 5, 4, 2, 3, 4, 7, 10, 4, 9, 7, 6, 6, 10, 6, 3, 1, 2, 5, 4, 6, 5, 4,
    4, 7, 5, 4, 2, 3, 4, 10, 5, 4, 10, 7, 6, 6, 10, 6, 3, 1, 2, 5, 4, 6, 5, 4, 4, 7, 5, 4, 2, 3, 4,
    13, 10, 14, 6, 4, 4, 1, 2, 8, 3, 2, 3, 1, 2, 3, 0, 0,
];
static _indic_syllable_machine_range_lengths: [i8; 140] = [
    0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1,
    0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0,
    1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0,
    0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
];
static _indic_syllable_machine_index_offsets: [i16; 140] = [
    0, 2, 8, 13, 18, 20, 26, 33, 39, 41, 46, 51, 53, 59, 66, 73, 80, 82, 87, 92, 94, 100, 106, 113,
    115, 120, 125, 127, 133, 139, 144, 146, 164, 175, 184, 192, 199, 203, 207, 209, 212, 219, 225,
    232, 240, 247, 253, 258, 265, 270, 275, 280, 289, 300, 311, 321, 330, 337, 348, 357, 365, 372,
    376, 378, 381, 388, 394, 402, 409, 415, 420, 427, 432, 436, 441, 446, 455, 466, 472, 482, 491,
    499, 506, 517, 524, 528, 530, 533, 540, 546, 554, 561, 567, 572, 581, 588, 593, 597, 602, 607,
    618, 625, 631, 642, 651, 659, 666, 677, 684, 688, 690, 693, 700, 706, 714, 721, 727, 732, 741,
    748, 753, 757, 762, 767, 782, 793, 808, 816, 821, 826, 828, 831, 841, 846, 850, 854, 856, 859,
    0, 0,
];
static _indic_syllable_machine_cond_targs: [i16; 1003] = [
    37, 31, 42, 43, 46, 43, 2, 31, 43, 4, 43, 2, 31, 43, 46, 43, 2, 31, 43, 31, 50, 43, 46, 43, 2,
    31, 42, 43, 46, 51, 43, 2, 31, 60, 66, 69, 66, 9, 31, 61, 31, 66, 11, 66, 9, 31, 66, 69, 66, 9,
    31, 66, 31, 74, 66, 69, 66, 9, 31, 60, 66, 69, 75, 66, 9, 31, 60, 66, 69, 78, 66, 9, 31, 83,
    89, 92, 93, 89, 17, 31, 84, 31, 89, 19, 89, 17, 31, 89, 92, 89, 17, 31, 89, 31, 83, 89, 92, 89,
    17, 31, 98, 89, 92, 89, 17, 31, 107, 113, 116, 117, 113, 24, 31, 108, 31, 113, 26, 113, 24, 31,
    113, 116, 113, 24, 31, 113, 31, 107, 113, 116, 113, 24, 31, 122, 113, 116, 113, 24, 31, 89,
    127, 89, 17, 31, 134, 31, 32, 53, 79, 81, 100, 101, 89, 92, 85, 93, 89, 123, 124, 94, 132, 137,
    102, 31, 33, 35, 6, 52, 43, 46, 38, 51, 43, 47, 31, 34, 35, 43, 46, 38, 43, 47, 1, 31, 35, 43,
    46, 38, 43, 47, 1, 31, 32, 36, 40, 37, 38, 32, 31, 37, 38, 0, 31, 38, 39, 38, 31, 38, 31, 38,
    38, 31, 32, 41, 37, 38, 32, 0, 31, 32, 37, 38, 32, 0, 31, 32, 0, 40, 37, 38, 32, 31, 44, 45,
    43, 46, 38, 43, 3, 31, 45, 43, 46, 38, 43, 3, 31, 43, 46, 38, 43, 3, 31, 38, 39, 38, 43, 31,
    48, 43, 46, 38, 43, 5, 31, 36, 49, 37, 38, 31, 36, 37, 38, 0, 31, 0, 49, 37, 38, 31, 33, 35,
    43, 46, 38, 43, 47, 1, 31, 33, 35, 6, 1, 43, 46, 38, 51, 43, 47, 31, 54, 56, 14, 77, 66, 69,
    62, 78, 66, 70, 31, 55, 56, 7, 77, 66, 69, 62, 66, 70, 31, 56, 7, 77, 66, 69, 62, 66, 70, 31,
    57, 72, 64, 61, 62, 57, 31, 58, 56, 13, 76, 66, 69, 62, 75, 66, 70, 31, 59, 56, 66, 69, 62, 66,
    70, 7, 31, 56, 66, 69, 62, 66, 70, 7, 31, 57, 8, 64, 61, 62, 57, 31, 62, 63, 62, 31, 62, 31,
    62, 62, 31, 57, 65, 61, 62, 57, 8, 31, 57, 61, 62, 57, 8, 31, 67, 68, 66, 69, 62, 66, 10, 31,
    68, 66, 69, 62, 66, 10, 31, 66, 69, 62, 66, 10, 31, 62, 63, 62, 66, 31, 71, 66, 69, 62, 66, 12,
    31, 72, 73, 61, 62, 31, 61, 62, 8, 31, 72, 61, 62, 8, 31, 8, 73, 61, 62, 31, 58, 56, 66, 69,
    62, 66, 70, 7, 31, 58, 56, 13, 7, 66, 69, 62, 75, 66, 70, 31, 60, 66, 69, 66, 9, 31, 54, 56, 7,
    77, 66, 69, 62, 66, 70, 31, 80, 81, 89, 92, 85, 89, 94, 20, 31, 81, 89, 92, 85, 89, 94, 20, 31,
    82, 96, 87, 84, 85, 82, 31, 79, 81, 15, 99, 89, 92, 85, 93, 89, 94, 31, 82, 16, 87, 84, 85, 82,
    31, 85, 86, 85, 31, 85, 31, 85, 85, 31, 82, 88, 84, 85, 82, 16, 31, 82, 84, 85, 82, 16, 31, 90,
    91, 89, 92, 85, 89, 18, 31, 91, 89, 92, 85, 89, 18, 31, 89, 92, 85, 89, 18, 31, 85, 86, 85, 89,
    31, 79, 81, 89, 92, 85, 89, 94, 20, 31, 95, 89, 92, 85, 89, 21, 31, 96, 97, 84, 85, 31, 84, 85,
    16, 31, 96, 84, 85, 16, 31, 16, 97, 84, 85, 31, 79, 81, 15, 20, 89, 92, 85, 93, 89, 94, 31, 83,
    89, 92, 93, 89, 17, 31, 83, 89, 92, 89, 17, 31, 103, 105, 22, 27, 113, 116, 109, 117, 113, 118,
    31, 104, 105, 113, 116, 109, 113, 118, 27, 31, 105, 113, 116, 109, 113, 118, 27, 31, 106, 120,
    111, 108, 109, 106, 31, 103, 105, 22, 102, 113, 116, 109, 117, 113, 118, 31, 106, 23, 111, 108,
    109, 106, 31, 109, 110, 109, 31, 109, 31, 109, 109, 31, 106, 112, 108, 109, 106, 23, 31, 106,
    108, 109, 106, 23, 31, 114, 115, 113, 116, 109, 113, 25, 31, 115, 113, 116, 109, 113, 25, 31,
    113, 116, 109, 113, 25, 31, 109, 110, 109, 113, 31, 103, 105, 113, 116, 109, 113, 118, 27, 31,
    119, 113, 116, 109, 113, 28, 31, 120, 121, 108, 109, 31, 108, 109, 23, 31, 120, 108, 109, 23,
    31, 23, 121, 108, 109, 31, 32, 53, 79, 81, 15, 20, 89, 92, 85, 93, 89, 32, 94, 102, 31, 33,
    125, 6, 52, 43, 46, 38, 51, 43, 47, 31, 32, 53, 79, 81, 126, 131, 89, 128, 129, 102, 93, 89,
    32, 94, 31, 83, 89, 128, 38, 93, 89, 29, 31, 38, 39, 38, 89, 31, 129, 130, 129, 89, 31, 129,
    31, 129, 129, 31, 32, 41, 83, 89, 128, 38, 89, 32, 29, 31, 133, 134, 135, 30, 31, 134, 135, 30,
    31, 135, 136, 135, 31, 135, 31, 135, 135, 31, 32, 102, 32, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 31,
    31, 31, 31, 31, 31, 31, 31, 31, 31, 31, 0, 0,
];
static _indic_syllable_machine_cond_actions: [i8; 1003] = [
    0, 21, 5, 5, 0, 5, 0, 21, 5, 0, 5, 0, 21, 5, 0, 5, 0, 21, 5, 21, 5, 5, 0, 5, 0, 21, 5, 5, 0, 5,
    5, 0, 21, 5, 5, 0, 5, 0, 23, 0, 23, 5, 0, 5, 0, 23, 5, 0, 5, 0, 23, 5, 23, 5, 5, 0, 5, 0, 23,
    5, 5, 0, 5, 5, 0, 23, 5, 5, 0, 5, 5, 0, 23, 5, 36, 0, 36, 36, 0, 29, 0, 29, 36, 0, 36, 0, 31,
    36, 0, 36, 0, 29, 36, 31, 5, 36, 0, 36, 0, 29, 5, 36, 0, 36, 0, 29, 5, 5, 0, 5, 5, 0, 25, 0,
    25, 5, 0, 5, 0, 25, 5, 0, 5, 0, 25, 5, 25, 5, 5, 0, 5, 0, 25, 5, 5, 0, 5, 0, 25, 36, 0, 36, 0,
    21, 0, 27, 5, 5, 36, 0, 39, 39, 36, 0, 0, 36, 36, 36, 5, 36, 5, 0, 5, 7, 5, 0, 0, 5, 5, 0, 0,
    5, 5, 5, 9, 5, 0, 5, 0, 0, 5, 5, 0, 9, 0, 5, 0, 0, 5, 5, 0, 9, 5, 5, 5, 0, 0, 5, 9, 0, 0, 0, 9,
    0, 0, 0, 9, 0, 9, 0, 0, 9, 5, 5, 0, 0, 5, 0, 9, 5, 0, 0, 5, 0, 9, 5, 0, 5, 0, 0, 5, 9, 5, 5, 5,
    0, 0, 5, 0, 9, 5, 5, 0, 0, 5, 0, 9, 5, 0, 0, 5, 0, 9, 0, 0, 0, 5, 9, 0, 5, 0, 0, 5, 0, 9, 5, 5,
    0, 0, 9, 5, 0, 0, 0, 9, 0, 5, 0, 0, 9, 5, 0, 5, 0, 0, 5, 5, 0, 9, 5, 0, 0, 0, 5, 0, 0, 5, 5, 5,
    9, 5, 0, 0, 5, 5, 0, 0, 5, 5, 5, 11, 5, 0, 0, 5, 5, 0, 0, 5, 5, 11, 0, 0, 5, 5, 0, 0, 5, 5, 11,
    5, 5, 5, 0, 0, 5, 11, 5, 0, 0, 5, 5, 0, 0, 5, 5, 5, 11, 5, 0, 5, 0, 0, 5, 5, 0, 11, 0, 5, 0, 0,
    5, 5, 0, 11, 5, 0, 5, 0, 0, 5, 11, 0, 0, 0, 11, 0, 11, 0, 0, 11, 5, 5, 0, 0, 5, 0, 11, 5, 0, 0,
    5, 0, 11, 5, 5, 5, 0, 0, 5, 0, 11, 5, 5, 0, 0, 5, 0, 11, 5, 0, 0, 5, 0, 11, 0, 0, 0, 5, 11, 0,
    5, 0, 0, 5, 0, 11, 5, 5, 0, 0, 11, 0, 0, 0, 11, 5, 0, 0, 0, 11, 0, 5, 0, 0, 11, 5, 0, 5, 0, 0,
    5, 5, 0, 11, 5, 0, 0, 0, 5, 0, 0, 5, 5, 5, 11, 5, 5, 0, 5, 0, 11, 5, 0, 0, 5, 5, 0, 0, 5, 5,
    11, 36, 0, 36, 0, 0, 36, 36, 0, 17, 0, 36, 0, 0, 36, 36, 0, 17, 36, 5, 5, 0, 0, 36, 17, 36, 0,
    0, 36, 36, 0, 0, 36, 36, 36, 17, 36, 0, 5, 0, 0, 36, 17, 0, 0, 0, 17, 0, 17, 0, 0, 17, 36, 5,
    0, 0, 36, 0, 17, 36, 0, 0, 36, 0, 17, 36, 36, 36, 0, 0, 36, 0, 17, 36, 36, 0, 0, 36, 0, 17, 36,
    0, 0, 36, 0, 17, 0, 0, 0, 36, 17, 36, 0, 36, 0, 0, 36, 36, 0, 17, 0, 36, 0, 0, 36, 0, 17, 5, 5,
    0, 0, 17, 0, 0, 0, 17, 5, 0, 0, 0, 17, 0, 5, 0, 0, 17, 36, 0, 0, 0, 36, 0, 0, 36, 36, 36, 17,
    5, 36, 0, 36, 36, 0, 19, 5, 36, 0, 36, 0, 19, 5, 0, 0, 0, 5, 0, 0, 5, 5, 5, 13, 5, 0, 5, 0, 0,
    5, 5, 0, 13, 0, 5, 0, 0, 5, 5, 0, 13, 5, 5, 5, 0, 0, 5, 13, 5, 0, 0, 5, 5, 0, 0, 5, 5, 5, 13,
    5, 0, 5, 0, 0, 5, 13, 0, 0, 0, 13, 0, 13, 0, 0, 13, 5, 5, 0, 0, 5, 0, 13, 5, 0, 0, 5, 0, 13, 5,
    5, 5, 0, 0, 5, 0, 13, 5, 5, 0, 0, 5, 0, 13, 5, 0, 0, 5, 0, 13, 0, 0, 0, 5, 13, 5, 0, 5, 0, 0,
    5, 5, 0, 13, 0, 5, 0, 0, 5, 0, 13, 5, 5, 0, 0, 13, 0, 0, 0, 13, 5, 0, 0, 0, 13, 0, 5, 0, 0, 13,
    5, 5, 36, 0, 0, 0, 36, 0, 0, 36, 36, 5, 36, 5, 17, 5, 0, 0, 5, 5, 0, 0, 5, 5, 5, 9, 5, 5, 36,
    0, 33, 33, 36, 0, 0, 5, 36, 36, 5, 36, 9, 5, 36, 0, 0, 36, 36, 0, 9, 0, 0, 0, 36, 9, 0, 0, 0,
    36, 9, 0, 9, 0, 0, 9, 5, 5, 5, 36, 0, 0, 36, 5, 0, 9, 5, 0, 0, 0, 15, 0, 0, 0, 15, 0, 0, 0, 15,
    0, 15, 0, 0, 15, 5, 5, 5, 19, 21, 21, 21, 21, 21, 21, 21, 23, 23, 23, 23, 23, 23, 23, 23, 29,
    29, 31, 29, 31, 29, 29, 25, 25, 25, 25, 25, 25, 25, 21, 27, 0, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
    11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17,
    17, 17, 17, 17, 17, 17, 17, 19, 19, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13, 13,
    13, 13, 13, 13, 13, 13, 17, 9, 9, 9, 9, 9, 9, 9, 9, 15, 15, 15, 15, 15, 19, 0, 0,
];
static _indic_syllable_machine_to_state_actions: [i8; 140] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _indic_syllable_machine_from_state_actions: [i8; 140] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _indic_syllable_machine_eof_trans: [i16; 140] = [
    864, 865, 866, 867, 868, 869, 870, 871, 872, 873, 874, 875, 876, 877, 878, 879, 880, 881, 882,
    883, 884, 885, 886, 887, 888, 889, 890, 891, 892, 893, 894, 895, 896, 897, 898, 899, 900, 901,
    902, 903, 904, 905, 906, 907, 908, 909, 910, 911, 912, 913, 914, 915, 916, 917, 918, 919, 920,
    921, 922, 923, 924, 925, 926, 927, 928, 929, 930, 931, 932, 933, 934, 935, 936, 937, 938, 939,
    940, 941, 942, 943, 944, 945, 946, 947, 948, 949, 950, 951, 952, 953, 954, 955, 956, 957, 958,
    959, 960, 961, 962, 963, 964, 965, 966, 967, 968, 969, 970, 971, 972, 973, 974, 975, 976, 977,
    978, 979, 980, 981, 982, 983, 984, 985, 986, 987, 988, 989, 990, 991, 992, 993, 994, 995, 996,
    997, 998, 999, 1000, 1001, 0, 0,
];
static indic_syllable_machine_start: i32 = 31;
static indic_syllable_machine_first_final: i32 = 31;
static indic_syllable_machine_error: i32 = -1;
static indic_syllable_machine_en_main: i32 = 31;
#[derive(Clone, Copy)]
pub enum SyllableType {
    ConsonantSyllable = 0,
    VowelSyllable,
    StandaloneCluster,
    SymbolCluster,
    BrokenCluster,
    NonIndicCluster,
}

pub fn find_syllables_indic(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let mut ts = 0;
    let mut te = 0;
    let mut act = 0;
    let mut p = 0;
    let pe = buffer.len;
    let eof = buffer.len;
    let mut syllable_serial = 1u8;

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts, te, &mut syllable_serial, $kind, buffer)
        }};
    }

    {
        cs = (indic_syllable_machine_start) as i32;
        ts = 0;
        te = 0;
        act = 0;
    }

    {
        let mut _klen = 0;
        let mut _trans = 0;
        let mut _keys: i32 = 0;
        let mut _acts: i32 = 0;
        let mut _nacts = 0;
        let mut __have = 0;
        '_resume: while (p != pe || p == eof) {
            '_again: while (true) {
                _acts = (_indic_syllable_machine_from_state_actions[(cs) as usize]) as i32;
                _nacts = (_indic_syllable_machine_actions[(_acts) as usize]) as u32;
                _acts += 1;
                while (_nacts > 0) {
                    match (_indic_syllable_machine_actions[(_acts) as usize]) {
                        1 => {
                            ts = p;
                        }

                        _ => {}
                    }
                    _nacts -= 1;
                    _acts += 1;
                }
                if (p == eof) {
                    {
                        if (_indic_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_indic_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (_indic_syllable_machine_key_offsets[(cs) as usize]) as i32;
                        _trans = (_indic_syllable_machine_index_offsets[(cs) as usize]) as u32;
                        _klen = (_indic_syllable_machine_single_lengths[(cs) as usize]) as i32;
                        __have = 0;
                        if (_klen > 0) {
                            {
                                let mut _lower: i32 = _keys;
                                let mut _upper: i32 = _keys + _klen - 1;
                                let mut _mid: i32 = 0;
                                while (true) {
                                    if (_upper < _lower) {
                                        {
                                            _keys += _klen;
                                            _trans += (_klen) as u32;
                                            break;
                                        }
                                    }
                                    _mid = _lower + ((_upper - _lower) >> 1);
                                    if ((buffer.info[p].indic_category() as u8)
                                        < _indic_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 1;
                                    } else if ((buffer.info[p].indic_category() as u8)
                                        > _indic_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _lower = _mid + 1;
                                    } else {
                                        {
                                            __have = 1;
                                            _trans += (_mid - _keys) as u32;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        _klen = (_indic_syllable_machine_range_lengths[(cs) as usize]) as i32;
                        if (__have == 0 && _klen > 0) {
                            {
                                let mut _lower: i32 = _keys;
                                let mut _upper: i32 = _keys + (_klen << 1) - 2;
                                let mut _mid: i32 = 0;
                                while (true) {
                                    if (_upper < _lower) {
                                        {
                                            _trans += (_klen) as u32;
                                            break;
                                        }
                                    }
                                    _mid = _lower + (((_upper - _lower) >> 1) & !1);
                                    if ((buffer.info[p].indic_category() as u8)
                                        < _indic_syllable_machine_trans_keys[(_mid) as usize])
                                    {
                                        _upper = _mid - 2;
                                    } else if ((buffer.info[p].indic_category() as u8)
                                        > _indic_syllable_machine_trans_keys[(_mid + 1) as usize])
                                    {
                                        _lower = _mid + 2;
                                    } else {
                                        {
                                            _trans += ((_mid - _keys) >> 1) as u32;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                cs = (_indic_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_indic_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        _acts = (_indic_syllable_machine_cond_actions[(_trans) as usize]) as i32;
                        _nacts = (_indic_syllable_machine_actions[(_acts) as usize]) as u32;
                        _acts += 1;
                        while (_nacts > 0) {
                            match (_indic_syllable_machine_actions[(_acts) as usize]) {
                                2 => {
                                    te = p + 1;
                                }
                                3 => {
                                    act = 1;
                                }
                                4 => {
                                    act = 5;
                                }
                                5 => {
                                    act = 6;
                                }
                                6 => {
                                    te = p + 1;
                                    {
                                        found_syllable!(SyllableType::NonIndicCluster);
                                    }
                                }
                                7 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::ConsonantSyllable);
                                    }
                                }
                                8 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::VowelSyllable);
                                    }
                                }
                                9 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::StandaloneCluster);
                                    }
                                }
                                10 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::SymbolCluster);
                                    }
                                }
                                11 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::BrokenCluster);
                                        buffer.scratch_flags |=
                                            HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                    }
                                }
                                12 => {
                                    te = p;
                                    p = p - 1;
                                    {
                                        found_syllable!(SyllableType::NonIndicCluster);
                                    }
                                }
                                13 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::ConsonantSyllable);
                                    }
                                }
                                14 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::VowelSyllable);
                                    }
                                }
                                15 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::StandaloneCluster);
                                    }
                                }
                                16 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::SymbolCluster);
                                    }
                                }
                                17 => {
                                    p = (te) - 1;
                                    {
                                        found_syllable!(SyllableType::BrokenCluster);
                                        buffer.scratch_flags |=
                                            HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                    }
                                }
                                18 => match (act) {
                                    1 => {
                                        p = (te) - 1;
                                        {
                                            found_syllable!(SyllableType::ConsonantSyllable);
                                        }
                                    }
                                    5 => {
                                        p = (te) - 1;
                                        {
                                            found_syllable!(SyllableType::BrokenCluster);
                                            buffer.scratch_flags |=
                                                HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                        }
                                    }
                                    6 => {
                                        p = (te) - 1;
                                        {
                                            found_syllable!(SyllableType::NonIndicCluster);
                                        }
                                    }

                                    _ => {}
                                },

                                _ => {}
                            }
                            _nacts -= 1;
                            _acts += 1;
                        }
                    }
                }
                break '_again;
            }
            if (p == eof) {
                {
                    if (cs >= 31) {
                        break '_resume;
                    }
                }
            } else {
                {
                    _acts = (_indic_syllable_machine_to_state_actions[(cs) as usize]) as i32;
                    _nacts = (_indic_syllable_machine_actions[(_acts) as usize]) as u32;
                    _acts += 1;
                    while (_nacts > 0) {
                        match (_indic_syllable_machine_actions[(_acts) as usize]) {
                            0 => {
                                ts = 0;
                            }

                            _ => {}
                        }
                        _nacts -= 1;
                        _acts += 1;
                    }
                    p += 1;
                    continue '_resume;
                }
            }
            break '_resume;
        }
    }
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &mut hb_buffer_t,
) {
    for i in start..end {
        buffer.info[i].set_syllable((*syllable_serial << 4) | kind as u8);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}
