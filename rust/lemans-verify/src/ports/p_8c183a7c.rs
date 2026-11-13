//! 0x8c183a7c — file a node into the first empty keyword slot, then walk children.
//!
//! The node's string is `[node+36]`, or the pool word at `0x8c05b2bc` when that
//! field is null. The first of `tyre`, `wheel`, `disc`, `spoke`, `kaching`
//! (the suffixes at `0x8c20e178`) that occurs in the string is stored: the node
//! pointer goes in the object at `+4`, `+0`, `+8`, `+12`, or `+16`. A slot that
//! already holds a pointer is left alone and the later keywords are not tried.
//! Afterwards `[node+12]` is the first child and each child's `+8` is the next
//! sibling; both are walked by calling this function again. The last `tst` of a
//! null link leaves T set. `r12`, `r13`, and `r14` are restored.

use sh4_oracle::{Cpu, Memory};

const ARMS: [(u32, u32, u32); 5] = [
    (7, 4, 0x8c183a9a),
    (12, 0, 0x8c183ab2),
    (18, 8, 0x8c183ace),
    (23, 12, 0x8c183ae8),
    (29, 16, 0x8c183b02),
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

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[12] = cpu.r[4];

    cpu.r[4] = cpu.r[13]; // jsr delay
    cpu.pr = 0x8c183a8e;
    crate::registry::p_8c05b2ac::run(cpu, mem);
    cpu.r[14] = cpu.r[0];

    let base = mem.read_u32(0x8c183b34).unwrap_or(0x8c20e178);
    for (add, slot, ret) in ARMS {
        cpu.r[5] = base.wrapping_add(add);
        cpu.r[4] = cpu.r[14];
        cpu.pr = ret;
        crate::registry::p_8c17fc44::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] == 0 {
            continue;
        }
        let addr = cpu.r[12].wrapping_add(slot);
        cpu.r[0] = mem.read_u32(addr).unwrap_or(0);
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] != 0 {
            break;
        }
        let _ = mem.write_u32(addr, cpu.r[13]);
        break;
    }

    cpu.r[13] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[13] == 0);
    while cpu.r[13] != 0 {
        cpu.r[5] = cpu.r[13];
        cpu.pr = 0x8c183b1e;
        cpu.r[4] = cpu.r[12]; // jsr delay
        run(cpu, mem);
        cpu.r[13] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
        cpu.set_t(cpu.r[13] == 0);
    }

    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
