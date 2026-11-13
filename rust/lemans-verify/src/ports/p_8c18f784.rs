//! 0x8c18f784 — recursively free a node and the pairs hanging off it.
//!
//! `r5` is the node, `r4` is an opaque context passed unchanged into every
//! recursive call. The node holds two pair pointers at `+0` and `+4`. Each
//! pair holds two leaf pointers at `+0` and `+4`; a non-null leaf recurses on
//! the nodes at its `+0` and `+4`. Every live leaf, pair and the node itself
//! is then `0x8c018bd8(p+12, -1)` and `0x8c01032c(p)`. `r11`–`r14` are
//! restored. A node whose pairs are both null still releases and frees itself.

use sh4_oracle::{Cpu, Memory};

const SELF_PTR: u32 = 0x8c18f894;
const REL_PTR: u32 = 0x8c18f898;
const FREE_PTR: u32 = 0x8c18f89c;

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

/// Leaf in `r11`. Recurse on its two node pointers, then release and free it.
fn drop_leaf(cpu: &mut Cpu, mem: &mut Memory, ret_l: u32, ret_r: u32, ret_rel: u32, ret_free: u32) {
    // mov.l @r11,r5 / tst r5,r5 / bt
    cpu.r[5] = mem.read_u32(cpu.r[11]).unwrap_or(0);
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        cpu.r[0] = mem.read_u32(SELF_PTR).unwrap_or(0);
        cpu.r[4] = cpu.r[13];
        cpu.pr = ret_l;
        run(cpu, mem);
    }
    // mov.l @(4,r11),r5 / tst / bt
    cpu.r[5] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        cpu.r[0] = mem.read_u32(SELF_PTR).unwrap_or(0);
        cpu.r[4] = cpu.r[13];
        cpu.pr = ret_r;
        run(cpu, mem);
    }
    // release(r11+12, -1)
    cpu.r[0] = mem.read_u32(REL_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[11].wrapping_add(12);
    cpu.r[5] = (-1i32) as u32;
    cpu.pr = ret_rel;
    crate::registry::p_8c018bd8::run(cpu, mem);
    // free(r11)
    cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[11];
    cpu.pr = ret_free;
    crate::registry::p_8c01032c::run(cpu, mem);
}

fn drop_pair(cpu: &mut Cpu, mem: &mut Memory, leaf0: (u32, u32, u32, u32), leaf1: (u32, u32, u32, u32), ret_rel: u32, ret_free: u32) {
    // mov.l @r12,r11 / tst r11,r11
    cpu.r[11] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.set_t(cpu.r[11] == 0);
    if cpu.r[11] != 0 {
        drop_leaf(cpu, mem, leaf0.0, leaf0.1, leaf0.2, leaf0.3);
    }
    // mov.l @(4,r12),r11 / tst r11,r11
    cpu.r[11] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[11] == 0);
    if cpu.r[11] != 0 {
        drop_leaf(cpu, mem, leaf1.0, leaf1.1, leaf1.2, leaf1.3);
    }
    // release(r12+12, -1) / free(r12)
    cpu.r[0] = mem.read_u32(REL_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[12].wrapping_add(12);
    cpu.r[5] = (-1i32) as u32;
    cpu.pr = ret_rel;
    crate::registry::p_8c018bd8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[12];
    cpu.pr = ret_free;
    crate::registry::p_8c01032c::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c18f784: mov.l r14,@-r15
    push(cpu, mem, cpu.r[14]);
    // 0x8c18f786: mov.l r13,@-r15
    push(cpu, mem, cpu.r[13]);
    // 0x8c18f788: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c18f78a: mov.l r12,@-r15
    push(cpu, mem, cpu.r[12]);
    // 0x8c18f78c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c18f78e: mov.l r11,@-r15
    push(cpu, mem, cpu.r[11]);
    // 0x8c18f790: sts.l pr,@-r15
    push(cpu, mem, cpu.pr);
    // 0x8c18f792: mov.l @r14,r12 / tst / bt 0x8c18f804
    cpu.r[12] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.set_t(cpu.r[12] == 0);
    if cpu.r[12] != 0 {
        drop_pair(
            cpu,
            mem,
            (0x8c18f7aa, 0x8c18f7b6, 0x8c18f7c0, 0x8c18f7c6),
            (0x8c18f7d8, 0x8c18f7e4, 0x8c18f7ec, 0x8c18f7f4),
            0x8c18f7fe,
            0x8c18f804,
        );
    }
    // 0x8c18f804: mov.l @(4,r14),r12 / tst / bt 0x8c18f876
    cpu.r[12] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[12] == 0);
    if cpu.r[12] != 0 {
        drop_pair(
            cpu,
            mem,
            (0x8c18f81c, 0x8c18f828, 0x8c18f830, 0x8c18f838),
            (0x8c18f84a, 0x8c18f856, 0x8c18f85e, 0x8c18f866),
            0x8c18f870,
            0x8c18f876,
        );
    }
    // 0x8c18f876: release(r14+12, -1)
    cpu.r[0] = mem.read_u32(REL_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[14].wrapping_add(12);
    cpu.r[5] = (-1i32) as u32;
    cpu.pr = 0x8c18f880;
    crate::registry::p_8c018bd8::run(cpu, mem);
    // 0x8c18f880: free(r14)
    cpu.r[0] = mem.read_u32(FREE_PTR).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.pr = 0x8c18f886;
    crate::registry::p_8c01032c::run(cpu, mem);
    // 0x8c18f886: lds.l @r15+,pr
    cpu.pr = pop(cpu, mem);
    // 0x8c18f888: mov.l @r15+,r11
    cpu.r[11] = pop(cpu, mem);
    // 0x8c18f88a: mov.l @r15+,r12
    cpu.r[12] = pop(cpu, mem);
    // 0x8c18f88c: mov.l @r15+,r13
    cpu.r[13] = pop(cpu, mem);
    // 0x8c18f88e: rts ; delay mov.l @r15+,r14
    cpu.r[14] = pop(cpu, mem);
}
