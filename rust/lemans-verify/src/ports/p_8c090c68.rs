//! 0x8c090c68 — clear a global slot and two bytes of an object, then free it if asked.
//!
//! Null r4 exits with only r0 = 0. Otherwise: `*0x8c26c350 = 0`, `r4[0x3a4] = 0`,
//! T = `exts.w(r5) > 0`, and the `bf.s` delay slot writes `r4[0x3a5] = 0` **on
//! both outcomes**. A positive low half then frees r4 through the thunk at
//! 0x8c01032c. `r0 = r14` at the end; r14 is pushed, so it ends at its entry
//! value. r1 keeps `exts.w(r5)`, r3 is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r14
    cpu.set_t(object == 0); // tst r14,r14
    if object != 0 {
        cpu.r[0] = mem.read_u32(0x8c090c94).unwrap_or(0x8c26c350);
        cpu.r[3] = 0; // mov #0,r3
        let count = (cpu.r[5] as u16 as i16) as i32; // exts.w r5,r1
        cpu.r[1] = count as u32;
        let _ = mem.write_u32(cpu.r[0], cpu.r[3]); // mov.l r3,@r0
        cpu.set_t(count > 0); // cmp/pl r1
        cpu.r[0] = mem.read_u32(0x8c090c98).unwrap_or(0x3a4);
        let _ = mem.write_u8(object.wrapping_add(cpu.r[0]), cpu.r[3] as u8); // mov.b r3,@(r0,r14)
        cpu.r[0] = mem.read_u32(0x8c090c9c).unwrap_or(0x3a5);
        let _ = mem.write_u8(object.wrapping_add(cpu.r[0]), cpu.r[3] as u8); // bf.s delay slot
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c090ca0).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c090c8c;
            crate::registry::p_8c01032c::run(cpu, mem); // jsr @r0 ; nop
        }
    }
    cpu.r[0] = object; // mov r14,r0
}
