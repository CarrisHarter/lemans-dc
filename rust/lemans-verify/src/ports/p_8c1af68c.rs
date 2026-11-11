//! 0x8c1af68c — copy_16_bytes_in_32_byte_strides.
//!
//! Copies 16 bytes at a time from `r4` to `r6`, advancing both by 32, until
//! `r4 == r5`. A null `r6` skips the copy but still advances it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut src = cpu.r[4];
    let mut dst = cpu.r[6];
    let hi = cpu.r[5];
    if src != hi {
        loop {
            if dst != 0 {
                let mut r7 = src;
                let mut r3 = dst;
                let mut r1: u32 = 16;
                loop {
                    cpu.r[0] = mem.read_u8(r7).unwrap_or(0) as u32;
                    r7 = r7.wrapping_add(1);
                    r1 = r1.wrapping_sub(1);
                    cpu.set_t(r1 == 0);
                    let _ = mem.write_u8(r3, cpu.r[0] as u8);
                    cpu.r[0] = mem.read_u8(r7).unwrap_or(0) as u32;
                    r7 = r7.wrapping_add(1);
                    let _ = mem.write_u8(r3 + 1, cpu.r[0] as u8);
                    r3 = r3.wrapping_add(2); // delay slot of `bf.s`
                    if r1 == 0 {
                        break;
                    }
                }
                cpu.r[7] = r7;
                cpu.r[3] = r3;
                cpu.r[1] = r1;
            }
            src = src.wrapping_add(32);
            let eq = src == hi;
            cpu.set_t(eq);
            dst = dst.wrapping_add(32); // delay slot of `bf.s`
            if eq {
                break;
            }
        }
    }
    cpu.r[4] = src;
    cpu.r[6] = dst;
    cpu.r[0] = dst; // delay slot of `rts`
}
