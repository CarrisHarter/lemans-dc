//! 0x8c216040 — bit-packing writer: emits r1 = r6<<6 outer iterations of bytes
//! to *r5, consuming r4 bits of r6 (MSB first) per byte.
//!
//! ```text
//! mov  r6,r1 ; shll2 r1 ; shll2 r1 ; bra 0x8c216082 ; shll2 r1 (delay)
//!        => r1 = r6 << 6
//! mov.l r12,@-r15 ; mov.l r11,@-r15   (saved/restored: r11/r12/r15 net unchanged)
//! mov #0,r11 ; mov #-1,r12 ; mov #8,r7
//! outer (0x8c216054):
//!   cmp/ge r7,r4 ; bf 0x8c216060
//!   mov.b r11,@r5 ; add #1,r5 ; bra 0x8c216080 (skips 0x8c21607e) ; add #-8,r4 (delay)
//!   0x8c216060: cmp/pl r4 ; bf 0x8c21607c
//!   mov r7,r0
//!   inner (0x8c216066): shll r6 ; cmp/pl r4 ; bf 0x8c216070
//!     bra 0x8c216074 ; add #-1,r4 (delay)
//!     0x8c216070: mov #1,r2 ; or r2,r6
//!     0x8c216074: dt r0 ; bf 0x8c216066
//!   bra 0x8c21607e ; mov.b r6,@r5 (delay)
//!   0x8c21607c: mov.b r12,@r5
//!   0x8c21607e: add #1,r5 ; 0x8c216080: add #-1,r1 ; 0x8c216082: cmp/pl r1 ; bt outer
//! mov.l @r15+,r11 ; mov #0,r0 ; rts ; mov.l @r15+,r12 (delay)
//! ```
//! The pushed r11/r12 are restored by the pops, so only the temporaries matter.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // r1 = r6 << 6 (three shll2: two inline, one in the bra delay slot)
    cpu.r[1] = cpu.r[6] << 6;
    let zero = 0u32; // mov #0,r11
    let ff = (-1i32) as u32; // mov #-1,r12
    cpu.r[7] = 8; // mov #8,r7

    loop {
        // 0x8c216082: cmp/pl r1 ; bt 0x8c216054 — tested before the first
        // byte is emitted, so r1 <= 0 leaves memory untouched.
        let t = (cpu.r[1] as i32) > 0;
        cpu.set_t(t);
        if !t {
            break;
        }
        // 0x8c216054: cmp/ge r7,r4  (T = r4 >= 8, signed)
        if (cpu.r[4] as i32) >= (cpu.r[7] as i32) {
            // mov.b r11,@r5 ; add #1,r5 ; bra 0x8c216080 (skips 0x8c21607e)
            let _ = mem.write_u8(cpu.r[5], zero as u8);
            cpu.r[5] = cpu.r[5].wrapping_add(1);
            cpu.r[4] = (cpu.r[4] as i32).wrapping_sub(8) as u32;
        } else if (cpu.r[4] as i32) > 0 {
            // 0x8c216060 taken: cmp/pl r4 was true; mov r7,r0
            cpu.r[0] = cpu.r[7]; // r0 = 8
            loop {
                // 0x8c216066: shll r6 (T = bit out, overwritten below)
                cpu.r[6] <<= 1;
                // 0x8c216068: cmp/pl r4 ; bf 0x8c216070
                if (cpu.r[4] as i32) > 0 {
                    // bra 0x8c216074 ; delay add #-1,r4
                    cpu.r[4] = (cpu.r[4] as i32).wrapping_sub(1) as u32;
                } else {
                    // 0x8c216070: mov #1,r2 ; or r2,r6
                    cpu.r[2] = 1;
                    cpu.r[6] |= cpu.r[2];
                }
                // 0x8c216074: dt r0
                cpu.r[0] = cpu.r[0].wrapping_sub(1);
                let t = cpu.r[0] == 0;
                cpu.set_t(t);
                if !t {
                    continue; // bf 0x8c216066
                }
                break;
            }
            // bra 0x8c21607e ; delay mov.b r6,@r5
            let _ = mem.write_u8(cpu.r[5], cpu.r[6] as u8);
            // 0x8c21607e: add #1,r5
            cpu.r[5] = cpu.r[5].wrapping_add(1);
        } else {
            // 0x8c21607c: mov.b r12,@r5 ; 0x8c21607e: add #1,r5
            let _ = mem.write_u8(cpu.r[5], ff as u8);
            cpu.r[5] = cpu.r[5].wrapping_add(1);
        }
        // 0x8c216080: add #-1,r1 ; then the cmp/pl at the loop head.
        cpu.r[1] = cpu.r[1].wrapping_sub(1);
    }
    // mov.l @r15+,r11 ; mov #0,r0 ; rts ; delay mov.l @r15+,r12
    cpu.r[0] = 0;
}
