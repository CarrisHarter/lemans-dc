//! 0x8c17678e - null-guarded field store.
//!
//! `r4 == 0` or `r5 == 0` emits through 0x8c174564 with code 101 and returns 0.
//! Otherwise `[r4 + 0x3c0] = r5` and r0 = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut emit = cpu.t(); // bt 0x8c176798
    if !emit {
        cpu.set_t(cpu.r[5] == 0); // tst r5,r5
        emit = cpu.t(); // bf 0x8c1767a6 is the store path, so emit only when T
    }
    if emit {
        cpu.r[3] = mem.read_u32(0x8c17689c).unwrap_or(0x8c174564); // mov.l 0x8c17689c,r3
        cpu.r[5] = mem.read_u32(0x8c176898).unwrap_or(0x8c1e1be4); // mov.l 0x8c176898,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c1767a0;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c1767a4
    } else {
        cpu.r[0] = mem.read_u16(0x8c176886).unwrap_or(0x3c0) as i16 as i32 as u32; // mov.w (sign-extends)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // mov.l r5,@(r0,r4)
        cpu.r[0] = 1; // mov #1,r0
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
