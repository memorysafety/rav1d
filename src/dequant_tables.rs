use crate::include::stdint::*;
use ::libc;

pub static mut dav1d_dq_tbl: [[[uint16_t; 2]; 256]; 3] = [
    [
        [4 as libc::c_int as uint16_t, 4 as libc::c_int as uint16_t],
        [8 as libc::c_int as uint16_t, 8 as libc::c_int as uint16_t],
        [8 as libc::c_int as uint16_t, 9 as libc::c_int as uint16_t],
        [9 as libc::c_int as uint16_t, 10 as libc::c_int as uint16_t],
        [10 as libc::c_int as uint16_t, 11 as libc::c_int as uint16_t],
        [11 as libc::c_int as uint16_t, 12 as libc::c_int as uint16_t],
        [12 as libc::c_int as uint16_t, 13 as libc::c_int as uint16_t],
        [12 as libc::c_int as uint16_t, 14 as libc::c_int as uint16_t],
        [13 as libc::c_int as uint16_t, 15 as libc::c_int as uint16_t],
        [14 as libc::c_int as uint16_t, 16 as libc::c_int as uint16_t],
        [15 as libc::c_int as uint16_t, 17 as libc::c_int as uint16_t],
        [16 as libc::c_int as uint16_t, 18 as libc::c_int as uint16_t],
        [17 as libc::c_int as uint16_t, 19 as libc::c_int as uint16_t],
        [18 as libc::c_int as uint16_t, 20 as libc::c_int as uint16_t],
        [19 as libc::c_int as uint16_t, 21 as libc::c_int as uint16_t],
        [19 as libc::c_int as uint16_t, 22 as libc::c_int as uint16_t],
        [20 as libc::c_int as uint16_t, 23 as libc::c_int as uint16_t],
        [21 as libc::c_int as uint16_t, 24 as libc::c_int as uint16_t],
        [22 as libc::c_int as uint16_t, 25 as libc::c_int as uint16_t],
        [23 as libc::c_int as uint16_t, 26 as libc::c_int as uint16_t],
        [24 as libc::c_int as uint16_t, 27 as libc::c_int as uint16_t],
        [25 as libc::c_int as uint16_t, 28 as libc::c_int as uint16_t],
        [26 as libc::c_int as uint16_t, 29 as libc::c_int as uint16_t],
        [26 as libc::c_int as uint16_t, 30 as libc::c_int as uint16_t],
        [27 as libc::c_int as uint16_t, 31 as libc::c_int as uint16_t],
        [28 as libc::c_int as uint16_t, 32 as libc::c_int as uint16_t],
        [29 as libc::c_int as uint16_t, 33 as libc::c_int as uint16_t],
        [30 as libc::c_int as uint16_t, 34 as libc::c_int as uint16_t],
        [31 as libc::c_int as uint16_t, 35 as libc::c_int as uint16_t],
        [32 as libc::c_int as uint16_t, 36 as libc::c_int as uint16_t],
        [32 as libc::c_int as uint16_t, 37 as libc::c_int as uint16_t],
        [33 as libc::c_int as uint16_t, 38 as libc::c_int as uint16_t],
        [34 as libc::c_int as uint16_t, 39 as libc::c_int as uint16_t],
        [35 as libc::c_int as uint16_t, 40 as libc::c_int as uint16_t],
        [36 as libc::c_int as uint16_t, 41 as libc::c_int as uint16_t],
        [37 as libc::c_int as uint16_t, 42 as libc::c_int as uint16_t],
        [38 as libc::c_int as uint16_t, 43 as libc::c_int as uint16_t],
        [38 as libc::c_int as uint16_t, 44 as libc::c_int as uint16_t],
        [39 as libc::c_int as uint16_t, 45 as libc::c_int as uint16_t],
        [40 as libc::c_int as uint16_t, 46 as libc::c_int as uint16_t],
        [41 as libc::c_int as uint16_t, 47 as libc::c_int as uint16_t],
        [42 as libc::c_int as uint16_t, 48 as libc::c_int as uint16_t],
        [43 as libc::c_int as uint16_t, 49 as libc::c_int as uint16_t],
        [43 as libc::c_int as uint16_t, 50 as libc::c_int as uint16_t],
        [44 as libc::c_int as uint16_t, 51 as libc::c_int as uint16_t],
        [45 as libc::c_int as uint16_t, 52 as libc::c_int as uint16_t],
        [46 as libc::c_int as uint16_t, 53 as libc::c_int as uint16_t],
        [47 as libc::c_int as uint16_t, 54 as libc::c_int as uint16_t],
        [48 as libc::c_int as uint16_t, 55 as libc::c_int as uint16_t],
        [48 as libc::c_int as uint16_t, 56 as libc::c_int as uint16_t],
        [49 as libc::c_int as uint16_t, 57 as libc::c_int as uint16_t],
        [50 as libc::c_int as uint16_t, 58 as libc::c_int as uint16_t],
        [51 as libc::c_int as uint16_t, 59 as libc::c_int as uint16_t],
        [52 as libc::c_int as uint16_t, 60 as libc::c_int as uint16_t],
        [53 as libc::c_int as uint16_t, 61 as libc::c_int as uint16_t],
        [53 as libc::c_int as uint16_t, 62 as libc::c_int as uint16_t],
        [54 as libc::c_int as uint16_t, 63 as libc::c_int as uint16_t],
        [55 as libc::c_int as uint16_t, 64 as libc::c_int as uint16_t],
        [56 as libc::c_int as uint16_t, 65 as libc::c_int as uint16_t],
        [57 as libc::c_int as uint16_t, 66 as libc::c_int as uint16_t],
        [57 as libc::c_int as uint16_t, 67 as libc::c_int as uint16_t],
        [58 as libc::c_int as uint16_t, 68 as libc::c_int as uint16_t],
        [59 as libc::c_int as uint16_t, 69 as libc::c_int as uint16_t],
        [60 as libc::c_int as uint16_t, 70 as libc::c_int as uint16_t],
        [61 as libc::c_int as uint16_t, 71 as libc::c_int as uint16_t],
        [62 as libc::c_int as uint16_t, 72 as libc::c_int as uint16_t],
        [62 as libc::c_int as uint16_t, 73 as libc::c_int as uint16_t],
        [63 as libc::c_int as uint16_t, 74 as libc::c_int as uint16_t],
        [64 as libc::c_int as uint16_t, 75 as libc::c_int as uint16_t],
        [65 as libc::c_int as uint16_t, 76 as libc::c_int as uint16_t],
        [66 as libc::c_int as uint16_t, 77 as libc::c_int as uint16_t],
        [66 as libc::c_int as uint16_t, 78 as libc::c_int as uint16_t],
        [67 as libc::c_int as uint16_t, 79 as libc::c_int as uint16_t],
        [68 as libc::c_int as uint16_t, 80 as libc::c_int as uint16_t],
        [69 as libc::c_int as uint16_t, 81 as libc::c_int as uint16_t],
        [70 as libc::c_int as uint16_t, 82 as libc::c_int as uint16_t],
        [70 as libc::c_int as uint16_t, 83 as libc::c_int as uint16_t],
        [71 as libc::c_int as uint16_t, 84 as libc::c_int as uint16_t],
        [72 as libc::c_int as uint16_t, 85 as libc::c_int as uint16_t],
        [73 as libc::c_int as uint16_t, 86 as libc::c_int as uint16_t],
        [74 as libc::c_int as uint16_t, 87 as libc::c_int as uint16_t],
        [74 as libc::c_int as uint16_t, 88 as libc::c_int as uint16_t],
        [75 as libc::c_int as uint16_t, 89 as libc::c_int as uint16_t],
        [76 as libc::c_int as uint16_t, 90 as libc::c_int as uint16_t],
        [77 as libc::c_int as uint16_t, 91 as libc::c_int as uint16_t],
        [78 as libc::c_int as uint16_t, 92 as libc::c_int as uint16_t],
        [78 as libc::c_int as uint16_t, 93 as libc::c_int as uint16_t],
        [79 as libc::c_int as uint16_t, 94 as libc::c_int as uint16_t],
        [80 as libc::c_int as uint16_t, 95 as libc::c_int as uint16_t],
        [81 as libc::c_int as uint16_t, 96 as libc::c_int as uint16_t],
        [81 as libc::c_int as uint16_t, 97 as libc::c_int as uint16_t],
        [82 as libc::c_int as uint16_t, 98 as libc::c_int as uint16_t],
        [83 as libc::c_int as uint16_t, 99 as libc::c_int as uint16_t],
        [
            84 as libc::c_int as uint16_t,
            100 as libc::c_int as uint16_t,
        ],
        [
            85 as libc::c_int as uint16_t,
            101 as libc::c_int as uint16_t,
        ],
        [
            85 as libc::c_int as uint16_t,
            102 as libc::c_int as uint16_t,
        ],
        [
            87 as libc::c_int as uint16_t,
            104 as libc::c_int as uint16_t,
        ],
        [
            88 as libc::c_int as uint16_t,
            106 as libc::c_int as uint16_t,
        ],
        [
            90 as libc::c_int as uint16_t,
            108 as libc::c_int as uint16_t,
        ],
        [
            92 as libc::c_int as uint16_t,
            110 as libc::c_int as uint16_t,
        ],
        [
            93 as libc::c_int as uint16_t,
            112 as libc::c_int as uint16_t,
        ],
        [
            95 as libc::c_int as uint16_t,
            114 as libc::c_int as uint16_t,
        ],
        [
            96 as libc::c_int as uint16_t,
            116 as libc::c_int as uint16_t,
        ],
        [
            98 as libc::c_int as uint16_t,
            118 as libc::c_int as uint16_t,
        ],
        [
            99 as libc::c_int as uint16_t,
            120 as libc::c_int as uint16_t,
        ],
        [
            101 as libc::c_int as uint16_t,
            122 as libc::c_int as uint16_t,
        ],
        [
            102 as libc::c_int as uint16_t,
            124 as libc::c_int as uint16_t,
        ],
        [
            104 as libc::c_int as uint16_t,
            126 as libc::c_int as uint16_t,
        ],
        [
            105 as libc::c_int as uint16_t,
            128 as libc::c_int as uint16_t,
        ],
        [
            107 as libc::c_int as uint16_t,
            130 as libc::c_int as uint16_t,
        ],
        [
            108 as libc::c_int as uint16_t,
            132 as libc::c_int as uint16_t,
        ],
        [
            110 as libc::c_int as uint16_t,
            134 as libc::c_int as uint16_t,
        ],
        [
            111 as libc::c_int as uint16_t,
            136 as libc::c_int as uint16_t,
        ],
        [
            113 as libc::c_int as uint16_t,
            138 as libc::c_int as uint16_t,
        ],
        [
            114 as libc::c_int as uint16_t,
            140 as libc::c_int as uint16_t,
        ],
        [
            116 as libc::c_int as uint16_t,
            142 as libc::c_int as uint16_t,
        ],
        [
            117 as libc::c_int as uint16_t,
            144 as libc::c_int as uint16_t,
        ],
        [
            118 as libc::c_int as uint16_t,
            146 as libc::c_int as uint16_t,
        ],
        [
            120 as libc::c_int as uint16_t,
            148 as libc::c_int as uint16_t,
        ],
        [
            121 as libc::c_int as uint16_t,
            150 as libc::c_int as uint16_t,
        ],
        [
            123 as libc::c_int as uint16_t,
            152 as libc::c_int as uint16_t,
        ],
        [
            125 as libc::c_int as uint16_t,
            155 as libc::c_int as uint16_t,
        ],
        [
            127 as libc::c_int as uint16_t,
            158 as libc::c_int as uint16_t,
        ],
        [
            129 as libc::c_int as uint16_t,
            161 as libc::c_int as uint16_t,
        ],
        [
            131 as libc::c_int as uint16_t,
            164 as libc::c_int as uint16_t,
        ],
        [
            134 as libc::c_int as uint16_t,
            167 as libc::c_int as uint16_t,
        ],
        [
            136 as libc::c_int as uint16_t,
            170 as libc::c_int as uint16_t,
        ],
        [
            138 as libc::c_int as uint16_t,
            173 as libc::c_int as uint16_t,
        ],
        [
            140 as libc::c_int as uint16_t,
            176 as libc::c_int as uint16_t,
        ],
        [
            142 as libc::c_int as uint16_t,
            179 as libc::c_int as uint16_t,
        ],
        [
            144 as libc::c_int as uint16_t,
            182 as libc::c_int as uint16_t,
        ],
        [
            146 as libc::c_int as uint16_t,
            185 as libc::c_int as uint16_t,
        ],
        [
            148 as libc::c_int as uint16_t,
            188 as libc::c_int as uint16_t,
        ],
        [
            150 as libc::c_int as uint16_t,
            191 as libc::c_int as uint16_t,
        ],
        [
            152 as libc::c_int as uint16_t,
            194 as libc::c_int as uint16_t,
        ],
        [
            154 as libc::c_int as uint16_t,
            197 as libc::c_int as uint16_t,
        ],
        [
            156 as libc::c_int as uint16_t,
            200 as libc::c_int as uint16_t,
        ],
        [
            158 as libc::c_int as uint16_t,
            203 as libc::c_int as uint16_t,
        ],
        [
            161 as libc::c_int as uint16_t,
            207 as libc::c_int as uint16_t,
        ],
        [
            164 as libc::c_int as uint16_t,
            211 as libc::c_int as uint16_t,
        ],
        [
            166 as libc::c_int as uint16_t,
            215 as libc::c_int as uint16_t,
        ],
        [
            169 as libc::c_int as uint16_t,
            219 as libc::c_int as uint16_t,
        ],
        [
            172 as libc::c_int as uint16_t,
            223 as libc::c_int as uint16_t,
        ],
        [
            174 as libc::c_int as uint16_t,
            227 as libc::c_int as uint16_t,
        ],
        [
            177 as libc::c_int as uint16_t,
            231 as libc::c_int as uint16_t,
        ],
        [
            180 as libc::c_int as uint16_t,
            235 as libc::c_int as uint16_t,
        ],
        [
            182 as libc::c_int as uint16_t,
            239 as libc::c_int as uint16_t,
        ],
        [
            185 as libc::c_int as uint16_t,
            243 as libc::c_int as uint16_t,
        ],
        [
            187 as libc::c_int as uint16_t,
            247 as libc::c_int as uint16_t,
        ],
        [
            190 as libc::c_int as uint16_t,
            251 as libc::c_int as uint16_t,
        ],
        [
            192 as libc::c_int as uint16_t,
            255 as libc::c_int as uint16_t,
        ],
        [
            195 as libc::c_int as uint16_t,
            260 as libc::c_int as uint16_t,
        ],
        [
            199 as libc::c_int as uint16_t,
            265 as libc::c_int as uint16_t,
        ],
        [
            202 as libc::c_int as uint16_t,
            270 as libc::c_int as uint16_t,
        ],
        [
            205 as libc::c_int as uint16_t,
            275 as libc::c_int as uint16_t,
        ],
        [
            208 as libc::c_int as uint16_t,
            280 as libc::c_int as uint16_t,
        ],
        [
            211 as libc::c_int as uint16_t,
            285 as libc::c_int as uint16_t,
        ],
        [
            214 as libc::c_int as uint16_t,
            290 as libc::c_int as uint16_t,
        ],
        [
            217 as libc::c_int as uint16_t,
            295 as libc::c_int as uint16_t,
        ],
        [
            220 as libc::c_int as uint16_t,
            300 as libc::c_int as uint16_t,
        ],
        [
            223 as libc::c_int as uint16_t,
            305 as libc::c_int as uint16_t,
        ],
        [
            226 as libc::c_int as uint16_t,
            311 as libc::c_int as uint16_t,
        ],
        [
            230 as libc::c_int as uint16_t,
            317 as libc::c_int as uint16_t,
        ],
        [
            233 as libc::c_int as uint16_t,
            323 as libc::c_int as uint16_t,
        ],
        [
            237 as libc::c_int as uint16_t,
            329 as libc::c_int as uint16_t,
        ],
        [
            240 as libc::c_int as uint16_t,
            335 as libc::c_int as uint16_t,
        ],
        [
            243 as libc::c_int as uint16_t,
            341 as libc::c_int as uint16_t,
        ],
        [
            247 as libc::c_int as uint16_t,
            347 as libc::c_int as uint16_t,
        ],
        [
            250 as libc::c_int as uint16_t,
            353 as libc::c_int as uint16_t,
        ],
        [
            253 as libc::c_int as uint16_t,
            359 as libc::c_int as uint16_t,
        ],
        [
            257 as libc::c_int as uint16_t,
            366 as libc::c_int as uint16_t,
        ],
        [
            261 as libc::c_int as uint16_t,
            373 as libc::c_int as uint16_t,
        ],
        [
            265 as libc::c_int as uint16_t,
            380 as libc::c_int as uint16_t,
        ],
        [
            269 as libc::c_int as uint16_t,
            387 as libc::c_int as uint16_t,
        ],
        [
            272 as libc::c_int as uint16_t,
            394 as libc::c_int as uint16_t,
        ],
        [
            276 as libc::c_int as uint16_t,
            401 as libc::c_int as uint16_t,
        ],
        [
            280 as libc::c_int as uint16_t,
            408 as libc::c_int as uint16_t,
        ],
        [
            284 as libc::c_int as uint16_t,
            416 as libc::c_int as uint16_t,
        ],
        [
            288 as libc::c_int as uint16_t,
            424 as libc::c_int as uint16_t,
        ],
        [
            292 as libc::c_int as uint16_t,
            432 as libc::c_int as uint16_t,
        ],
        [
            296 as libc::c_int as uint16_t,
            440 as libc::c_int as uint16_t,
        ],
        [
            300 as libc::c_int as uint16_t,
            448 as libc::c_int as uint16_t,
        ],
        [
            304 as libc::c_int as uint16_t,
            456 as libc::c_int as uint16_t,
        ],
        [
            309 as libc::c_int as uint16_t,
            465 as libc::c_int as uint16_t,
        ],
        [
            313 as libc::c_int as uint16_t,
            474 as libc::c_int as uint16_t,
        ],
        [
            317 as libc::c_int as uint16_t,
            483 as libc::c_int as uint16_t,
        ],
        [
            322 as libc::c_int as uint16_t,
            492 as libc::c_int as uint16_t,
        ],
        [
            326 as libc::c_int as uint16_t,
            501 as libc::c_int as uint16_t,
        ],
        [
            330 as libc::c_int as uint16_t,
            510 as libc::c_int as uint16_t,
        ],
        [
            335 as libc::c_int as uint16_t,
            520 as libc::c_int as uint16_t,
        ],
        [
            340 as libc::c_int as uint16_t,
            530 as libc::c_int as uint16_t,
        ],
        [
            344 as libc::c_int as uint16_t,
            540 as libc::c_int as uint16_t,
        ],
        [
            349 as libc::c_int as uint16_t,
            550 as libc::c_int as uint16_t,
        ],
        [
            354 as libc::c_int as uint16_t,
            560 as libc::c_int as uint16_t,
        ],
        [
            359 as libc::c_int as uint16_t,
            571 as libc::c_int as uint16_t,
        ],
        [
            364 as libc::c_int as uint16_t,
            582 as libc::c_int as uint16_t,
        ],
        [
            369 as libc::c_int as uint16_t,
            593 as libc::c_int as uint16_t,
        ],
        [
            374 as libc::c_int as uint16_t,
            604 as libc::c_int as uint16_t,
        ],
        [
            379 as libc::c_int as uint16_t,
            615 as libc::c_int as uint16_t,
        ],
        [
            384 as libc::c_int as uint16_t,
            627 as libc::c_int as uint16_t,
        ],
        [
            389 as libc::c_int as uint16_t,
            639 as libc::c_int as uint16_t,
        ],
        [
            395 as libc::c_int as uint16_t,
            651 as libc::c_int as uint16_t,
        ],
        [
            400 as libc::c_int as uint16_t,
            663 as libc::c_int as uint16_t,
        ],
        [
            406 as libc::c_int as uint16_t,
            676 as libc::c_int as uint16_t,
        ],
        [
            411 as libc::c_int as uint16_t,
            689 as libc::c_int as uint16_t,
        ],
        [
            417 as libc::c_int as uint16_t,
            702 as libc::c_int as uint16_t,
        ],
        [
            423 as libc::c_int as uint16_t,
            715 as libc::c_int as uint16_t,
        ],
        [
            429 as libc::c_int as uint16_t,
            729 as libc::c_int as uint16_t,
        ],
        [
            435 as libc::c_int as uint16_t,
            743 as libc::c_int as uint16_t,
        ],
        [
            441 as libc::c_int as uint16_t,
            757 as libc::c_int as uint16_t,
        ],
        [
            447 as libc::c_int as uint16_t,
            771 as libc::c_int as uint16_t,
        ],
        [
            454 as libc::c_int as uint16_t,
            786 as libc::c_int as uint16_t,
        ],
        [
            461 as libc::c_int as uint16_t,
            801 as libc::c_int as uint16_t,
        ],
        [
            467 as libc::c_int as uint16_t,
            816 as libc::c_int as uint16_t,
        ],
        [
            475 as libc::c_int as uint16_t,
            832 as libc::c_int as uint16_t,
        ],
        [
            482 as libc::c_int as uint16_t,
            848 as libc::c_int as uint16_t,
        ],
        [
            489 as libc::c_int as uint16_t,
            864 as libc::c_int as uint16_t,
        ],
        [
            497 as libc::c_int as uint16_t,
            881 as libc::c_int as uint16_t,
        ],
        [
            505 as libc::c_int as uint16_t,
            898 as libc::c_int as uint16_t,
        ],
        [
            513 as libc::c_int as uint16_t,
            915 as libc::c_int as uint16_t,
        ],
        [
            522 as libc::c_int as uint16_t,
            933 as libc::c_int as uint16_t,
        ],
        [
            530 as libc::c_int as uint16_t,
            951 as libc::c_int as uint16_t,
        ],
        [
            539 as libc::c_int as uint16_t,
            969 as libc::c_int as uint16_t,
        ],
        [
            549 as libc::c_int as uint16_t,
            988 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            1007 as libc::c_int as uint16_t,
        ],
        [
            569 as libc::c_int as uint16_t,
            1026 as libc::c_int as uint16_t,
        ],
        [
            579 as libc::c_int as uint16_t,
            1046 as libc::c_int as uint16_t,
        ],
        [
            590 as libc::c_int as uint16_t,
            1066 as libc::c_int as uint16_t,
        ],
        [
            602 as libc::c_int as uint16_t,
            1087 as libc::c_int as uint16_t,
        ],
        [
            614 as libc::c_int as uint16_t,
            1108 as libc::c_int as uint16_t,
        ],
        [
            626 as libc::c_int as uint16_t,
            1129 as libc::c_int as uint16_t,
        ],
        [
            640 as libc::c_int as uint16_t,
            1151 as libc::c_int as uint16_t,
        ],
        [
            654 as libc::c_int as uint16_t,
            1173 as libc::c_int as uint16_t,
        ],
        [
            668 as libc::c_int as uint16_t,
            1196 as libc::c_int as uint16_t,
        ],
        [
            684 as libc::c_int as uint16_t,
            1219 as libc::c_int as uint16_t,
        ],
        [
            700 as libc::c_int as uint16_t,
            1243 as libc::c_int as uint16_t,
        ],
        [
            717 as libc::c_int as uint16_t,
            1267 as libc::c_int as uint16_t,
        ],
        [
            736 as libc::c_int as uint16_t,
            1292 as libc::c_int as uint16_t,
        ],
        [
            755 as libc::c_int as uint16_t,
            1317 as libc::c_int as uint16_t,
        ],
        [
            775 as libc::c_int as uint16_t,
            1343 as libc::c_int as uint16_t,
        ],
        [
            796 as libc::c_int as uint16_t,
            1369 as libc::c_int as uint16_t,
        ],
        [
            819 as libc::c_int as uint16_t,
            1396 as libc::c_int as uint16_t,
        ],
        [
            843 as libc::c_int as uint16_t,
            1423 as libc::c_int as uint16_t,
        ],
        [
            869 as libc::c_int as uint16_t,
            1451 as libc::c_int as uint16_t,
        ],
        [
            896 as libc::c_int as uint16_t,
            1479 as libc::c_int as uint16_t,
        ],
        [
            925 as libc::c_int as uint16_t,
            1508 as libc::c_int as uint16_t,
        ],
        [
            955 as libc::c_int as uint16_t,
            1537 as libc::c_int as uint16_t,
        ],
        [
            988 as libc::c_int as uint16_t,
            1567 as libc::c_int as uint16_t,
        ],
        [
            1022 as libc::c_int as uint16_t,
            1597 as libc::c_int as uint16_t,
        ],
        [
            1058 as libc::c_int as uint16_t,
            1628 as libc::c_int as uint16_t,
        ],
        [
            1098 as libc::c_int as uint16_t,
            1660 as libc::c_int as uint16_t,
        ],
        [
            1139 as libc::c_int as uint16_t,
            1692 as libc::c_int as uint16_t,
        ],
        [
            1184 as libc::c_int as uint16_t,
            1725 as libc::c_int as uint16_t,
        ],
        [
            1232 as libc::c_int as uint16_t,
            1759 as libc::c_int as uint16_t,
        ],
        [
            1282 as libc::c_int as uint16_t,
            1793 as libc::c_int as uint16_t,
        ],
        [
            1336 as libc::c_int as uint16_t,
            1828 as libc::c_int as uint16_t,
        ],
    ],
    [
        [4 as libc::c_int as uint16_t, 4 as libc::c_int as uint16_t],
        [9 as libc::c_int as uint16_t, 9 as libc::c_int as uint16_t],
        [10 as libc::c_int as uint16_t, 11 as libc::c_int as uint16_t],
        [13 as libc::c_int as uint16_t, 13 as libc::c_int as uint16_t],
        [15 as libc::c_int as uint16_t, 16 as libc::c_int as uint16_t],
        [17 as libc::c_int as uint16_t, 18 as libc::c_int as uint16_t],
        [20 as libc::c_int as uint16_t, 21 as libc::c_int as uint16_t],
        [22 as libc::c_int as uint16_t, 24 as libc::c_int as uint16_t],
        [25 as libc::c_int as uint16_t, 27 as libc::c_int as uint16_t],
        [28 as libc::c_int as uint16_t, 30 as libc::c_int as uint16_t],
        [31 as libc::c_int as uint16_t, 33 as libc::c_int as uint16_t],
        [34 as libc::c_int as uint16_t, 37 as libc::c_int as uint16_t],
        [37 as libc::c_int as uint16_t, 40 as libc::c_int as uint16_t],
        [40 as libc::c_int as uint16_t, 44 as libc::c_int as uint16_t],
        [43 as libc::c_int as uint16_t, 48 as libc::c_int as uint16_t],
        [47 as libc::c_int as uint16_t, 51 as libc::c_int as uint16_t],
        [50 as libc::c_int as uint16_t, 55 as libc::c_int as uint16_t],
        [53 as libc::c_int as uint16_t, 59 as libc::c_int as uint16_t],
        [57 as libc::c_int as uint16_t, 63 as libc::c_int as uint16_t],
        [60 as libc::c_int as uint16_t, 67 as libc::c_int as uint16_t],
        [64 as libc::c_int as uint16_t, 71 as libc::c_int as uint16_t],
        [68 as libc::c_int as uint16_t, 75 as libc::c_int as uint16_t],
        [71 as libc::c_int as uint16_t, 79 as libc::c_int as uint16_t],
        [75 as libc::c_int as uint16_t, 83 as libc::c_int as uint16_t],
        [78 as libc::c_int as uint16_t, 88 as libc::c_int as uint16_t],
        [82 as libc::c_int as uint16_t, 92 as libc::c_int as uint16_t],
        [86 as libc::c_int as uint16_t, 96 as libc::c_int as uint16_t],
        [
            90 as libc::c_int as uint16_t,
            100 as libc::c_int as uint16_t,
        ],
        [
            93 as libc::c_int as uint16_t,
            105 as libc::c_int as uint16_t,
        ],
        [
            97 as libc::c_int as uint16_t,
            109 as libc::c_int as uint16_t,
        ],
        [
            101 as libc::c_int as uint16_t,
            114 as libc::c_int as uint16_t,
        ],
        [
            105 as libc::c_int as uint16_t,
            118 as libc::c_int as uint16_t,
        ],
        [
            109 as libc::c_int as uint16_t,
            122 as libc::c_int as uint16_t,
        ],
        [
            113 as libc::c_int as uint16_t,
            127 as libc::c_int as uint16_t,
        ],
        [
            116 as libc::c_int as uint16_t,
            131 as libc::c_int as uint16_t,
        ],
        [
            120 as libc::c_int as uint16_t,
            136 as libc::c_int as uint16_t,
        ],
        [
            124 as libc::c_int as uint16_t,
            140 as libc::c_int as uint16_t,
        ],
        [
            128 as libc::c_int as uint16_t,
            145 as libc::c_int as uint16_t,
        ],
        [
            132 as libc::c_int as uint16_t,
            149 as libc::c_int as uint16_t,
        ],
        [
            136 as libc::c_int as uint16_t,
            154 as libc::c_int as uint16_t,
        ],
        [
            140 as libc::c_int as uint16_t,
            158 as libc::c_int as uint16_t,
        ],
        [
            143 as libc::c_int as uint16_t,
            163 as libc::c_int as uint16_t,
        ],
        [
            147 as libc::c_int as uint16_t,
            168 as libc::c_int as uint16_t,
        ],
        [
            151 as libc::c_int as uint16_t,
            172 as libc::c_int as uint16_t,
        ],
        [
            155 as libc::c_int as uint16_t,
            177 as libc::c_int as uint16_t,
        ],
        [
            159 as libc::c_int as uint16_t,
            181 as libc::c_int as uint16_t,
        ],
        [
            163 as libc::c_int as uint16_t,
            186 as libc::c_int as uint16_t,
        ],
        [
            166 as libc::c_int as uint16_t,
            190 as libc::c_int as uint16_t,
        ],
        [
            170 as libc::c_int as uint16_t,
            195 as libc::c_int as uint16_t,
        ],
        [
            174 as libc::c_int as uint16_t,
            199 as libc::c_int as uint16_t,
        ],
        [
            178 as libc::c_int as uint16_t,
            204 as libc::c_int as uint16_t,
        ],
        [
            182 as libc::c_int as uint16_t,
            208 as libc::c_int as uint16_t,
        ],
        [
            185 as libc::c_int as uint16_t,
            213 as libc::c_int as uint16_t,
        ],
        [
            189 as libc::c_int as uint16_t,
            217 as libc::c_int as uint16_t,
        ],
        [
            193 as libc::c_int as uint16_t,
            222 as libc::c_int as uint16_t,
        ],
        [
            197 as libc::c_int as uint16_t,
            226 as libc::c_int as uint16_t,
        ],
        [
            200 as libc::c_int as uint16_t,
            231 as libc::c_int as uint16_t,
        ],
        [
            204 as libc::c_int as uint16_t,
            235 as libc::c_int as uint16_t,
        ],
        [
            208 as libc::c_int as uint16_t,
            240 as libc::c_int as uint16_t,
        ],
        [
            212 as libc::c_int as uint16_t,
            244 as libc::c_int as uint16_t,
        ],
        [
            215 as libc::c_int as uint16_t,
            249 as libc::c_int as uint16_t,
        ],
        [
            219 as libc::c_int as uint16_t,
            253 as libc::c_int as uint16_t,
        ],
        [
            223 as libc::c_int as uint16_t,
            258 as libc::c_int as uint16_t,
        ],
        [
            226 as libc::c_int as uint16_t,
            262 as libc::c_int as uint16_t,
        ],
        [
            230 as libc::c_int as uint16_t,
            267 as libc::c_int as uint16_t,
        ],
        [
            233 as libc::c_int as uint16_t,
            271 as libc::c_int as uint16_t,
        ],
        [
            237 as libc::c_int as uint16_t,
            275 as libc::c_int as uint16_t,
        ],
        [
            241 as libc::c_int as uint16_t,
            280 as libc::c_int as uint16_t,
        ],
        [
            244 as libc::c_int as uint16_t,
            284 as libc::c_int as uint16_t,
        ],
        [
            248 as libc::c_int as uint16_t,
            289 as libc::c_int as uint16_t,
        ],
        [
            251 as libc::c_int as uint16_t,
            293 as libc::c_int as uint16_t,
        ],
        [
            255 as libc::c_int as uint16_t,
            297 as libc::c_int as uint16_t,
        ],
        [
            259 as libc::c_int as uint16_t,
            302 as libc::c_int as uint16_t,
        ],
        [
            262 as libc::c_int as uint16_t,
            306 as libc::c_int as uint16_t,
        ],
        [
            266 as libc::c_int as uint16_t,
            311 as libc::c_int as uint16_t,
        ],
        [
            269 as libc::c_int as uint16_t,
            315 as libc::c_int as uint16_t,
        ],
        [
            273 as libc::c_int as uint16_t,
            319 as libc::c_int as uint16_t,
        ],
        [
            276 as libc::c_int as uint16_t,
            324 as libc::c_int as uint16_t,
        ],
        [
            280 as libc::c_int as uint16_t,
            328 as libc::c_int as uint16_t,
        ],
        [
            283 as libc::c_int as uint16_t,
            332 as libc::c_int as uint16_t,
        ],
        [
            287 as libc::c_int as uint16_t,
            337 as libc::c_int as uint16_t,
        ],
        [
            290 as libc::c_int as uint16_t,
            341 as libc::c_int as uint16_t,
        ],
        [
            293 as libc::c_int as uint16_t,
            345 as libc::c_int as uint16_t,
        ],
        [
            297 as libc::c_int as uint16_t,
            349 as libc::c_int as uint16_t,
        ],
        [
            300 as libc::c_int as uint16_t,
            354 as libc::c_int as uint16_t,
        ],
        [
            304 as libc::c_int as uint16_t,
            358 as libc::c_int as uint16_t,
        ],
        [
            307 as libc::c_int as uint16_t,
            362 as libc::c_int as uint16_t,
        ],
        [
            310 as libc::c_int as uint16_t,
            367 as libc::c_int as uint16_t,
        ],
        [
            314 as libc::c_int as uint16_t,
            371 as libc::c_int as uint16_t,
        ],
        [
            317 as libc::c_int as uint16_t,
            375 as libc::c_int as uint16_t,
        ],
        [
            321 as libc::c_int as uint16_t,
            379 as libc::c_int as uint16_t,
        ],
        [
            324 as libc::c_int as uint16_t,
            384 as libc::c_int as uint16_t,
        ],
        [
            327 as libc::c_int as uint16_t,
            388 as libc::c_int as uint16_t,
        ],
        [
            331 as libc::c_int as uint16_t,
            392 as libc::c_int as uint16_t,
        ],
        [
            334 as libc::c_int as uint16_t,
            396 as libc::c_int as uint16_t,
        ],
        [
            337 as libc::c_int as uint16_t,
            401 as libc::c_int as uint16_t,
        ],
        [
            343 as libc::c_int as uint16_t,
            409 as libc::c_int as uint16_t,
        ],
        [
            350 as libc::c_int as uint16_t,
            417 as libc::c_int as uint16_t,
        ],
        [
            356 as libc::c_int as uint16_t,
            425 as libc::c_int as uint16_t,
        ],
        [
            362 as libc::c_int as uint16_t,
            433 as libc::c_int as uint16_t,
        ],
        [
            369 as libc::c_int as uint16_t,
            441 as libc::c_int as uint16_t,
        ],
        [
            375 as libc::c_int as uint16_t,
            449 as libc::c_int as uint16_t,
        ],
        [
            381 as libc::c_int as uint16_t,
            458 as libc::c_int as uint16_t,
        ],
        [
            387 as libc::c_int as uint16_t,
            466 as libc::c_int as uint16_t,
        ],
        [
            394 as libc::c_int as uint16_t,
            474 as libc::c_int as uint16_t,
        ],
        [
            400 as libc::c_int as uint16_t,
            482 as libc::c_int as uint16_t,
        ],
        [
            406 as libc::c_int as uint16_t,
            490 as libc::c_int as uint16_t,
        ],
        [
            412 as libc::c_int as uint16_t,
            498 as libc::c_int as uint16_t,
        ],
        [
            418 as libc::c_int as uint16_t,
            506 as libc::c_int as uint16_t,
        ],
        [
            424 as libc::c_int as uint16_t,
            514 as libc::c_int as uint16_t,
        ],
        [
            430 as libc::c_int as uint16_t,
            523 as libc::c_int as uint16_t,
        ],
        [
            436 as libc::c_int as uint16_t,
            531 as libc::c_int as uint16_t,
        ],
        [
            442 as libc::c_int as uint16_t,
            539 as libc::c_int as uint16_t,
        ],
        [
            448 as libc::c_int as uint16_t,
            547 as libc::c_int as uint16_t,
        ],
        [
            454 as libc::c_int as uint16_t,
            555 as libc::c_int as uint16_t,
        ],
        [
            460 as libc::c_int as uint16_t,
            563 as libc::c_int as uint16_t,
        ],
        [
            466 as libc::c_int as uint16_t,
            571 as libc::c_int as uint16_t,
        ],
        [
            472 as libc::c_int as uint16_t,
            579 as libc::c_int as uint16_t,
        ],
        [
            478 as libc::c_int as uint16_t,
            588 as libc::c_int as uint16_t,
        ],
        [
            484 as libc::c_int as uint16_t,
            596 as libc::c_int as uint16_t,
        ],
        [
            490 as libc::c_int as uint16_t,
            604 as libc::c_int as uint16_t,
        ],
        [
            499 as libc::c_int as uint16_t,
            616 as libc::c_int as uint16_t,
        ],
        [
            507 as libc::c_int as uint16_t,
            628 as libc::c_int as uint16_t,
        ],
        [
            516 as libc::c_int as uint16_t,
            640 as libc::c_int as uint16_t,
        ],
        [
            525 as libc::c_int as uint16_t,
            652 as libc::c_int as uint16_t,
        ],
        [
            533 as libc::c_int as uint16_t,
            664 as libc::c_int as uint16_t,
        ],
        [
            542 as libc::c_int as uint16_t,
            676 as libc::c_int as uint16_t,
        ],
        [
            550 as libc::c_int as uint16_t,
            688 as libc::c_int as uint16_t,
        ],
        [
            559 as libc::c_int as uint16_t,
            700 as libc::c_int as uint16_t,
        ],
        [
            567 as libc::c_int as uint16_t,
            713 as libc::c_int as uint16_t,
        ],
        [
            576 as libc::c_int as uint16_t,
            725 as libc::c_int as uint16_t,
        ],
        [
            584 as libc::c_int as uint16_t,
            737 as libc::c_int as uint16_t,
        ],
        [
            592 as libc::c_int as uint16_t,
            749 as libc::c_int as uint16_t,
        ],
        [
            601 as libc::c_int as uint16_t,
            761 as libc::c_int as uint16_t,
        ],
        [
            609 as libc::c_int as uint16_t,
            773 as libc::c_int as uint16_t,
        ],
        [
            617 as libc::c_int as uint16_t,
            785 as libc::c_int as uint16_t,
        ],
        [
            625 as libc::c_int as uint16_t,
            797 as libc::c_int as uint16_t,
        ],
        [
            634 as libc::c_int as uint16_t,
            809 as libc::c_int as uint16_t,
        ],
        [
            644 as libc::c_int as uint16_t,
            825 as libc::c_int as uint16_t,
        ],
        [
            655 as libc::c_int as uint16_t,
            841 as libc::c_int as uint16_t,
        ],
        [
            666 as libc::c_int as uint16_t,
            857 as libc::c_int as uint16_t,
        ],
        [
            676 as libc::c_int as uint16_t,
            873 as libc::c_int as uint16_t,
        ],
        [
            687 as libc::c_int as uint16_t,
            889 as libc::c_int as uint16_t,
        ],
        [
            698 as libc::c_int as uint16_t,
            905 as libc::c_int as uint16_t,
        ],
        [
            708 as libc::c_int as uint16_t,
            922 as libc::c_int as uint16_t,
        ],
        [
            718 as libc::c_int as uint16_t,
            938 as libc::c_int as uint16_t,
        ],
        [
            729 as libc::c_int as uint16_t,
            954 as libc::c_int as uint16_t,
        ],
        [
            739 as libc::c_int as uint16_t,
            970 as libc::c_int as uint16_t,
        ],
        [
            749 as libc::c_int as uint16_t,
            986 as libc::c_int as uint16_t,
        ],
        [
            759 as libc::c_int as uint16_t,
            1002 as libc::c_int as uint16_t,
        ],
        [
            770 as libc::c_int as uint16_t,
            1018 as libc::c_int as uint16_t,
        ],
        [
            782 as libc::c_int as uint16_t,
            1038 as libc::c_int as uint16_t,
        ],
        [
            795 as libc::c_int as uint16_t,
            1058 as libc::c_int as uint16_t,
        ],
        [
            807 as libc::c_int as uint16_t,
            1078 as libc::c_int as uint16_t,
        ],
        [
            819 as libc::c_int as uint16_t,
            1098 as libc::c_int as uint16_t,
        ],
        [
            831 as libc::c_int as uint16_t,
            1118 as libc::c_int as uint16_t,
        ],
        [
            844 as libc::c_int as uint16_t,
            1138 as libc::c_int as uint16_t,
        ],
        [
            856 as libc::c_int as uint16_t,
            1158 as libc::c_int as uint16_t,
        ],
        [
            868 as libc::c_int as uint16_t,
            1178 as libc::c_int as uint16_t,
        ],
        [
            880 as libc::c_int as uint16_t,
            1198 as libc::c_int as uint16_t,
        ],
        [
            891 as libc::c_int as uint16_t,
            1218 as libc::c_int as uint16_t,
        ],
        [
            906 as libc::c_int as uint16_t,
            1242 as libc::c_int as uint16_t,
        ],
        [
            920 as libc::c_int as uint16_t,
            1266 as libc::c_int as uint16_t,
        ],
        [
            933 as libc::c_int as uint16_t,
            1290 as libc::c_int as uint16_t,
        ],
        [
            947 as libc::c_int as uint16_t,
            1314 as libc::c_int as uint16_t,
        ],
        [
            961 as libc::c_int as uint16_t,
            1338 as libc::c_int as uint16_t,
        ],
        [
            975 as libc::c_int as uint16_t,
            1362 as libc::c_int as uint16_t,
        ],
        [
            988 as libc::c_int as uint16_t,
            1386 as libc::c_int as uint16_t,
        ],
        [
            1001 as libc::c_int as uint16_t,
            1411 as libc::c_int as uint16_t,
        ],
        [
            1015 as libc::c_int as uint16_t,
            1435 as libc::c_int as uint16_t,
        ],
        [
            1030 as libc::c_int as uint16_t,
            1463 as libc::c_int as uint16_t,
        ],
        [
            1045 as libc::c_int as uint16_t,
            1491 as libc::c_int as uint16_t,
        ],
        [
            1061 as libc::c_int as uint16_t,
            1519 as libc::c_int as uint16_t,
        ],
        [
            1076 as libc::c_int as uint16_t,
            1547 as libc::c_int as uint16_t,
        ],
        [
            1090 as libc::c_int as uint16_t,
            1575 as libc::c_int as uint16_t,
        ],
        [
            1105 as libc::c_int as uint16_t,
            1603 as libc::c_int as uint16_t,
        ],
        [
            1120 as libc::c_int as uint16_t,
            1631 as libc::c_int as uint16_t,
        ],
        [
            1137 as libc::c_int as uint16_t,
            1663 as libc::c_int as uint16_t,
        ],
        [
            1153 as libc::c_int as uint16_t,
            1695 as libc::c_int as uint16_t,
        ],
        [
            1170 as libc::c_int as uint16_t,
            1727 as libc::c_int as uint16_t,
        ],
        [
            1186 as libc::c_int as uint16_t,
            1759 as libc::c_int as uint16_t,
        ],
        [
            1202 as libc::c_int as uint16_t,
            1791 as libc::c_int as uint16_t,
        ],
        [
            1218 as libc::c_int as uint16_t,
            1823 as libc::c_int as uint16_t,
        ],
        [
            1236 as libc::c_int as uint16_t,
            1859 as libc::c_int as uint16_t,
        ],
        [
            1253 as libc::c_int as uint16_t,
            1895 as libc::c_int as uint16_t,
        ],
        [
            1271 as libc::c_int as uint16_t,
            1931 as libc::c_int as uint16_t,
        ],
        [
            1288 as libc::c_int as uint16_t,
            1967 as libc::c_int as uint16_t,
        ],
        [
            1306 as libc::c_int as uint16_t,
            2003 as libc::c_int as uint16_t,
        ],
        [
            1323 as libc::c_int as uint16_t,
            2039 as libc::c_int as uint16_t,
        ],
        [
            1342 as libc::c_int as uint16_t,
            2079 as libc::c_int as uint16_t,
        ],
        [
            1361 as libc::c_int as uint16_t,
            2119 as libc::c_int as uint16_t,
        ],
        [
            1379 as libc::c_int as uint16_t,
            2159 as libc::c_int as uint16_t,
        ],
        [
            1398 as libc::c_int as uint16_t,
            2199 as libc::c_int as uint16_t,
        ],
        [
            1416 as libc::c_int as uint16_t,
            2239 as libc::c_int as uint16_t,
        ],
        [
            1436 as libc::c_int as uint16_t,
            2283 as libc::c_int as uint16_t,
        ],
        [
            1456 as libc::c_int as uint16_t,
            2327 as libc::c_int as uint16_t,
        ],
        [
            1476 as libc::c_int as uint16_t,
            2371 as libc::c_int as uint16_t,
        ],
        [
            1496 as libc::c_int as uint16_t,
            2415 as libc::c_int as uint16_t,
        ],
        [
            1516 as libc::c_int as uint16_t,
            2459 as libc::c_int as uint16_t,
        ],
        [
            1537 as libc::c_int as uint16_t,
            2507 as libc::c_int as uint16_t,
        ],
        [
            1559 as libc::c_int as uint16_t,
            2555 as libc::c_int as uint16_t,
        ],
        [
            1580 as libc::c_int as uint16_t,
            2603 as libc::c_int as uint16_t,
        ],
        [
            1601 as libc::c_int as uint16_t,
            2651 as libc::c_int as uint16_t,
        ],
        [
            1624 as libc::c_int as uint16_t,
            2703 as libc::c_int as uint16_t,
        ],
        [
            1647 as libc::c_int as uint16_t,
            2755 as libc::c_int as uint16_t,
        ],
        [
            1670 as libc::c_int as uint16_t,
            2807 as libc::c_int as uint16_t,
        ],
        [
            1692 as libc::c_int as uint16_t,
            2859 as libc::c_int as uint16_t,
        ],
        [
            1717 as libc::c_int as uint16_t,
            2915 as libc::c_int as uint16_t,
        ],
        [
            1741 as libc::c_int as uint16_t,
            2971 as libc::c_int as uint16_t,
        ],
        [
            1766 as libc::c_int as uint16_t,
            3027 as libc::c_int as uint16_t,
        ],
        [
            1791 as libc::c_int as uint16_t,
            3083 as libc::c_int as uint16_t,
        ],
        [
            1817 as libc::c_int as uint16_t,
            3143 as libc::c_int as uint16_t,
        ],
        [
            1844 as libc::c_int as uint16_t,
            3203 as libc::c_int as uint16_t,
        ],
        [
            1871 as libc::c_int as uint16_t,
            3263 as libc::c_int as uint16_t,
        ],
        [
            1900 as libc::c_int as uint16_t,
            3327 as libc::c_int as uint16_t,
        ],
        [
            1929 as libc::c_int as uint16_t,
            3391 as libc::c_int as uint16_t,
        ],
        [
            1958 as libc::c_int as uint16_t,
            3455 as libc::c_int as uint16_t,
        ],
        [
            1990 as libc::c_int as uint16_t,
            3523 as libc::c_int as uint16_t,
        ],
        [
            2021 as libc::c_int as uint16_t,
            3591 as libc::c_int as uint16_t,
        ],
        [
            2054 as libc::c_int as uint16_t,
            3659 as libc::c_int as uint16_t,
        ],
        [
            2088 as libc::c_int as uint16_t,
            3731 as libc::c_int as uint16_t,
        ],
        [
            2123 as libc::c_int as uint16_t,
            3803 as libc::c_int as uint16_t,
        ],
        [
            2159 as libc::c_int as uint16_t,
            3876 as libc::c_int as uint16_t,
        ],
        [
            2197 as libc::c_int as uint16_t,
            3952 as libc::c_int as uint16_t,
        ],
        [
            2236 as libc::c_int as uint16_t,
            4028 as libc::c_int as uint16_t,
        ],
        [
            2276 as libc::c_int as uint16_t,
            4104 as libc::c_int as uint16_t,
        ],
        [
            2319 as libc::c_int as uint16_t,
            4184 as libc::c_int as uint16_t,
        ],
        [
            2363 as libc::c_int as uint16_t,
            4264 as libc::c_int as uint16_t,
        ],
        [
            2410 as libc::c_int as uint16_t,
            4348 as libc::c_int as uint16_t,
        ],
        [
            2458 as libc::c_int as uint16_t,
            4432 as libc::c_int as uint16_t,
        ],
        [
            2508 as libc::c_int as uint16_t,
            4516 as libc::c_int as uint16_t,
        ],
        [
            2561 as libc::c_int as uint16_t,
            4604 as libc::c_int as uint16_t,
        ],
        [
            2616 as libc::c_int as uint16_t,
            4692 as libc::c_int as uint16_t,
        ],
        [
            2675 as libc::c_int as uint16_t,
            4784 as libc::c_int as uint16_t,
        ],
        [
            2737 as libc::c_int as uint16_t,
            4876 as libc::c_int as uint16_t,
        ],
        [
            2802 as libc::c_int as uint16_t,
            4972 as libc::c_int as uint16_t,
        ],
        [
            2871 as libc::c_int as uint16_t,
            5068 as libc::c_int as uint16_t,
        ],
        [
            2944 as libc::c_int as uint16_t,
            5168 as libc::c_int as uint16_t,
        ],
        [
            3020 as libc::c_int as uint16_t,
            5268 as libc::c_int as uint16_t,
        ],
        [
            3102 as libc::c_int as uint16_t,
            5372 as libc::c_int as uint16_t,
        ],
        [
            3188 as libc::c_int as uint16_t,
            5476 as libc::c_int as uint16_t,
        ],
        [
            3280 as libc::c_int as uint16_t,
            5584 as libc::c_int as uint16_t,
        ],
        [
            3375 as libc::c_int as uint16_t,
            5692 as libc::c_int as uint16_t,
        ],
        [
            3478 as libc::c_int as uint16_t,
            5804 as libc::c_int as uint16_t,
        ],
        [
            3586 as libc::c_int as uint16_t,
            5916 as libc::c_int as uint16_t,
        ],
        [
            3702 as libc::c_int as uint16_t,
            6032 as libc::c_int as uint16_t,
        ],
        [
            3823 as libc::c_int as uint16_t,
            6148 as libc::c_int as uint16_t,
        ],
        [
            3953 as libc::c_int as uint16_t,
            6268 as libc::c_int as uint16_t,
        ],
        [
            4089 as libc::c_int as uint16_t,
            6388 as libc::c_int as uint16_t,
        ],
        [
            4236 as libc::c_int as uint16_t,
            6512 as libc::c_int as uint16_t,
        ],
        [
            4394 as libc::c_int as uint16_t,
            6640 as libc::c_int as uint16_t,
        ],
        [
            4559 as libc::c_int as uint16_t,
            6768 as libc::c_int as uint16_t,
        ],
        [
            4737 as libc::c_int as uint16_t,
            6900 as libc::c_int as uint16_t,
        ],
        [
            4929 as libc::c_int as uint16_t,
            7036 as libc::c_int as uint16_t,
        ],
        [
            5130 as libc::c_int as uint16_t,
            7172 as libc::c_int as uint16_t,
        ],
        [
            5347 as libc::c_int as uint16_t,
            7312 as libc::c_int as uint16_t,
        ],
    ],
    [
        [4 as libc::c_int as uint16_t, 4 as libc::c_int as uint16_t],
        [12 as libc::c_int as uint16_t, 13 as libc::c_int as uint16_t],
        [18 as libc::c_int as uint16_t, 19 as libc::c_int as uint16_t],
        [25 as libc::c_int as uint16_t, 27 as libc::c_int as uint16_t],
        [33 as libc::c_int as uint16_t, 35 as libc::c_int as uint16_t],
        [41 as libc::c_int as uint16_t, 44 as libc::c_int as uint16_t],
        [50 as libc::c_int as uint16_t, 54 as libc::c_int as uint16_t],
        [60 as libc::c_int as uint16_t, 64 as libc::c_int as uint16_t],
        [70 as libc::c_int as uint16_t, 75 as libc::c_int as uint16_t],
        [80 as libc::c_int as uint16_t, 87 as libc::c_int as uint16_t],
        [91 as libc::c_int as uint16_t, 99 as libc::c_int as uint16_t],
        [
            103 as libc::c_int as uint16_t,
            112 as libc::c_int as uint16_t,
        ],
        [
            115 as libc::c_int as uint16_t,
            126 as libc::c_int as uint16_t,
        ],
        [
            127 as libc::c_int as uint16_t,
            139 as libc::c_int as uint16_t,
        ],
        [
            140 as libc::c_int as uint16_t,
            154 as libc::c_int as uint16_t,
        ],
        [
            153 as libc::c_int as uint16_t,
            168 as libc::c_int as uint16_t,
        ],
        [
            166 as libc::c_int as uint16_t,
            183 as libc::c_int as uint16_t,
        ],
        [
            180 as libc::c_int as uint16_t,
            199 as libc::c_int as uint16_t,
        ],
        [
            194 as libc::c_int as uint16_t,
            214 as libc::c_int as uint16_t,
        ],
        [
            208 as libc::c_int as uint16_t,
            230 as libc::c_int as uint16_t,
        ],
        [
            222 as libc::c_int as uint16_t,
            247 as libc::c_int as uint16_t,
        ],
        [
            237 as libc::c_int as uint16_t,
            263 as libc::c_int as uint16_t,
        ],
        [
            251 as libc::c_int as uint16_t,
            280 as libc::c_int as uint16_t,
        ],
        [
            266 as libc::c_int as uint16_t,
            297 as libc::c_int as uint16_t,
        ],
        [
            281 as libc::c_int as uint16_t,
            314 as libc::c_int as uint16_t,
        ],
        [
            296 as libc::c_int as uint16_t,
            331 as libc::c_int as uint16_t,
        ],
        [
            312 as libc::c_int as uint16_t,
            349 as libc::c_int as uint16_t,
        ],
        [
            327 as libc::c_int as uint16_t,
            366 as libc::c_int as uint16_t,
        ],
        [
            343 as libc::c_int as uint16_t,
            384 as libc::c_int as uint16_t,
        ],
        [
            358 as libc::c_int as uint16_t,
            402 as libc::c_int as uint16_t,
        ],
        [
            374 as libc::c_int as uint16_t,
            420 as libc::c_int as uint16_t,
        ],
        [
            390 as libc::c_int as uint16_t,
            438 as libc::c_int as uint16_t,
        ],
        [
            405 as libc::c_int as uint16_t,
            456 as libc::c_int as uint16_t,
        ],
        [
            421 as libc::c_int as uint16_t,
            475 as libc::c_int as uint16_t,
        ],
        [
            437 as libc::c_int as uint16_t,
            493 as libc::c_int as uint16_t,
        ],
        [
            453 as libc::c_int as uint16_t,
            511 as libc::c_int as uint16_t,
        ],
        [
            469 as libc::c_int as uint16_t,
            530 as libc::c_int as uint16_t,
        ],
        [
            484 as libc::c_int as uint16_t,
            548 as libc::c_int as uint16_t,
        ],
        [
            500 as libc::c_int as uint16_t,
            567 as libc::c_int as uint16_t,
        ],
        [
            516 as libc::c_int as uint16_t,
            586 as libc::c_int as uint16_t,
        ],
        [
            532 as libc::c_int as uint16_t,
            604 as libc::c_int as uint16_t,
        ],
        [
            548 as libc::c_int as uint16_t,
            623 as libc::c_int as uint16_t,
        ],
        [
            564 as libc::c_int as uint16_t,
            642 as libc::c_int as uint16_t,
        ],
        [
            580 as libc::c_int as uint16_t,
            660 as libc::c_int as uint16_t,
        ],
        [
            596 as libc::c_int as uint16_t,
            679 as libc::c_int as uint16_t,
        ],
        [
            611 as libc::c_int as uint16_t,
            698 as libc::c_int as uint16_t,
        ],
        [
            627 as libc::c_int as uint16_t,
            716 as libc::c_int as uint16_t,
        ],
        [
            643 as libc::c_int as uint16_t,
            735 as libc::c_int as uint16_t,
        ],
        [
            659 as libc::c_int as uint16_t,
            753 as libc::c_int as uint16_t,
        ],
        [
            674 as libc::c_int as uint16_t,
            772 as libc::c_int as uint16_t,
        ],
        [
            690 as libc::c_int as uint16_t,
            791 as libc::c_int as uint16_t,
        ],
        [
            706 as libc::c_int as uint16_t,
            809 as libc::c_int as uint16_t,
        ],
        [
            721 as libc::c_int as uint16_t,
            828 as libc::c_int as uint16_t,
        ],
        [
            737 as libc::c_int as uint16_t,
            846 as libc::c_int as uint16_t,
        ],
        [
            752 as libc::c_int as uint16_t,
            865 as libc::c_int as uint16_t,
        ],
        [
            768 as libc::c_int as uint16_t,
            884 as libc::c_int as uint16_t,
        ],
        [
            783 as libc::c_int as uint16_t,
            902 as libc::c_int as uint16_t,
        ],
        [
            798 as libc::c_int as uint16_t,
            920 as libc::c_int as uint16_t,
        ],
        [
            814 as libc::c_int as uint16_t,
            939 as libc::c_int as uint16_t,
        ],
        [
            829 as libc::c_int as uint16_t,
            957 as libc::c_int as uint16_t,
        ],
        [
            844 as libc::c_int as uint16_t,
            976 as libc::c_int as uint16_t,
        ],
        [
            859 as libc::c_int as uint16_t,
            994 as libc::c_int as uint16_t,
        ],
        [
            874 as libc::c_int as uint16_t,
            1012 as libc::c_int as uint16_t,
        ],
        [
            889 as libc::c_int as uint16_t,
            1030 as libc::c_int as uint16_t,
        ],
        [
            904 as libc::c_int as uint16_t,
            1049 as libc::c_int as uint16_t,
        ],
        [
            919 as libc::c_int as uint16_t,
            1067 as libc::c_int as uint16_t,
        ],
        [
            934 as libc::c_int as uint16_t,
            1085 as libc::c_int as uint16_t,
        ],
        [
            949 as libc::c_int as uint16_t,
            1103 as libc::c_int as uint16_t,
        ],
        [
            964 as libc::c_int as uint16_t,
            1121 as libc::c_int as uint16_t,
        ],
        [
            978 as libc::c_int as uint16_t,
            1139 as libc::c_int as uint16_t,
        ],
        [
            993 as libc::c_int as uint16_t,
            1157 as libc::c_int as uint16_t,
        ],
        [
            1008 as libc::c_int as uint16_t,
            1175 as libc::c_int as uint16_t,
        ],
        [
            1022 as libc::c_int as uint16_t,
            1193 as libc::c_int as uint16_t,
        ],
        [
            1037 as libc::c_int as uint16_t,
            1211 as libc::c_int as uint16_t,
        ],
        [
            1051 as libc::c_int as uint16_t,
            1229 as libc::c_int as uint16_t,
        ],
        [
            1065 as libc::c_int as uint16_t,
            1246 as libc::c_int as uint16_t,
        ],
        [
            1080 as libc::c_int as uint16_t,
            1264 as libc::c_int as uint16_t,
        ],
        [
            1094 as libc::c_int as uint16_t,
            1282 as libc::c_int as uint16_t,
        ],
        [
            1108 as libc::c_int as uint16_t,
            1299 as libc::c_int as uint16_t,
        ],
        [
            1122 as libc::c_int as uint16_t,
            1317 as libc::c_int as uint16_t,
        ],
        [
            1136 as libc::c_int as uint16_t,
            1335 as libc::c_int as uint16_t,
        ],
        [
            1151 as libc::c_int as uint16_t,
            1352 as libc::c_int as uint16_t,
        ],
        [
            1165 as libc::c_int as uint16_t,
            1370 as libc::c_int as uint16_t,
        ],
        [
            1179 as libc::c_int as uint16_t,
            1387 as libc::c_int as uint16_t,
        ],
        [
            1192 as libc::c_int as uint16_t,
            1405 as libc::c_int as uint16_t,
        ],
        [
            1206 as libc::c_int as uint16_t,
            1422 as libc::c_int as uint16_t,
        ],
        [
            1220 as libc::c_int as uint16_t,
            1440 as libc::c_int as uint16_t,
        ],
        [
            1234 as libc::c_int as uint16_t,
            1457 as libc::c_int as uint16_t,
        ],
        [
            1248 as libc::c_int as uint16_t,
            1474 as libc::c_int as uint16_t,
        ],
        [
            1261 as libc::c_int as uint16_t,
            1491 as libc::c_int as uint16_t,
        ],
        [
            1275 as libc::c_int as uint16_t,
            1509 as libc::c_int as uint16_t,
        ],
        [
            1288 as libc::c_int as uint16_t,
            1526 as libc::c_int as uint16_t,
        ],
        [
            1302 as libc::c_int as uint16_t,
            1543 as libc::c_int as uint16_t,
        ],
        [
            1315 as libc::c_int as uint16_t,
            1560 as libc::c_int as uint16_t,
        ],
        [
            1329 as libc::c_int as uint16_t,
            1577 as libc::c_int as uint16_t,
        ],
        [
            1342 as libc::c_int as uint16_t,
            1595 as libc::c_int as uint16_t,
        ],
        [
            1368 as libc::c_int as uint16_t,
            1627 as libc::c_int as uint16_t,
        ],
        [
            1393 as libc::c_int as uint16_t,
            1660 as libc::c_int as uint16_t,
        ],
        [
            1419 as libc::c_int as uint16_t,
            1693 as libc::c_int as uint16_t,
        ],
        [
            1444 as libc::c_int as uint16_t,
            1725 as libc::c_int as uint16_t,
        ],
        [
            1469 as libc::c_int as uint16_t,
            1758 as libc::c_int as uint16_t,
        ],
        [
            1494 as libc::c_int as uint16_t,
            1791 as libc::c_int as uint16_t,
        ],
        [
            1519 as libc::c_int as uint16_t,
            1824 as libc::c_int as uint16_t,
        ],
        [
            1544 as libc::c_int as uint16_t,
            1856 as libc::c_int as uint16_t,
        ],
        [
            1569 as libc::c_int as uint16_t,
            1889 as libc::c_int as uint16_t,
        ],
        [
            1594 as libc::c_int as uint16_t,
            1922 as libc::c_int as uint16_t,
        ],
        [
            1618 as libc::c_int as uint16_t,
            1954 as libc::c_int as uint16_t,
        ],
        [
            1643 as libc::c_int as uint16_t,
            1987 as libc::c_int as uint16_t,
        ],
        [
            1668 as libc::c_int as uint16_t,
            2020 as libc::c_int as uint16_t,
        ],
        [
            1692 as libc::c_int as uint16_t,
            2052 as libc::c_int as uint16_t,
        ],
        [
            1717 as libc::c_int as uint16_t,
            2085 as libc::c_int as uint16_t,
        ],
        [
            1741 as libc::c_int as uint16_t,
            2118 as libc::c_int as uint16_t,
        ],
        [
            1765 as libc::c_int as uint16_t,
            2150 as libc::c_int as uint16_t,
        ],
        [
            1789 as libc::c_int as uint16_t,
            2183 as libc::c_int as uint16_t,
        ],
        [
            1814 as libc::c_int as uint16_t,
            2216 as libc::c_int as uint16_t,
        ],
        [
            1838 as libc::c_int as uint16_t,
            2248 as libc::c_int as uint16_t,
        ],
        [
            1862 as libc::c_int as uint16_t,
            2281 as libc::c_int as uint16_t,
        ],
        [
            1885 as libc::c_int as uint16_t,
            2313 as libc::c_int as uint16_t,
        ],
        [
            1909 as libc::c_int as uint16_t,
            2346 as libc::c_int as uint16_t,
        ],
        [
            1933 as libc::c_int as uint16_t,
            2378 as libc::c_int as uint16_t,
        ],
        [
            1957 as libc::c_int as uint16_t,
            2411 as libc::c_int as uint16_t,
        ],
        [
            1992 as libc::c_int as uint16_t,
            2459 as libc::c_int as uint16_t,
        ],
        [
            2027 as libc::c_int as uint16_t,
            2508 as libc::c_int as uint16_t,
        ],
        [
            2061 as libc::c_int as uint16_t,
            2556 as libc::c_int as uint16_t,
        ],
        [
            2096 as libc::c_int as uint16_t,
            2605 as libc::c_int as uint16_t,
        ],
        [
            2130 as libc::c_int as uint16_t,
            2653 as libc::c_int as uint16_t,
        ],
        [
            2165 as libc::c_int as uint16_t,
            2701 as libc::c_int as uint16_t,
        ],
        [
            2199 as libc::c_int as uint16_t,
            2750 as libc::c_int as uint16_t,
        ],
        [
            2233 as libc::c_int as uint16_t,
            2798 as libc::c_int as uint16_t,
        ],
        [
            2267 as libc::c_int as uint16_t,
            2847 as libc::c_int as uint16_t,
        ],
        [
            2300 as libc::c_int as uint16_t,
            2895 as libc::c_int as uint16_t,
        ],
        [
            2334 as libc::c_int as uint16_t,
            2943 as libc::c_int as uint16_t,
        ],
        [
            2367 as libc::c_int as uint16_t,
            2992 as libc::c_int as uint16_t,
        ],
        [
            2400 as libc::c_int as uint16_t,
            3040 as libc::c_int as uint16_t,
        ],
        [
            2434 as libc::c_int as uint16_t,
            3088 as libc::c_int as uint16_t,
        ],
        [
            2467 as libc::c_int as uint16_t,
            3137 as libc::c_int as uint16_t,
        ],
        [
            2499 as libc::c_int as uint16_t,
            3185 as libc::c_int as uint16_t,
        ],
        [
            2532 as libc::c_int as uint16_t,
            3234 as libc::c_int as uint16_t,
        ],
        [
            2575 as libc::c_int as uint16_t,
            3298 as libc::c_int as uint16_t,
        ],
        [
            2618 as libc::c_int as uint16_t,
            3362 as libc::c_int as uint16_t,
        ],
        [
            2661 as libc::c_int as uint16_t,
            3426 as libc::c_int as uint16_t,
        ],
        [
            2704 as libc::c_int as uint16_t,
            3491 as libc::c_int as uint16_t,
        ],
        [
            2746 as libc::c_int as uint16_t,
            3555 as libc::c_int as uint16_t,
        ],
        [
            2788 as libc::c_int as uint16_t,
            3619 as libc::c_int as uint16_t,
        ],
        [
            2830 as libc::c_int as uint16_t,
            3684 as libc::c_int as uint16_t,
        ],
        [
            2872 as libc::c_int as uint16_t,
            3748 as libc::c_int as uint16_t,
        ],
        [
            2913 as libc::c_int as uint16_t,
            3812 as libc::c_int as uint16_t,
        ],
        [
            2954 as libc::c_int as uint16_t,
            3876 as libc::c_int as uint16_t,
        ],
        [
            2995 as libc::c_int as uint16_t,
            3941 as libc::c_int as uint16_t,
        ],
        [
            3036 as libc::c_int as uint16_t,
            4005 as libc::c_int as uint16_t,
        ],
        [
            3076 as libc::c_int as uint16_t,
            4069 as libc::c_int as uint16_t,
        ],
        [
            3127 as libc::c_int as uint16_t,
            4149 as libc::c_int as uint16_t,
        ],
        [
            3177 as libc::c_int as uint16_t,
            4230 as libc::c_int as uint16_t,
        ],
        [
            3226 as libc::c_int as uint16_t,
            4310 as libc::c_int as uint16_t,
        ],
        [
            3275 as libc::c_int as uint16_t,
            4390 as libc::c_int as uint16_t,
        ],
        [
            3324 as libc::c_int as uint16_t,
            4470 as libc::c_int as uint16_t,
        ],
        [
            3373 as libc::c_int as uint16_t,
            4550 as libc::c_int as uint16_t,
        ],
        [
            3421 as libc::c_int as uint16_t,
            4631 as libc::c_int as uint16_t,
        ],
        [
            3469 as libc::c_int as uint16_t,
            4711 as libc::c_int as uint16_t,
        ],
        [
            3517 as libc::c_int as uint16_t,
            4791 as libc::c_int as uint16_t,
        ],
        [
            3565 as libc::c_int as uint16_t,
            4871 as libc::c_int as uint16_t,
        ],
        [
            3621 as libc::c_int as uint16_t,
            4967 as libc::c_int as uint16_t,
        ],
        [
            3677 as libc::c_int as uint16_t,
            5064 as libc::c_int as uint16_t,
        ],
        [
            3733 as libc::c_int as uint16_t,
            5160 as libc::c_int as uint16_t,
        ],
        [
            3788 as libc::c_int as uint16_t,
            5256 as libc::c_int as uint16_t,
        ],
        [
            3843 as libc::c_int as uint16_t,
            5352 as libc::c_int as uint16_t,
        ],
        [
            3897 as libc::c_int as uint16_t,
            5448 as libc::c_int as uint16_t,
        ],
        [
            3951 as libc::c_int as uint16_t,
            5544 as libc::c_int as uint16_t,
        ],
        [
            4005 as libc::c_int as uint16_t,
            5641 as libc::c_int as uint16_t,
        ],
        [
            4058 as libc::c_int as uint16_t,
            5737 as libc::c_int as uint16_t,
        ],
        [
            4119 as libc::c_int as uint16_t,
            5849 as libc::c_int as uint16_t,
        ],
        [
            4181 as libc::c_int as uint16_t,
            5961 as libc::c_int as uint16_t,
        ],
        [
            4241 as libc::c_int as uint16_t,
            6073 as libc::c_int as uint16_t,
        ],
        [
            4301 as libc::c_int as uint16_t,
            6185 as libc::c_int as uint16_t,
        ],
        [
            4361 as libc::c_int as uint16_t,
            6297 as libc::c_int as uint16_t,
        ],
        [
            4420 as libc::c_int as uint16_t,
            6410 as libc::c_int as uint16_t,
        ],
        [
            4479 as libc::c_int as uint16_t,
            6522 as libc::c_int as uint16_t,
        ],
        [
            4546 as libc::c_int as uint16_t,
            6650 as libc::c_int as uint16_t,
        ],
        [
            4612 as libc::c_int as uint16_t,
            6778 as libc::c_int as uint16_t,
        ],
        [
            4677 as libc::c_int as uint16_t,
            6906 as libc::c_int as uint16_t,
        ],
        [
            4742 as libc::c_int as uint16_t,
            7034 as libc::c_int as uint16_t,
        ],
        [
            4807 as libc::c_int as uint16_t,
            7162 as libc::c_int as uint16_t,
        ],
        [
            4871 as libc::c_int as uint16_t,
            7290 as libc::c_int as uint16_t,
        ],
        [
            4942 as libc::c_int as uint16_t,
            7435 as libc::c_int as uint16_t,
        ],
        [
            5013 as libc::c_int as uint16_t,
            7579 as libc::c_int as uint16_t,
        ],
        [
            5083 as libc::c_int as uint16_t,
            7723 as libc::c_int as uint16_t,
        ],
        [
            5153 as libc::c_int as uint16_t,
            7867 as libc::c_int as uint16_t,
        ],
        [
            5222 as libc::c_int as uint16_t,
            8011 as libc::c_int as uint16_t,
        ],
        [
            5291 as libc::c_int as uint16_t,
            8155 as libc::c_int as uint16_t,
        ],
        [
            5367 as libc::c_int as uint16_t,
            8315 as libc::c_int as uint16_t,
        ],
        [
            5442 as libc::c_int as uint16_t,
            8475 as libc::c_int as uint16_t,
        ],
        [
            5517 as libc::c_int as uint16_t,
            8635 as libc::c_int as uint16_t,
        ],
        [
            5591 as libc::c_int as uint16_t,
            8795 as libc::c_int as uint16_t,
        ],
        [
            5665 as libc::c_int as uint16_t,
            8956 as libc::c_int as uint16_t,
        ],
        [
            5745 as libc::c_int as uint16_t,
            9132 as libc::c_int as uint16_t,
        ],
        [
            5825 as libc::c_int as uint16_t,
            9308 as libc::c_int as uint16_t,
        ],
        [
            5905 as libc::c_int as uint16_t,
            9484 as libc::c_int as uint16_t,
        ],
        [
            5984 as libc::c_int as uint16_t,
            9660 as libc::c_int as uint16_t,
        ],
        [
            6063 as libc::c_int as uint16_t,
            9836 as libc::c_int as uint16_t,
        ],
        [
            6149 as libc::c_int as uint16_t,
            10028 as libc::c_int as uint16_t,
        ],
        [
            6234 as libc::c_int as uint16_t,
            10220 as libc::c_int as uint16_t,
        ],
        [
            6319 as libc::c_int as uint16_t,
            10412 as libc::c_int as uint16_t,
        ],
        [
            6404 as libc::c_int as uint16_t,
            10604 as libc::c_int as uint16_t,
        ],
        [
            6495 as libc::c_int as uint16_t,
            10812 as libc::c_int as uint16_t,
        ],
        [
            6587 as libc::c_int as uint16_t,
            11020 as libc::c_int as uint16_t,
        ],
        [
            6678 as libc::c_int as uint16_t,
            11228 as libc::c_int as uint16_t,
        ],
        [
            6769 as libc::c_int as uint16_t,
            11437 as libc::c_int as uint16_t,
        ],
        [
            6867 as libc::c_int as uint16_t,
            11661 as libc::c_int as uint16_t,
        ],
        [
            6966 as libc::c_int as uint16_t,
            11885 as libc::c_int as uint16_t,
        ],
        [
            7064 as libc::c_int as uint16_t,
            12109 as libc::c_int as uint16_t,
        ],
        [
            7163 as libc::c_int as uint16_t,
            12333 as libc::c_int as uint16_t,
        ],
        [
            7269 as libc::c_int as uint16_t,
            12573 as libc::c_int as uint16_t,
        ],
        [
            7376 as libc::c_int as uint16_t,
            12813 as libc::c_int as uint16_t,
        ],
        [
            7483 as libc::c_int as uint16_t,
            13053 as libc::c_int as uint16_t,
        ],
        [
            7599 as libc::c_int as uint16_t,
            13309 as libc::c_int as uint16_t,
        ],
        [
            7715 as libc::c_int as uint16_t,
            13565 as libc::c_int as uint16_t,
        ],
        [
            7832 as libc::c_int as uint16_t,
            13821 as libc::c_int as uint16_t,
        ],
        [
            7958 as libc::c_int as uint16_t,
            14093 as libc::c_int as uint16_t,
        ],
        [
            8085 as libc::c_int as uint16_t,
            14365 as libc::c_int as uint16_t,
        ],
        [
            8214 as libc::c_int as uint16_t,
            14637 as libc::c_int as uint16_t,
        ],
        [
            8352 as libc::c_int as uint16_t,
            14925 as libc::c_int as uint16_t,
        ],
        [
            8492 as libc::c_int as uint16_t,
            15213 as libc::c_int as uint16_t,
        ],
        [
            8635 as libc::c_int as uint16_t,
            15502 as libc::c_int as uint16_t,
        ],
        [
            8788 as libc::c_int as uint16_t,
            15806 as libc::c_int as uint16_t,
        ],
        [
            8945 as libc::c_int as uint16_t,
            16110 as libc::c_int as uint16_t,
        ],
        [
            9104 as libc::c_int as uint16_t,
            16414 as libc::c_int as uint16_t,
        ],
        [
            9275 as libc::c_int as uint16_t,
            16734 as libc::c_int as uint16_t,
        ],
        [
            9450 as libc::c_int as uint16_t,
            17054 as libc::c_int as uint16_t,
        ],
        [
            9639 as libc::c_int as uint16_t,
            17390 as libc::c_int as uint16_t,
        ],
        [
            9832 as libc::c_int as uint16_t,
            17726 as libc::c_int as uint16_t,
        ],
        [
            10031 as libc::c_int as uint16_t,
            18062 as libc::c_int as uint16_t,
        ],
        [
            10245 as libc::c_int as uint16_t,
            18414 as libc::c_int as uint16_t,
        ],
        [
            10465 as libc::c_int as uint16_t,
            18766 as libc::c_int as uint16_t,
        ],
        [
            10702 as libc::c_int as uint16_t,
            19134 as libc::c_int as uint16_t,
        ],
        [
            10946 as libc::c_int as uint16_t,
            19502 as libc::c_int as uint16_t,
        ],
        [
            11210 as libc::c_int as uint16_t,
            19886 as libc::c_int as uint16_t,
        ],
        [
            11482 as libc::c_int as uint16_t,
            20270 as libc::c_int as uint16_t,
        ],
        [
            11776 as libc::c_int as uint16_t,
            20670 as libc::c_int as uint16_t,
        ],
        [
            12081 as libc::c_int as uint16_t,
            21070 as libc::c_int as uint16_t,
        ],
        [
            12409 as libc::c_int as uint16_t,
            21486 as libc::c_int as uint16_t,
        ],
        [
            12750 as libc::c_int as uint16_t,
            21902 as libc::c_int as uint16_t,
        ],
        [
            13118 as libc::c_int as uint16_t,
            22334 as libc::c_int as uint16_t,
        ],
        [
            13501 as libc::c_int as uint16_t,
            22766 as libc::c_int as uint16_t,
        ],
        [
            13913 as libc::c_int as uint16_t,
            23214 as libc::c_int as uint16_t,
        ],
        [
            14343 as libc::c_int as uint16_t,
            23662 as libc::c_int as uint16_t,
        ],
        [
            14807 as libc::c_int as uint16_t,
            24126 as libc::c_int as uint16_t,
        ],
        [
            15290 as libc::c_int as uint16_t,
            24590 as libc::c_int as uint16_t,
        ],
        [
            15812 as libc::c_int as uint16_t,
            25070 as libc::c_int as uint16_t,
        ],
        [
            16356 as libc::c_int as uint16_t,
            25551 as libc::c_int as uint16_t,
        ],
        [
            16943 as libc::c_int as uint16_t,
            26047 as libc::c_int as uint16_t,
        ],
        [
            17575 as libc::c_int as uint16_t,
            26559 as libc::c_int as uint16_t,
        ],
        [
            18237 as libc::c_int as uint16_t,
            27071 as libc::c_int as uint16_t,
        ],
        [
            18949 as libc::c_int as uint16_t,
            27599 as libc::c_int as uint16_t,
        ],
        [
            19718 as libc::c_int as uint16_t,
            28143 as libc::c_int as uint16_t,
        ],
        [
            20521 as libc::c_int as uint16_t,
            28687 as libc::c_int as uint16_t,
        ],
        [
            21387 as libc::c_int as uint16_t,
            29247 as libc::c_int as uint16_t,
        ],
    ],
];
