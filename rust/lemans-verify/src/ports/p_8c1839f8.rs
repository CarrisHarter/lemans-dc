//! 0x8c1839f8 - recursive tree walk that records the first matching node.
//!
//! For the node `r12`, 0x8c05b2ac resolves its name and 0x8c17fc44 tests it
//! against the needle at `0x8c20e178`. When it matches and `[r13]` is still null,
//! `[r13] = node`. The walk then descends `[node+12]` (recursing with the same
//! `r13`) and finally `[node+8]`, repeating until both are null.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[5]; // mov r5,r12

    // Probe the first node.
    cpu.r[0] = mem.read_u32(0x8c183a6c).unwrap_or(0x8c05b2ac); // mov.l 0x8c183a6c,r0
    cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
    cpu.pr = 0x8c183a0a;
    crate::registry::p_8c05b2ac::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.r[5] = mem.read_u32(0x8c183a70).unwrap_or(0x8c20e178); // mov.l 0x8c183a70,r5
    cpu.r[0] = mem.read_u32(0x8c183a74).unwrap_or(0x8c17fc44); // mov.l 0x8c183a74,r0
    cpu.pr = 0x8c183a14; // jsr delay is nop
    crate::registry::p_8c17fc44::run(cpu, mem);
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            let _ = mem.write_u32(cpu.r[13], cpu.r[12]); // mov.l r12,@r13
        }
    }

    cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r12),r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.t() {
        finish(cpu, mem);
        return;
    }
    loop {
        cpu.r[0] = mem.read_u32(0x8c183a6c).unwrap_or(0x8c05b2ac); // mov.l 0x8c183a6c,r0
        cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
        cpu.pr = 0x8c183a2e;
        crate::registry::p_8c05b2ac::run(cpu, mem);
        cpu.r[4] = cpu.r[0]; // mov r0,r4
        cpu.r[5] = mem.read_u32(0x8c183a70).unwrap_or(0x8c20e178); // mov.l 0x8c183a70,r5
        cpu.r[0] = mem.read_u32(0x8c183a74).unwrap_or(0x8c17fc44); // mov.l 0x8c183a74,r0
        cpu.pr = 0x8c183a38; // jsr delay is nop
        crate::registry::p_8c17fc44::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                let _ = mem.write_u32(cpu.r[13], cpu.r[14]); // mov.l r14,@r13
            }
        }
        cpu.r[12] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r14),r12
        cpu.set_t(cpu.r[12] == 0); // tst r12,r12
        if !cpu.t() {
            loop {
                cpu.r[0] = mem.read_u32(0x8c183a78).unwrap_or(0x8c1839f8); // mov.l 0x8c183a78,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
                cpu.pr = 0x8c183a54;
                crate::registry::p_8c1839f8::run(cpu, mem);
                cpu.r[12] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r12),r12
                cpu.set_t(cpu.r[12] == 0); // tst r12,r12
                if cpu.t() {
                    break;
                }
            }
        }
        cpu.r[14] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r14
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if cpu.t() {
            break;
        }
    }
    finish(cpu, mem);
}

fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
