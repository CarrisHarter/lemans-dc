//! 0x8c21f9d8 — publish one byte into a device object chosen by the mode word
//! at 0x8c2bdcb8, then acknowledge the object's status bits.
//!
//! ```text
//! 8c21f9d8  mov.l 0x8c21f9f4,r3   r3 = 0x8c2bdcb8
//! 8c21f9da  mov.l @r3,r0          r0 = mode
//! 8c21f9dc  cmp/eq #0,r0
//! 8c21f9de  bt.s 0x8c21fa14       mode 0 -> object at 0x8c2bdcb4
//! 8c21f9e0  mov   r4,r6           delay: always runs, taken or not
//! 8c21f9e2  cmp/eq #1,r0
//! 8c21f9e4  bt    0x8c21fa54      mode 1 -> object at 0x8c2bdcb0
//! 8c21f9e6  bra   0x8c21fa86
//! 8c21f9e8  nop                    delay
//! ```
//!
//! Because the `bt.s` delay slot runs unconditionally, every path publishes
//! the low byte of the entry `r4` (via `r6`), not `r4` at the publish site.
//!
//! mode 0 (0x8c21fa14): poll the halfword at `[[0x8c2bdcb4]+16]` until bit 5
//! reads set, store the byte at `+12`, clear bits 5 and 6, then poll bit 6.
//! That second poll cannot terminate under the oracle's RAM model: the `and
//! 0xffbf` immediately before it clears the bit it waits for, and the only
//! writer of that word is the routine itself. On hardware the status register
//! is raised by the device; here a mode-0 spec exhausts its step budget, so the
//! halting specs take modes 1 and >= 2. The loop is still reproduced exactly.
//!
//! mode 1 (0x8c21fa54): poll the byte at `[[0x8c2bdcb0]+16]` until bit 7 reads
//! set, store the byte at `+12`, clear bit 7, then poll bit 2 (which the byte
//! still has, so this one can halt). Modes >= 2 return at once with T clear
//! and `r0` still holding the mode word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c21f9d8..0x8c21f9e0
    cpu.r[3] = mem.read_u32(0x8c21f9f4).unwrap_or(0); // 0x8c2bdcb8
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0); // cmp/eq #0,r0
    let mode0 = cpu.t();
    cpu.r[6] = cpu.r[4]; // bt.s delay slot: executed whether taken or not

    if mode0 {
        // 0x8c21fa14..0x8c21fa22: wait for status bit 5 on the 0x8c2bdcb4 object.
        cpu.r[4] = mem.read_u32(0x8c21fb28).unwrap_or(0); // 0x8c2bdcb4
        cpu.r[5] = 32;
        loop {
            cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.r[0] = mem.read_u16(cpu.r[0].wrapping_add(16)).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[0] &= 0xffff; // extu.w
            cpu.r[0] &= cpu.r[5];
            cpu.set_t(cpu.r[0] == 32); // cmp/eq #32,r0
            if cpu.t() {
                break; // bf 0x8c21fa18
            }
        }

        // 0x8c21fa24..0x8c21fa34: publish the byte, clear status bit 5.
        cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[2] = cpu.r[2].wrapping_add(12);
        let _ = mem.write_u8(cpu.r[2], cpu.r[6] as u8); // mov.b r6,@r2
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[2] = mem.read_u32(0x8c21fb2c).unwrap_or(0); // 0x0000ffdf
        cpu.r[3] = cpu.r[3].wrapping_add(16);
        cpu.r[1] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[1] &= cpu.r[2]; // and r2,r1
        let _ = mem.write_u16(cpu.r[3], cpu.r[1] as u16);

        // 0x8c21fa36..0x8c21fa40: clear status bit 6.
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[1] = mem.read_u32(0x8c21fb30).unwrap_or(0); // 0x0000ffbf
        cpu.r[3] = cpu.r[3].wrapping_add(16);
        cpu.r[0] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[0] &= cpu.r[1]; // and r1,r0
        let _ = mem.write_u16(cpu.r[3], cpu.r[0] as u16);

        // 0x8c21fa42..0x8c21fa50: wait for status bit 6 (device-raised; cannot
        // halt on plain RAM — see the module comment).
        cpu.r[5] = 64;
        loop {
            cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.r[0] = mem.read_u16(cpu.r[0].wrapping_add(16)).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[0] &= 0xffff; // extu.w
            cpu.r[0] &= cpu.r[5];
            cpu.set_t(cpu.r[0] == 64); // cmp/eq #64,r0
            if cpu.t() {
                break; // bf 0x8c21fa44
            }
        }
    } else {
        // 0x8c21f9e2..0x8c21f9e4
        let mode1 = cpu.r[0] == 1; // cmp/eq #1,r0
        cpu.set_t(mode1);

        if mode1 {
            // 0x8c21fa54..0x8c21fa64: wait for status bit 7 on the 0x8c2bdcb0 object.
            cpu.r[4] = mem.read_u32(0x8c21fb34).unwrap_or(0); // 0x8c2bdcb0
            cpu.r[5] = mem.read_u16(0x8c21fb22).unwrap_or(0) as i16 as i32 as u32; // 0x80
            loop {
                cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
                cpu.r[3] = cpu.r[3].wrapping_add(16);
                cpu.r[2] = mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[2] &= 0xff; // extu.b
                cpu.r[2] &= cpu.r[5];
                cpu.set_t(cpu.r[2] == cpu.r[5]); // cmp/eq r5,r2
                if cpu.t() {
                    break; // bf 0x8c21fa58
                }
            }

            // 0x8c21fa66..0x8c21fa74: publish the byte, clear status bit 7.
            cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(12);
            let _ = mem.write_u8(cpu.r[2], cpu.r[6] as u8); // mov.b r6,@r2
            cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(16);
            cpu.r[0] = mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[0] &= 0x7f; // and #127,r0
            let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8);

            // 0x8c21fa76..0x8c21fa84: wait for status bit 2.
            cpu.r[5] = 4;
            loop {
                cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
                cpu.r[0] = cpu.r[0].wrapping_add(16);
                cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[0] &= 0xff; // extu.b
                cpu.r[0] &= cpu.r[5];
                cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
                if cpu.t() {
                    break; // bf 0x8c21fa78
                }
            }
        }
    }
    // 0x8c21fa86 rts ; 0x8c21fa88 nop — T keeps the last cmp/eq of the path.
}
