//! 0x8c1556d0 — count_nonzero_bytes.
//!
//! Counts non-zero bytes in the buffer at `[r4+4]` for `r0` in `[0, [r4+12])`.
//! `r0` ends as the count: the `rts` delay slot is `mov r3,r0`, so the loop
//! index is overwritten by the return value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r5 = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    let base = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let mut r3: u32 = 0;
    let mut r0: u32 = 0; // delay slot of `bra`: mov r3,r0
    cpu.r[5] = r5;
    loop {
        // cmp/gt r0,r5 is signed: r5 > r0. It runs before the body (and first,
        // via the `bra`), so T is always rewritten.
        let more = (r5 as i32) > (r0 as i32);
        cpu.set_t(more);
        if !more {
            break;
        }
        let byte = mem.read_u8(base.wrapping_add(r0)).unwrap_or(0);
        cpu.r[1] = byte as u32; // mov.b then extu.b: the loaded byte is unsigned
        if byte != 0 {
            r3 = r3.wrapping_add(1);
        }
        r0 = r0.wrapping_add(1);
    }
    cpu.r[3] = r3;
    cpu.r[0] = cpu.r[3]; // delay slot of `rts`: mov r3,r0
}
