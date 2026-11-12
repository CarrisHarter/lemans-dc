//! 0x8c23b8c4 — claim the first free slot whose flag word has bit 0 clear.
//!
//! `base = [0x8c23b920]`, `count = [base+8]`, array at `[base+32]`, stride 24.
//! Each entry's 16-bit flags word is tested with `tst 1`; the first clear entry
//! gets 1 stored and its address returned (`r0 = array + i*24` through the `rts`
//! delay slot). No free slot returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8c23b920).unwrap_or(0x8c23d798); // mov.l 0x8c23b920,r4
    cpu.r[5] = 0; // mov #0,r5
    cpu.r[3] = cpu.r[5]; // mov r5,r3
    cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r7
    cpu.set_t(cpu.r[3] >= cpu.r[7]); // cmp/hs r7,r3
    cpu.r[6] = 1; // bt.s delay slot: mov #1,r6
    if !cpu.t() {
        loop {
            // 0x8c23b8d2
            cpu.r[1] = cpu.r[5]; // mov r5,r1
            cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1 sets T from the shifted-out bit
            cpu.r[1] = cpu.r[1].wrapping_shl(1); // shll r1
            cpu.r[3] = cpu.r[5]; // mov r5,r3
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0); // mov.l @(32,r4),r2
            cpu.r[1] = cpu.r[1].wrapping_shl(2); // shll2 r1
            cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1 sets T from the shifted-out bit
            cpu.set_t(cpu.r[1] >> 31 != 0); // shll r1
            cpu.r[1] = cpu.r[1].wrapping_shl(1); // shll r1
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[2]); // add r2,r1
            cpu.r[3] = mem.read_u16(cpu.r[1]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r1,r3
            cpu.r[3] &= 0xffff; // extu.w r3,r3
            cpu.set_t((cpu.r[6] & cpu.r[3]) == 0); // tst r6,r3
            if (cpu.r[6] & cpu.r[3]) != 0 {
                cpu.r[5] = cpu.r[5].wrapping_add(1); // 0x8c23b940
                cpu.set_t(cpu.r[5] >= cpu.r[7]); // cmp/hs r7,r5
                if cpu.t() {
                    break;
                }
                continue;
            }
            // 0x8c23b8ea
            cpu.r[7] = cpu.r[5]; // mov r5,r7
            cpu.set_t(cpu.r[7] >> 31 != 0); // shll r7
            cpu.r[7] = cpu.r[7].wrapping_shl(1); // shll r7
            cpu.r[3] = cpu.r[5]; // mov r5,r3
            cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]); // add r3,r7
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0); // mov.l @(32,r4),r2
            cpu.r[7] = cpu.r[7].wrapping_shl(2); // shll2 r7
            cpu.set_t(cpu.r[7] >> 31 != 0); // shll r7
            cpu.r[7] = cpu.r[7].wrapping_shl(1); // shll r7
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[7]); // add r7,r2
            let _ = mem.write_u16(cpu.r[2], cpu.r[6] as u16); // mov.w r6,@r2
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0); // mov.l @(32,r4),r0
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[7]); // rts delay: add r7,r0
            return;
        }
    }
    cpu.r[0] = 0; // mov #0,r0
}
