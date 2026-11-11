//! 0x8c1a3ad0 — indexed pair lookup with a sentinel.
//!
//! ```text
//! r0 = [r4+16] ; cmp/gt r5,r0 ; bf miss
//! r1 = [r4+8] ; r5 <<= 2 ; r5 <<= 1 ; r1 += r5
//! r0 = [r1] ; [r6] = r0 ; r0 = [r1+4] ; rts ; [r7] = r0
//! miss: r0 = 0 ; [r6] = 0 ; r0 = -1 ; rts ; [r7] = r0
//! ```
//!
//! Hidden side effects: `shll2` does not touch T but the following `shll`
//! does, so the hit path ends with T = bit 31 of `idx*4` rather than the
//! `cmp/gt` result; `r5` is scaled in place; and the miss path writes 0 and
//! -1 to the two out pointers.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0);
    let hit = (cpu.r[0] as i32) > (cpu.r[5] as i32); // cmp/gt r5,r0
    cpu.set_t(hit);
    if hit {
        cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
        cpu.r[5] = cpu.r[5] << 2; // shll2: no T
        let carry = cpu.r[5] >> 31 != 0;
        cpu.r[5] = cpu.r[5] << 1; // shll: T = old bit 31
        cpu.set_t(carry);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[1] + 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[7], cpu.r[0]); // delay slot of `rts`
    } else {
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[6], 0);
        cpu.r[0] = u32::MAX;
        let _ = mem.write_u32(cpu.r[7], u32::MAX); // delay slot of `rts`
    }
}
