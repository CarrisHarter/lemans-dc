//! 0x8c176df2 - store `r5` into a 52-byte-stride slot, with two range guards.
//!
//! `r4 == 0` emits str@0x8c1e1e10 with code 101 and returns 0; `r6 >= 16` emits
//! str@0x8c1e1e44 with code 0x450 and returns 0. Otherwise `[r4 + 52*r6 + 40] = r5`
//! and, when `r6 == 0`, also `[r4 + 0x3c4] = r5`; r0 = 1 and T ends as `(r6 == 0)`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] == 0 {
        cpu.r[2] = mem.read_u32(0x8c176ee0).unwrap_or(0x8c174564); // mov.l 0x8c176ee0,r2
        cpu.r[5] = mem.read_u32(0x8c176edc).unwrap_or(0x8c1e1e10); // mov.l 0x8c176edc,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c176e00;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c176e18
    } else {
        cpu.r[3] = 16; // mov #16,r3
        cpu.set_t(cpu.r[6] >= cpu.r[3]); // cmp/hs r3,r6
        if cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c176ee0).unwrap_or(0x8c174564); // mov.l 0x8c176ee0,r1
            cpu.r[4] = mem.read_u16(0x8c176ecc).unwrap_or(0x450) as i16 as i32 as u32; // mov.w
            cpu.r[5] = mem.read_u32(0x8c176ee4).unwrap_or(0x8c1e1e44); // mov.l 0x8c176ee4,r5
            cpu.pr = 0x8c176e14; // jsr delay is nop
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0; // rts delay at 0x8c176e18
        } else {
            cpu.r[3] = 52; // mov #52,r3
            cpu.r[2] = cpu.r[4]; // mov r4,r2
            let product = (cpu.r[3] as i32 as i64) * (cpu.r[6] as i32 as i64); // mul.l r3,r6
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.set_t(cpu.r[6] == 0); // tst r6,r6
            cpu.r[3] = cpu.macl; // sts macl,r3
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]); // add r2,r3
            let _ = mem.write_u32(cpu.r[3].wrapping_add(40), cpu.r[5]); // mov.l r5,@(40,r3)
            if cpu.t() {
                cpu.r[0] = mem.read_u16(0x8c176eca).unwrap_or(0x3c4) as i16 as i32 as u32; // mov.w
                let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // mov.l r5,@(r0,r4)
            }
            cpu.r[0] = 1; // mov #1,r0
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
