//! 0x8c0f9aa8 — strcmp-style byte loop (no pseudocode; raw disassembly).
//!
//! ```text
//! 0x8c0f9aa8: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c0f9ab2   ; r3==0 -> return 0
//! rts ; mov #0,r0                                       ; delay slot
//! 0x8c0f9ab2: mov.b @r4,r3 ; mov.b @r5,r1
//! extu.b r3,r3 ; extu.b r3,r0 ; extu.b r1,r1 (x2)
//! add #1,r4 ; cmp/eq r1,r0
//! bt.s 0x8c0f9aa8
//! add #1,r5                                             ; delay slot, always
//! rts ; sub r1,r0                                       ; mismatch: r0 = a - b
//! ```
//! Hidden side effects: r4/r5 both advance one per compared byte (delay slot
//! runs even when bt.s is not taken), r3 = zero-extended a-byte, r1 =
//! zero-extended b-byte, T = 0 on mismatch / 1 on the NUL return.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.r[0] = cpu.r[3] & 0xFF; // extu.b r3,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            cpu.r[0] = 0; // rts delay slot: mov #0,r0
            return;
        }
        // bf taken -> 0x8c0f9ab2
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap() as i8 as i32 as u32; // mov.b @r4,r3
        cpu.r[1] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r1
        cpu.r[3] &= 0xFF; // extu.b r3,r3
        cpu.r[0] = cpu.r[3] & 0xFF; // extu.b r3,r0
        cpu.r[1] &= 0xFF; // extu.b r1,r1 (twice)
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
        cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r1,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot: add #1,r5, always
        if cpu.r[1] != cpu.r[0] {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay slot: sub r1,r0
            return;
        }
    }
}
