//! 0x8c1b67fc — clear eleven float fields.
//!
//! For each of the seven words at `[r4+0x1c..0x34]` and the four at
//! `[r4+0x38..0x44]`, call 0x8c16ee30 with `fr4 = 0.0`; then set the flag
//! byte `[r4] = 1`. Both loops count with r13 and keep the byte offset in
//! r12, which the branch delay slot advances by 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = 0;
    cpu.r[12] = cpu.r[13];
    cpu.r[12] <<= 2;
    loop {
        cpu.r[1] = cpu.r[14].wrapping_add(28);
        cpu.r[0] = cpu.r[12];
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c1b6850).unwrap_or(0);
        cpu.fr[4] = 0.0; // delay slot of `jsr`: fldi0 fr4
        crate::registry::p_8c16ee30::run(cpu, mem);
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        cpu.r[0] = 7;
        let t = (cpu.r[0] as i32) > (cpu.r[13] as i32); // cmp/gt r13,r0
        cpu.set_t(t);
        cpu.r[12] = cpu.r[12].wrapping_add(4); // delay slot of `bt.s`
        if !t {
            break;
        }
    }
    cpu.r[13] = 0;
    cpu.r[12] = cpu.r[13];
    cpu.r[12] <<= 2;
    loop {
        cpu.r[1] = cpu.r[14].wrapping_add(56);
        cpu.r[0] = cpu.r[12];
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c1b6850).unwrap_or(0);
        cpu.fr[4] = 0.0; // delay slot of `jsr`: fldi0 fr4
        crate::registry::p_8c16ee30::run(cpu, mem);
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        cpu.r[0] = 4;
        let t = (cpu.r[0] as i32) > (cpu.r[13] as i32); // cmp/gt r13,r0
        cpu.set_t(t);
        cpu.r[12] = cpu.r[12].wrapping_add(4); // delay slot of `bt.s`
        if !t {
            break;
        }
    }
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[14], cpu.r[0] as u8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
