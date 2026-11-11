//! 0x8c010d6c — `return PTR_DAT_8c010d78[param_1] & 7;`
//!
//! `mov.l 0x8c010d78,r1 ; extu.b r4,r0 ; mov.b @(r0,r1),r0 ; extu.b r0,r0 ; rts ; and #7,r0`
//! Hidden side effects: r1 is left holding the table base, the byte load
//! sign-extends before `extu.b` re-truncates it, and the mask `& 7` is a
//! delay-slot instruction after `rts` — the C shows it, the delay slot does not.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8C010D78).unwrap();
    cpu.r[0] = cpu.r[4] & 0xFF; // extu.b r4,r0
    let addr = cpu.r[0].wrapping_add(cpu.r[1]);
    let b = mem.read_u8(addr).unwrap();
    cpu.r[0] = (b as i8 as i32) as u32; // mov.b sign-extends
    cpu.r[0] &= 0xFF; // extu.b r0,r0
    cpu.r[0] &= 7; // delay slot of `rts`: and #7,r0
}
