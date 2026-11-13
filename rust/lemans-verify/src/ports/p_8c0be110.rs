//! 0x8c0be110 — drain `object+32`, then free the object on a positive count.
//!
//! A null object returns 0 with T set. Otherwise `0x8c0f72fc(object+32, -1)`
//! runs: the inner count is `-1`, so that wrapper drains and does not free
//! `object+32`. A positive low half of the saved count then frees the original
//! object through `0x8c01032c`. `r0` is the object. `r13`/`r14` restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.r[0] = rd(mem, 0x8c0be13c); // 0x8c0f72fc
        cpu.r[4] = cpu.r[4].wrapping_add(32);
        cpu.pr = 0x8c0be126;
        cpu.r[5] = (-1i32) as u32; // jsr delay
        crate::registry::p_8c0f72fc::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0be140); // 0x8c01032c
            cpu.pr = 0x8c0be132;
            cpu.r[4] = cpu.r[13]; // jsr delay
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
