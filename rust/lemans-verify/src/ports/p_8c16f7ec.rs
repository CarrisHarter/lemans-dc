//! 0x8c16f7ec — stamp twelve indexed slots from a mode word.
//!
//! `r5` in `0..3` loads one word through `[0x8c27a238]` (field +36, then
//! `+8 + r5*4`) and writes that word into twelve slots of the object in `r4`.
//! Any other `r5` writes zero. `cmp/gt` against 4 leaves T set only when the
//! mode was a real index; the stores do not touch T. `r13` and `r14` are
//! restored. The last store adds `0x52c` to the object base before the call,
//! so `r4` exits as that slot plus the callee's `+92`.

use sh4_oracle::{Cpu, Memory};

const MID_OFFSETS: [u32; 10] = [
    0xa4, 0x118, 0x18c, 0x200, 0x274, 0x2e8, 0x35c, 0x3d0, 0x444, 0x4b8,
];

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn store(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[5] = cpu.r[14];
    cpu.pr = ret;
    crate::registry::p_8c16eadc::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.pr);
    cpu.r[13] = cpu.r[4];
    let negative = (cpu.r[5] as i32) < 0;
    cpu.set_t(!negative);
    cpu.r[14] = 0; // bf.s delay
    if !negative {
        cpu.r[0] = 4;
        let in_range = (cpu.r[5] as i32) < 4;
        cpu.set_t(in_range);
        if in_range {
            cpu.r[4] = mem.read_u32(0x8c27a238).unwrap_or(0);
            cpu.pr = 0x8c16f80a;
            crate::registry::p_8c0c0ecc::run(cpu, mem);
            cpu.r[14] = cpu.r[0];
        }
    }
    cpu.r[4] = cpu.r[13].wrapping_add(0x30);
    store(cpu, mem, 0x8c16f816);
    let mut ret = 0x8c16f822;
    for offset in MID_OFFSETS {
        cpu.r[4] = cpu.r[13].wrapping_add(offset);
        store(cpu, mem, ret);
        ret += 12;
    }
    cpu.r[13] = cpu.r[13].wrapping_add(0x52c);
    cpu.r[4] = cpu.r[13];
    store(cpu, mem, 0x8c16f89a);
    cpu.pr = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
