//! 0x8c175a7e - flag gate on `[r5]`, then store `[r4+60]` or emit.
//!
//! Tests `[r5]` (not r5) for the first branch, then the usual r4 gate and `+60` field.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.r[3] == 0 {
        cpu.r[3] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r3
        cpu.r[5] = mem.read_u32(0x8c175af8).unwrap_or(0x8c1e0f4c); // mov.l 0x8c175af8,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c175a90;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay
    } else {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        cpu.r[2] = 0; // mov #0,r2
        let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bf.s delay: mov.l r2,@r5
        if cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r2
            cpu.r[5] = mem.read_u32(0x8c175afc).unwrap_or(0x8c1e0f78); // mov.l 0x8c175afc,r5
            cpu.r[4] = 101; // jsr delay: mov #101,r4
            cpu.pr = 0x8c175aa2;
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0;
        } else {
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(60)).unwrap_or(0); // mov.l @(60,r4),r3
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // mov.l r3,@r5
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
