//! 0x8c056b18 — copy 56-byte records from [r4..r5) to r6; return r6.
//!
//! Per record, when the current r6 != 0, 14 dwords are copied in offset
//! order (0,4,...,52): word 0 and word 13 by `mov.l`, words 1..=12 through
//! `fmov` into fr0, so fr0 ends as the bits of src+48. Leftovers the C does
//! not show: r1 = dst+48, r3 = src+44, r7 = dst+44 after a copied record
//! (untouched when the copy is skipped), r4 = r5 and r6 advanced by 56 per
//! record (r6 even when its copy was skipped), r0 = final r6 (return value),
//! r13/r14 restored by the epilogue pops, r15 net unchanged, T = 1 (the
//! loop only exits through `cmp/eq` equal).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    const STRIDE: u32 = 56;
    let end = cpu.r[5];
    cpu.set_t(cpu.r[4] == end); // 0x8c056b1c cmp/eq r5,r4
    if cpu.r[4] != end {
        loop {
            let src = cpu.r[4];
            let dst = cpu.r[6];
            cpu.set_t(dst == 0); // 0x8c056b20 tst r6,r6
            if dst != 0 {
                for i in 0..14u32 {
                    let v = mem.read_u32(src.wrapping_add(i * 4)).unwrap_or(0);
                    if (1..=12).contains(&i) {
                        cpu.fr[0] = f32::from_bits(v); // fmov @rX,fr0
                    }
                    let _ = mem.write_u32(dst.wrapping_add(i * 4), v);
                }
                cpu.r[1] = dst.wrapping_add(48); // 0x8c056ba0/bae
                cpu.r[3] = src.wrapping_add(44); // 0x8c056b9a/ba6
                cpu.r[7] = dst.wrapping_add(44); // 0x8c056b9c/baa
            }
            cpu.r[4] = cpu.r[4].wrapping_add(STRIDE); // 0x8c056bbc
            cpu.set_t(cpu.r[4] == end); // 0x8c056bbe cmp/eq r5,r4
            cpu.r[6] = cpu.r[6].wrapping_add(STRIDE); // bf.s delay slot 0x8c056bc2
            if cpu.r[4] == end {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // 0x8c056bc6 mov r6,r0 (return value)
}
