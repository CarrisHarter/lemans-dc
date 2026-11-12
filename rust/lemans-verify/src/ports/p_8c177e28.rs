//! 0x8c177e28 - bounds-checked slot clear across five parallel tables.
//!
//! `r4 >= [0x8c2b52d4]` emits str@0x8c1e2530 with code 102 and returns 0.
//! Otherwise 0x8c1739f4(15) supplies the new SR field, and the word at
//! `0x8c2b5528 + {0x10c4,0x11c4,0x12c4,0x13c4,0x14c4} + r4*4` is cleared in each
//! of the five tables. SR becomes `(SR & 0xff0f) | ((r7&15)<<4)` and the
//! function returns 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[1] = mem.read_u32(0x8c177f4c).unwrap_or(0x8c2b52d4); // mov.l 0x8c177f4c,r1
    cpu.r[2] = cpu.r[4]; // mov r4,r2
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r3
    cpu.set_t(cpu.r[2] >= cpu.r[3]); // cmp/hs r3,r2
    let mut result;
    if cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c177f54).unwrap_or(0x8c174564); // mov.l 0x8c177f54,r2
        cpu.r[5] = mem.read_u32(0x8c177f50).unwrap_or(0x8c1e2530); // mov.l 0x8c177f50,r5
        cpu.r[4] = 102; // jsr delay: mov #102,r4
        cpu.pr = 0x8c177e40;
        crate::registry::p_8c174564::run(cpu, mem);
        result = 0; // rts delay at 0x8c177e46
    } else {
        cpu.r[3] = mem.read_u32(0x8c177f58).unwrap_or(0x8c1739f4); // mov.l 0x8c177f58,r3
        cpu.r[4] = 15; // jsr delay: mov #15,r4
        cpu.pr = 0x8c177e4e;
        crate::registry::p_8c1739f4::run(cpu, mem);
        cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r5
        cpu.r[4] = 0; // mov #0,r4
        cpu.r[3] = mem.read_u16(0x8c177f3a).unwrap_or(0x10c4) as i16 as i32 as u32; // mov.w
        cpu.r[7] = cpu.r[0]; // mov r0,r7
        cpu.r[6] = mem.read_u32(0x8c177f5c).unwrap_or(0x8c2b5528); // mov.l 0x8c177f5c,r6
        cpu.r[5] = cpu.r[5].wrapping_shl(2); // shll2 r5
        cpu.r[2] = mem.read_u16(0x8c177f3c).unwrap_or(0x11c4) as i16 as i32 as u32; // mov.w
        cpu.r[0] = cpu.r[7]; // mov r7,r0
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // add r6,r3
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // add r5,r3
        let _ = mem.write_u32(cpu.r[3], cpu.r[4]); // mov.l r4,@r3
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[6]); // add r6,r2
        cpu.r[3] = mem.read_u16(0x8c177f3e).unwrap_or(0x12c4) as i16 as i32 as u32; // mov.w
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[5]); // add r5,r2
        let _ = mem.write_u32(cpu.r[2], cpu.r[4]); // mov.l r4,@r2
        cpu.r[0] &= 15; // and #15,r0
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // add r6,r3
        cpu.r[2] = mem.read_u16(0x8c177f40).unwrap_or(0x13c4) as i16 as i32 as u32; // mov.w
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // add r5,r3
        let _ = mem.write_u32(cpu.r[3], cpu.r[4]); // mov.l r4,@r3
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[6]); // add r6,r2
        cpu.r[3] = mem.read_u16(0x8c177f42).unwrap_or(0x14c4) as i16 as i32 as u32; // mov.w
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[5]); // add r5,r2
        let _ = mem.write_u32(cpu.r[2], cpu.r[4]); // mov.l r4,@r2
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[6]); // add r6,r3
        cpu.r[2] = cpu.sr; // stc sr,r2
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]); // add r3,r5
        cpu.r[3] = mem.read_u16(0x8c177f44).unwrap_or(0xff0f) as i16 as i32 as u32; // mov.w
        let _ = mem.write_u32(cpu.r[5], cpu.r[4]); // mov.l r4,@r5
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.r[2] &= cpu.r[3]; // and r3,r2
        cpu.r[0] |= cpu.r[2]; // or r2,r0
        cpu.sr = cpu.r[0]; // ldc r0,sr
        result = 1; // mov #1,r0
    }
    cpu.r[0] = result;
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
