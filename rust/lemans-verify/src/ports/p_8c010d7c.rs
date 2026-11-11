//! 0x8c010d7c — `return PTR_DAT_8c010d88[param_1] & 0x40;`
//!
//! `mov.l 0x8c010d88,r1 ; extu.b r4,r0 ; mov.b @(r0,r1),r0 ; extu.b r0,r0 ; rts ; and #64,r0`
//! Same shape as 0x8c010d6c: r1 keeps the table base, and the `& 0x40`
//! mask runs in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8C010D88).unwrap();
    cpu.r[0] = cpu.r[4] & 0xFF; // extu.b r4,r0
    let addr = cpu.r[0].wrapping_add(cpu.r[1]);
    let b = mem.read_u8(addr).unwrap();
    cpu.r[0] = (b as i8 as i32) as u32; // mov.b sign-extends
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.r[0] &= 0x40; // delay slot of `rts`: and #64,r0
}
