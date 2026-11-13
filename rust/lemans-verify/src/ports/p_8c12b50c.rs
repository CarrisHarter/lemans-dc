//! 0x8c12b50c — release the inner block, then the container when asked.
//!
//! A null container returns 0. A null inner skips straight to the count
//! test. Otherwise three address pairs at `inner+56`, `+44` and `+32` are
//! handed to `0x8c043a68` and then to `0x8c01e460` with `r5 = -1`, and the
//! handles at `inner+24`, `+20` and `+16` go to `0x8c018bd8` with the same
//! `-1`. That negative count makes those callees refuse to free the handle.
//! The inner block itself is then freed unconditionally.
//!
//! The container is freed only when `sext16(r5) > 0`. A value such as
//! `0x00010000` looks positive and is not; `0x80000001` looks negative and
//! is. `r12`, `r13` and `r14` are restored, and `r0` is the container.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn release_pair(cpu: &mut Cpu, mem: &mut Memory, offset: u32, ret_free: u32, ret_guard: u32) {
    cpu.r[4] = cpu.r[14].wrapping_add(offset);
    cpu.set_t(cpu.r[4] == 0);
    if cpu.t() {
        return;
    }
    cpu.set_t(cpu.r[4] == 0); // second tst of the same address
    if cpu.t() {
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c12b5b4).unwrap_or(0x8c043a68);
    cpu.pr = ret_free;
    crate::registry::p_8c043a68::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c12b5b8).unwrap_or(0x8c01e460);
    cpu.r[4] = cpu.r[14].wrapping_add(offset);
    cpu.pr = ret_guard;
    cpu.r[5] = 0xffff_ffff; // jsr delay
    crate::registry::p_8c01e460::run(cpu, mem);
}

fn release_handle(cpu: &mut Cpu, mem: &mut Memory, offset: u32, ret: u32) {
    cpu.r[0] = mem.read_u32(0x8c12b5bc).unwrap_or(0x8c018bd8);
    cpu.r[4] = cpu.r[14].wrapping_add(offset);
    cpu.pr = ret;
    cpu.r[5] = 0xffff_ffff; // jsr delay
    crate::registry::p_8c018bd8::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[12] = cpu.r[4];
    cpu.set_t(cpu.r[12] == 0);
    if !cpu.t() {
        cpu.r[14] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.set_t(cpu.r[14] == 0);
        if !cpu.t() {
            release_pair(cpu, mem, 56, 0x8c12b534, 0x8c12b53e);
            release_pair(cpu, mem, 44, 0x8c12b550, 0x8c12b55a);
            release_pair(cpu, mem, 32, 0x8c12b56c, 0x8c12b576);
            release_handle(cpu, mem, 24, 0x8c12b580);
            release_handle(cpu, mem, 20, 0x8c12b58a);
            release_handle(cpu, mem, 16, 0x8c12b594);
            cpu.r[0] = mem.read_u32(0x8c12b5c0).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c12b59a;
            cpu.r[4] = cpu.r[14]; // jsr delay
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive); // cmp/pl
        if positive {
            cpu.r[0] = mem.read_u32(0x8c12b5c0).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c12b5a6;
            cpu.r[4] = cpu.r[12]; // jsr delay
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[12];
    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
