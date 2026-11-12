//! 0x8c0f1b78 — store a byte at +12 and zero +13 when `(r5-1)&0xff` is 0 or 1.
//!
//! `r1 = extu.b(r5-1)`, `r0 = 1`, `cmp/hs r1,r0` → T = (1 >= r1). Not taken
//! (r1 >= 2) returns with r0 = 1. Taken: `r0 = r5`, the `bra` delay slot stores
//! its low byte at `+12`, then `r0 = 0` and `[r4+13] = 0`; T stays from the
//! `cmp/hs`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5].wrapping_sub(1); // mov r5,r0 ; add #-1,r0
    cpu.r[1] = cpu.r[0] & 0xff; // extu.b r0,r1
    cpu.r[0] = 1; // mov #1,r0
    cpu.set_t(cpu.r[0] >= cpu.r[1]); // cmp/hs r1,r0
    if cpu.r[0] < cpu.r[1] {
        return; // bf: r0 keeps 1
    }
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(12), cpu.r[0] as u8); // bra delay slot
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(13), cpu.r[0] as u8); // mov.b r0,@(13,r4)
}
