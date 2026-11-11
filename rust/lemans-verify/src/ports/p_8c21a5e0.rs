//! 0x8c21a5e0 — expand 1536 packed flag bytes (r5) into 192 output bytes
//! (r4 += 0xba first): for each group of 6 output bytes, 8 input bytes are
//! read; input bit 3 of byte k sets bit (0x80>>k) of the output byte.
//! After each group of 6, r4 is decremented by 12 (net -6 per group).
//!
//! ```text
//! mov.w 0x8c21a60e,r1   ; r1 = 0x00ba
//! mov #32,r2 ; add r1,r4 ; mov #6,r3
//! outer (0x8c21a5e6): mov #6,r3
//!   inner (0x8c21a5e8): mov #0,r6 ; mov.w 0x8c21a610,r1 (=0x0080) ; mov #8,r7
//!     bit loop (0x8c21a5ee):
//!       mov.b @r5+,r0 ; tst #8,r0 ; bt 0x8c21a5f6 ; or r1,r6
//!       0x8c21a5f6: shlr r1 ; dt r7 ; bf 0x8c21a5ee
//!     mov.b r6,@r4 ; add #1,r4 ; dt r3 ; bf 0x8c21a5e8
//!   dt r2 ; bf.s 0x8c21a5e6 ; add #-12,r4 (delay)
//! rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u16(0x8C21A60E).unwrap() as i16 as i32 as u32; // 0x00ba
    cpu.r[2] = 32;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // add r1,r4
    loop {
        // 0x8c21a5e6 (outer head)
        cpu.r[3] = 6;
        loop {
            // 0x8c21a5e8 (byte head)
            cpu.r[6] = 0;
            cpu.r[1] = mem.read_u16(0x8C21A610).unwrap() as i16 as i32 as u32; // 0x0080
            cpu.r[7] = 8;
            loop {
                // 0x8c21a5ee
                cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32;
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                cpu.set_t(cpu.r[0] & 8 == 0); // tst #8,r0
                if cpu.r[0] & 8 != 0 {
                    cpu.r[6] |= cpu.r[1]; // or r1,r6
                }
                let out = cpu.r[1] & 1 != 0;
                cpu.r[1] >>= 1; // shlr r1
                cpu.set_t(out);
                cpu.r[7] = cpu.r[7].wrapping_sub(1);
                cpu.set_t(cpu.r[7] == 0); // dt r7
                if cpu.r[7] != 0 {
                    continue; // bf 0x8c21a5ee
                }
                break;
            }
            let _ = mem.write_u8(cpu.r[4], cpu.r[6] as u8);
            cpu.r[4] = cpu.r[4].wrapping_add(1);
            cpu.r[3] = cpu.r[3].wrapping_sub(1);
            cpu.set_t(cpu.r[3] == 0); // dt r3
            if cpu.r[3] != 0 {
                continue; // bf 0x8c21a5e8
            }
            break;
        }
        cpu.r[2] = cpu.r[2].wrapping_sub(1);
        cpu.set_t(cpu.r[2] == 0); // dt r2
        // bf.s 0x8c21a5e6 ; delay add #-12,r4 (runs either way)
        cpu.r[4] = cpu.r[4].wrapping_sub(12);
        if cpu.r[2] != 0 {
            continue;
        }
        break;
    }
    // rts ; nop
}
