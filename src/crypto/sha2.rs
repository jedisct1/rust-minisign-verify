// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::cryptoutil::{
    add_bytes_to_bits_tuple, read_u64v_be, write_u64_be, FixedBuffer, FixedBuffer128,
    StandardPadding,
};
use super::digest::Digest;
use super::simd::u64x2;

const STATE_LEN: usize = 8;
const BLOCK_LEN: usize = 16;

/// Not an intrinsic, but works like an unaligned load.
#[inline]
fn sha512load(v0: u64x2, v1: u64x2) -> u64x2 {
    u64x2(v1.1, v0.0)
}

/// Performs 2 rounds of the SHA-512 message schedule update.
pub fn sha512_schedule_x2(v0: u64x2, v1: u64x2, v4to5: u64x2, v7: u64x2) -> u64x2 {
    // sigma 0
    fn sigma0(x: u64) -> u64 {
        ((x << 63) | (x >> 1)) ^ ((x << 56) | (x >> 8)) ^ (x >> 7)
    }

    // sigma 1
    fn sigma1(x: u64) -> u64 {
        ((x << 45) | (x >> 19)) ^ ((x << 3) | (x >> 61)) ^ (x >> 6)
    }

    let u64x2(w1, w0) = v0;
    let u64x2(_, w2) = v1;
    let u64x2(w10, w9) = v4to5;
    let u64x2(w15, w14) = v7;

    let w16 = sigma1(w14)
        .wrapping_add(w9)
        .wrapping_add(sigma0(w1))
        .wrapping_add(w0);
    let w17 = sigma1(w15)
        .wrapping_add(w10)
        .wrapping_add(sigma0(w2))
        .wrapping_add(w1);

    u64x2(w17, w16)
}

/// Performs one round of the SHA-512 message block digest.
pub fn sha512_digest_round(ae: u64x2, bf: u64x2, cg: u64x2, dh: u64x2, wk0: u64) -> u64x2 {
    macro_rules! big_sigma0 {
        ($a:expr) => {
            ($a.rotate_right(28) ^ $a.rotate_right(34) ^ $a.rotate_right(39))
        };
    }
    macro_rules! big_sigma1 {
        ($a:expr) => {
            ($a.rotate_right(14) ^ $a.rotate_right(18) ^ $a.rotate_right(41))
        };
    }
    macro_rules! bool3ary_202 {
        ($a:expr, $b:expr, $c:expr) => {
            $c ^ ($a & ($b ^ $c))
        };
    } // Choose, MD5F, SHA1C
    macro_rules! bool3ary_232 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) ^ ($a & $c) ^ ($b & $c)
        };
    } // Majority, SHA1M

    let u64x2(a0, e0) = ae;
    let u64x2(b0, f0) = bf;
    let u64x2(c0, g0) = cg;
    let u64x2(d0, h0) = dh;

    // a round
    let x0 = big_sigma1!(e0)
        .wrapping_add(bool3ary_202!(e0, f0, g0))
        .wrapping_add(wk0)
        .wrapping_add(h0);
    let y0 = big_sigma0!(a0).wrapping_add(bool3ary_232!(a0, b0, c0));
    let (a1, _, _, _, e1, _, _, _) = (
        x0.wrapping_add(y0),
        a0,
        b0,
        c0,
        x0.wrapping_add(d0),
        e0,
        f0,
        g0,
    );

    u64x2(a1, e1)
}

