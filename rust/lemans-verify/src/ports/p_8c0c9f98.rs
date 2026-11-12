//! 0x8c0c9f98 — free ten pointer slots, clear them, then free the array.
//!
//! Null r4 exits after the prologue (`tst` leaves T=1). Otherwise ten iterations
//! free `[r4+4i]` (the load is the `jsr` delay slot) and store 0 over it; the
//! `bt.s` delay advances the cursor by 4 even on the final not-taken test. Then
//! `[0x8c27a6e0] = 0` runs on both arms of the `bf.s`, and a positive
//! `exts.w r5` finally frees the array. r11..r14 are pushed, so they end at
//! entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let array = cpu.r[4]; // mov r4,r11
    let flag_reg = cpu.r[5]; // mov r5,r12
    cpu.set_t(array == 0); // tst r11,r11
    if array != 0 {
        let mut cursor = array; // mov r11,r13
        for _ in 0..10 {
            cpu.r[0] = mem.read_u32(0x8c0c9fe0).unwrap_or(0x8c01032c); // mov.l 0x8c0c9fe0,r0
            cpu.r[4] = mem.read_u32(cursor).unwrap_or(0); // delay slot: mov.l @r13,r4
            cpu.pr = 0x8c0c9fc2;
            crate::registry::p_8c01032c::run(cpu, mem);
            cpu.r[3] = 0; // mov #0,r3
            let _ = mem.write_u32(cursor, cpu.r[3]); // mov.l r3,@r13
            cursor = cursor.wrapping_add(4); // bt.s delay slot: add #4,r13
        }
        cpu.r[1] = mem.read_u32(0x8c0c9fe4).unwrap_or(0x8c27a6e0); // mov.l 0x8c0c9fe4,r1
        let count = (flag_reg as u16 as i16) as i32; // exts.w r12,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[1], cpu.r[3]); // bf.s delay slot: both outcomes
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0c9fe0).unwrap_or(0x8c01032c);
            cpu.r[4] = array; // delay slot: mov r11,r4
            cpu.pr = 0x8c0c9fd2;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = array; // mov r11,r0
}
