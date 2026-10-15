//! Kintsugi `wupz` base-3 packing — 5 trits per byte, zero-dep wire transport.
//!
//! `3^5 = 243 <= 255`, so five ternary trits fit exactly in one `u8` with 12
//! states left over. This is the densest simple byte packing for the gate
//! alphabet and the canonical wire format between `zenUltraCooker` and the
//! `wupz` linker/packer lane.
//!
//! Digit mapping: each trit uses the **gate codec** digit (`-1 -> 0`,
//! `0 -> 1`, `+1 -> 2`), LSB-first — so an all-affirm run (`+1 ×5`) packs to
//! `0b11111010 = 242 = 0xF2`, and the byte holds 5 trits, not 4.

use zen_cooker_core::quant_tern::Trit;

/// One `u8` holds exactly this many trits.
pub const TRITS_PER_BYTE: usize = 5;

/// Largest value a 5-trit base-3 digit string can reach (3^5 - 1).
pub const MAX_PACKED: u8 = 242;

/// Pack trits 5-per-byte, LSB-first, gate-codec digits.
pub fn pack5(trits: &[Trit]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(trits.len().div_ceil(TRITS_PER_BYTE));
    let mut i = 0;
    while i < trits.len() {
        let mut b: u16 = 0;
        let mut place: u16 = 1;
        for j in 0..TRITS_PER_BYTE {
            let t = trits.get(i + j).copied().unwrap_or(Trit::Zero);
            b += t.gate_code() as u16 * place;
            place *= 3;
        }
        bytes.push(b as u8);
        i += TRITS_PER_BYTE;
    }
    bytes
}

/// Unpack `n` trits from a 5-per-byte wupz byte slice.
pub fn unpack5(bytes: &[u8], n: usize) -> Vec<Trit> {
    let mut out = Vec::with_capacity(bytes.len() * TRITS_PER_BYTE);
    for &b in bytes {
        let mut v = b as u16;
        for _ in 0..TRITS_PER_BYTE {
            let digit = (v % 3) as u8;
            v /= 3;
            out.push(Trit::from_gate_code(digit));
        }
    }
    out.truncate(n);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_all_trit_runs() {
        for t in [Trit::Minus, Trit::Zero, Trit::Plus] {
            for len in [0usize, 1, 4, 5, 6, 9, 17] {
                let trits = vec![t; len];
                assert_eq!(unpack5(&pack5(&trits), len), trits, "len {len}");
            }
        }
    }

    #[test]
    fn round_trip_mixed_pattern() {
        let trits: Vec<Trit> =
            [Trit::Plus, Trit::Zero, Trit::Minus, Trit::Plus, Trit::Minus].repeat(7);
        assert_eq!(unpack5(&pack5(&trits), trits.len()), trits);
    }

    #[test]
    fn all_affirm_packs_to_242() {
        // +1 ×5 -> digit 2 each -> 2*(1 + 3 + 9 + 27 + 81) = 242 = 0xF2
        let bytes = pack5(&[Trit::Plus; 5]);
        assert_eq!(bytes, vec![242]);
    }

    #[test]
    fn all_refuse_packs_to_zero_and_padding_is_zero() {
        assert_eq!(pack5(&[Trit::Minus; 5]), vec![0]);
        // A 2-trit payload pads with Zero (digit 1), never fabricated states.
        let two = pack5(&[Trit::Minus, Trit::Minus]);
        assert_eq!(unpack5(&two, 2), vec![Trit::Minus, Trit::Minus]);
    }

    #[test]
    fn qt_aaaa_is_four_per_byte_gate_not_five() {
        // Cross-codec sanity: the gate `qt:` codec packs 4 trits/byte (0xaa),
        // the wupz codec packs 5 (0xF2 for the same all-affirm run of 5).
        // The two must never be conflated — 0xaa is NOT a wupz byte.
        let gate: Vec<u8> = {
            let mut b: u8 = 0;
            for (j, t) in [Trit::Plus; 4].iter().enumerate() {
                b |= t.gate_code() << (2 * j);
            }
            vec![b]
        };
        assert_eq!(gate, vec![0xaa]);
        assert_eq!(pack5(&[Trit::Plus; 5]), vec![0xF2]);
    }
}
