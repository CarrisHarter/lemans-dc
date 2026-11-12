//! 0x8c146ca0 - release the 40 handle slots at `r4`, then free the table.
//!
//! Walks `i = 0..39` (the loop counter runs to 40 against a signed `cmp/gt`) and,
//! for each non-null `[r10 + 4i]`, calls 0x8c146e8c with `r5 = 1`, zeroes the
//! slot and bumps r14 (which is otherwise unused). A positive `exts.w(r11)` then
//! frees the table. Returns r10.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[11] = cpu.r[5]; // mov r5,r11
    cpu.r[10] = cpu.r[4]; // mov r4,r10
    cpu.set_t(cpu.r[10] == 0); // tst r10,r10
    if cpu.r[10] != 0 {
        cpu.r[14] = 0; // mov #0,r14
        cpu.r[13] = cpu.r[14]; // mov r14,r13
        cpu.r[12] = cpu.r[14]; // mov r14,r12
        loop {
            cpu.r[0] = cpu.r[12]; // mov r12,r0
            cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r10),r4
            cpu.set_t(cpu.r[4] == 0); // tst r4,r4
            if cpu.r[4] != 0 {
                cpu.r[0] = mem.read_u32(0x8c146cf8).unwrap_or(0x8c146e8c); // mov.l 0x8c146cf8,r0
                cpu.r[5] = 1; // jsr delay: mov #1,r5
                cpu.pr = 0x8c146cc8;
                crate::registry::p_8c146e8c::run(cpu, mem);
                cpu.r[0] = cpu.r[12]; // mov r12,r0
                cpu.r[1] = 0; // mov #0,r1
                let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r10)
                cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
            }
            cpu.r[13] = cpu.r[13].wrapping_add(1); // add #1,r13
            cpu.r[0] = 40; // mov #40,r0
            cpu.set_t((cpu.r[0] as i32) > (cpu.r[13] as i32)); // cmp/gt r13,r0
            cpu.r[12] = cpu.r[12].wrapping_add(4); // bt.s delay: add #4,r12
            if !cpu.t() {
                break;
            }
        }
        cpu.r[0] = cpu.r[11] as i16 as i32 as u32; // exts.w r11,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c146cfc).unwrap_or(0x8c01032c); // mov.l 0x8c146cfc,r0
            cpu.r[4] = cpu.r[10]; // jsr delay: mov r10,r4
            cpu.pr = 0x8c146ce6;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[10]; // mov r10,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
