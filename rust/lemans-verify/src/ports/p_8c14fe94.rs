//! 0x8c14fe94 - stamp a vtable word, tear down a sub-record, free both.
//!
//! `[r12] = 0x8c1ef390`, then `r14 = [r12+4]`. When the sub-record exists its
//! `+24` slot is released through 0x8c13844c and 0x8c043bac, its `+12` slot
//! through 0x8c043a68 and 0x8c01e460, and the record itself through the same
//! pair before 0x8c01032c frees it. A positive `exts.w(r13)` then frees the
//! container. The doubled `tst` pairs at 0x8c14fed0/0x8c14fee8 only re-test the
//! same register, so they change nothing. Returns r12.

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
        cpu.r[0] = mem.read_u32(0x8c14ff1c).unwrap_or(0x8c1ef390); // mov.l 0x8c14ff1c,r0
        let _ = mem.write_u32(cpu.r[12], cpu.r[0]); // mov.l r0,@r12
        cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r12),r14
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if cpu.r[14] != 0 {
            cpu.r[4] = cpu.r[14].wrapping_add(24); // mov r14,r4 ; add #24,r4
            cpu.set_t(cpu.r[4] == 0); // tst r4,r4
            if cpu.r[4] != 0 {
                cpu.r[0] = mem.read_u32(0x8c14ff20).unwrap_or(0x8c13844c); // mov.l 0x8c14ff20,r0
                cpu.pr = 0x8c14febc;
                crate::registry::p_8c13844c::run(cpu, mem);
                cpu.r[4] = cpu.r[14].wrapping_add(24); // mov r14,r4 ; add #24,r4
                cpu.set_t(cpu.r[4] == 0); // tst r4,r4
                if cpu.r[4] != 0 {
                    cpu.r[0] = mem.read_u32(0x8c14ff24).unwrap_or(0x8c043bac); // mov.l 0x8c14ff24,r0
                    cpu.r[5] = 0; // jsr delay: mov #0,r5
                    cpu.pr = 0x8c14fec4;
                    crate::registry::p_8c043bac::run(cpu, mem);
                }
            }
            cpu.r[4] = cpu.r[14].wrapping_add(12); // mov r14,r4 ; add #12,r4
            cpu.set_t(cpu.r[4] == 0); // tst r4,r4 (doubled in the image)
            cpu.set_t(cpu.r[4] == 0); // tst r4,r4
            if cpu.r[4] != 0 {
                cpu.r[0] = mem.read_u32(0x8c14ff28).unwrap_or(0x8c043a68); // mov.l 0x8c14ff28,r0
                cpu.pr = 0x8c14fedc;
                crate::registry::p_8c043a68::run(cpu, mem);
                cpu.r[0] = mem.read_u32(0x8c14ff2c).unwrap_or(0x8c01e460); // mov.l 0x8c14ff2c,r0
                cpu.r[4] = cpu.r[14].wrapping_add(12); // mov r14,r4 ; add #12,r4
                cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
                cpu.pr = 0x8c14fee6;
                crate::registry::p_8c01e460::run(cpu, mem);
            }
            cpu.set_t(cpu.r[14] == 0); // tst r14,r14 (doubled in the image)
            cpu.set_t(cpu.r[14] == 0); // tst r14,r14
            if cpu.r[14] != 0 {
                cpu.r[0] = mem.read_u32(0x8c14ff28).unwrap_or(0x8c043a68); // mov.l 0x8c14ff28,r0
                cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
                cpu.pr = 0x8c14fef4;
                crate::registry::p_8c043a68::run(cpu, mem);
                cpu.r[0] = mem.read_u32(0x8c14ff2c).unwrap_or(0x8c01e460); // mov.l 0x8c14ff2c,r0
                cpu.r[4] = cpu.r[14]; // mov r14,r4
                cpu.r[5] = 0xffff_ffff; // jsr delay: mov #-1,r5
                cpu.pr = 0x8c14fefc;
                crate::registry::p_8c01e460::run(cpu, mem);
            }
            cpu.r[0] = mem.read_u32(0x8c14ff30).unwrap_or(0x8c01032c); // mov.l 0x8c14ff30,r0
            cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
            cpu.pr = 0x8c14ff02;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w r13,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c14ff30).unwrap_or(0x8c01032c); // mov.l 0x8c14ff30,r0
            cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
            cpu.pr = 0x8c14ff0e;
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
