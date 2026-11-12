//! 0x8c09035c — tear down an object: clear a global, hand its node list to
//! 0x8c090ac0, release the embedded handle at +0x190, then free the object when
//! the low half of r5 is positive.
//!
//! Null r4 exits with r0 = 0. Non-null: `*0x8c26c348 = 0`; if `[r4+0x1c0]` is
//! non-null, call `0x8c090ac0([*[r4+0x1c0]], r4+4)` (both arguments prepared in
//! front of the `jsr`, the second in its delay slot); then call `0x8c061064` on
//! `r4+0x190` with `r5 = -1` (its guard declines, so it only clears); then
//! `exts.w(r5) > 0` decides the final free of r4 through 0x8c01032c. r0 = r13
//! at exit; r13/r14 are pushed and end at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r13
    let count_reg = cpu.r[5]; // mov r5,r14
    cpu.set_t(object == 0); // tst r13,r13
    if object != 0 {
        cpu.r[1] = mem.read_u32(0x8c0903a4).unwrap_or(0x8c26c348);
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u32(cpu.r[1], 0); // mov.l r0,@r1

        cpu.r[0] = mem.read_u32(0x8c0903a8).unwrap_or(0x1c0);
        cpu.r[0] = mem.read_u32(object.wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r13),r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r4
            cpu.r[5] = object; // mov r13,r5
            cpu.r[0] = mem.read_u32(0x8c0903ac).unwrap_or(0x8c090ac0);
            cpu.r[5] = cpu.r[5].wrapping_add(4); // delay slot: add #4,r5
            cpu.pr = 0x8c090382;
            crate::registry::p_8c090ac0::run(cpu, mem);
        }

        cpu.r[0] = mem.read_u32(0x8c0903b0).unwrap_or(0x190);
        cpu.r[4] = object.wrapping_add(cpu.r[0]); // mov r13,r4 ; add r0,r4
        cpu.r[0] = mem.read_u32(0x8c0903b4).unwrap_or(0x8c061064);
        cpu.r[5] = u32::MAX; // delay slot: mov #-1,r5
        cpu.pr = 0x8c09038e;
        crate::registry::p_8c061064::run(cpu, mem);

        let count = (count_reg as u16 as i16) as i32; // exts.w r14,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0903b8).unwrap_or(0x8c01032c);
            cpu.r[4] = object; // delay slot: mov r13,r4
            cpu.pr = 0x8c09039a;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = object; // mov r13,r0
}
