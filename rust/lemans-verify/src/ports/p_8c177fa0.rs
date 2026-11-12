//! 0x8c177fa0 - bounds-checked table store.
//!
//! `n = [0x8c2b52d4]`; `r4 >= n` (unsigned `cmp/hs`) emits through 0x8c174564
//! with code 102 and returns 0. Otherwise `[0x8c2b65ec + r4*4] = r5` and r0 = 1.
//! `shll2` leaves T alone, so T on the store path is the `cmp/hs` result (0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c178050).unwrap_or(0x8c2b52d4); // mov.l 0x8c178050,r2
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r3
    cpu.set_t(cpu.r[4] >= cpu.r[3]); // cmp/hs r3,r4
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c178064).unwrap_or(0x8c2b65ec); // mov.l 0x8c178064,r0
        cpu.r[4] = cpu.r[4].wrapping_shl(2); // shll2 r4
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5]); // mov.l r5,@(r0,r4)
        cpu.r[0] = 1; // mov #1,r0
    } else {
        cpu.r[3] = mem.read_u32(0x8c178058).unwrap_or(0x8c174564); // mov.l 0x8c178058,r3
        cpu.r[5] = mem.read_u32(0x8c178060).unwrap_or(0x8c1e2614); // mov.l 0x8c178060,r5
        cpu.r[4] = 102; // jsr delay: mov #102,r4
        cpu.pr = 0x8c177fb2;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c177fb6
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
