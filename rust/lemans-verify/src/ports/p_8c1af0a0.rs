//! 0x8c1af0a0 - release the four sub-objects of a record, then optionally free it.
//!
//! Null record returns null. Otherwise the sub-objects at `+24` and `+12` are
//! released (guarded by a self-`tst`, so only a pointer equal to `-24`/`-12`
//! skips them), the three handles at `+8`, `+4` and `+0` are released through
//! 0x8c018bd8 with `-1`, and when the sign-extended halfword `r5` is positive the
//! record itself goes to 0x8c01032c. Returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t((cpu.r[13] & cpu.r[13]) == 0); // tst r13,r13
    if !cpu.t() {
        cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
        cpu.set_t((cpu.r[4] & cpu.r[4]) == 0); // tst r4,r4
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1af108).unwrap_or(0x8c1af3b4); // mov.l 0x8c1af108,r0
            cpu.pr = 0x8c1af0ba; // jsr delay is nop
            crate::registry::p_8c1af3b4::run(cpu, mem);
            cpu.r[0] = cpu.r[13].wrapping_add(24); // mov r13,r0 / add #24,r0
            cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
        }
        cpu.r[4] = cpu.r[13].wrapping_add(12); // mov r13,r4 / add #12,r4
        cpu.set_t((cpu.r[4] & cpu.r[4]) == 0); // tst r4,r4
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1af10c).unwrap_or(0x8c1af164); // mov.l 0x8c1af10c,r0
            cpu.pr = 0x8c1af0ce; // jsr delay is nop
            crate::registry::p_8c1af164::run(cpu, mem);
            cpu.r[0] = cpu.r[13].wrapping_add(12); // mov r13,r0 / add #12,r0
            cpu.set_t((cpu.r[0] & cpu.r[0]) == 0); // tst r0,r0
        }
        cpu.r[0] = mem.read_u32(0x8c1af110).unwrap_or(0x8c018bd8); // mov.l 0x8c1af110,r0
        cpu.r[4] = cpu.r[13].wrapping_add(8); // mov r13,r4 / add #8,r4
        cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
        cpu.pr = 0x8c1af0de;
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = mem.read_u32(0x8c1af110).unwrap_or(0x8c018bd8); // mov.l 0x8c1af110,r0
        cpu.r[4] = cpu.r[13].wrapping_add(4); // mov r13,r4 / add #4,r4
        cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
        cpu.pr = 0x8c1af0e8;
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = mem.read_u32(0x8c1af110).unwrap_or(0x8c018bd8); // mov.l 0x8c1af110,r0
        cpu.r[4] = cpu.r[13]; // mov r13,r4
        cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
        cpu.pr = 0x8c1af0f0;
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = cpu.r[14] << 16 >> 16; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1af114).unwrap_or(0x8c01032c); // mov.l 0x8c1af114,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c1af0fc;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
