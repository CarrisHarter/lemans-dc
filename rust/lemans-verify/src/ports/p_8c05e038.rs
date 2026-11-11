//! 0x8c05e038 — compare bytes at r4 vs r5 until mismatch or a zero byte.
//!
//! Original side effects: r0 = extu.b(r3) entry / difference, r3 = last dst
//! byte, r1 = last src byte, r4/r5 advanced, T = last tst/cmp; equal strings
//! return 0 via the entry tst.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[3] & 0xff; // extu.b r3,r0
    loop {
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            return; // rts delay slot: mov #0,r0 (r0 already 0)
        }
        let d = mem.read_u8(cpu.r[4]).unwrap_or(0);
        let s = mem.read_u8(cpu.r[5]).unwrap_or(0);
        cpu.r[3] = d as u32; // extu.b r3,r3
        cpu.r[0] = cpu.r[3]; // extu.b r3,r0
        cpu.r[1] = s as u32; // extu.b r1,r1
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot
        if cpu.r[0] == cpu.r[1] {
            continue;
        }
        cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay slot: sub r1,r0
        return;
    }
}
