//! 0x8c052cb0 — forward one mode word to whichever of three blocks is live.
//!
//! `r13` keeps the object and `r14` keeps the entry `r5`. Each of
//! `[r13+0x1e4]`, `[r13+0x1e8]`, and `[r13+0x1ec]` is tested and, when
//! non-null, passed to `0x8c0c9d84`, `0x8c0d0274`, and `0x8c16f7ec`.
//! The first call's delay is `nop`, so that callee sees `r5` as it still
//! stands. The other two reload `r5` from `r14` in the `jsr` delay.
//! `r13` and `r14` are restored. The flag that survives is the third
//! `tst` when that pointer is null, and the range compare inside
//! `0x8c16f7ec` when it is not.

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

fn field(cpu: &mut Cpu, mem: &mut Memory, pool: u32) -> bool {
    cpu.r[0] = mem.read_u32(pool).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0);
    cpu.t()
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[13] = cpu.r[4];

    if !field(cpu, mem, 0x8c052cec) {
        cpu.r[0] = mem.read_u32(0x8c052cf0).unwrap_or(0x8c0c9d84);
        cpu.pr = 0x8c052cc8;
        crate::registry::p_8c0c9d84::run(cpu, mem);
    }
    if !field(cpu, mem, 0x8c052cf4) {
        cpu.r[0] = mem.read_u32(0x8c052cf8).unwrap_or(0x8c0d0274);
        cpu.pr = 0x8c052cd6;
        cpu.r[5] = cpu.r[14]; // jsr delay
        crate::registry::p_8c0d0274::run(cpu, mem);
    }
    if !field(cpu, mem, 0x8c052cfc) {
        cpu.r[0] = mem.read_u32(0x8c052d00).unwrap_or(0x8c16f7ec);
        cpu.pr = 0x8c052ce4;
        cpu.r[5] = cpu.r[14]; // jsr delay
        crate::registry::p_8c16f7ec::run(cpu, mem);
    }

    cpu.pr = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
