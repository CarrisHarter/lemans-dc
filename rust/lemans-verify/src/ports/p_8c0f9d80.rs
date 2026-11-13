//! 0x8c0f9d80 — release and free a binary tree of nodes.
//!
//! `r4` is a context forwarded to the recursive calls. `r5` is the root. For
//! each of `[root]` and `[root+4]`, both of that child's slots are released by
//! a recursive call when non-null, then `0x8c0f8f10(slot+12, -1)` and
//! `free(slot)`. The child itself is released through `0x8c018bd8(child+12, -1)`
//! and freed. The root is released and freed the same way, unconditionally.
//! Only the first recursive site leaves `r4` alone; every later site reloads it
//! from `r13`. `r11`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

const SELF_PTR: u32 = 0x8c0f9e9c;
const SLOT_PTR: u32 = 0x8c0f9ea0;
const FREE_PTR: u32 = 0x8c0f9ea4;
const RELEASE_PTR: u32 = 0x8c0f9ea8;

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn call_self(cpu: &mut Cpu, mem: &mut Memory, ret: u32, reload: bool) {
    cpu.r[0] = rd(mem, SELF_PTR);
    if reload {
        cpu.r[4] = cpu.r[13];
    }
    cpu.pr = ret;
    crate::registry::p_8c0f9d80::run(cpu, mem);
}

fn call_slot(cpu: &mut Cpu, mem: &mut Memory, node: u32, ret: u32) {
    cpu.r[0] = rd(mem, SLOT_PTR);
    cpu.r[4] = node.wrapping_add(12);
    cpu.pr = ret;
    cpu.r[5] = (-1i32) as u32;
    crate::registry::p_8c0f8f10::run(cpu, mem);
}

fn call_release(cpu: &mut Cpu, mem: &mut Memory, node: u32, ret: u32) {
    cpu.r[4] = node.wrapping_add(12);
    cpu.set_t(cpu.r[4] == 0);
    if cpu.t() {
        return;
    }
    cpu.r[0] = rd(mem, RELEASE_PTR);
    cpu.pr = ret;
    cpu.r[5] = (-1i32) as u32;
    crate::registry::p_8c018bd8::run(cpu, mem);
}

fn call_free(cpu: &mut Cpu, mem: &mut Memory, node: u32, ret: u32) {
    cpu.r[0] = rd(mem, FREE_PTR);
    cpu.pr = ret;
    cpu.r[4] = node;
    crate::registry::p_8c01032c::run(cpu, mem);
}

fn grandchild(cpu: &mut Cpu, mem: &mut Memory, first_reload: bool, rets: (u32, u32, u32, u32)) {
    let node = cpu.r[11];
    let (first_ret, second_ret, slot_ret, free_ret) = rets;
    cpu.r[5] = rd(mem, node);
    cpu.set_t(cpu.r[5] == 0);
    if !cpu.t() {
        call_self(cpu, mem, first_ret, first_reload);
    }
    cpu.r[5] = rd(mem, node.wrapping_add(4));
    cpu.set_t(cpu.r[5] == 0);
    if !cpu.t() {
        call_self(cpu, mem, second_ret, true);
    }
    call_slot(cpu, mem, node, slot_ret);
    call_free(cpu, mem, node, free_ret);
}

fn child(cpu: &mut Cpu, mem: &mut Memory, first_reload: bool, left: (u32, u32, u32, u32), right: (u32, u32, u32, u32), rel_ret: u32, free_ret: u32) {
    cpu.r[11] = rd(mem, cpu.r[12]);
    cpu.set_t(cpu.r[11] == 0);
    if !cpu.t() {
        grandchild(cpu, mem, first_reload, left);
    }
    cpu.r[11] = rd(mem, cpu.r[12].wrapping_add(4));
    cpu.set_t(cpu.r[11] == 0);
    if !cpu.t() {
        grandchild(cpu, mem, true, right);
    }
    call_release(cpu, mem, cpu.r[12], rel_ret);
    call_free(cpu, mem, cpu.r[12], free_ret);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = rd(mem, cpu.r[14]);
    cpu.set_t(cpu.r[12] == 0);
    if !cpu.t() {
        child(
            cpu, mem, false,
            (0x8c0f9da6, 0x8c0f9db2, 0x8c0f9dbc, 0x8c0f9dc2),
            (0x8c0f9dd4, 0x8c0f9de0, 0x8c0f9dea, 0x8c0f9df0),
            0x8c0f9dfe, 0x8c0f9e04,
        );
    }
    cpu.r[12] = rd(mem, cpu.r[14].wrapping_add(4));
    cpu.set_t(cpu.r[12] == 0);
    if !cpu.t() {
        child(
            cpu, mem, true,
            (0x8c0f9e1c, 0x8c0f9e28, 0x8c0f9e32, 0x8c0f9e38),
            (0x8c0f9e4a, 0x8c0f9e56, 0x8c0f9e60, 0x8c0f9e66),
            0x8c0f9e74, 0x8c0f9e7a,
        );
    }
    call_release(cpu, mem, cpu.r[14], 0x8c0f9e88);
    call_free(cpu, mem, cpu.r[14], 0x8c0f9e8e);

    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
