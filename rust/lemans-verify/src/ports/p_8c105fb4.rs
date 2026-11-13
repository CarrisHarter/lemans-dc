//! 0x8c105fb4 — insert `r5` into the record at `[r4+4] + r6*12`.
//!
//! The insert returns an index. When the record's count is still greater than
//! that index, element 0 is looked up and the pointer is discarded. The index
//! is what comes back in `r0`. `r13`, `r14` and `pr` are saved.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.pr);
    cpu.r[14] = cpu.r[6];
    let out = cpu.r[14] >> 31 != 0;
    cpu.r[14] <<= 1; // shll
    cpu.set_t(out);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[6]); // *3
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[14] <<= 2; // *12
    cpu.r[0] = mem.read_u32(0x8c105fe8).unwrap_or(0x8c18b8f4);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[1]);
    cpu.r[4] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c105fcc;
    crate::registry::p_8c18b8f4::run(cpu, mem);
    cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32)); // cmp/gt r0,r1
    cpu.r[13] = cpu.r[0]; // bf.s delay
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c105fec).unwrap_or(0x8c18b9c8);
        cpu.r[4] = cpu.r[14];
        cpu.r[5] = 0;
        cpu.pr = 0x8c105fdc;
        crate::registry::p_8c18b9c8::run(cpu, mem);
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
