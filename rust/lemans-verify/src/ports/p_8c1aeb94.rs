//! 0x8c1aeb94 — byte-string compare loop.
//!
//! ```text
//! 0x8c1aeb94: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c1aeb9e
//!             rts ; mov #0,r0            ; r3&0xff == 0 -> return 0, T=1
//! 0x8c1aeb9e: mov.b @r4,r3 ; mov.b @r5,r1 ; extu.b r3,r3 ; extu.b r3,r0
//!             extu.b r1,r1 ; extu.b r1,r1 ; add #1,r4
//!             cmp/eq r1,r0 ; bt.s 0x8c1aeb94 ; add #1,r5   ; delay runs always
//!             rts ; sub r1,r0            ; rts delay: r0 = a - b, T=0
//! ```
//! The `add #1,r5` delay slot of `bt.s` executes on both paths, so r4 and
//! r5 always advance together. r3 ends as the last byte read from r4,
//! r1 as the last from r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.r[0] = cpu.r[3] & 0xFF; // extu.b r3,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            cpu.r[0] = 0; // rts delay slot: mov #0,r0
            return;
        }
        cpu.r[3] = (mem.read_u8(cpu.r[4]).unwrap() as i8 as i32) as u32; // mov.b @r4,r3
        cpu.r[1] = (mem.read_u8(cpu.r[5]).unwrap() as i8 as i32) as u32; // mov.b @r5,r1
        cpu.r[3] &= 0xFF; // extu.b r3,r3
        cpu.r[0] = cpu.r[3]; // extu.b r3,r0
        cpu.r[1] &= 0xFF; // extu.b r1,r1 (x2)
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0 -> T = (r0 == r1)
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot, both paths
        if cpu.r[0] == cpu.r[1] {
            continue; // bt.s taken
        }
        cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay slot: sub r1,r0
        return;
    }
}
