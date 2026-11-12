//! 0x8c175a4a - flag gate, then store `[r4+44]` or emit an error string.
//!
//! Twin with the `+44` field.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[2] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r2
        cpu.r[5] = mem.read_u32(0x8c175af0).unwrap_or(0x8c1e0ef0); // mov.l 0x8c175af0,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c175a58;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay
    } else {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        cpu.r[3] = 0; // mov #0,r3
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bf.s delay: mov.l r3,@r5
        if cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c175ad0).unwrap_or(0x8c174564); // mov.l 0x8c175ad0,r3
            cpu.r[5] = mem.read_u32(0x8c175af4).unwrap_or(0x8c1e0f20); // mov.l 0x8c175af4,r5
            cpu.r[4] = 101; // jsr delay: mov #101,r4
            cpu.pr = 0x8c175a6c;
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0;
        } else {
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(44)).unwrap_or(0); // mov.l @(44,r4),r2
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // mov.l r2,@r5
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
