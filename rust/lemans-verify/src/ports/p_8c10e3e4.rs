//! 0x8c10e3e4 - release a record's nine sub-handles, free the record, then free
//! the container when the count is positive.
//!
//! `r12` is the container; `r14 = [r12]` is the record. Nine releases go to
//! 0x8c018bd8 with `r5 = -1` at `r14+36, +32, +28, +24, +20, +16, +12, +8, +4`,
//! then 0x8c01032c frees the record. A null record skips straight to the count
//! gate; a positive `exts.w(r13)` then frees the container. Returns r12.

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
    cpu.set_t(cpu.r[12] == 0); // tst r12,r12
    if cpu.r[12] != 0 {
        cpu.r[14] = mem.read_u32(cpu.r[12]).unwrap_or(0); // mov.l @r12,r14
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if cpu.r[14] != 0 {
            for (off, ret) in [
                (36u32, 0x8c10e404u32),
                (32, 0x8c10e40e),
                (28, 0x8c10e418),
                (24, 0x8c10e422),
                (20, 0x8c10e42c),
                (16, 0x8c10e436),
                (12, 0x8c10e440),
                (8, 0x8c10e44a),
                (4, 0x8c10e454),
            ] {
                cpu.r[0] = mem.read_u32(0x8c10e474).unwrap_or(0x8c018bd8); // mov.l 0x8c10e474,r0
                cpu.r[4] = cpu.r[14].wrapping_add(off); // mov r14,r4 ; add #off,r4
                cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
                cpu.pr = ret;
                crate::registry::p_8c018bd8::run(cpu, mem);
            }
            cpu.r[0] = mem.read_u32(0x8c10e478).unwrap_or(0x8c01032c); // mov.l 0x8c10e478,r0
            cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
            cpu.pr = 0x8c10e45a;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w r13,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c10e478).unwrap_or(0x8c01032c); // mov.l 0x8c10e478,r0
            cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
            cpu.pr = 0x8c10e466;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[12]; // mov r12,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
