//! 0x8c04b420 — `*(undefined1 *)(DAT_8c04b428 + param_1) = param_2;`
//!
//! ```text
//! mov.l 0x8c04b428,r0   ; r0 = 0x98 (literal pool)
//! rts
//! mov.b r5,@(r0,r4)     ; delay slot — the store happens after `rts`
//! ```
//! Hidden side effects: the base pointer 0x98 is left in r0 and the byte
//! store itself sits in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C04B428).unwrap();
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u8(addr, cpu.r[5] as u8); // delay slot of `rts`
}
