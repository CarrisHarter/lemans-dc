//! 0x8c059a88 — byte-compare loop over (r4, r5), seeded by r3.
//!
//! `extu.b r3,r0 ; tst r0,r0 ; bf L1 ; bra L2 ; mov #0,r1`
//! `L1: mov.b @r4,r3 ; mov.b @r5,r0 ; extu.b r3,r3 ; extu.b r3,r1 ; extu.b r0,r0 ; extu.b r0,r0`
//! `    add #1,r4 ; cmp/eq r0,r1 ; bt.s L0 ; add #1,r5 ; sub r0,r1`
//! `L2: tst r1,r1 ; rts ; movt r0`
//! The C shows none of it: r4/r5 are permanently advanced past every byte
//! compared (r5 in the `bt.s` delay slot, so it advances even on mismatch),
//! r3 keeps the last byte read from r4, r1 ends as the byte difference (or 0
//! on the NUL exit), and r0 = T of the final `tst r1,r1` (1 = r4 string
//! ended, 0 = mismatch).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        // loop head 0x8c059a88: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c059a92
        let head = cpu.r[3] & 0xFF;
        cpu.set_t(head == 0);
        if head != 0 {
            // 0x8c059a92: compare one byte pair
            cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap() as u32; // mov.b @r4,r3
            cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap() as u32; // mov.b @r5,r0
            cpu.r[3] &= 0xFF; // extu.b r3,r3
            cpu.r[1] = cpu.r[3] & 0xFF; // extu.b r3,r1
            cpu.r[0] &= 0xFF; // extu.b r0,r0 (twice, idempotent)
            cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
            cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
            cpu.r[5] = cpu.r[5].wrapping_add(1); // delay slot of `bt.s` — always runs
            if cpu.t() {
                continue; // bt.s 0x8c059a88
            }
            cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]); // sub r0,r1
            break;
        }
        // bf not taken: bra 0x8c059aa8 with `mov #0,r1` in its delay slot
        cpu.r[1] = 0;
        break;
    }
    // 0x8c059aa8: tst r1,r1 ; rts ; movt r0 (delay slot)
    cpu.set_t(cpu.r[1] == 0);
    cpu.r[0] = cpu.t() as u32;
}
