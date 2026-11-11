//! 0x8c159e7c — copy_three_words_to_globals.
//!
//! Copies three words from `r4` to `0x8c2abd90` and writes 1 to `0x8c2abda8`.
//! Both addresses come from the literal pool; `r0` ends as 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r5 = mem.read_u32(cpu.r[4]).unwrap_or(0);
    let r0: u32 = 1;
    let r6 = mem.read_u32(0x8c159e94).unwrap_or(0);
    let r3 = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let r1 = mem.read_u32(0x8c159e98).unwrap_or(0);
    let _ = mem.write_u32(r6, r5);
    let _ = mem.write_u32(r6 + 4, r3);
    let r3 = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    let _ = mem.write_u32(r6 + 8, r3);
    let _ = mem.write_u32(r1, r0); // delay slot of `rts`
    cpu.r[5] = r5;
    cpu.r[0] = r0;
    cpu.r[6] = r6;
    cpu.r[3] = r3;
    cpu.r[1] = r1;
}
