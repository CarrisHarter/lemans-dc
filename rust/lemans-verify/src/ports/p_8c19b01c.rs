//! 0x8c19b01c — copy_12_byte_records.
//!
//! Copies 12-byte records (word, float, byte) from `r4` to `r6` until `r4 == r5`;
//! a null `r6` skips the copy but still advances both pointers. Returns `r6`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut src = cpu.r[4];
    let mut dst = cpu.r[6];
    let hi = cpu.r[5];
    if src != hi {
        loop {
            if dst != 0 {
                let r3 = mem.read_u32(src).unwrap_or(0);
                let mut r0 = src;
                let mut r1 = dst;
                let _ = mem.write_u32(dst, r3);
                r0 = r0.wrapping_add(4);
                cpu.fr[0] = mem.read_f32(r0).unwrap_or(0.0);
                r1 = r1.wrapping_add(4);
                let _ = mem.write_f32(r1, cpu.fr[0]);
                let r0 = mem.read_u8(src + 8).unwrap_or(0) as u32;
                let _ = mem.write_u8(dst + 8, r0 as u8);
                cpu.r[3] = r3;
                cpu.r[1] = r1;
            }
            src = src.wrapping_add(12);
            let eq = src == hi;
            cpu.set_t(eq);
            dst = dst.wrapping_add(12); // delay slot of `bf.s`
            if eq {
                break;
            }
        }
    }
    cpu.r[4] = src;
    cpu.r[6] = dst;
    cpu.r[0] = dst; // delay slot of `rts`
}
