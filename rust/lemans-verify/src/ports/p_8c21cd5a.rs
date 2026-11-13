//! 0x8c21cd5a — clear mode 8, mode 9, one indexed word, then a global.
//!
//! Two calls to `0x8c219f0e` store 0 through modes 8 and 9. The word at
//! `0x8c23c50c` is then passed to `0x8c21a04a` with value 0, which writes the
//! table only when the index is `<= 31` and leaves T set when it is not.
//! The delay of `rts` stores 0 at `0x8c2ba640`. Nothing else touches T, so the
//! indexed compare is the flag that survives.

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

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.pr);
    cpu.r[3] = mem.read_u32(0x8c21cddc).unwrap_or(0x8c219f0e);
    cpu.r[5] = 0;
    cpu.pr = 0x8c21cd64;
    cpu.r[4] = 8; // jsr delay
    crate::registry::p_8c219f0e::run(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c21cddc).unwrap_or(0x8c219f0e);
    cpu.r[5] = 0;
    cpu.pr = 0x8c21cd6c;
    cpu.r[4] = 9; // jsr delay
    crate::registry::p_8c219f0e::run(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c21cdf4).unwrap_or(0x8c21a04a);
    cpu.r[5] = 0;
    cpu.r[3] = mem.read_u32(0x8c21cdf0).unwrap_or(0x8c23c50c);
    cpu.pr = 0x8c21cd76;
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0); // jsr delay
    crate::registry::p_8c21a04a::run(cpu, mem);
    cpu.pr = pop(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c21cdec).unwrap_or(0x8c2ba640);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // rts delay
}
