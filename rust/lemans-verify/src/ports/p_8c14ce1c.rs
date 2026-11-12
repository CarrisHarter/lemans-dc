//! 0x8c14ce1c - release every 48-byte record in `[r4+8]`, then zero `[r4+4]`.
//!
//! `end = base + [r4+4]*48` built as `(c*2 + c) << 4` (`shll`, `add`, two
//! `shll2`). Records go to 0x8c0443c8 with `r5 = -1`. The plain `bf` has no
//! delay slot, so `r11 = -1` only appears when the walk runs.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut saved = [0u32; 5];
    for (i, reg) in [14usize, 13, 12, 11].iter().enumerate() {
        saved[i] = cpu.r[*reg];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[*reg]);
    }
    saved[4] = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r0
    cpu.r[13] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r13
    cpu.r[12] = cpu.r[0]; // mov r0,r12
    cpu.set_t(cpu.r[12] >> 31 != 0); // shll r12
    cpu.r[12] = cpu.r[12].wrapping_shl(1);
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[0]); // add r0,r12
    cpu.r[12] = cpu.r[12].wrapping_shl(2); // shll2 r12
    cpu.r[12] = cpu.r[12].wrapping_shl(2); // shll2 r12
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[13]); // add r13,r12
    cpu.set_t(cpu.r[12] > cpu.r[13]); // cmp/hi r13,r12
    if cpu.t() {
        cpu.r[11] = 0xffff_ffff; // mov #-1,r11
        loop {
            cpu.r[0] = mem.read_u32(0x8c14ce5c).unwrap_or(0x8c0443c8); // mov.l 0x8c14ce5c,r0
            cpu.r[5] = cpu.r[11]; // mov r11,r5
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c14ce46;
            crate::registry::p_8c0443c8::run(cpu, mem);
            cpu.r[13] = cpu.r[13].wrapping_add(48); // add #48,r13
            cpu.set_t(cpu.r[12] > cpu.r[13]); // cmp/hi r13,r12
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r14)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for (i, reg) in [11usize, 12, 13].iter().enumerate() {
        cpu.r[*reg] = mem.read_u32(cpu.r[15]).unwrap_or(saved[i + 1]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
