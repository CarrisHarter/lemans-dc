//! 0x8c1642dc — copy a 56-byte struct from r5 to r4.
//!
//! If r4 == r5, rts immediately with r0 = r4 (T=1). Otherwise copies the
//! words at r5+0,4,8,12,16,24,28,20,32,36,40,44,48 and (rts delay) r5+52
//! to the same offsets at r4; r0 = r4, r1 = mem[r5+52], T = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    cpu.r[0] = r4; // mov r4,r0
    if r4 == r5 {
        cpu.set_t(true); // cmp/eq r5,r4 ; rts ; delay mov r4,r0
        return;
    }
    cpu.set_t(false);
    let mut r1 = 0u32;
    for off in [0u32, 4, 8, 12, 16, 24, 28, 20, 32, 36, 40, 44, 48, 52] {
        r1 = mem.read_u32(r5.wrapping_add(off)).unwrap_or(0);
        let _ = mem.write_u32(r4.wrapping_add(off), r1);
    }
    cpu.r[1] = r1;
}
