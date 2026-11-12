//! 0x8c23292a - set bit 0 of the first clear halfword in a 76-byte stride list.
//!
//! The list base is `[0x8c23d798+72]`, the count is `[0x8c23d798+12]`. A clear
//! bit is stored as 1 and the function returns that halfword's address. No
//! clear bit, or a zero count, returns 0. `mul.l` writes both MAC halves.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8c232984).unwrap_or(0x8c23d798);
    cpu.r[5] = 0;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let saved14 = cpu.r[14];
    let _ = mem.write_u32(cpu.r[15], saved14);
    cpu.r[3] = cpu.r[5];
    cpu.r[14] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[7] = 1;
    cpu.set_t(cpu.r[3] >= cpu.r[14]); // cmp/hs r14,r3
    cpu.r[6] = 76; // bt.s delay
    if !cpu.t() {
        loop {
            let product = (cpu.r[6] as i32 as i64) * (cpu.r[5] as i32 as i64);
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.r[0] = 72;
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
            cpu.r[1] = cpu.macl;
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
            cpu.r[2] = mem.read_u16(cpu.r[1]).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[2] &= 0xffff;
            cpu.set_t((cpu.r[7] & cpu.r[2]) == 0); // tst r7,r2
            if cpu.t() {
                let product = (cpu.r[6] as i32 as i64) * (cpu.r[5] as i32 as i64);
                cpu.mach = (product >> 32) as u32;
                cpu.macl = product as u32;
                cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
                cpu.r[14] = cpu.macl;
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[14]);
                let _ = mem.write_u16(cpu.r[2], cpu.r[7] as u16);
                cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(72)).unwrap_or(0);
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[14]);
                cpu.r[14] = saved14; // rts delay
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                return;
            }
            cpu.r[5] = cpu.r[5].wrapping_add(1);
            cpu.set_t(cpu.r[5] >= cpu.r[14]); // cmp/hs r14,r5
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 0;
    cpu.r[14] = saved14; // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
