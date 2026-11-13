//! 0x8c0f72fc — stamp two vtables, drain the shared list, then maybe free.
//!
//! A null object returns 0 with T set. Otherwise `[object]` is stored as
//! `0x8c1ecf4c` in the delay of a dead `bt.s`, then overwritten with
//! `0x8c1ecfd8`, and `0x8c0f8960(object, 0)` overwrites it again with
//! `0x8c1ed000`. The incoming count is not forwarded. A positive low half of
//! the saved count frees the object through `0x8c01032c`. `r0` is the object.
//! `r13`/`r14` are restored. Here `r14` holds the object and `r13` the count.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[5];
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        cpu.r[0] = rd(mem, 0x8c0f7334); // 0x8c1ecf4c
        cpu.set_t(cpu.r[14] == 0);
        let skip = cpu.t();
        let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // bt.s delay, always
        if !skip {
            cpu.r[0] = rd(mem, 0x8c0f7338); // 0x8c1ecfd8
            let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
            cpu.r[0] = rd(mem, 0x8c0f733c); // 0x8c0f8960
            cpu.pr = 0x8c0f731c;
            cpu.r[5] = 0; // jsr delay
            crate::registry::p_8c0f8960::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0f7340); // 0x8c01032c
            cpu.pr = 0x8c0f7328;
            cpu.r[4] = cpu.r[14]; // jsr delay
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
