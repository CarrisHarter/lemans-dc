//! 0x8c1261d4 — allocate-ish init: publishes param_1 to a global, writes
//! param_2, a flag byte and a global-sourced word into the struct, returns it.
//!
//! ```text
//! mov.l 0x8c1261e8,r0   ; r0 = 0x8c2ab5e0 (global slot, literal pool)
//! mov.l r4,@r0          ; *0x8c2ab5e0 = param_1
//! mov.l r5,@r4          ; param_1[0] = param_2
//! mov   #1,r0
//! mov.l 0x8c1261ec,r1   ; r1 = 0x8c206c28 (literal pool)
//! mov.b r0,@(4,r4)      ; param_1[1] (byte) = 1
//! mov.l @r1,r1          ; r1 = *0x8c206c28
//! mov   r4,r0
//! rts
//! mov.l r1,@(8,r4)      ; delay slot: param_1[2] = r1
//! ```
//! Hidden side effects: the store to the global at 0x8c2ab5e0, the byte store
//! at r4+4, the rts-delay store at r4+8, r0 = r4 and r1 = *0x8c206c28.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let slot = mem.read_u32(0x8C1261E8).unwrap(); // literal: 0x8c2ab5e0
    let _ = mem.write_u32(slot, cpu.r[4]); // mov.l r4,@r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]); // mov.l r5,@r4
    cpu.r[0] = 1; // mov #1,r0
    let ptr = mem.read_u32(0x8C1261EC).unwrap(); // literal: 0x8c206c28
    let _ = mem.write_u8(cpu.r[4].wrapping_add(4), 1); // mov.b r0,@(4,r4)
    let r1 = mem.read_u32(ptr).unwrap(); // mov.l @r1,r1
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), r1); // delay slot of rts
    cpu.r[1] = r1;
}
