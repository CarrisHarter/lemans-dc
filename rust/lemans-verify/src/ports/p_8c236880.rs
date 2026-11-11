//! 0x8c236880 — release_slot.
//!
//! Scans the 52-byte slots of `0x8c2c8740` (0xd0 bytes) for `r4`. When found and
//! `[r4+40]` is non-zero, zeroes it. Always returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    cpu.r[6] = 0; // delay slot of `bt.s`
    if !t {
        cpu.r[5] = cpu.r[4];
        cpu.r[4] = mem.read_u32(0x8c236924).unwrap_or(0);
        cpu.r[3] = mem.read_u16(0x8c236920).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[7] = 1;
        cpu.r[1] = cpu.r[4];
        let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        loop {
            let eq = cpu.r[5] == cpu.r[4];
            cpu.set_t(eq);
            cpu.r[4] = cpu.r[4].wrapping_add(52); // delay slot of `bf.s`
            if eq {
                cpu.r[6] = cpu.r[7];
            }
            let hs = cpu.r[4] >= cpu.r[1];
            cpu.set_t(hs);
            if hs {
                break;
            }
        }
        let t2 = cpu.r[6] == 0;
        cpu.set_t(t2);
        if !t2 {
            cpu.r[2] = mem.read_u32(cpu.r[5] + 40).unwrap_or(0);
            let t3 = cpu.r[2] == 0;
            cpu.set_t(t3);
            if !t3 {
                cpu.r[3] = 0;
                let _ = mem.write_u32(cpu.r[5] + 40, cpu.r[3]);
            }
        }
    }
    cpu.r[0] = 0;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // delay slot of `rts`
}
