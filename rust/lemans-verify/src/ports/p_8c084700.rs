//! 0x8c084700 — release and free a binary tree of nodes.
//!
//! `r4` is a context forwarded to the recursive calls. `r5` is the root. For
//! each of `[root]` and `[root+4]`, both of that child's slots are released by
//! a recursive call when non-null, then `0x8c083ce8(slot+12, -1)` and
//! `free(slot)`. The child itself is released through `0x8c018bd8(child+12, -1)`
//! and freed. The root is released and freed the same way, unconditionally.
//! Only the first recursive site leaves `r4` alone; every later site reloads it
//! from `r13`. `r11`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

const SELF_PTR: u32 = 0x8c08481c;
const SLOT_PTR: u32 = 0x8c084820;
const FREE_PTR: u32 = 0x8c084824;
const RELEASE_PTR: u32 = 0x8c084828;

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn call_self(cpu: &mut Cpu, mem: &mut Memory, ret: u32, reload: bool) {
    cpu.r[0] = rd(mem, SELF_PTR);
    if reload {
        cpu.r[4] = cpu.r[13];
    }
    cpu.pr = ret;
    crate::registry::p_8c084700::run(cpu, mem);
}

fn call_slot(cpu: &mut Cpu, mem: &mut Memory, node: u32, ret: u32) {
    cpu.r[0] = rd(mem, SLOT_PTR);
    cpu.r[4] = node.wrapping_add(12);
    cpu.pr = ret;
    cpu.r[5] = (-1i32) as u32;
    crate::registry::p_8c083ce8::run(cpu, mem);
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
            (0x8c084726, 0x8c084732, 0x8c08473c, 0x8c084742),
            (0x8c084754, 0x8c084760, 0x8c08476a, 0x8c084770),
            0x8c08477e, 0x8c084784,
        );
    }
    cpu.r[12] = rd(mem, cpu.r[14].wrapping_add(4));
    cpu.set_t(cpu.r[12] == 0);
    if !cpu.t() {
        child(
            cpu, mem, true,
            (0x8c08479c, 0x8c0847a8, 0x8c0847b2, 0x8c0847b8),
            (0x8c0847ca, 0x8c0847d6, 0x8c0847e0, 0x8c0847e6),
            0x8c0847f4, 0x8c0847fa,
        );
    }
    call_release(cpu, mem, cpu.r[14], 0x8c084808);
    call_free(cpu, mem, cpu.r[14], 0x8c08480e);

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
