//! 0x8c176d7e - store `r5`/`r6` into a 52-byte-stride slot, with two guards.
//!
//! `r4 == 0` emits str@0x8c1e1d9c with code 101 and returns 0; `r7 >= 16` emits
//! str@0x8c1e1dd0 with code 0x450 and returns 0. Otherwise `[r4 + 52*r7 + 44] = r5`
//! and `[r4 + 52*r7 + 48] = r6`; when `r7 == 0` also `[r4+0x3c4] = r5`. r0 = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] == 0 {
        cpu.r[2] = mem.read_u32(0x8c176dc4).unwrap_or(0x8c174564); // mov.l 0x8c176dc4,r2
        cpu.r[5] = mem.read_u32(0x8c176dc0).unwrap_or(0x8c1e1d9c); // mov.l 0x8c176dc0,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c176d8e;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c176da8
    } else {
        cpu.r[3] = 16; // mov #16,r3
        cpu.set_t(cpu.r[7] >= cpu.r[3]); // cmp/hs r3,r7
        if cpu.t() {
            cpu.r[1] = mem.read_u32(0x8c176dc4).unwrap_or(0x8c174564); // mov.l 0x8c176dc4,r1
            cpu.r[4] = mem.read_u16(0x8c176db8).unwrap_or(0x450) as i16 as i32 as u32; // mov.w
            cpu.r[5] = mem.read_u32(0x8c176dc8).unwrap_or(0x8c1e1dd0); // mov.l 0x8c176dc8,r5
            cpu.pr = 0x8c176da2; // jsr delay is nop
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0; // rts delay at 0x8c176da8
        } else {
            cpu.r[3] = 52; // mov #52,r3
            cpu.r[2] = cpu.r[4]; // mov r4,r2
            let product = (cpu.r[3] as i32 as i64) * (cpu.r[7] as i32 as i64); // mul.l r3,r7
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.set_t(cpu.r[7] == 0); // tst r7,r7
            cpu.r[3] = cpu.macl; // sts macl,r3
            let _ = mem.write_u32(cpu.r[15], cpu.r[3]); // mov.l r3,@r15
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]); // add r2,r3
            let _ = mem.write_u32(cpu.r[3].wrapping_add(44), cpu.r[5]); // mov.l r5,@(44,r3)
            cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]); // add r3,r2
            let _ = mem.write_u32(cpu.r[2].wrapping_add(48), cpu.r[6]); // bf.s delay: mov.l r6,@(48,r2)
            if cpu.t() {
                cpu.r[0] = mem.read_u16(0x8c176eca).unwrap_or(0x3c4) as i16 as i32 as u32; // mov.w
                let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // mov.l r5,@(r0,r4)
            }
            cpu.r[0] = 1; // mov #1,r0
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
