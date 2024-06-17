#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use core::cell::Cell;

use super::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE};
use super::hb_glyph_info_t;
use super::machine_cursor::MachineCursor;
use super::ot_layout::*;
use super::ot_shaper_use::category;

static _use_syllable_machine_trans_keys: [u8; 254] = [
    0, 39, 5, 39, 5, 39, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8, 32, 8, 32, 8, 8, 8, 34, 8, 34, 8,
    34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6, 39, 6, 39, 6, 39, 5, 39, 1, 8, 1,
    34, 5, 39, 8, 28, 8, 28, 5, 39, 5, 39, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8, 32, 8, 32, 8, 8,
    8, 34, 8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6, 39, 6, 39, 6,
    39, 5, 39, 1, 8, 1, 8, 1, 34, 7, 8, 3, 8, 5, 39, 5, 39, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8,
    32, 8, 32, 8, 8, 8, 34, 8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6,
    39, 6, 39, 6, 39, 5, 39, 1, 8, 1, 8, 1, 34, 5, 39, 5, 39, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8,
    32, 8, 32, 8, 8, 8, 34, 8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6,
    39, 6, 39, 6, 39, 5, 39, 1, 8, 1, 34, 3, 8, 7, 8, 1, 39, 5, 39, 8, 28, 8, 28, 1, 4, 8, 38, 8,
    38, 8, 37, 0, 0,
];
static _use_syllable_machine_char_class: [i8; 56] = [
    0, 1, 2, 2, 3, 4, 2, 2, 2, 2, 2, 5, 6, 7, 8, 2, 2, 2, 9, 2, 2, 2, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 2, 24, 25, 26, 2, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
    39, 0, 0,
];
static _use_syllable_machine_index_offsets: [i16; 128] = [
    0, 40, 75, 110, 149, 176, 202, 228, 254, 279, 304, 305, 332, 359, 386, 394, 421, 453, 485, 517,
    549, 583, 615, 649, 683, 717, 752, 760, 794, 829, 850, 871, 906, 941, 980, 1007, 1033, 1059,
    1085, 1110, 1135, 1136, 1163, 1190, 1217, 1225, 1252, 1284, 1316, 1348, 1380, 1414, 1446, 1480,
    1514, 1548, 1583, 1591, 1599, 1633, 1635, 1641, 1676, 1711, 1750, 1777, 1803, 1829, 1855, 1880,
    1905, 1906, 1933, 1960, 1987, 1995, 2022, 2054, 2086, 2118, 2150, 2184, 2216, 2250, 2284, 2318,
    2353, 2361, 2369, 2403, 2438, 2473, 2512, 2539, 2565, 2591, 2617, 2642, 2667, 2668, 2695, 2722,
    2749, 2757, 2784, 2816, 2848, 2880, 2912, 2946, 2978, 3012, 3046, 3080, 3115, 3123, 3157, 3163,
    3165, 3204, 3239, 3260, 3281, 3285, 3316, 3347, 0, 0,
];
static _use_syllable_machine_indices: [i16; 3379] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 31, 35, 3, 36, 3, 37, 39, 40, 38, 41, 38, 42, 43, 44, 45, 46,
    47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 38, 61, 62, 63, 64, 61, 38, 38, 38, 38,
    65, 39, 40, 38, 41, 38, 42, 43, 44, 45, 46, 47, 48, 49, 50, 52, 52, 53, 54, 55, 56, 57, 58, 38,
    38, 38, 61, 62, 63, 64, 61, 38, 38, 38, 38, 65, 39, 38, 38, 38, 38, 38, 38, 41, 38, 38, 43, 44,
    45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38,
    38, 38, 43, 41, 38, 38, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 56, 57, 58, 38, 38,
    38, 38, 62, 63, 64, 66, 41, 38, 38, 38, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 62, 63, 64, 41, 38, 38, 38, 38, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 62, 63, 64, 41, 38, 38, 38, 38, 38, 46, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 38, 38, 62, 63, 64, 41, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 62, 63, 41, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 63, 41, 41, 38, 38, 38, 44, 45, 46, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 41, 38, 38, 38, 44, 45,
    46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 41, 38, 38,
    38, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 58, 38, 38, 38, 38, 62, 63, 64, 66,
    67, 38, 38, 38, 38, 38, 38, 41, 41, 38, 38, 38, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 62, 63, 64, 66, 41, 38, 42, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38,
    53, 54, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 41, 38, 38, 43, 44,
    45, 46, 38, 38, 38, 38, 38, 38, 53, 54, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38,
    38, 38, 43, 41, 38, 38, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 54, 55, 56, 57, 58, 38, 38,
    38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 41, 38, 38, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38,
    38, 38, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 68, 38, 41, 38, 42,
    43, 44, 45, 46, 38, 48, 49, 38, 38, 38, 53, 54, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66,
    38, 38, 38, 38, 43, 41, 38, 38, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 56, 57, 58,
    38, 38, 38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 68, 38, 41, 38, 42, 43, 44, 45, 46, 38, 38,
    49, 38, 38, 38, 53, 54, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 68,
    38, 41, 38, 42, 43, 44, 45, 46, 38, 38, 38, 38, 38, 38, 53, 54, 55, 56, 57, 58, 38, 38, 38, 38,
    62, 63, 64, 66, 38, 38, 38, 38, 43, 68, 38, 41, 38, 42, 43, 44, 45, 46, 47, 48, 49, 38, 38, 38,
    53, 54, 55, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66, 38, 38, 38, 38, 43, 39, 40, 38, 41, 38,
    42, 43, 44, 45, 46, 47, 48, 49, 50, 38, 52, 53, 54, 55, 56, 57, 58, 38, 38, 38, 61, 62, 63, 64,
    61, 38, 38, 38, 38, 65, 39, 38, 38, 38, 38, 38, 38, 41, 39, 38, 38, 38, 38, 38, 38, 41, 38, 38,
    43, 44, 45, 46, 38, 38, 38, 38, 38, 38, 38, 38, 38, 56, 57, 58, 38, 38, 38, 38, 62, 63, 64, 66,
    39, 40, 38, 41, 38, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 38, 38,
    38, 61, 62, 63, 64, 61, 38, 38, 38, 38, 65, 41, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 59, 60, 41, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 60, 70, 71, 69, 72, 69, 73, 74, 75, 76, 77, 78, 79, 80, 81, 2, 82, 83, 84, 85,
    86, 87, 88, 69, 69, 69, 89, 90, 91, 92, 93, 69, 69, 69, 69, 94, 70, 71, 69, 72, 69, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 82, 83, 84, 85, 86, 87, 88, 69, 69, 69, 89, 90, 91, 92, 93, 69, 69,
    69, 69, 94, 70, 69, 69, 69, 69, 69, 69, 72, 69, 69, 74, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 72, 69, 69, 74, 75, 76,
    77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 72, 69, 69,
    69, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 90, 91, 92, 72,
    69, 69, 69, 69, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 90, 91,
    92, 72, 69, 69, 69, 69, 69, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    90, 91, 92, 72, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 90, 91, 72, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 91, 72, 72, 69, 69, 69, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 86, 87,
    88, 69, 69, 69, 69, 90, 91, 92, 95, 72, 69, 69, 69, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 72, 69, 69, 69, 75, 76, 77, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 88, 69, 69, 69, 69, 90, 91, 92, 95, 97, 96, 96, 96, 96, 96, 96, 98, 72,
    69, 69, 69, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 90, 91,
    92, 95, 72, 69, 73, 74, 75, 76, 77, 69, 69, 69, 69, 69, 69, 83, 84, 85, 86, 87, 88, 69, 69, 69,
    69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 72, 69, 69, 74, 75, 76, 77, 69, 69, 69, 69, 69, 69, 83,
    84, 85, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 72, 69, 69, 74, 75, 76,
    77, 69, 69, 69, 69, 69, 69, 69, 84, 85, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69,
    69, 74, 72, 69, 69, 74, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 85, 86, 87, 88, 69, 69, 69,
    69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 99, 69, 72, 69, 73, 74, 75, 76, 77, 69, 79, 80, 69, 69,
    69, 83, 84, 85, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 72, 69, 69, 74,
    75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69,
    69, 69, 69, 74, 99, 69, 72, 69, 73, 74, 75, 76, 77, 69, 69, 80, 69, 69, 69, 83, 84, 85, 86, 87,
    88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 99, 69, 72, 69, 73, 74, 75, 76, 77, 69,
    69, 69, 69, 69, 69, 83, 84, 85, 86, 87, 88, 69, 69, 69, 69, 90, 91, 92, 95, 69, 69, 69, 69, 74,
    99, 69, 72, 69, 73, 74, 75, 76, 77, 78, 79, 80, 69, 69, 69, 83, 84, 85, 86, 87, 88, 69, 69, 69,
    69, 90, 91, 92, 95, 69, 69, 69, 69, 74, 70, 71, 69, 72, 69, 73, 74, 75, 76, 77, 78, 79, 80, 81,
    69, 82, 83, 84, 85, 86, 87, 88, 69, 69, 69, 89, 90, 91, 92, 93, 69, 69, 69, 69, 94, 70, 100,
    100, 100, 100, 100, 100, 101, 70, 96, 96, 96, 96, 96, 96, 98, 70, 69, 69, 69, 69, 69, 69, 72,
    69, 69, 74, 75, 76, 77, 69, 69, 69, 69, 69, 69, 69, 69, 69, 86, 87, 88, 69, 69, 69, 69, 90, 91,
    92, 95, 103, 104, 4, 105, 105, 105, 105, 106, 107, 108, 69, 72, 69, 109, 110, 111, 112, 113,
    114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 59, 60, 69, 126, 127, 128, 129,
    130, 69, 69, 69, 69, 131, 107, 108, 69, 72, 69, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    119, 119, 120, 121, 122, 123, 124, 125, 69, 69, 69, 126, 127, 128, 129, 130, 69, 69, 69, 69,
    131, 107, 69, 69, 69, 69, 69, 69, 72, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110, 72, 69, 69,
    110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 123, 124, 125, 69, 69, 69, 69, 127,
    128, 129, 132, 72, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 127, 128, 129, 72, 69, 69, 69, 69, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 127, 128, 129, 72, 69, 69, 69, 69, 69, 113, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 127, 128, 129, 72, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 127, 128, 72, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 128, 72, 72, 69, 69, 69,
    111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 123, 124, 125, 69, 69, 69, 69, 127, 128,
    129, 132, 72, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 124, 125, 69,
    69, 69, 69, 127, 128, 129, 132, 72, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 125, 69, 69, 69, 69, 127, 128, 129, 132, 133, 96, 96, 96, 96, 96, 96, 98, 72, 69,
    69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 127,
    128, 129, 132, 72, 69, 109, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 120, 121, 122, 123,
    124, 125, 69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110, 72, 69, 69, 110, 111, 112,
    113, 69, 69, 69, 69, 69, 69, 120, 121, 122, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129, 132,
    69, 69, 69, 69, 110, 72, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 121, 122, 123,
    124, 125, 69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110, 72, 69, 69, 110, 111, 112,
    113, 69, 69, 69, 69, 69, 69, 69, 69, 122, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129, 132,
    69, 69, 69, 69, 110, 134, 69, 72, 69, 109, 110, 111, 112, 113, 69, 115, 116, 69, 69, 69, 120,
    121, 122, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110, 72, 69, 69,
    110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 123, 124, 125, 69, 69, 69, 69, 127,
    128, 129, 132, 69, 69, 69, 69, 110, 134, 69, 72, 69, 109, 110, 111, 112, 113, 69, 69, 116, 69,
    69, 69, 120, 121, 122, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110,
    134, 69, 72, 69, 109, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 120, 121, 122, 123, 124, 125,
    69, 69, 69, 69, 127, 128, 129, 132, 69, 69, 69, 69, 110, 134, 69, 72, 69, 109, 110, 111, 112,
    113, 114, 115, 116, 69, 69, 69, 120, 121, 122, 123, 124, 125, 69, 69, 69, 69, 127, 128, 129,
    132, 69, 69, 69, 69, 110, 107, 108, 69, 72, 69, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    69, 119, 120, 121, 122, 123, 124, 125, 69, 69, 69, 126, 127, 128, 129, 130, 69, 69, 69, 69,
    131, 107, 100, 100, 100, 100, 100, 100, 101, 107, 96, 96, 96, 96, 96, 96, 98, 107, 69, 69, 69,
    69, 69, 69, 72, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 123, 124, 125,
    69, 69, 69, 69, 127, 128, 129, 132, 107, 108, 69, 72, 69, 109, 110, 111, 112, 113, 114, 115,
    116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 69, 69, 69, 126, 127, 128, 129, 130, 69, 69,
    69, 69, 131, 6, 7, 135, 9, 135, 11, 12, 13, 14, 15, 16, 17, 18, 19, 21, 21, 22, 23, 24, 25, 26,
    27, 135, 135, 135, 31, 32, 33, 34, 31, 135, 135, 135, 135, 37, 6, 135, 135, 135, 135, 135, 135,
    9, 135, 135, 12, 13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 25, 26, 27, 135, 135,
    135, 135, 32, 33, 34, 136, 135, 135, 135, 135, 12, 9, 135, 135, 12, 13, 14, 15, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 9, 135, 135,
    135, 13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 32, 33, 34, 9, 135, 135, 135, 135, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 32, 33, 34, 9, 135, 135, 135, 135, 135, 15, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 32, 33, 34, 9, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    32, 33, 9, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 33, 9, 9, 135, 135, 135, 13, 14, 15, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 9, 135, 135, 135, 13, 14,
    15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 26, 27, 135, 135, 135, 135, 32, 33, 34,
    136, 9, 135, 135, 135, 13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 27,
    135, 135, 135, 135, 32, 33, 34, 136, 137, 135, 135, 135, 135, 135, 135, 9, 9, 135, 135, 135,
    13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 32,
    33, 34, 136, 9, 135, 11, 12, 13, 14, 15, 135, 135, 135, 135, 135, 135, 22, 23, 24, 25, 26, 27,
    135, 135, 135, 135, 32, 33, 34, 136, 135, 135, 135, 135, 12, 9, 135, 135, 12, 13, 14, 15, 135,
    135, 135, 135, 135, 135, 22, 23, 24, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 135, 135,
    135, 135, 12, 9, 135, 135, 12, 13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 23, 24, 25, 26,
    27, 135, 135, 135, 135, 32, 33, 34, 136, 135, 135, 135, 135, 12, 9, 135, 135, 12, 13, 14, 15,
    135, 135, 135, 135, 135, 135, 135, 135, 24, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136,
    135, 135, 135, 135, 12, 138, 135, 9, 135, 11, 12, 13, 14, 15, 135, 17, 18, 135, 135, 135, 22,
    23, 24, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 135, 135, 135, 135, 12, 9, 135, 135,
    12, 13, 14, 15, 135, 135, 135, 135, 135, 135, 135, 135, 135, 25, 26, 27, 135, 135, 135, 135,
    32, 33, 34, 136, 135, 135, 135, 135, 12, 138, 135, 9, 135, 11, 12, 13, 14, 15, 135, 135, 18,
    135, 135, 135, 22, 23, 24, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 135, 135, 135, 135,
    12, 138, 135, 9, 135, 11, 12, 13, 14, 15, 135, 135, 135, 135, 135, 135, 22, 23, 24, 25, 26, 27,
    135, 135, 135, 135, 32, 33, 34, 136, 135, 135, 135, 135, 12, 138, 135, 9, 135, 11, 12, 13, 14,
    15, 16, 17, 18, 135, 135, 135, 22, 23, 24, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136,
    135, 135, 135, 135, 12, 6, 7, 135, 9, 135, 11, 12, 13, 14, 15, 16, 17, 18, 19, 135, 21, 22, 23,
    24, 25, 26, 27, 135, 135, 135, 31, 32, 33, 34, 31, 135, 135, 135, 135, 37, 6, 135, 135, 135,
    135, 135, 135, 9, 6, 135, 135, 135, 135, 135, 135, 9, 135, 135, 12, 13, 14, 15, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 25, 26, 27, 135, 135, 135, 135, 32, 33, 34, 136, 139, 135, 135,
    135, 135, 9, 8, 9, 2, 135, 135, 2, 6, 7, 8, 9, 135, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    22, 23, 24, 25, 26, 27, 28, 29, 135, 31, 32, 33, 34, 31, 135, 135, 135, 135, 37, 6, 7, 135, 9,
    135, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 135, 135, 135, 31, 32,
    33, 34, 31, 135, 135, 135, 135, 37, 9, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 28, 29, 9, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 29, 2, 140, 140, 2, 142, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 143, 141, 35, 142, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 35, 143, 141, 143, 142,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 35, 141, 36, 0, 0,
];
static _use_syllable_machine_index_defaults: [i16; 128] = [
    3, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38, 38,
    38, 38, 38, 38, 38, 38, 38, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 96, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 100, 96, 69, 102, 105, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 96, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 100, 96, 69, 69, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 140, 141, 141, 141, 0, 0,
];
static _use_syllable_machine_cond_targs: [i8; 146] = [
    0, 1, 31, 0, 59, 61, 90, 91, 116, 0, 118, 104, 92, 93, 94, 95, 108, 110, 111, 112, 119, 113,
    105, 106, 107, 99, 100, 101, 120, 121, 122, 114, 96, 97, 98, 123, 125, 115, 0, 2, 3, 0, 16, 4,
    5, 6, 7, 20, 22, 23, 24, 28, 25, 17, 18, 19, 11, 12, 13, 29, 30, 26, 8, 9, 10, 27, 14, 15, 21,
    0, 32, 33, 0, 46, 34, 35, 36, 37, 50, 52, 53, 54, 55, 47, 48, 49, 41, 42, 43, 56, 38, 39, 40,
    57, 58, 44, 0, 45, 0, 51, 0, 0, 0, 60, 0, 0, 0, 62, 63, 76, 64, 65, 66, 67, 80, 82, 83, 84, 89,
    85, 77, 78, 79, 71, 72, 73, 86, 68, 69, 70, 87, 88, 74, 75, 81, 0, 102, 103, 109, 117, 0, 0, 0,
    124, 0, 0,
];
static _use_syllable_machine_cond_actions: [i8; 146] = [
    0, 0, 0, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    9, 0, 10, 0, 11, 12, 13, 0, 14, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 18, 19, 20, 0, 0, 0,
];
static _use_syllable_machine_to_state_actions: [i8; 128] = [
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _use_syllable_machine_from_state_actions: [i8; 128] = [
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
static _use_syllable_machine_eof_trans: [i16; 128] = [
    1, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    39, 39, 39, 39, 39, 39, 39, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 97, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 70, 101, 97, 70, 103, 106, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 97, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 101, 97, 70, 70, 136, 136, 136,
    136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136,
    136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 141, 142, 142, 142, 0, 0,
];
static use_syllable_machine_start: i32 = 0;
static use_syllable_machine_first_final: i32 = 0;
static use_syllable_machine_error: i32 = -1;
static use_syllable_machine_en_main: i32 = 0;
#[derive(Clone, Copy)]
pub enum SyllableType {
    IndependentCluster,
    ViramaTerminatedCluster,
    SakotTerminatedCluster,
    StandardCluster,
    NumberJoinerTerminatedCluster,
    NumeralCluster,
    SymbolCluster,
    HieroglyphCluster,
    BrokenCluster,
    NonCluster,
}

pub fn find_syllables(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let infos = Cell::as_slice_of_cells(Cell::from_mut(&mut buffer.info));
    let p0 = MachineCursor::new(infos, included);
    let mut p = p0;
    let mut ts = p0;
    let mut te = p0;
    let pe = p.end();
    let eof = p.end();
    let mut syllable_serial = 1u8;

    // Please manually replace assignments of 0 to p, ts, and te
    // to use p0 instead

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts.index(), te.index(), &mut syllable_serial, $kind, infos);
        }};
    }

    {
        cs = (use_syllable_machine_start) as i32;
        ts = p0;
        te = p0;
    }

    {
        let mut _trans = 0;
        let mut _keys: i32 = 0;
        let mut _inds: i32 = 0;
        let mut _ic = 0;
        '_resume: while (p != pe || p == eof) {
            '_again: while (true) {
                match (_use_syllable_machine_from_state_actions[(cs) as usize]) {
                    2 => {
                        ts = p;
                    }

                    _ => {}
                }
                if (p == eof) {
                    {
                        if (_use_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_use_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (cs << 1) as i32;
                        _inds = (_use_syllable_machine_index_offsets[(cs) as usize]) as i32;
                        if ((infos[p.index()].get().use_category() as u8) <= 53) {
                            {
                                _ic = (_use_syllable_machine_char_class[((infos[p.index()]
                                    .get()
                                    .use_category()
                                    as u8)
                                    as i32
                                    - 0)
                                    as usize]) as i32;
                                if (_ic
                                    <= (_use_syllable_machine_trans_keys[(_keys + 1) as usize])
                                        as i32
                                    && _ic
                                        >= (_use_syllable_machine_trans_keys[(_keys) as usize])
                                            as i32)
                                {
                                    _trans = (_use_syllable_machine_indices[(_inds
                                        + (_ic
                                            - (_use_syllable_machine_trans_keys[(_keys) as usize])
                                                as i32)
                                            as i32)
                                        as usize])
                                        as u32;
                                } else {
                                    _trans = (_use_syllable_machine_index_defaults[(cs) as usize])
                                        as u32;
                                }
                            }
                        } else {
                            {
                                _trans =
                                    (_use_syllable_machine_index_defaults[(cs) as usize]) as u32;
                            }
                        }
                    }
                }
                cs = (_use_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_use_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        match (_use_syllable_machine_cond_actions[(_trans) as usize]) {
                            12 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::ViramaTerminatedCluster);
                                }
                            }
                            10 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::SakotTerminatedCluster);
                                }
                            }
                            8 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            16 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NumberJoinerTerminatedCluster);
                                }
                            }
                            14 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NumeralCluster);
                                }
                            }
                            6 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }
                            20 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::HieroglyphCluster);
                                }
                            }
                            4 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                    buffer.scratch_flags |=
                                        HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                }
                            }
                            3 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }
                            11 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::ViramaTerminatedCluster);
                                }
                            }
                            9 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SakotTerminatedCluster);
                                }
                            }
                            7 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            15 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumberJoinerTerminatedCluster);
                                }
                            }
                            13 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumeralCluster);
                                }
                            }
                            5 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }
                            19 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::HieroglyphCluster);
                                }
                            }
                            17 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                    buffer.scratch_flags |=
                                        HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                }
                            }
                            18 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }

                            _ => {}
                        }
                    }
                }
                break '_again;
            }
            if (p == eof) {
                {
                    if (cs >= 0) {
                        break '_resume;
                    }
                }
            } else {
                {
                    match (_use_syllable_machine_to_state_actions[(cs) as usize]) {
                        1 => {
                            ts = p0;
                        }

                        _ => {}
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
    buffer: &[Cell<hb_glyph_info_t>],
) {
    for i in start..end {
        let mut glyph = buffer[i].get();
        glyph.set_syllable((*syllable_serial << 4) | kind as u8);
        buffer[i].set(glyph);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}

fn not_ccs_default_ignorable(i: &hb_glyph_info_t) -> bool {
    i.use_category() != category::CGJ
}

fn included(infos: &[Cell<hb_glyph_info_t>], i: usize) -> bool {
    let glyph = infos[i].get();
    if !not_ccs_default_ignorable(&glyph) {
        return false;
    }
    if glyph.use_category() == category::ZWNJ {
        for glyph2 in &infos[i + 1..] {
            if not_ccs_default_ignorable(&glyph2.get()) {
                return !_hb_glyph_info_is_unicode_mark(&glyph2.get());
            }
        }
    }
    true
}