/// Process a block with the SHA-512 algorithm.
pub fn sha512_digest_block_u64(state: &mut [u64; 8], block: &[u64; 16]) {
    let k = &K64X2;

    macro_rules! schedule {
        ($v0:expr, $v1:expr, $v4:expr, $v5:expr, $v7:expr) => {
            sha512_schedule_x2($v0, $v1, sha512load($v4, $v5), $v7)
        };
    }

    macro_rules! rounds4 {
        ($ae:ident, $bf:ident, $cg:ident, $dh:ident, $wk0:expr, $wk1:expr) => {{
            let u64x2(u, t) = $wk0;
            let u64x2(w, v) = $wk1;

            $dh = sha512_digest_round($ae, $bf, $cg, $dh, t);
            $cg = sha512_digest_round($dh, $ae, $bf, $cg, u);
            $bf = sha512_digest_round($cg, $dh, $ae, $bf, v);
            $ae = sha512_digest_round($bf, $cg, $dh, $ae, w);
        }};
    }

    let mut ae = u64x2(state[0], state[4]);
    let mut bf = u64x2(state[1], state[5]);
    let mut cg = u64x2(state[2], state[6]);
    let mut dh = u64x2(state[3], state[7]);

    // Rounds 0..20
    let (mut w1, mut w0) = (u64x2(block[3], block[2]), u64x2(block[1], block[0]));
    rounds4!(ae, bf, cg, dh, k[0] + w0, k[1] + w1);
    let (mut w3, mut w2) = (u64x2(block[7], block[6]), u64x2(block[5], block[4]));
    rounds4!(ae, bf, cg, dh, k[2] + w2, k[3] + w3);
    let (mut w5, mut w4) = (u64x2(block[11], block[10]), u64x2(block[9], block[8]));
    rounds4!(ae, bf, cg, dh, k[4] + w4, k[5] + w5);
    let (mut w7, mut w6) = (u64x2(block[15], block[14]), u64x2(block[13], block[12]));
    rounds4!(ae, bf, cg, dh, k[6] + w6, k[7] + w7);
    let mut w8 = schedule!(w0, w1, w4, w5, w7);
    let mut w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, k[8] + w8, k[9] + w9);

    // Rounds 20..40
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, k[10] + w0, k[11] + w1);
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, k[12] + w2, k[13] + w3);
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, k[14] + w4, k[15] + w5);
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, k[16] + w6, k[17] + w7);
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, k[18] + w8, k[19] + w9);

    // Rounds 40..60
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, k[20] + w0, k[21] + w1);
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, k[22] + w2, k[23] + w3);
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, k[24] + w4, k[25] + w5);
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, k[26] + w6, k[27] + w7);
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, k[28] + w8, k[29] + w9);

    // Rounds 60..80
    w0 = schedule!(w2, w3, w6, w7, w9);
    w1 = schedule!(w3, w4, w7, w8, w0);
    rounds4!(ae, bf, cg, dh, k[30] + w0, k[31] + w1);
    w2 = schedule!(w4, w5, w8, w9, w1);
    w3 = schedule!(w5, w6, w9, w0, w2);
    rounds4!(ae, bf, cg, dh, k[32] + w2, k[33] + w3);
    w4 = schedule!(w6, w7, w0, w1, w3);
    w5 = schedule!(w7, w8, w1, w2, w4);
    rounds4!(ae, bf, cg, dh, k[34] + w4, k[35] + w5);
    w6 = schedule!(w8, w9, w2, w3, w5);
    w7 = schedule!(w9, w0, w3, w4, w6);
    rounds4!(ae, bf, cg, dh, k[36] + w6, k[37] + w7);
    w8 = schedule!(w0, w1, w4, w5, w7);
    w9 = schedule!(w1, w2, w5, w6, w8);
    rounds4!(ae, bf, cg, dh, k[38] + w8, k[39] + w9);

    let u64x2(a, e) = ae;
    let u64x2(b, f) = bf;
    let u64x2(c, g) = cg;
    let u64x2(d, h) = dh;

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

pub fn sha512_digest_block(state: &mut [u64; 8], block: &[u8]) {
    assert_eq!(block.len(), BLOCK_LEN * 8);
    let mut block2 = [0u64; BLOCK_LEN];
    read_u64v_be(&mut block2[..], block);
    sha512_digest_block_u64(state, &block2);
}

#[derive(Copy, Clone)]
struct Engine512State {
    h: [u64; 8],
}

impl Engine512State {
    fn new(h: &[u64; 8]) -> Engine512State {
        Engine512State { h: *h }
    }

    fn reset(&mut self, h: &[u64; STATE_LEN]) {
        self.h = *h;
    }

    pub fn process_block(&mut self, data: &[u8]) {
        sha512_digest_block(&mut self.h, data);
    }
}

