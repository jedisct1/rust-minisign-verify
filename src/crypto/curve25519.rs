use std::cmp::{min, Eq, PartialEq};
use std::ops::{Add, Mul, Sub};

use super::util::fixed_time_eq;

#[derive(Clone, Copy)]
pub struct Fe(pub [i32; 10]);

impl PartialEq for Fe {
    fn eq(&self, other: &Fe) -> bool {
        let &Fe(self_elems) = self;
        let &Fe(other_elems) = other;
        self_elems.to_vec() == other_elems.to_vec()
    }
}
impl Eq for Fe {}

static FE_ZERO: Fe = Fe([0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static FE_ONE: Fe = Fe([1, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
static FE_SQRTM1: Fe = Fe([
    -32_595_792,
    -7_943_725,
    9_377_950,
    3_500_415,
    12_389_472,
    -272_473,
    -25_146_209,
    -2_005_654,
    326_686,
    11_406_482,
]);
static FE_D: Fe = Fe([
    -10_913_610,
    13_857_413,
    -15_372_611,
    6_949_391,
    114_729,
    -8_787_816,
    -6_275_908,
    -3_247_719,
    -18_696_448,
    -12_055_116,
]);
static FE_D2: Fe = Fe([
    -21_827_239,
    -5_839_606,
    -30_745_221,
    13_898_782,
    229_458,
    15_978_800,
    -12_551_817,
    -6_495_438,
    29_715_968,
    9_444_199,
]);

fn load_4u(s: &[u8]) -> u64 {
    (s[0] as u64) | ((s[1] as u64) << 8) | ((s[2] as u64) << 16) | ((s[3] as u64) << 24)
}
fn load_4i(s: &[u8]) -> i64 {
    load_4u(s) as i64
}
fn load_3u(s: &[u8]) -> u64 {
    (s[0] as u64) | ((s[1] as u64) << 8) | ((s[2] as u64) << 16)
}
fn load_3i(s: &[u8]) -> i64 {
    load_3u(s) as i64
}

impl Add for Fe {
    type Output = Fe;

    fn add(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let h0 = f0 + g0;
        let h1 = f1 + g1;
        let h2 = f2 + g2;
        let h3 = f3 + g3;
        let h4 = f4 + g4;
        let h5 = f5 + g5;
        let h6 = f6 + g6;
        let h7 = f7 + g7;
        let h8 = f8 + g8;
        let h9 = f9 + g9;
        Fe([h0, h1, h2, h3, h4, h5, h6, h7, h8, h9])
    }
}

impl Sub for Fe {
    type Output = Fe;

    fn sub(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let h0 = f0 - g0;
        let h1 = f1 - g1;
        let h2 = f2 - g2;
        let h3 = f3 - g3;
        let h4 = f4 - g4;
        let h5 = f5 - g5;
        let h6 = f6 - g6;
        let h7 = f7 - g7;
        let h8 = f8 - g8;
        let h9 = f9 - g9;
        Fe([h0, h1, h2, h3, h4, h5, h6, h7, h8, h9])
    }
}

impl Mul for Fe {
    type Output = Fe;

    fn mul(self, _rhs: Fe) -> Fe {
        let Fe(f) = self;
        let Fe(g) = _rhs;
        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let g0 = g[0];
        let g1 = g[1];
        let g2 = g[2];
        let g3 = g[3];
        let g4 = g[4];
        let g5 = g[5];
        let g6 = g[6];
        let g7 = g[7];
        let g8 = g[8];
        let g9 = g[9];
        let g1_19 = 19 * g1; /* 1.4*2^29 */
        let g2_19 = 19 * g2; /* 1.4*2^30; still ok */
        let g3_19 = 19 * g3;
        let g4_19 = 19 * g4;
        let g5_19 = 19 * g5;
        let g6_19 = 19 * g6;
        let g7_19 = 19 * g7;
        let g8_19 = 19 * g8;
        let g9_19 = 19 * g9;
        let f1_2 = 2 * f1;
        let f3_2 = 2 * f3;
        let f5_2 = 2 * f5;
        let f7_2 = 2 * f7;
        let f9_2 = 2 * f9;
        let f0g0 = (f0 as i64) * (g0 as i64);
        let f0g1 = (f0 as i64) * (g1 as i64);
        let f0g2 = (f0 as i64) * (g2 as i64);
        let f0g3 = (f0 as i64) * (g3 as i64);
        let f0g4 = (f0 as i64) * (g4 as i64);
        let f0g5 = (f0 as i64) * (g5 as i64);
        let f0g6 = (f0 as i64) * (g6 as i64);
        let f0g7 = (f0 as i64) * (g7 as i64);
        let f0g8 = (f0 as i64) * (g8 as i64);
        let f0g9 = (f0 as i64) * (g9 as i64);
        let f1g0 = (f1 as i64) * (g0 as i64);
        let f1g1_2 = (f1_2 as i64) * (g1 as i64);
        let f1g2 = (f1 as i64) * (g2 as i64);
        let f1g3_2 = (f1_2 as i64) * (g3 as i64);
        let f1g4 = (f1 as i64) * (g4 as i64);
        let f1g5_2 = (f1_2 as i64) * (g5 as i64);
        let f1g6 = (f1 as i64) * (g6 as i64);
        let f1g7_2 = (f1_2 as i64) * (g7 as i64);
        let f1g8 = (f1 as i64) * (g8 as i64);
        let f1g9_38 = (f1_2 as i64) * (g9_19 as i64);
        let f2g0 = (f2 as i64) * (g0 as i64);
        let f2g1 = (f2 as i64) * (g1 as i64);
        let f2g2 = (f2 as i64) * (g2 as i64);
        let f2g3 = (f2 as i64) * (g3 as i64);
        let f2g4 = (f2 as i64) * (g4 as i64);
        let f2g5 = (f2 as i64) * (g5 as i64);
        let f2g6 = (f2 as i64) * (g6 as i64);
        let f2g7 = (f2 as i64) * (g7 as i64);
        let f2g8_19 = (f2 as i64) * (g8_19 as i64);
        let f2g9_19 = (f2 as i64) * (g9_19 as i64);
        let f3g0 = (f3 as i64) * (g0 as i64);
        let f3g1_2 = (f3_2 as i64) * (g1 as i64);
        let f3g2 = (f3 as i64) * (g2 as i64);
        let f3g3_2 = (f3_2 as i64) * (g3 as i64);
        let f3g4 = (f3 as i64) * (g4 as i64);
        let f3g5_2 = (f3_2 as i64) * (g5 as i64);
        let f3g6 = (f3 as i64) * (g6 as i64);
        let f3g7_38 = (f3_2 as i64) * (g7_19 as i64);
        let f3g8_19 = (f3 as i64) * (g8_19 as i64);
        let f3g9_38 = (f3_2 as i64) * (g9_19 as i64);
        let f4g0 = (f4 as i64) * (g0 as i64);
        let f4g1 = (f4 as i64) * (g1 as i64);
        let f4g2 = (f4 as i64) * (g2 as i64);
        let f4g3 = (f4 as i64) * (g3 as i64);
        let f4g4 = (f4 as i64) * (g4 as i64);
        let f4g5 = (f4 as i64) * (g5 as i64);
        let f4g6_19 = (f4 as i64) * (g6_19 as i64);
        let f4g7_19 = (f4 as i64) * (g7_19 as i64);
        let f4g8_19 = (f4 as i64) * (g8_19 as i64);
        let f4g9_19 = (f4 as i64) * (g9_19 as i64);
        let f5g0 = (f5 as i64) * (g0 as i64);
        let f5g1_2 = (f5_2 as i64) * (g1 as i64);
        let f5g2 = (f5 as i64) * (g2 as i64);
        let f5g3_2 = (f5_2 as i64) * (g3 as i64);
        let f5g4 = (f5 as i64) * (g4 as i64);
        let f5g5_38 = (f5_2 as i64) * (g5_19 as i64);
        let f5g6_19 = (f5 as i64) * (g6_19 as i64);
        let f5g7_38 = (f5_2 as i64) * (g7_19 as i64);
        let f5g8_19 = (f5 as i64) * (g8_19 as i64);
        let f5g9_38 = (f5_2 as i64) * (g9_19 as i64);
        let f6g0 = (f6 as i64) * (g0 as i64);
        let f6g1 = (f6 as i64) * (g1 as i64);
        let f6g2 = (f6 as i64) * (g2 as i64);
        let f6g3 = (f6 as i64) * (g3 as i64);
        let f6g4_19 = (f6 as i64) * (g4_19 as i64);
        let f6g5_19 = (f6 as i64) * (g5_19 as i64);
        let f6g6_19 = (f6 as i64) * (g6_19 as i64);
        let f6g7_19 = (f6 as i64) * (g7_19 as i64);
        let f6g8_19 = (f6 as i64) * (g8_19 as i64);
        let f6g9_19 = (f6 as i64) * (g9_19 as i64);
        let f7g0 = (f7 as i64) * (g0 as i64);
        let f7g1_2 = (f7_2 as i64) * (g1 as i64);
        let f7g2 = (f7 as i64) * (g2 as i64);
        let f7g3_38 = (f7_2 as i64) * (g3_19 as i64);
        let f7g4_19 = (f7 as i64) * (g4_19 as i64);
        let f7g5_38 = (f7_2 as i64) * (g5_19 as i64);
        let f7g6_19 = (f7 as i64) * (g6_19 as i64);
        let f7g7_38 = (f7_2 as i64) * (g7_19 as i64);
        let f7g8_19 = (f7 as i64) * (g8_19 as i64);
        let f7g9_38 = (f7_2 as i64) * (g9_19 as i64);
        let f8g0 = (f8 as i64) * (g0 as i64);
        let f8g1 = (f8 as i64) * (g1 as i64);
        let f8g2_19 = (f8 as i64) * (g2_19 as i64);
        let f8g3_19 = (f8 as i64) * (g3_19 as i64);
        let f8g4_19 = (f8 as i64) * (g4_19 as i64);
        let f8g5_19 = (f8 as i64) * (g5_19 as i64);
        let f8g6_19 = (f8 as i64) * (g6_19 as i64);
        let f8g7_19 = (f8 as i64) * (g7_19 as i64);
        let f8g8_19 = (f8 as i64) * (g8_19 as i64);
        let f8g9_19 = (f8 as i64) * (g9_19 as i64);
        let f9g0 = (f9 as i64) * (g0 as i64);
        let f9g1_38 = (f9_2 as i64) * (g1_19 as i64);
        let f9g2_19 = (f9 as i64) * (g2_19 as i64);
        let f9g3_38 = (f9_2 as i64) * (g3_19 as i64);
        let f9g4_19 = (f9 as i64) * (g4_19 as i64);
        let f9g5_38 = (f9_2 as i64) * (g5_19 as i64);
        let f9g6_19 = (f9 as i64) * (g6_19 as i64);
        let f9g7_38 = (f9_2 as i64) * (g7_19 as i64);
        let f9g8_19 = (f9 as i64) * (g8_19 as i64);
        let f9g9_38 = (f9_2 as i64) * (g9_19 as i64);
        let mut h0 = f0g0
            + f1g9_38
            + f2g8_19
            + f3g7_38
            + f4g6_19
            + f5g5_38
            + f6g4_19
            + f7g3_38
            + f8g2_19
            + f9g1_38;
        let mut h1 = f0g1
            + f1g0
            + f2g9_19
            + f3g8_19
            + f4g7_19
            + f5g6_19
            + f6g5_19
            + f7g4_19
            + f8g3_19
            + f9g2_19;
        let mut h2 = f0g2
            + f1g1_2
            + f2g0
            + f3g9_38
            + f4g8_19
            + f5g7_38
            + f6g6_19
            + f7g5_38
            + f8g4_19
            + f9g3_38;
        let mut h3 =
            f0g3 + f1g2 + f2g1 + f3g0 + f4g9_19 + f5g8_19 + f6g7_19 + f7g6_19 + f8g5_19 + f9g4_19;
        let mut h4 =
            f0g4 + f1g3_2 + f2g2 + f3g1_2 + f4g0 + f5g9_38 + f6g8_19 + f7g7_38 + f8g6_19 + f9g5_38;
        let mut h5 =
            f0g5 + f1g4 + f2g3 + f3g2 + f4g1 + f5g0 + f6g9_19 + f7g8_19 + f8g7_19 + f9g6_19;
        let mut h6 =
            f0g6 + f1g5_2 + f2g4 + f3g3_2 + f4g2 + f5g1_2 + f6g0 + f7g9_38 + f8g8_19 + f9g7_38;
        let mut h7 = f0g7 + f1g6 + f2g5 + f3g4 + f4g3 + f5g2 + f6g1 + f7g0 + f8g9_19 + f9g8_19;
        let mut h8 = f0g8 + f1g7_2 + f2g6 + f3g5_2 + f4g4 + f5g3_2 + f6g2 + f7g1_2 + f8g0 + f9g9_38;
        let mut h9 = f0g9 + f1g8 + f2g7 + f3g6 + f4g5 + f5g4 + f6g3 + f7g2 + f8g1 + f9g0;
        let mut carry0;
        let carry1;
        let carry2;
        let carry3;
        let mut carry4;
        let carry5;
        let carry6;
        let carry7;
        let carry8;
        let carry9;

        carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        /* |h0| <= 2^25 */
        /* |h4| <= 2^25 */
        /* |h1| <= 1.51*2^58 */
        /* |h5| <= 1.51*2^58 */

        carry1 = (h1 + (1 << 24)) >> 25;
        h2 += carry1;
        h1 -= carry1 << 25;
        carry5 = (h5 + (1 << 24)) >> 25;
        h6 += carry5;
        h5 -= carry5 << 25;
        /* |h1| <= 2^24; from now on fits into int32 */
        /* |h5| <= 2^24; from now on fits into int32 */
        /* |h2| <= 1.21*2^59 */
        /* |h6| <= 1.21*2^59 */

        carry2 = (h2 + (1 << 25)) >> 26;
        h3 += carry2;
        h2 -= carry2 << 26;
        carry6 = (h6 + (1 << 25)) >> 26;
        h7 += carry6;
        h6 -= carry6 << 26;
        /* |h2| <= 2^25; from now on fits into int32 unchanged */
        /* |h6| <= 2^25; from now on fits into int32 unchanged */
        /* |h3| <= 1.51*2^58 */
        /* |h7| <= 1.51*2^58 */

        carry3 = (h3 + (1 << 24)) >> 25;
        h4 += carry3;
        h3 -= carry3 << 25;
        carry7 = (h7 + (1 << 24)) >> 25;
        h8 += carry7;
        h7 -= carry7 << 25;
        /* |h3| <= 2^24; from now on fits into int32 unchanged */
        /* |h7| <= 2^24; from now on fits into int32 unchanged */
        /* |h4| <= 1.52*2^33 */
        /* |h8| <= 1.52*2^33 */

        carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        carry8 = (h8 + (1 << 25)) >> 26;
        h9 += carry8;
        h8 -= carry8 << 26;
        /* |h4| <= 2^25; from now on fits into int32 unchanged */
        /* |h8| <= 2^25; from now on fits into int32 unchanged */
        /* |h5| <= 1.01*2^24 */
        /* |h9| <= 1.51*2^58 */

        carry9 = (h9 + (1 << 24)) >> 25;
        h0 += carry9 * 19;
        h9 -= carry9 << 25;
        /* |h9| <= 2^24; from now on fits into int32 unchanged */
        /* |h0| <= 1.8*2^37 */

        carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        /* |h0| <= 2^25; from now on fits into int32 unchanged */
        /* |h1| <= 1.01*2^24 */

        Fe([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }
}

impl Fe {
    pub fn from_bytes(s: &[u8]) -> Fe {
        let mut h0 = load_4i(&s[0..4]);
        let mut h1 = load_3i(&s[4..7]) << 6;
        let mut h2 = load_3i(&s[7..10]) << 5;
        let mut h3 = load_3i(&s[10..13]) << 3;
        let mut h4 = load_3i(&s[13..16]) << 2;
        let mut h5 = load_4i(&s[16..20]);
        let mut h6 = load_3i(&s[20..23]) << 7;
        let mut h7 = load_3i(&s[23..26]) << 5;
        let mut h8 = load_3i(&s[26..29]) << 4;
        let mut h9 = (load_3i(&s[29..32]) & 8_388_607) << 2;

        let carry9 = (h9 + (1 << 24)) >> 25;
        h0 += carry9 * 19;
        h9 -= carry9 << 25;
        let carry1 = (h1 + (1 << 24)) >> 25;
        h2 += carry1;
        h1 -= carry1 << 25;
        let carry3 = (h3 + (1 << 24)) >> 25;
        h4 += carry3;
        h3 -= carry3 << 25;
        let carry5 = (h5 + (1 << 24)) >> 25;
        h6 += carry5;
        h5 -= carry5 << 25;
        let carry7 = (h7 + (1 << 24)) >> 25;
        h8 += carry7;
        h7 -= carry7 << 25;

        let carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        let carry2 = (h2 + (1 << 25)) >> 26;
        h3 += carry2;
        h2 -= carry2 << 26;
        let carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        let carry6 = (h6 + (1 << 25)) >> 26;
        h7 += carry6;
        h6 -= carry6 << 26;
        let carry8 = (h8 + (1 << 25)) >> 26;
        h9 += carry8;
        h8 -= carry8 << 26;

        Fe([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let &Fe(es) = self;
        let mut h0 = es[0];
        let mut h1 = es[1];
        let mut h2 = es[2];
        let mut h3 = es[3];
        let mut h4 = es[4];
        let mut h5 = es[5];
        let mut h6 = es[6];
        let mut h7 = es[7];
        let mut h8 = es[8];
        let mut h9 = es[9];
        let mut q;

        q = (19 * h9 + (1 << 24)) >> 25;
        q = (h0 + q) >> 26;
        q = (h1 + q) >> 25;
        q = (h2 + q) >> 26;
        q = (h3 + q) >> 25;
        q = (h4 + q) >> 26;
        q = (h5 + q) >> 25;
        q = (h6 + q) >> 26;
        q = (h7 + q) >> 25;
        q = (h8 + q) >> 26;
        q = (h9 + q) >> 25;

        /* Goal: Output h-(2^255-19)q, which is between 0 and 2^255-20. */
        h0 += 19 * q;
        /* Goal: Output h-2^255 q, which is between 0 and 2^255-20. */

        let carry0 = h0 >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        let carry1 = h1 >> 25;
        h2 += carry1;
        h1 -= carry1 << 25;
        let carry2 = h2 >> 26;
        h3 += carry2;
        h2 -= carry2 << 26;
        let carry3 = h3 >> 25;
        h4 += carry3;
        h3 -= carry3 << 25;
        let carry4 = h4 >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        let carry5 = h5 >> 25;
        h6 += carry5;
        h5 -= carry5 << 25;
        let carry6 = h6 >> 26;
        h7 += carry6;
        h6 -= carry6 << 26;
        let carry7 = h7 >> 25;
        h8 += carry7;
        h7 -= carry7 << 25;
        let carry8 = h8 >> 26;
        h9 += carry8;
        h8 -= carry8 << 26;
        let carry9 = h9 >> 25;
        h9 -= carry9 << 25;
        /* h10 = carry9 */

        /*
        Goal: Output h0+...+2^255 h10-2^255 q, which is between 0 and 2^255-20.
        Have h0+...+2^230 h9 between 0 and 2^255-1;
        evidently 2^255 h10-2^255 q = 0.
        Goal: Output h0+...+2^230 h9.
        */
        [
            (h0 >> 0) as u8,
            (h0 >> 8) as u8,
            (h0 >> 16) as u8,
            ((h0 >> 24) | (h1 << 2)) as u8,
            (h1 >> 6) as u8,
            (h1 >> 14) as u8,
            ((h1 >> 22) | (h2 << 3)) as u8,
            (h2 >> 5) as u8,
            (h2 >> 13) as u8,
            ((h2 >> 21) | (h3 << 5)) as u8,
            (h3 >> 3) as u8,
            (h3 >> 11) as u8,
            ((h3 >> 19) | (h4 << 6)) as u8,
            (h4 >> 2) as u8,
            (h4 >> 10) as u8,
            (h4 >> 18) as u8,
            (h5 >> 0) as u8,
            (h5 >> 8) as u8,
            (h5 >> 16) as u8,
            ((h5 >> 24) | (h6 << 1)) as u8,
            (h6 >> 7) as u8,
            (h6 >> 15) as u8,
            ((h6 >> 23) | (h7 << 3)) as u8,
            (h7 >> 5) as u8,
            (h7 >> 13) as u8,
            ((h7 >> 21) | (h8 << 4)) as u8,
            (h8 >> 4) as u8,
            (h8 >> 12) as u8,
            ((h8 >> 20) | (h9 << 6)) as u8,
            (h9 >> 2) as u8,
            (h9 >> 10) as u8,
            (h9 >> 18) as u8,
        ]
    }

    fn square(&self) -> Fe {
        let &Fe(f) = self;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let f0_2 = 2 * f0;
        let f1_2 = 2 * f1;
        let f2_2 = 2 * f2;
        let f3_2 = 2 * f3;
        let f4_2 = 2 * f4;
        let f5_2 = 2 * f5;
        let f6_2 = 2 * f6;
        let f7_2 = 2 * f7;
        let f5_38 = 38 * f5; /* 1.31*2^30 */
        let f6_19 = 19 * f6; /* 1.31*2^30 */
        let f7_38 = 38 * f7; /* 1.31*2^30 */
        let f8_19 = 19 * f8; /* 1.31*2^30 */
        let f9_38 = 38 * f9; /* 1.31*2^30 */
        let f0f0 = (f0 as i64) * (f0 as i64);
        let f0f1_2 = (f0_2 as i64) * (f1 as i64);
        let f0f2_2 = (f0_2 as i64) * (f2 as i64);
        let f0f3_2 = (f0_2 as i64) * (f3 as i64);
        let f0f4_2 = (f0_2 as i64) * (f4 as i64);
        let f0f5_2 = (f0_2 as i64) * (f5 as i64);
        let f0f6_2 = (f0_2 as i64) * (f6 as i64);
        let f0f7_2 = (f0_2 as i64) * (f7 as i64);
        let f0f8_2 = (f0_2 as i64) * (f8 as i64);
        let f0f9_2 = (f0_2 as i64) * (f9 as i64);
        let f1f1_2 = (f1_2 as i64) * (f1 as i64);
        let f1f2_2 = (f1_2 as i64) * (f2 as i64);
        let f1f3_4 = (f1_2 as i64) * (f3_2 as i64);
        let f1f4_2 = (f1_2 as i64) * (f4 as i64);
        let f1f5_4 = (f1_2 as i64) * (f5_2 as i64);
        let f1f6_2 = (f1_2 as i64) * (f6 as i64);
        let f1f7_4 = (f1_2 as i64) * (f7_2 as i64);
        let f1f8_2 = (f1_2 as i64) * (f8 as i64);
        let f1f9_76 = (f1_2 as i64) * (f9_38 as i64);
        let f2f2 = (f2 as i64) * (f2 as i64);
        let f2f3_2 = (f2_2 as i64) * (f3 as i64);
        let f2f4_2 = (f2_2 as i64) * (f4 as i64);
        let f2f5_2 = (f2_2 as i64) * (f5 as i64);
        let f2f6_2 = (f2_2 as i64) * (f6 as i64);
        let f2f7_2 = (f2_2 as i64) * (f7 as i64);
        let f2f8_38 = (f2_2 as i64) * (f8_19 as i64);
        let f2f9_38 = (f2 as i64) * (f9_38 as i64);
        let f3f3_2 = (f3_2 as i64) * (f3 as i64);
        let f3f4_2 = (f3_2 as i64) * (f4 as i64);
        let f3f5_4 = (f3_2 as i64) * (f5_2 as i64);
        let f3f6_2 = (f3_2 as i64) * (f6 as i64);
        let f3f7_76 = (f3_2 as i64) * (f7_38 as i64);
        let f3f8_38 = (f3_2 as i64) * (f8_19 as i64);
        let f3f9_76 = (f3_2 as i64) * (f9_38 as i64);
        let f4f4 = (f4 as i64) * (f4 as i64);
        let f4f5_2 = (f4_2 as i64) * (f5 as i64);
        let f4f6_38 = (f4_2 as i64) * (f6_19 as i64);
        let f4f7_38 = (f4 as i64) * (f7_38 as i64);
        let f4f8_38 = (f4_2 as i64) * (f8_19 as i64);
        let f4f9_38 = (f4 as i64) * (f9_38 as i64);
        let f5f5_38 = (f5 as i64) * (f5_38 as i64);
        let f5f6_38 = (f5_2 as i64) * (f6_19 as i64);
        let f5f7_76 = (f5_2 as i64) * (f7_38 as i64);
        let f5f8_38 = (f5_2 as i64) * (f8_19 as i64);
        let f5f9_76 = (f5_2 as i64) * (f9_38 as i64);
        let f6f6_19 = (f6 as i64) * (f6_19 as i64);
        let f6f7_38 = (f6 as i64) * (f7_38 as i64);
        let f6f8_38 = (f6_2 as i64) * (f8_19 as i64);
        let f6f9_38 = (f6 as i64) * (f9_38 as i64);
        let f7f7_38 = (f7 as i64) * (f7_38 as i64);
        let f7f8_38 = (f7_2 as i64) * (f8_19 as i64);
        let f7f9_76 = (f7_2 as i64) * (f9_38 as i64);
        let f8f8_19 = (f8 as i64) * (f8_19 as i64);
        let f8f9_38 = (f8 as i64) * (f9_38 as i64);
        let f9f9_38 = (f9 as i64) * (f9_38 as i64);
        let mut h0 = f0f0 + f1f9_76 + f2f8_38 + f3f7_76 + f4f6_38 + f5f5_38;
        let mut h1 = f0f1_2 + f2f9_38 + f3f8_38 + f4f7_38 + f5f6_38;
        let mut h2 = f0f2_2 + f1f1_2 + f3f9_76 + f4f8_38 + f5f7_76 + f6f6_19;
        let mut h3 = f0f3_2 + f1f2_2 + f4f9_38 + f5f8_38 + f6f7_38;
        let mut h4 = f0f4_2 + f1f3_4 + f2f2 + f5f9_76 + f6f8_38 + f7f7_38;
        let mut h5 = f0f5_2 + f1f4_2 + f2f3_2 + f6f9_38 + f7f8_38;
        let mut h6 = f0f6_2 + f1f5_4 + f2f4_2 + f3f3_2 + f7f9_76 + f8f8_19;
        let mut h7 = f0f7_2 + f1f6_2 + f2f5_2 + f3f4_2 + f8f9_38;
        let mut h8 = f0f8_2 + f1f7_4 + f2f6_2 + f3f5_4 + f4f4 + f9f9_38;
        let mut h9 = f0f9_2 + f1f8_2 + f2f7_2 + f3f6_2 + f4f5_2;

        let carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        let carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;

        let carry1 = (h1 + (1 << 24)) >> 25;
        h2 += carry1;
        h1 -= carry1 << 25;
        let carry5 = (h5 + (1 << 24)) >> 25;
        h6 += carry5;
        h5 -= carry5 << 25;

        let carry2 = (h2 + (1 << 25)) >> 26;
        h3 += carry2;
        h2 -= carry2 << 26;
        let carry6 = (h6 + (1 << 25)) >> 26;
        h7 += carry6;
        h6 -= carry6 << 26;

        let carry3 = (h3 + (1 << 24)) >> 25;
        h4 += carry3;
        h3 -= carry3 << 25;
        let carry7 = (h7 + (1 << 24)) >> 25;
        h8 += carry7;
        h7 -= carry7 << 25;

        let carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        let carry8 = (h8 + (1 << 25)) >> 26;
        h9 += carry8;
        h8 -= carry8 << 26;

        let carry9 = (h9 + (1 << 24)) >> 25;
        h0 += carry9 * 19;
        h9 -= carry9 << 25;

        let carrya = (h0 + (1 << 25)) >> 26;
        h1 += carrya;
        h0 -= carrya << 26;

        Fe([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }

    fn square_and_double(&self) -> Fe {
        let &Fe(f) = self;

        let f0 = f[0];
        let f1 = f[1];
        let f2 = f[2];
        let f3 = f[3];
        let f4 = f[4];
        let f5 = f[5];
        let f6 = f[6];
        let f7 = f[7];
        let f8 = f[8];
        let f9 = f[9];
        let f0_2 = 2 * f0;
        let f1_2 = 2 * f1;
        let f2_2 = 2 * f2;
        let f3_2 = 2 * f3;
        let f4_2 = 2 * f4;
        let f5_2 = 2 * f5;
        let f6_2 = 2 * f6;
        let f7_2 = 2 * f7;
        let f5_38 = 38 * f5; /* 1.959375*2^30 */
        let f6_19 = 19 * f6; /* 1.959375*2^30 */
        let f7_38 = 38 * f7; /* 1.959375*2^30 */
        let f8_19 = 19 * f8; /* 1.959375*2^30 */
        let f9_38 = 38 * f9; /* 1.959375*2^30 */
        let f0f0 = (f0 as i64) * (f0 as i64);
        let f0f1_2 = (f0_2 as i64) * (f1 as i64);
        let f0f2_2 = (f0_2 as i64) * (f2 as i64);
        let f0f3_2 = (f0_2 as i64) * (f3 as i64);
        let f0f4_2 = (f0_2 as i64) * (f4 as i64);
        let f0f5_2 = (f0_2 as i64) * (f5 as i64);
        let f0f6_2 = (f0_2 as i64) * (f6 as i64);
        let f0f7_2 = (f0_2 as i64) * (f7 as i64);
        let f0f8_2 = (f0_2 as i64) * (f8 as i64);
        let f0f9_2 = (f0_2 as i64) * (f9 as i64);
        let f1f1_2 = (f1_2 as i64) * (f1 as i64);
        let f1f2_2 = (f1_2 as i64) * (f2 as i64);
        let f1f3_4 = (f1_2 as i64) * (f3_2 as i64);
        let f1f4_2 = (f1_2 as i64) * (f4 as i64);
        let f1f5_4 = (f1_2 as i64) * (f5_2 as i64);
        let f1f6_2 = (f1_2 as i64) * (f6 as i64);
        let f1f7_4 = (f1_2 as i64) * (f7_2 as i64);
        let f1f8_2 = (f1_2 as i64) * (f8 as i64);
        let f1f9_76 = (f1_2 as i64) * (f9_38 as i64);
        let f2f2 = (f2 as i64) * (f2 as i64);
        let f2f3_2 = (f2_2 as i64) * (f3 as i64);
        let f2f4_2 = (f2_2 as i64) * (f4 as i64);
        let f2f5_2 = (f2_2 as i64) * (f5 as i64);
        let f2f6_2 = (f2_2 as i64) * (f6 as i64);
        let f2f7_2 = (f2_2 as i64) * (f7 as i64);
        let f2f8_38 = (f2_2 as i64) * (f8_19 as i64);
        let f2f9_38 = (f2 as i64) * (f9_38 as i64);
        let f3f3_2 = (f3_2 as i64) * (f3 as i64);
        let f3f4_2 = (f3_2 as i64) * (f4 as i64);
        let f3f5_4 = (f3_2 as i64) * (f5_2 as i64);
        let f3f6_2 = (f3_2 as i64) * (f6 as i64);
        let f3f7_76 = (f3_2 as i64) * (f7_38 as i64);
        let f3f8_38 = (f3_2 as i64) * (f8_19 as i64);
        let f3f9_76 = (f3_2 as i64) * (f9_38 as i64);
        let f4f4 = (f4 as i64) * (f4 as i64);
        let f4f5_2 = (f4_2 as i64) * (f5 as i64);
        let f4f6_38 = (f4_2 as i64) * (f6_19 as i64);
        let f4f7_38 = (f4 as i64) * (f7_38 as i64);
        let f4f8_38 = (f4_2 as i64) * (f8_19 as i64);
        let f4f9_38 = (f4 as i64) * (f9_38 as i64);
        let f5f5_38 = (f5 as i64) * (f5_38 as i64);
        let f5f6_38 = (f5_2 as i64) * (f6_19 as i64);
        let f5f7_76 = (f5_2 as i64) * (f7_38 as i64);
        let f5f8_38 = (f5_2 as i64) * (f8_19 as i64);
        let f5f9_76 = (f5_2 as i64) * (f9_38 as i64);
        let f6f6_19 = (f6 as i64) * (f6_19 as i64);
        let f6f7_38 = (f6 as i64) * (f7_38 as i64);
        let f6f8_38 = (f6_2 as i64) * (f8_19 as i64);
        let f6f9_38 = (f6 as i64) * (f9_38 as i64);
        let f7f7_38 = (f7 as i64) * (f7_38 as i64);
        let f7f8_38 = (f7_2 as i64) * (f8_19 as i64);
        let f7f9_76 = (f7_2 as i64) * (f9_38 as i64);
        let f8f8_19 = (f8 as i64) * (f8_19 as i64);
        let f8f9_38 = (f8 as i64) * (f9_38 as i64);
        let f9f9_38 = (f9 as i64) * (f9_38 as i64);
        let mut h0 = f0f0 + f1f9_76 + f2f8_38 + f3f7_76 + f4f6_38 + f5f5_38;
        let mut h1 = f0f1_2 + f2f9_38 + f3f8_38 + f4f7_38 + f5f6_38;
        let mut h2 = f0f2_2 + f1f1_2 + f3f9_76 + f4f8_38 + f5f7_76 + f6f6_19;
        let mut h3 = f0f3_2 + f1f2_2 + f4f9_38 + f5f8_38 + f6f7_38;
        let mut h4 = f0f4_2 + f1f3_4 + f2f2 + f5f9_76 + f6f8_38 + f7f7_38;
        let mut h5 = f0f5_2 + f1f4_2 + f2f3_2 + f6f9_38 + f7f8_38;
        let mut h6 = f0f6_2 + f1f5_4 + f2f4_2 + f3f3_2 + f7f9_76 + f8f8_19;
        let mut h7 = f0f7_2 + f1f6_2 + f2f5_2 + f3f4_2 + f8f9_38;
        let mut h8 = f0f8_2 + f1f7_4 + f2f6_2 + f3f5_4 + f4f4 + f9f9_38;
        let mut h9 = f0f9_2 + f1f8_2 + f2f7_2 + f3f6_2 + f4f5_2;
        let mut carry0: i64;
        let carry1: i64;
        let carry2: i64;
        let carry3: i64;
        let mut carry4: i64;
        let carry5: i64;
        let carry6: i64;
        let carry7: i64;
        let carry8: i64;
        let carry9: i64;

        h0 += h0;
        h1 += h1;
        h2 += h2;
        h3 += h3;
        h4 += h4;
        h5 += h5;
        h6 += h6;
        h7 += h7;
        h8 += h8;
        h9 += h9;

        carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;
        carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;

        carry1 = (h1 + (1 << 24)) >> 25;
        h2 += carry1;
        h1 -= carry1 << 25;
        carry5 = (h5 + (1 << 24)) >> 25;
        h6 += carry5;
        h5 -= carry5 << 25;

        carry2 = (h2 + (1 << 25)) >> 26;
        h3 += carry2;
        h2 -= carry2 << 26;
        carry6 = (h6 + (1 << 25)) >> 26;
        h7 += carry6;
        h6 -= carry6 << 26;

        carry3 = (h3 + (1 << 24)) >> 25;
        h4 += carry3;
        h3 -= carry3 << 25;
        carry7 = (h7 + (1 << 24)) >> 25;
        h8 += carry7;
        h7 -= carry7 << 25;

        carry4 = (h4 + (1 << 25)) >> 26;
        h5 += carry4;
        h4 -= carry4 << 26;
        carry8 = (h8 + (1 << 25)) >> 26;
        h9 += carry8;
        h8 -= carry8 << 26;

        carry9 = (h9 + (1 << 24)) >> 25;
        h0 += carry9 * 19;
        h9 -= carry9 << 25;

        carry0 = (h0 + (1 << 25)) >> 26;
        h1 += carry0;
        h0 -= carry0 << 26;

        Fe([
            h0 as i32, h1 as i32, h2 as i32, h3 as i32, h4 as i32, h5 as i32, h6 as i32, h7 as i32,
            h8 as i32, h9 as i32,
        ])
    }

    pub fn invert(&self) -> Fe {
        let z1 = *self;

        /* qhasm: z2 = z1^2^1 */
        let z2 = z1.square();
        /* qhasm: z8 = z2^2^2 */
        let z8 = z2.square().square();
        /* qhasm: z9 = z1*z8 */
        let z9 = z1 * z8;

        /* qhasm: z11 = z2*z9 */
        let z11 = z2 * z9;

        /* qhasm: z22 = z11^2^1 */
        let z22 = z11.square();

        /* qhasm: z_5_0 = z9*z22 */
        let z_5_0 = z9 * z22;

        /* qhasm: z_10_5 = z_5_0^2^5 */
        let z_10_5 = (0..5).fold(z_5_0, |z_5_n, _| z_5_n.square());

        /* qhasm: z_10_0 = z_10_5*z_5_0 */
        let z_10_0 = z_10_5 * z_5_0;

        /* qhasm: z_20_10 = z_10_0^2^10 */
        let z_20_10 = (0..10).fold(z_10_0, |x, _| x.square());

        /* qhasm: z_20_0 = z_20_10*z_10_0 */
        let z_20_0 = z_20_10 * z_10_0;

        /* qhasm: z_40_20 = z_20_0^2^20 */
        let z_40_20 = (0..20).fold(z_20_0, |x, _| x.square());

        /* qhasm: z_40_0 = z_40_20*z_20_0 */
        let z_40_0 = z_40_20 * z_20_0;

        /* qhasm: z_50_10 = z_40_0^2^10 */
        let z_50_10 = (0..10).fold(z_40_0, |x, _| x.square());

        /* qhasm: z_50_0 = z_50_10*z_10_0 */
        let z_50_0 = z_50_10 * z_10_0;

        /* qhasm: z_100_50 = z_50_0^2^50 */
        let z_100_50 = (0..50).fold(z_50_0, |x, _| x.square());

        /* qhasm: z_100_0 = z_100_50*z_50_0 */
        let z_100_0 = z_100_50 * z_50_0;

        /* qhasm: z_200_100 = z_100_0^2^100 */
        let z_200_100 = (0..100).fold(z_100_0, |x, _| x.square());

        /* qhasm: z_200_0 = z_200_100*z_100_0 */
        /* asm 1: fe_mul(>z_200_0=fe#3,<z_200_100=fe#4,<z_100_0=fe#3); */
        /* asm 2: fe_mul(>z_200_0=t2,<z_200_100=t3,<z_100_0=t2); */
        let z_200_0 = z_200_100 * z_100_0;

        /* qhasm: z_250_50 = z_200_0^2^50 */
        let z_250_50 = (0..50).fold(z_200_0, |x, _| x.square());

        /* qhasm: z_250_0 = z_250_50*z_50_0 */
        let z_250_0 = z_250_50 * z_50_0;

        /* qhasm: z_255_5 = z_250_0^2^5 */
        let z_255_5 = (0..5).fold(z_250_0, |x, _| x.square());

        /* qhasm: z_255_21 = z_255_5*z11 */
        /* asm 1: fe_mul(>z_255_21=fe#12,<z_255_5=fe#2,<z11=fe#1); */
        /* asm 2: fe_mul(>z_255_21=out,<z_255_5=t1,<z11=t0); */
        let z_255_21 = z_255_5 * z11;

        z_255_21
    }

    fn is_nonzero(&self) -> bool {
        let bs = self.to_bytes();
        let zero = [0; 32];
        !fixed_time_eq(bs.as_ref(), zero.as_ref())
    }

    fn is_negative(&self) -> bool {
        (self.to_bytes()[0] & 1) != 0
    }

    fn neg(&self) -> Fe {
        let &Fe(f) = self;
        Fe([
            -f[0], -f[1], -f[2], -f[3], -f[4], -f[5], -f[6], -f[7], -f[8], -f[9],
        ])
    }

    fn pow25523(&self) -> Fe {
        let z2 = self.square();
        let z8 = (0..2).fold(z2, |x, _| x.square());
        let z9 = *self * z8;
        let z11 = z2 * z9;
        let z22 = z11.square();
        let z_5_0 = z9 * z22;
        let z_10_5 = (0..5).fold(z_5_0, |x, _| x.square());
        let z_10_0 = z_10_5 * z_5_0;
        let z_20_10 = (0..10).fold(z_10_0, |x, _| x.square());
        let z_20_0 = z_20_10 * z_10_0;
        let z_40_20 = (0..20).fold(z_20_0, |x, _| x.square());
        let z_40_0 = z_40_20 * z_20_0;
        let z_50_10 = (0..10).fold(z_40_0, |x, _| x.square());
        let z_50_0 = z_50_10 * z_10_0;
        let z_100_50 = (0..50).fold(z_50_0, |x, _| x.square());
        let z_100_0 = z_100_50 * z_50_0;
        let z_200_100 = (0..100).fold(z_100_0, |x, _| x.square());
        let z_200_0 = z_200_100 * z_100_0;
        let z_250_50 = (0..50).fold(z_200_0, |x, _| x.square());
        let z_250_0 = z_250_50 * z_50_0;
        let z_252_2 = (0..2).fold(z_250_0, |x, _| x.square());
        let z_252_3 = z_252_2 * *self;

        z_252_3
    }
}

#[derive(Clone, Copy)]
pub struct GeP2 {
    x: Fe,
    y: Fe,
    z: Fe,
}

#[derive(Clone, Copy)]
pub struct GeP3 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

#[derive(Clone, Copy)]
pub struct GeP1P1 {
    x: Fe,
    y: Fe,
    z: Fe,
    t: Fe,
}

#[derive(Clone, Copy)]
pub struct GePrecomp {
    y_plus_x: Fe,
    y_minus_x: Fe,
    xy2d: Fe,
}

#[derive(Clone, Copy)]
pub struct GeCached {
    y_plus_x: Fe,
    y_minus_x: Fe,
    z: Fe,
    t2d: Fe,
}

impl GeP1P1 {
    fn to_p2(&self) -> GeP2 {
        GeP2 {
            x: self.x * self.t,
            y: self.y * self.z,
            z: self.z * self.t,
        }
    }

    fn to_p3(&self) -> GeP3 {
        GeP3 {
            x: self.x * self.t,
            y: self.y * self.z,
            z: self.z * self.t,
            t: self.x * self.y,
        }
    }
}

impl GeP2 {
    fn zero() -> GeP2 {
        GeP2 {
            x: FE_ZERO,
            y: FE_ONE,
            z: FE_ONE,
        }
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let recip = self.z.invert();
        let x = self.x * recip;
        let y = self.y * recip;
        let mut bs = y.to_bytes();
        bs[31] ^= (if x.is_negative() { 1 } else { 0 }) << 7;
        bs
    }

    fn dbl(&self) -> GeP1P1 {
        let xx = self.x.square();
        let yy = self.y.square();
        let b = self.z.square_and_double();
        let a = self.x + self.y;
        let aa = a.square();
        let y3 = yy + xx;
        let z3 = yy - xx;
        let x3 = aa - y3;
        let t3 = b - z3;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }

    fn slide(a: &[u8]) -> [i8; 256] {
        let mut r = [0i8; 256];
        for i in 0..256 {
            r[i] = (1 & (a[i >> 3] >> (i & 7))) as i8;
        }
        for i in 0..256 {
            if r[i] != 0 {
                for b in 1..min(7, 256 - i) {
                    if r[i + b] != 0 {
                        if r[i] + (r[i + b] << b) <= 15 {
                            r[i] += r[i + b] << b;
                            r[i + b] = 0;
                        } else if r[i] - (r[i + b] << b) >= -15 {
                            r[i] -= r[i + b] << b;
                            for k in i + b..256 {
                                if r[k] == 0 {
                                    r[k] = 1;
                                    break;
                                }
                                r[k] = 0;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        r
    }

    pub fn double_scalarmult_vartime(a_scalar: &[u8], a_point: GeP3, b_scalar: &[u8]) -> GeP2 {
        let aslide = GeP2::slide(a_scalar);
        let bslide = GeP2::slide(b_scalar);

        let mut ai = [GeCached {
            y_plus_x: FE_ZERO,
            y_minus_x: FE_ZERO,
            z: FE_ZERO,
            t2d: FE_ZERO,
        }; 8]; /* A,3A,5A,7A,9A,11A,13A,15A */
        ai[0] = a_point.to_cached();
        let a2 = a_point.dbl().to_p3();
        ai[1] = (a2 + ai[0]).to_p3().to_cached();
        ai[2] = (a2 + ai[1]).to_p3().to_cached();
        ai[3] = (a2 + ai[2]).to_p3().to_cached();
        ai[4] = (a2 + ai[3]).to_p3().to_cached();
        ai[5] = (a2 + ai[4]).to_p3().to_cached();
        ai[6] = (a2 + ai[5]).to_p3().to_cached();
        ai[7] = (a2 + ai[6]).to_p3().to_cached();

        let mut r = GeP2::zero();

        let mut i: usize = 255;
        loop {
            if aslide[i] != 0 || bslide[i] != 0 {
                break;
            }
            if i == 0 {
                return r;
            }
            i -= 1;
        }

        loop {
            let mut t = r.dbl();
            if aslide[i] > 0 {
                t = t.to_p3() + ai[(aslide[i] / 2) as usize];
            } else if aslide[i] < 0 {
                t = t.to_p3() - ai[(-aslide[i] / 2) as usize];
            }

            if bslide[i] > 0 {
                t = t.to_p3() + BI[(bslide[i] / 2) as usize];
            } else if bslide[i] < 0 {
                t = t.to_p3() - BI[(-bslide[i] / 2) as usize];
            }

            r = t.to_p2();

            if i == 0 {
                return r;
            }
            i -= 1;
        }
    }
}

impl GeP3 {
    pub fn from_bytes_negate_vartime(s: &[u8]) -> Option<GeP3> {
        let y = Fe::from_bytes(s);
        let z = FE_ONE;
        let y_squared = y.square();
        let u = y_squared - FE_ONE;
        let v = (y_squared * FE_D) + FE_ONE;
        let v_raise_3 = v.square() * v;
        let v_raise_7 = v_raise_3.square() * v;
        let uv7 = v_raise_7 * u; // Is this commutative? u comes second in the code, but not in the notation...

        let mut x = uv7.pow25523() * v_raise_3 * u;

        let vxx = x.square() * v;
        let check = vxx - u;
        if check.is_nonzero() {
            let check2 = vxx + u;
            if check2.is_nonzero() {
                return None;
            }
            x = x * FE_SQRTM1;
        }

        if x.is_negative() == ((s[31] >> 7) != 0) {
            x = x.neg();
        }

        let t = x * y;

        Some(GeP3 { x, y, z, t })
    }

    fn to_p2(&self) -> GeP2 {
        GeP2 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    fn to_cached(&self) -> GeCached {
        GeCached {
            y_plus_x: self.y + self.x,
            y_minus_x: self.y - self.x,
            z: self.z,
            t2d: self.t * FE_D2,
        }
    }

    fn dbl(&self) -> GeP1P1 {
        self.to_p2().dbl()
    }
}

impl Add<GeCached> for GeP3 {
    type Output = GeP1P1;

    fn add(self, _rhs: GeCached) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_plus_x;
        let b = y1_minus_x1 * _rhs.y_minus_x;
        let c = _rhs.t2d * self.t;
        let zz = self.z * _rhs.z;
        let d = zz + zz;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d + c;
        let t3 = d - c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Add<GePrecomp> for GeP3 {
    type Output = GeP1P1;

    fn add(self, _rhs: GePrecomp) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_plus_x;
        let b = y1_minus_x1 * _rhs.y_minus_x;
        let c = _rhs.xy2d * self.t;
        let d = self.z + self.z;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d + c;
        let t3 = d - c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Sub<GeCached> for GeP3 {
    type Output = GeP1P1;

    fn sub(self, _rhs: GeCached) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_minus_x;
        let b = y1_minus_x1 * _rhs.y_plus_x;
        let c = _rhs.t2d * self.t;
        let zz = self.z * _rhs.z;
        let d = zz + zz;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d - c;
        let t3 = d + c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

impl Sub<GePrecomp> for GeP3 {
    type Output = GeP1P1;

    fn sub(self, _rhs: GePrecomp) -> GeP1P1 {
        let y1_plus_x1 = self.y + self.x;
        let y1_minus_x1 = self.y - self.x;
        let a = y1_plus_x1 * _rhs.y_minus_x;
        let b = y1_minus_x1 * _rhs.y_plus_x;
        let c = _rhs.xy2d * self.t;
        let d = self.z + self.z;
        let x3 = a - b;
        let y3 = a + b;
        let z3 = d - c;
        let t3 = d + c;

        GeP1P1 {
            x: x3,
            y: y3,
            z: z3,
            t: t3,
        }
    }
}

pub fn sc_reduce(s: &mut [u8]) {
    let mut s0: i64 = 2_097_151 & load_3i(s);
    let mut s1: i64 = 2_097_151 & (load_4i(&s[2..6]) >> 5);
    let mut s2: i64 = 2_097_151 & (load_3i(&s[5..8]) >> 2);
    let mut s3: i64 = 2_097_151 & (load_4i(&s[7..11]) >> 7);
    let mut s4: i64 = 2_097_151 & (load_4i(&s[10..14]) >> 4);
    let mut s5: i64 = 2_097_151 & (load_3i(&s[13..16]) >> 1);
    let mut s6: i64 = 2_097_151 & (load_4i(&s[15..19]) >> 6);
    let mut s7: i64 = 2_097_151 & (load_3i(&s[18..21]) >> 3);
    let mut s8: i64 = 2_097_151 & load_3i(&s[21..24]);
    let mut s9: i64 = 2_097_151 & (load_4i(&s[23..27]) >> 5);
    let mut s10: i64 = 2_097_151 & (load_3i(&s[26..29]) >> 2);
    let mut s11: i64 = 2_097_151 & (load_4i(&s[28..32]) >> 7);
    let mut s12: i64 = 2_097_151 & (load_4i(&s[31..35]) >> 4);
    let mut s13: i64 = 2_097_151 & (load_3i(&s[34..37]) >> 1);
    let mut s14: i64 = 2_097_151 & (load_4i(&s[36..40]) >> 6);
    let mut s15: i64 = 2_097_151 & (load_3i(&s[39..42]) >> 3);
    let mut s16: i64 = 2_097_151 & load_3i(&s[42..45]);
    let mut s17: i64 = 2_097_151 & (load_4i(&s[44..48]) >> 5);
    let s18: i64 = 2_097_151 & (load_3i(&s[47..50]) >> 2);
    let s19: i64 = 2_097_151 & (load_4i(&s[49..53]) >> 7);
    let s20: i64 = 2_097_151 & (load_4i(&s[52..56]) >> 4);
    let s21: i64 = 2_097_151 & (load_3i(&s[55..58]) >> 1);
    let s22: i64 = 2_097_151 & (load_4i(&s[57..61]) >> 6);
    let s23: i64 = load_4i(&s[60..64]) >> 3;
    let mut carry0: i64;
    let mut carry1: i64;
    let mut carry2: i64;
    let mut carry3: i64;
    let mut carry4: i64;
    let mut carry5: i64;
    let mut carry6: i64;
    let mut carry7: i64;
    let mut carry8: i64;
    let mut carry9: i64;
    let mut carry10: i64;
    let mut carry11: i64;
    let carry12: i64;
    let carry13: i64;
    let carry14: i64;
    let carry15: i64;
    let carry16: i64;

    s11 += s23 * 666_643;
    s12 += s23 * 470_296;
    s13 += s23 * 654_183;
    s14 -= s23 * 997_805;
    s15 += s23 * 136_657;
    s16 -= s23 * 683_901;

    s10 += s22 * 666_643;
    s11 += s22 * 470_296;
    s12 += s22 * 654_183;
    s13 -= s22 * 997_805;
    s14 += s22 * 136_657;
    s15 -= s22 * 683_901;

    s9 += s21 * 666_643;
    s10 += s21 * 470_296;
    s11 += s21 * 654_183;
    s12 -= s21 * 997_805;
    s13 += s21 * 136_657;
    s14 -= s21 * 683_901;

    s8 += s20 * 666_643;
    s9 += s20 * 470_296;
    s10 += s20 * 654_183;
    s11 -= s20 * 997_805;
    s12 += s20 * 136_657;
    s13 -= s20 * 683_901;

    s7 += s19 * 666_643;
    s8 += s19 * 470_296;
    s9 += s19 * 654_183;
    s10 -= s19 * 997_805;
    s11 += s19 * 136_657;
    s12 -= s19 * 683_901;

    s6 += s18 * 666_643;
    s7 += s18 * 470_296;
    s8 += s18 * 654_183;
    s9 -= s18 * 997_805;
    s10 += s18 * 136_657;
    s11 -= s18 * 683_901;

    carry6 = (s6 + (1 << 20)) >> 21;
    s7 += carry6;
    s6 -= carry6 << 21;
    carry8 = (s8 + (1 << 20)) >> 21;
    s9 += carry8;
    s8 -= carry8 << 21;
    carry10 = (s10 + (1 << 20)) >> 21;
    s11 += carry10;
    s10 -= carry10 << 21;
    carry12 = (s12 + (1 << 20)) >> 21;
    s13 += carry12;
    s12 -= carry12 << 21;
    carry14 = (s14 + (1 << 20)) >> 21;
    s15 += carry14;
    s14 -= carry14 << 21;
    carry16 = (s16 + (1 << 20)) >> 21;
    s17 += carry16;
    s16 -= carry16 << 21;

    carry7 = (s7 + (1 << 20)) >> 21;
    s8 += carry7;
    s7 -= carry7 << 21;
    carry9 = (s9 + (1 << 20)) >> 21;
    s10 += carry9;
    s9 -= carry9 << 21;
    carry11 = (s11 + (1 << 20)) >> 21;
    s12 += carry11;
    s11 -= carry11 << 21;
    carry13 = (s13 + (1 << 20)) >> 21;
    s14 += carry13;
    s13 -= carry13 << 21;
    carry15 = (s15 + (1 << 20)) >> 21;
    s16 += carry15;
    s15 -= carry15 << 21;

    s5 += s17 * 666_643;
    s6 += s17 * 470_296;
    s7 += s17 * 654_183;
    s8 -= s17 * 997_805;
    s9 += s17 * 136_657;
    s10 -= s17 * 683_901;

    s4 += s16 * 666_643;
    s5 += s16 * 470_296;
    s6 += s16 * 654_183;
    s7 -= s16 * 997_805;
    s8 += s16 * 136_657;
    s9 -= s16 * 683_901;

    s3 += s15 * 666_643;
    s4 += s15 * 470_296;
    s5 += s15 * 654_183;
    s6 -= s15 * 997_805;
    s7 += s15 * 136_657;
    s8 -= s15 * 683_901;

    s2 += s14 * 666_643;
    s3 += s14 * 470_296;
    s4 += s14 * 654_183;
    s5 -= s14 * 997_805;
    s6 += s14 * 136_657;
    s7 -= s14 * 683_901;

    s1 += s13 * 666_643;
    s2 += s13 * 470_296;
    s3 += s13 * 654_183;
    s4 -= s13 * 997_805;
    s5 += s13 * 136_657;
    s6 -= s13 * 683_901;

    s0 += s12 * 666_643;
    s1 += s12 * 470_296;
    s2 += s12 * 654_183;
    s3 -= s12 * 997_805;
    s4 += s12 * 136_657;
    s5 -= s12 * 683_901;
    s12 = 0;

    carry0 = (s0 + (1 << 20)) >> 21;
    s1 += carry0;
    s0 -= carry0 << 21;
    carry2 = (s2 + (1 << 20)) >> 21;
    s3 += carry2;
    s2 -= carry2 << 21;
    carry4 = (s4 + (1 << 20)) >> 21;
    s5 += carry4;
    s4 -= carry4 << 21;
    carry6 = (s6 + (1 << 20)) >> 21;
    s7 += carry6;
    s6 -= carry6 << 21;
    carry8 = (s8 + (1 << 20)) >> 21;
    s9 += carry8;
    s8 -= carry8 << 21;
    carry10 = (s10 + (1 << 20)) >> 21;
    s11 += carry10;
    s10 -= carry10 << 21;

    carry1 = (s1 + (1 << 20)) >> 21;
    s2 += carry1;
    s1 -= carry1 << 21;
    carry3 = (s3 + (1 << 20)) >> 21;
    s4 += carry3;
    s3 -= carry3 << 21;
    carry5 = (s5 + (1 << 20)) >> 21;
    s6 += carry5;
    s5 -= carry5 << 21;
    carry7 = (s7 + (1 << 20)) >> 21;
    s8 += carry7;
    s7 -= carry7 << 21;
    carry9 = (s9 + (1 << 20)) >> 21;
    s10 += carry9;
    s9 -= carry9 << 21;
    carry11 = (s11 + (1 << 20)) >> 21;
    s12 += carry11;
    s11 -= carry11 << 21;

    s0 += s12 * 666_643;
    s1 += s12 * 470_296;
    s2 += s12 * 654_183;
    s3 -= s12 * 997_805;
    s4 += s12 * 136_657;
    s5 -= s12 * 683_901;
    s12 = 0;

    carry0 = s0 >> 21;
    s1 += carry0;
    s0 -= carry0 << 21;
    carry1 = s1 >> 21;
    s2 += carry1;
    s1 -= carry1 << 21;
    carry2 = s2 >> 21;
    s3 += carry2;
    s2 -= carry2 << 21;
    carry3 = s3 >> 21;
    s4 += carry3;
    s3 -= carry3 << 21;
    carry4 = s4 >> 21;
    s5 += carry4;
    s4 -= carry4 << 21;
    carry5 = s5 >> 21;
    s6 += carry5;
    s5 -= carry5 << 21;
    carry6 = s6 >> 21;
    s7 += carry6;
    s6 -= carry6 << 21;
    carry7 = s7 >> 21;
    s8 += carry7;
    s7 -= carry7 << 21;
    carry8 = s8 >> 21;
    s9 += carry8;
    s8 -= carry8 << 21;
    carry9 = s9 >> 21;
    s10 += carry9;
    s9 -= carry9 << 21;
    carry10 = s10 >> 21;
    s11 += carry10;
    s10 -= carry10 << 21;
    carry11 = s11 >> 21;
    s12 += carry11;
    s11 -= carry11 << 21;

    s0 += s12 * 666_643;
    s1 += s12 * 470_296;
    s2 += s12 * 654_183;
    s3 -= s12 * 997_805;
    s4 += s12 * 136_657;
    s5 -= s12 * 683_901;

    carry0 = s0 >> 21;
    s1 += carry0;
    s0 -= carry0 << 21;
    carry1 = s1 >> 21;
    s2 += carry1;
    s1 -= carry1 << 21;
    carry2 = s2 >> 21;
    s3 += carry2;
    s2 -= carry2 << 21;
    carry3 = s3 >> 21;
    s4 += carry3;
    s3 -= carry3 << 21;
    carry4 = s4 >> 21;
    s5 += carry4;
    s4 -= carry4 << 21;
    carry5 = s5 >> 21;
    s6 += carry5;
    s5 -= carry5 << 21;
    carry6 = s6 >> 21;
    s7 += carry6;
    s6 -= carry6 << 21;
    carry7 = s7 >> 21;
    s8 += carry7;
    s7 -= carry7 << 21;
    carry8 = s8 >> 21;
    s9 += carry8;
    s8 -= carry8 << 21;
    carry9 = s9 >> 21;
    s10 += carry9;
    s9 -= carry9 << 21;
    carry10 = s10 >> 21;
    s11 += carry10;
    s10 -= carry10 << 21;

    s[0] = (s0 >> 0) as u8;
    s[1] = (s0 >> 8) as u8;
    s[2] = ((s0 >> 16) | (s1 << 5)) as u8;
    s[3] = (s1 >> 3) as u8;
    s[4] = (s1 >> 11) as u8;
    s[5] = ((s1 >> 19) | (s2 << 2)) as u8;
    s[6] = (s2 >> 6) as u8;
    s[7] = ((s2 >> 14) | (s3 << 7)) as u8;
    s[8] = (s3 >> 1) as u8;
    s[9] = (s3 >> 9) as u8;
    s[10] = ((s3 >> 17) | (s4 << 4)) as u8;
    s[11] = (s4 >> 4) as u8;
    s[12] = (s4 >> 12) as u8;
    s[13] = ((s4 >> 20) | (s5 << 1)) as u8;
    s[14] = (s5 >> 7) as u8;
    s[15] = ((s5 >> 15) | (s6 << 6)) as u8;
    s[16] = (s6 >> 2) as u8;
    s[17] = (s6 >> 10) as u8;
    s[18] = ((s6 >> 18) | (s7 << 3)) as u8;
    s[19] = (s7 >> 5) as u8;
    s[20] = (s7 >> 13) as u8;
    s[21] = (s8 >> 0) as u8;
    s[22] = (s8 >> 8) as u8;
    s[23] = ((s8 >> 16) | (s9 << 5)) as u8;
    s[24] = (s9 >> 3) as u8;
    s[25] = (s9 >> 11) as u8;
    s[26] = ((s9 >> 19) | (s10 << 2)) as u8;
    s[27] = (s10 >> 6) as u8;
    s[28] = ((s10 >> 14) | (s11 << 7)) as u8;
    s[29] = (s11 >> 1) as u8;
    s[30] = (s11 >> 9) as u8;
    s[31] = (s11 >> 17) as u8;
}

static BI: [GePrecomp; 8] = [
    GePrecomp {
        y_plus_x: Fe([
            25_967_493,
            -14_356_035,
            29_566_456,
            3_660_896,
            -12_694_345,
            4_014_787,
            27_544_626,
            -11_754_271,
            -6_079_156,
            2_047_605,
        ]),
        y_minus_x: Fe([
            -12_545_711,
            934_262,
            -2_722_910,
            3_049_990,
            -727_428,
            9_406_986,
            12_720_692,
            5_043_384,
            19_500_929,
            -15_469_378,
        ]),
        xy2d: Fe([
            -8_738_181,
            4_489_570,
            9_688_441,
            -14_785_194,
            10_184_609,
            -12_363_380,
            29_287_919,
            11_864_899,
            -24_514_362,
            -4_438_546,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            15_636_291,
            -9_688_557,
            24_204_773,
            -7_912_398,
            616_977,
            -16_685_262,
            27_787_600,
            -14_772_189,
            28_944_400,
            -1_550_024,
        ]),
        y_minus_x: Fe([
            16_568_933,
            4_717_097,
            -11_556_148,
            -1_102_322,
            15_682_896,
            -11_807_043,
            16_354_577,
            -11_775_962,
            7_689_662,
            11_199_574,
        ]),
        xy2d: Fe([
            30_464_156,
            -5_976_125,
            -11_779_434,
            -15_670_865,
            23_220_365,
            15_915_852,
            7_512_774,
            10_017_326,
            -17_749_093,
            -9_920_357,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            10_861_363,
            11_473_154,
            27_284_546,
            1_981_175,
            -30_064_349,
            12_577_861,
            32_867_885,
            14_515_107,
            -15_438_304,
            10_819_380,
        ]),
        y_minus_x: Fe([
            4_708_026,
            6_336_745,
            20_377_586,
            9_066_809,
            -11_272_109,
            6_594_696,
            -25_653_668,
            12_483_688,
            -12_668_491,
            5_581_306,
        ]),
        xy2d: Fe([
            19_563_160,
            16_186_464,
            -29_386_857,
            4_097_519,
            10_237_984,
            -4_348_115,
            28_542_350,
            13_850_243,
            -23_678_021,
            -15_815_942,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            5_153_746,
            9_909_285,
            1_723_747,
            -2_777_874,
            30_523_605,
            5_516_873,
            19_480_852,
            5_230_134,
            -23_952_439,
            -15_175_766,
        ]),
        y_minus_x: Fe([
            -30_269_007,
            -3_463_509,
            7_665_486,
            10_083_793,
            28_475_525,
            1_649_722,
            20_654_025,
            16_520_125,
            30_598_449,
            7_715_701,
        ]),
        xy2d: Fe([
            28_881_845,
            14_381_568,
            9_657_904,
            3_680_757,
            -20_181_635,
            7_843_316,
            -31_400_660,
            1_370_708,
            29_794_553,
            -1_409_300,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            -22_518_993,
            -6_692_182,
            14_201_702,
            -8_745_502,
            -23_510_406,
            8_844_726,
            18_474_211,
            -1_361_450,
            -13_062_696,
            13_821_877,
        ]),
        y_minus_x: Fe([
            -6_455_177,
            -7_839_871,
            3_374_702,
            -4_740_862,
            -27_098_617,
            -10_571_707,
            31_655_028,
            -7_212_327,
            18_853_322,
            -14_220_951,
        ]),
        xy2d: Fe([
            4_566_830,
            -12_963_868,
            -28_974_889,
            -12_240_689,
            -7_602_672,
            -2_830_569,
            -8_514_358,
            -10_431_137,
            2_207_753,
            -3_209_784,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            -25_154_831,
            -4_185_821,
            29_681_144,
            7_868_801,
            -6_854_661,
            -9_423_865,
            -12_437_364,
            -663_000,
            -31_111_463,
            -16_132_436,
        ]),
        y_minus_x: Fe([
            25_576_264,
            -2_703_214,
            7_349_804,
            -11_814_844,
            16_472_782,
            9_300_885,
            3_844_789,
            15_725_684,
            171_356,
            6_466_918,
        ]),
        xy2d: Fe([
            23_103_977,
            13_316_479,
            9_739_013,
            -16_149_481,
            817_875,
            -15_038_942,
            8_965_339,
            -14_088_058,
            -30_714_912,
            16_193_877,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            -33_521_811,
            3_180_713,
            -2_394_130,
            14_003_687,
            -16_903_474,
            -16_270_840,
            17_238_398,
            4_729_455,
            -18_074_513,
            9_256_800,
        ]),
        y_minus_x: Fe([
            -25_182_317,
            -4_174_131,
            32_336_398,
            5_036_987,
            -21_236_817,
            11_360_617,
            22_616_405,
            9_761_698,
            -19_827_198,
            630_305,
        ]),
        xy2d: Fe([
            -13_720_693,
            2_639_453,
            -24_237_460,
            -7_406_481,
            9_494_427,
            -5_774_029,
            -6_554_551,
            -15_960_994,
            -2_449_256,
            -14_291_300,
        ]),
    },
    GePrecomp {
        y_plus_x: Fe([
            -3_151_181,
            -5_046_075,
            9_282_714,
            6_866_145,
            -31_907_062,
            -863_023,
            -18_940_575,
            15_033_784,
            25_105_118,
            -7_894_876,
        ]),
        y_minus_x: Fe([
            -24_326_370,
            15_950_226,
            -31_801_215,
            -14_592_823,
            -11_662_737,
            -5_090_925,
            1_573_892,
            -2_625_887,
            2_198_790,
            -15_804_619,
        ]),
        xy2d: Fe([
            -3_099_351,
            10_324_967,
            -2_241_613,
            7_453_183,
            -5_446_979,
            -2_735_503,
            -13_812_022,
            -16_236_442,
            -32_461_234,
            -12_290_683,
        ]),
    },
];
