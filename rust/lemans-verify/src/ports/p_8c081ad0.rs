//! 0x8c081ad0 — reverse scan for a byte in a length-prefixed buffer.
//!
//! `r4` points to a struct whose first word points at a header; the header's
//! first word is a byte count and its word at `+0xc` is the buffer address.
//! The scan starts at `min(r6, count-1)` and walks down to index 0, returning
//! the index of the first byte equal to `r5`'s low byte, or `-1`. Both callee
//! calls (0x8c01ec2c) reload the header to fetch that buffer address.
//!
//! Note the order of the loop tests: the byte is loaded and compared before
//! the `r4 == buffer` termination check, so a byte at index 0 can match even
//! though the decompiled C would bail first. The asm is authoritative.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 / mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5]; // mov r5,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[11] = cpu.r[4]; // mov r4,r11
    cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r4
    cpu.r[13] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r13

    if cpu.r[13] == 0 {
        // tst r13,r13 ; bf 0x8c081aec — zero count takes the early exit.
        cpu.set_t(true);
        cpu.r[0] = 0xffff_ffff; // bra 0x8c081b32 ; mov #-1,r0
    } else {
        cpu.set_t(false);
        cpu.r[0] = 0xffff_ffff; // mov #-1,r0
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]); // add r0,r13
        cpu.set_t(cpu.r[14] > cpu.r[13]); // cmp/hi r13,r14 ; bf 0x8c081af6
        if cpu.t() {
            cpu.r[14] = cpu.r[13]; // mov r13,r14 — clamp r6 to count-1
        }
        cpu.r[0] = mem.read_u32(0x8c081b40).unwrap_or(0); // literal: 0x8c01ec2c
        cpu.pr = 0x8c081afc; // jsr @r0 ; nop
        crate::registry::p_8c01ec2c::run(cpu, mem);

        cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r4
        cpu.r[13] = cpu.r[13].wrapping_sub(cpu.r[14]); // sub r14,r13
        cpu.r[1] = mem.read_u32(0x8c081b40).unwrap_or(0); // literal reload
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r3
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // add r3,r0
        cpu.r[14] = cpu.r[0]; // mov r0,r14
        cpu.pr = 0x8c081b0c; // jsr @r1
        cpu.r[14] = cpu.r[14].wrapping_sub(cpu.r[13]); // delay slot: sub r13,r14
        crate::registry::p_8c01ec2c::run(cpu, mem);

        cpu.set_t(cpu.r[14] == cpu.r[0]); // cmp/eq r0,r14 ; bt 0x8c081b30
        if cpu.t() {
            cpu.r[0] = 0xffff_ffff; // mov #-1,r0
        } else {
            cpu.r[1] = (cpu.r[12] as u8 as i8 as i32) as u32; // exts.b r12,r1
            loop {
                cpu.r[4] = cpu.r[14]; // mov r14,r4
                cpu.r[4] = cpu.r[4].wrapping_sub(1); // add #-1,r4
                let byte = mem.read_u8(cpu.r[4]).unwrap_or(0);
                cpu.r[3] = (byte as i8 as i32) as u32; // mov.b @r4,r3 ; exts.b r3,r3
                cpu.set_t(cpu.r[3] == cpu.r[1]); // cmp/eq r1,r3
                if cpu.t() {
                    cpu.r[0] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r0
                    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0);
                    cpu.r[14] = cpu.r[14].wrapping_sub(cpu.r[0]); // sub r0,r14
                    cpu.r[14] = cpu.r[14].wrapping_sub(1); // add #-1,r14
                    cpu.r[0] = cpu.r[14]; // bra 0x8c081b32 ; mov r14,r0
                    break;
                }
                cpu.set_t(cpu.r[4] == cpu.r[0]); // cmp/eq r0,r4 ; bf.s 0x8c081b12
                if cpu.t() {
                    cpu.r[0] = 0xffff_ffff; // fall through: mov #-1,r0
                    break;
                }
                cpu.r[14] = cpu.r[4]; // delay slot: mov r4,r14
            }
        }
    }

    // lds.l @r15+,pr / mov.l @r15+,r11 / r12 / r13 / rts ; mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