pub const K64: [u64; 80] = [
    0x428a_2f98_d728_ae22,
    0x7137_4491_23ef_65cd,
    0xb5c0_fbcf_ec4d_3b2f,
    0xe9b5_dba5_8189_dbbc,
    0x3956_c25b_f348_b538,
    0x59f1_11f1_b605_d019,
    0x923f_82a4_af19_4f9b,
    0xab1c_5ed5_da6d_8118,
    0xd807_aa98_a303_0242,
    0x1283_5b01_4570_6fbe,
    0x2431_85be_4ee4_b28c,
    0x550c_7dc3_d5ff_b4e2,
    0x72be_5d74_f27b_896f,
    0x80de_b1fe_3b16_96b1,
    0x9bdc_06a7_25c7_1235,
    0xc19b_f174_cf69_2694,
    0xe49b_69c1_9ef1_4ad2,
    0xefbe_4786_384f_25e3,
    0x0fc1_9dc6_8b8c_d5b5,
    0x240c_a1cc_77ac_9c65,
    0x2de9_2c6f_592b_0275,
    0x4a74_84aa_6ea6_e483,
    0x5cb0_a9dc_bd41_fbd4,
    0x76f9_88da_8311_53b5,
    0x983e_5152_ee66_dfab,
    0xa831_c66d_2db4_3210,
    0xb003_27c8_98fb_213f,
    0xbf59_7fc7_beef_0ee4,
    0xc6e0_0bf3_3da8_8fc2,
    0xd5a7_9147_930a_a725,
    0x06ca_6351_e003_826f,
    0x1429_2967_0a0e_6e70,
    0x27b7_0a85_46d2_2ffc,
    0x2e1b_2138_5c26_c926,
    0x4d2c_6dfc_5ac4_2aed,
    0x5338_0d13_9d95_b3df,
    0x650a_7354_8baf_63de,
    0x766a_0abb_3c77_b2a8,
    0x81c2_c92e_47ed_aee6,
    0x9272_2c85_1482_353b,
    0xa2bf_e8a1_4cf1_0364,
    0xa81a_664b_bc42_3001,
    0xc24b_8b70_d0f8_9791,
    0xc76c_51a3_0654_be30,
    0xd192_e819_d6ef_5218,
    0xd699_0624_5565_a910,
    0xf40e_3585_5771_202a,
    0x106a_a070_32bb_d1b8,
    0x19a4_c116_b8d2_d0c8,
    0x1e37_6c08_5141_ab53,
    0x2748_774c_df8e_eb99,
    0x34b0_bcb5_e19b_48a8,
    0x391c_0cb3_c5c9_5a63,
    0x4ed8_aa4a_e341_8acb,
    0x5b9c_ca4f_7763_e373,
    0x682e_6ff3_d6b2_b8a3,
    0x748f_82ee_5def_b2fc,
    0x78a5_636f_4317_2f60,
    0x84c8_7814_a1f0_ab72,
    0x8cc7_0208_1a64_39ec,
    0x90be_fffa_2363_1e28,
    0xa450_6ceb_de82_bde9,
    0xbef9_a3f7_b2c6_7915,
    0xc671_78f2_e372_532b,
    0xca27_3ece_ea26_619c,
    0xd186_b8c7_21c0_c207,
    0xeada_7dd6_cde0_eb1e,
    0xf57d_4f7f_ee6e_d178,
    0x06f0_67aa_7217_6fba,
    0x0a63_7dc5_a2c8_98a6,
    0x113f_9804_bef9_0dae,
    0x1b71_0b35_131c_471b,
    0x28db_77f5_2304_7d84,
    0x32ca_ab7b_40c7_2493,
    0x3c9e_be0a_15c9_bebc,
    0x431d_67c4_9c10_0d4c,
    0x4cc5_d4be_cb3e_42b6,
    0x597f_299c_fc65_7e2a,
    0x5fcb_6fab_3ad6_faec,
    0x6c44_198c_4a47_5817,
];

