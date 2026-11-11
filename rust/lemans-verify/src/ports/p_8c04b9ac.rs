//! 0x8c04b9ac — `*(undefined4 *)(DAT_8c04b9b4 + param_1) = param_2;`
//!
//! ```text
//! mov.l 0x8c04b9b4,r0   ; r0 = 0xcc (literal pool)
//! rts
//! mov.l r5,@(r0,r4)     ; delay slot — the store happens after `rts`
//! ```
//! Hidden side effects: the base pointer 0xcc is left in r0 and the word
//! store itself sits in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C04B9B4).unwrap();
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(addr, cpu.r[5]); // delay slot of `rts`
}
