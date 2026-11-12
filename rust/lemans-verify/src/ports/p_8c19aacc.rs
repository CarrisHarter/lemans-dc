//! 0x8c19aacc - copy 0xe4-byte records from `[r4, r5)` into the buffer at r6.
//!
//! `bt.s` on `r12 == r13` skips the loop and its delay slot still copies r6 to
//! r14. Each iteration calls 0x8c19a648(dst = r14, src = r12) only when r14 is
//! non-null, then advances both cursors by 0xe4. Returns r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.set_t(cpu.r[12] == cpu.r[13]); // cmp/eq r13,r12
    cpu.r[14] = cpu.r[6]; // bt.s delay: mov r6,r14
    if !cpu.t() {
        loop {
            cpu.set_t(cpu.r[14] == 0); // tst r14,r14
            if cpu.r[14] != 0 {
                cpu.r[0] = mem.read_u32(0x8c19ab00).unwrap_or(0x8c19a648); // mov.l 0x8c19ab00,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
                cpu.pr = 0x8c19aaea;
                crate::registry::p_8c19a648::run(cpu, mem);
            }
            cpu.r[0] = mem.read_u32(0x8c19ab04).unwrap_or(0xe4); // mov.l 0x8c19ab04,r0
            cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[0]); // add r0,r12
            cpu.set_t(cpu.r[12] == cpu.r[13]); // cmp/eq r13,r12
            cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // bf.s delay: add r0,r14
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
