//! 0x8c226b70 — turn a table hit into 120 and a miss into 0.
//!
//! The word at 0x8c2c9db8 is the key. 0x8c239440 returns 0x100 on a hit and
//! 0x101 on a miss or a negative key. A match against the 0x101 pool halfword
//! returns 0; anything else returns 120. r3 is that halfword on the way out.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c226b70 sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c226b72 mov.l 0x8c226ba4,r3 / jsr / nop
    cpu.r[3] = mem.read_u32(0x8c226ba4).unwrap_or(0);
    cpu.pr = 0x8c226b78;
    crate::registry::p_8c237640::run(cpu, mem);
    // 0x8c226b78 mov.l 0x8c226ba8,r2 / jsr / mov r0,r4
    cpu.r[2] = mem.read_u32(0x8c226ba8).unwrap_or(0);
    cpu.r[4] = cpu.r[0];
    cpu.pr = 0x8c226b7e;
    crate::registry::p_8c239440::run(cpu, mem);
    // 0x8c226b7e mov.w 0x8c226ba0,r3 / cmp/eq r3,r0
    cpu.r[3] = mem.read_u16(0x8c226ba0).unwrap_or(0) as i16 as i32 as u32;
    cpu.set_t(cpu.r[0] == cpu.r[3]);
    if cpu.t() {
        // 0x8c226b84 bra / mov #0,r4
        cpu.r[4] = 0;
    } else {
        // 0x8c226b88 mov #120,r4
        cpu.r[4] = 120;
    }
    // 0x8c226b8a lds.l @r15+,pr / rts / mov r4,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[4];
}
