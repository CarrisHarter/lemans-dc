//! 0x8c18f904 — free the node at `[r4+4]` and clear the three slots.
//!
//! `[r4+4] == 0` returns without writing. Otherwise `[node+0]` and `[node+4]`
//! are children. A child's `[+0]` and `[+4]`, when set, are nodes passed to
//! `0x8c18f784` with the original `r4` as context. Each live child, then the
//! node, is `release(p+12, -1)` and `free(p)`. The exit stores are `[r4] = 0`,
//! `[r4+4] = 0`, `[r4+8] = r4+4`, and `r0 = r4+4`. `r12`–`r14` are restored.
//! The first `0x8c18f784` call does not reload `r4`; every later one does.

use sh4_oracle::{Cpu, Memory};

const NODE_PTR: u32 = 0x8c18f998;
const REL_PTR: u32 = 0x8c18f99c;
const FREE_PTR: u32 = 0x8c18f9a0;

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn release_and_free(cpu: &mut Cpu, mem: &mut Memory, ptr: u32, pr_rel: u32, pr_free: u32) {
    cpu.r[0] = mem.read_u32(REL_PTR).unwrap_or(0);
    cpu.r[4] = ptr.wrapping_add(12);
    cpu.r[5] = (-1i32) as u32;
    cpu.pr = pr_rel;
    crate::registry::p_8c018bd8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(0);
    cpu.r[4] = ptr;
    cpu.pr = pr_free;
    crate::registry::p_8c01032c::run(cpu, mem);
}

/// `r12` is the child. `reload_first` is false only for the nop delay slot
/// at 0x8c18f924; every other call reloads `r4` from `r13`.
fn drop_child(
    cpu: &mut Cpu,
    mem: &mut Memory,
    reload_first: bool,
    pr_lo: u32,
    pr_hi: u32,
    pr_rel: u32,
    pr_free: u32,
) {
    cpu.r[5] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        cpu.r[0] = mem.read_u32(NODE_PTR).unwrap_or(0);
        if reload_first {
            cpu.r[4] = cpu.r[13];
        }
        cpu.pr = pr_lo;
        crate::registry::p_8c18f784::run(cpu, mem);
    }
    cpu.r[5] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        cpu.r[0] = mem.read_u32(NODE_PTR).unwrap_or(0);
        cpu.r[4] = cpu.r[13];
        cpu.pr = pr_hi;
        crate::registry::p_8c18f784::run(cpu, mem);
    }
    let child = cpu.r[12];
    release_and_free(cpu, mem, child, pr_rel, pr_free);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c18f904
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    push(cpu, mem, cpu.pr);
    cpu.r[14] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[14] == 0);
    if cpu.r[14] != 0 {
        cpu.r[12] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.set_t(cpu.r[12] == 0);
        if cpu.r[12] != 0 {
            drop_child(
                cpu, mem, false, 0x8c18f926, 0x8c18f932, 0x8c18f93c, 0x8c18f942,
            );
        }
        cpu.r[12] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[12] == 0);
        if cpu.r[12] != 0 {
            drop_child(
                cpu, mem, true, 0x8c18f954, 0x8c18f960, 0x8c18f96a, 0x8c18f970,
            );
        }
        let node = cpu.r[14];
        release_and_free(cpu, mem, node, 0x8c18f97a, 0x8c18f980);
        cpu.r[1] = 0;
        let _ = mem.write_u32(cpu.r[13], 0);
        cpu.r[0] = cpu.r[13];
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), 0);
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
    }
    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
