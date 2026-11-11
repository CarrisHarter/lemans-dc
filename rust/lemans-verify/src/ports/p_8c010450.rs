//! 0x8c010450 — interrupt-masked walk of a circular node list: sums
//! (count-1) into `*r4` and tracks max(count-1) into `*r5`, both scaled by 64.
//!
//! Raises IMASK to 15 (saving the old SR nibble on the stack), walks nodes
//! from the head at 0x8c23d87c until a node equals the head, then restores
//! IMASK from the stack word. Accumulates r14 += count-1 unconditionally
//! (the `bt.s` delay slot `add r0,r14` runs on both paths) and r7 = max.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c010516).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    let _ = mem.write_u16(cpu.r[15], cpu.r[0] as u16); // mov.w r0,@r15
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[2] = mem.read_u32(0x8c010528).unwrap_or(0); // 0x8c23d87c
    cpu.r[6] = 0;
    cpu.r[7] = cpu.r[6];
    cpu.r[14] = cpu.r[6];
    cpu.r[12] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[6] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    loop {
        cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
        cpu.r[3] = 0;
        cpu.set_t(cpu.r[2] > cpu.r[3]); // cmp/hi r3,r2
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            cpu.r[0] = cpu.r[0].wrapping_sub(1);
            cpu.set_t(cpu.r[7] >= cpu.r[0]); // cmp/hs r0,r7
            cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // bt.s delay slot: always runs
            if !cpu.t() {
                cpu.r[7] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
                cpu.r[7] = cpu.r[7].wrapping_sub(1);
            }
        }
        cpu.set_t(cpu.r[6] == cpu.r[12]); // cmp/eq r12,r6
        if cpu.t() {
            break;
        }
        cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0); // bra delay slot
    }
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[14] <<= 2; // shll2
    cpu.r[2] = mem.read_u16(0x8c010516).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[7] <<= 2; // shll2
    cpu.r[14] <<= 2; // shll2
    cpu.r[7] <<= 2; // shll2
    cpu.r[14] <<= 1; // shll
    cpu.r[7] <<= 1; // shll
    let _ = mem.write_u32(cpu.r[4], cpu.r[14]);
    cpu.r[3] &= cpu.r[2];
    let _ = mem.write_u32(cpu.r[5], cpu.r[7]);
    cpu.r[0] = mem.read_u16(cpu.r[15]).unwrap_or(0) as u32; // mov.w @r15,r0
    cpu.r[0] &= 0xffff; // extu.w r0,r0
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
