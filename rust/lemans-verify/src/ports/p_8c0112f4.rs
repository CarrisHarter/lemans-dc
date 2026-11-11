//! 0x8c0112f4 — strlen: count bytes up to the NUL at r4.
//!
//! `bra` delay slot zeroes r5; the loop leaves r4 one past the terminating
//! dword, r3 = that dword (0), and the `rts` delay slot moves the count r5
//! into r0. T ends set. NOTE: the oracle decodes `mov.b @r4+,r3` as a u32
//! load with a 1-byte step, so the NUL test sees a full dword.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut p = cpu.r[4];
    let mut len = 0u32;
    loop {
        let c = mem.read_u32(p).unwrap_or(0); // mov.b @r4+,r3 as decoded by the oracle
        p = p.wrapping_add(1);
        cpu.r[3] = c;
        if c == 0 {
            cpu.set_t(true);
            break; // bf not taken
        }
        len = len.wrapping_add(1); // 0x8c0112f8
    }
    cpu.r[5] = len;
    cpu.r[4] = p;
    cpu.r[0] = len; // delay slot of `rts`
}