/// Constants necessary for SHA-512 family of digests.
pub const K64X2: [u64x2; 40] = [
    u64x2(K64[1], K64[0]),
    u64x2(K64[3], K64[2]),
    u64x2(K64[5], K64[4]),
    u64x2(K64[7], K64[6]),
    u64x2(K64[9], K64[8]),
    u64x2(K64[11], K64[10]),
    u64x2(K64[13], K64[12]),
    u64x2(K64[15], K64[14]),
    u64x2(K64[17], K64[16]),
    u64x2(K64[19], K64[18]),
    u64x2(K64[21], K64[20]),
    u64x2(K64[23], K64[22]),
    u64x2(K64[25], K64[24]),
    u64x2(K64[27], K64[26]),
    u64x2(K64[29], K64[28]),
    u64x2(K64[31], K64[30]),
    u64x2(K64[33], K64[32]),
    u64x2(K64[35], K64[34]),
    u64x2(K64[37], K64[36]),
    u64x2(K64[39], K64[38]),
    u64x2(K64[41], K64[40]),
    u64x2(K64[43], K64[42]),
    u64x2(K64[45], K64[44]),
    u64x2(K64[47], K64[46]),
    u64x2(K64[49], K64[48]),
    u64x2(K64[51], K64[50]),
    u64x2(K64[53], K64[52]),
    u64x2(K64[55], K64[54]),
    u64x2(K64[57], K64[56]),
    u64x2(K64[59], K64[58]),
    u64x2(K64[61], K64[60]),
    u64x2(K64[63], K64[62]),
    u64x2(K64[65], K64[64]),
    u64x2(K64[67], K64[66]),
    u64x2(K64[69], K64[68]),
    u64x2(K64[71], K64[70]),
    u64x2(K64[73], K64[72]),
    u64x2(K64[75], K64[74]),
    u64x2(K64[77], K64[76]),
    u64x2(K64[79], K64[78]),
];

#[derive(Copy, Clone)]
struct Engine512 {
    length_bits: (u64, u64),
    buffer: FixedBuffer128,
    state: Engine512State,
    finished: bool,
}

impl Engine512 {
    fn new(h: &[u64; STATE_LEN]) -> Engine512 {
        Engine512 {
            length_bits: (0, 0),
            buffer: FixedBuffer128::new(),
            state: Engine512State::new(h),
            finished: false,
        }
    }

    fn reset(&mut self, h: &[u64; STATE_LEN]) {
        self.length_bits = (0, 0);
        self.buffer.reset();
        self.state.reset(h);
        self.finished = false;
    }

    fn input(&mut self, input: &[u8]) {
        assert!(!self.finished);
        self.length_bits = add_bytes_to_bits_tuple(self.length_bits, input.len() as u64);
        let self_state = &mut self.state;
        self.buffer
            .input(input, |input: &[u8]| self_state.process_block(input));
    }

    fn finish(&mut self) {
        if self.finished {
            return;
        }

        let self_state = &mut self.state;
        self.buffer
            .standard_padding(16, |input: &[u8]| self_state.process_block(input));
        match self.length_bits {
            (hi, low) => {
                write_u64_be(self.buffer.next(8), hi);
                write_u64_be(self.buffer.next(8), low);
            }
        }
        self_state.process_block(self.buffer.full_buffer());

        self.finished = true;
    }
}

#[derive(Copy, Clone)]
pub struct Sha512 {
    engine: Engine512,
}

impl Sha512 {
    pub fn new() -> Sha512 {
        Sha512 {
            engine: Engine512::new(&H512),
        }
    }
}

impl Digest for Sha512 {
    fn input(&mut self, d: &[u8]) {
        self.engine.input(d);
    }

    fn result(&mut self, out: &mut [u8]) {
        self.engine.finish();

        write_u64_be(&mut out[0..8], self.engine.state.h[0]);
        write_u64_be(&mut out[8..16], self.engine.state.h[1]);
        write_u64_be(&mut out[16..24], self.engine.state.h[2]);
        write_u64_be(&mut out[24..32], self.engine.state.h[3]);
        write_u64_be(&mut out[32..40], self.engine.state.h[4]);
        write_u64_be(&mut out[40..48], self.engine.state.h[5]);
        write_u64_be(&mut out[48..56], self.engine.state.h[6]);
        write_u64_be(&mut out[56..64], self.engine.state.h[7]);
    }

    fn reset(&mut self) {
        self.engine.reset(&H512);
    }

    fn output_bits(&self) -> usize {
        512
    }

    fn block_size(&self) -> usize {
        128
    }
}

static H512: [u64; STATE_LEN] = [
    0x6a09_e667_f3bc_c908,
    0xbb67_ae85_84ca_a73b,
    0x3c6e_f372_fe94_f82b,
    0xa54f_f53a_5f1d_36f1,
    0x510e_527f_ade6_82d1,
    0x9b05_688c_2b3e_6c1f,
    0x1f83_d9ab_fb41_bd6b,
    0x5be0_cd19_137e_2179,
];
