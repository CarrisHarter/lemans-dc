//! 0x8c14e708 - copy `[r4, r5)` 48-byte records into the buffer at r6.
//!
//! `bt.s` on `r12 == r13` skips the loop and its delay slot still copies r6 to
//! r14. Each iteration calls 0x8c14cfac(dst = r14, src = r12) only when r14 is
//! non-null, then advances both cursors by 48. Returns r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut saved = [0u32; 4];
    for (i, reg) in [14usize, 13, 12].iter().enumerate() {
        saved[i] = cpu.r[*reg];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[*reg]);
    }
    saved[3] = cpu.pr;
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
                cpu.r[0] = mem.read_u32(0x8c14e73c).unwrap_or(0x8c14cfac); // mov.l 0x8c14e73c,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
                cpu.pr = 0x8c14e726;
                crate::registry::p_8c14cfac::run(cpu, mem);
            }
            cpu.r[12] = cpu.r[12].wrapping_add(48); // add #48,r12
            cpu.set_t(cpu.r[12] == cpu.r[13]); // cmp/eq r13,r12
            cpu.r[14] = cpu.r[14].wrapping_add(48); // bf.s delay: add #48,r14
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for (i, reg) in [12usize, 13].iter().enumerate() {
        cpu.r[*reg] = mem.read_u32(cpu.r[15]).unwrap_or(saved[i + 1]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
