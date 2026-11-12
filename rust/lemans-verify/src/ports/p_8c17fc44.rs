//! 0x8c17fc44 - substring search: does `r5` occur in `r4`?
//!
//! `n = strlen(r4) - strlen(r5) + 1`; for `i` in `0..n` (signed `cmp/gt`), it
//! calls 0x8c070ac8 with `r4 = r4 + i`, `r5 = needle`, `r6 = strlen(needle)` and
//! returns 1 on the first zero result. Falls out with 0 when nothing matches.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[0] = mem.read_u32(0x8c17fc98).unwrap_or(0x8c0112f4); // mov.l 0x8c17fc98,r0
    cpu.pr = 0x8c17fc5a; // jsr delay is nop
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[14] = cpu.r[0]; // mov r0,r14
    cpu.r[0] = mem.read_u32(0x8c17fc98).unwrap_or(0x8c0112f4); // mov.l 0x8c17fc98,r0
    cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
    cpu.pr = 0x8c17fc62;
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[14] = cpu.r[14].wrapping_sub(cpu.r[0]); // sub r0,r14
    cpu.r[11] = cpu.r[0]; // mov r0,r11
    cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
    cpu.r[10] = 0; // bra delay: mov #0,r10
    let mut result = 0u32;
    loop {
        cpu.set_t((cpu.r[14] as i32) > (cpu.r[10] as i32)); // cmp/gt r10,r14
        if !cpu.t() {
            break;
        }
        cpu.r[0] = mem.read_u32(0x8c17fc9c).unwrap_or(0x8c070ac8); // mov.l 0x8c17fc9c,r0
        cpu.r[4] = cpu.r[12]; // mov r12,r4
        cpu.r[6] = cpu.r[11]; // mov r11,r6
        cpu.r[5] = cpu.r[13]; // mov r13,r5
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[10]); // jsr delay: add r10,r4
        cpu.pr = 0x8c17fc78;
        crate::registry::p_8c070ac8::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            result = 1; // bra delay: mov #1,r0
            break;
        }
        cpu.r[10] = cpu.r[10].wrapping_add(1); // add #1,r10
    }
    cpu.r[0] = result; // 0 when the loop falls through
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
