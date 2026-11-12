//! 0x8c059a92 — byte-wise string compare ending at the first NUL.
//!
//! ```text
//! entry 0x8c059a92:
//!   mov.b  @r4,r3 ; mov.b @r5,r0
//!   extu.b r3,r3 ; extu.b r3,r1 ; extu.b r0,r0 ; extu.b r0,r0
//!   add    #1,r4
//!   cmp/eq r0,r1
//!   bt.s   0x8c059a88
//!   add    #1,r5          ; delay slot, taken and not taken
//!   sub    r0,r1          ; not taken: bytes differ
//!   tst    r1,r1
//!   rts
//!   movt   r0             ; delay slot
//! loop test 0x8c059a88:
//!   extu.b r3,r0
//!   tst    r0,r0
//!   bf     0x8c059a92     ; left byte nonzero: keep comparing
//!   bra    0x8c059aa8     ; both bytes were NUL: equal
//!   mov    #0,r1          ; delay slot
//! ```
//!
//! Entry is 0x8c059a92 but the `bt.s` loop-back target 0x8c059a88 lies below
//! it, so a run can execute code outside the nominal entry block. The two
//! `mov.b` loads sign-extend, then `extu.b` truncates: r3 ends holding the
//! zero-extended left byte. Mismatch returns r0 = 0 with r1 = left - right
//! (both truncated bytes); a NUL match returns r0 = 1 with r1 = 0. r4/r5 end
//! one past the compared bytes and T is r0's low bit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        // 0x8c059a92: mov.b @r4,r3 ; mov.b @r5,r0, then extu.b on both.
        // The sign extension is undone before any use, so keep the bytes.
        let left = mem.read_u8(cpu.r[4]).unwrap();
        let right = mem.read_u8(cpu.r[5]).unwrap();
        cpu.r[3] = u32::from(left);
        cpu.r[0] = u32::from(right);
        cpu.r[1] = cpu.r[3]; // extu.b r3,r1
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
        cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1
        cpu.r[5] = cpu.r[5].wrapping_add(1); // delay slot of bt.s
        if cpu.t() {
            // bt.s 0x8c059a88
            cpu.r[0] = cpu.r[3] & 0xFF; // extu.b r3,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if !cpu.t() {
                continue; // bf 0x8c059a92
            }
            // Both bytes equal NUL: bra 0x8c059aa8, delay `mov #0,r1`.
            cpu.r[1] = 0;
            cpu.set_t(true); // tst r1,r1
            cpu.r[0] = u32::from(cpu.t()); // movt r0 (delay slot of rts)
            return;
        }
        // Bytes differ: sub r0,r1 ; tst r1,r1 ; rts ; movt r0.
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
        cpu.set_t(cpu.r[1] == 0);
        cpu.r[0] = u32::from(cpu.t()); // movt r0 (delay slot of rts)
        return;
    }
}
