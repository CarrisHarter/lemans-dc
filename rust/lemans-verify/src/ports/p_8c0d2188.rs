//! 0x8c0d2188 — count a three-level linked tree, recursively.
//!
//! For every node reachable from `[r4+0xc]` the routine walks two further
//! `+0xc` links and a `+0x8` sibling chain at each level, calling itself on
//! each innermost node (`jsr @r0` where the literal at 0x8c0d21e0 is this
//! function's own address). Each frame starts its local totals at 1, so the
//! result is the number of nodes in the whole tree plus the per-level counts.
//!
//! The `bt.s`/`bf.s` delay slots (`mov #1,r14`, `mov #1,r11`, `mov #1,r9`,
//! `add r0,r9`, `add r9,r11`, `add r11,r14`) run on both paths, and the
//! recursion leaves its return value in `r0` for the innermost accumulation.
//! The epilogue's `rts` delay slot pops `r14`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d2188  mov.l r14,@-r15 ... 0x8c0d2194 sts.l pr,@-r15
    for reg in [14usize, 13, 12, 11, 10, 9] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c0d2196  mov.l @(12,r4),r13
    cpu.r[13] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    // 0x8c0d2198  tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0d219c  mov #1,r14  (delay slot of the bt.s, always runs)
    cpu.r[14] = 1;
    if !cpu.t() {
        loop {
            // 0x8c0d219e  mov.l @(12,r13),r12
            cpu.r[12] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
            // 0x8c0d21a0  tst r12,r12
            cpu.set_t(cpu.r[12] == 0);
            // 0x8c0d21a4  mov #1,r11  (delay slot, always runs)
            cpu.r[11] = 1;
            if !cpu.t() {
                loop {
                    // 0x8c0d21a6  mov.l @(12,r12),r10
                    cpu.r[10] = mem.read_u32(cpu.r[12].wrapping_add(12)).unwrap_or(0);
                    // 0x8c0d21a8  tst r10,r10
                    cpu.set_t(cpu.r[10] == 0);
                    // 0x8c0d21ac  mov #1,r9  (delay slot, always runs)
                    cpu.r[9] = 1;
                    if !cpu.t() {
                        loop {
                            // 0x8c0d21ae  mov.l 0x8c0d21e0,r0  (0x8c0d2188)
                            cpu.r[0] = mem.read_u32(0x8c0d21e0).unwrap_or(0);
                            // 0x8c0d21b0 jsr @r0; 0x8c0d21b2 mov r10,r4 (delay)
                            cpu.r[4] = cpu.r[10];
                            cpu.pr = 0x8c0d21b4;
                            crate::registry::p_8c0d2188::run(cpu, mem);
                            // 0x8c0d21b4  mov.l @(8,r10),r10
                            cpu.r[10] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
                            // 0x8c0d21b6  tst r10,r10
                            cpu.set_t(cpu.r[10] == 0);
                            // 0x8c0d21ba  add r0,r9  (bf.s delay slot, always runs)
                            cpu.r[9] = cpu.r[9].wrapping_add(cpu.r[0]);
                            if cpu.t() {
                                break;
                            }
                        }
                    }
                    // 0x8c0d21bc  mov.l @(8,r12),r12
                    cpu.r[12] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0);
                    // 0x8c0d21be  tst r12,r12
                    cpu.set_t(cpu.r[12] == 0);
                    // 0x8c0d21c2  add r9,r11  (bf.s delay slot, always runs)
                    cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[9]);
                    if cpu.t() {
                        break;
                    }
                }
            }
            // 0x8c0d21c4  mov.l @(8,r13),r13
            cpu.r[13] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
            // 0x8c0d21c6  tst r13,r13
            cpu.set_t(cpu.r[13] == 0);
            // 0x8c0d21ca  add r11,r14  (bf.s delay slot, always runs)
            cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[11]);
            if cpu.t() {
                break;
            }
        }
    }
    // 0x8c0d21cc  mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0d21ce  lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21d0  mov.l @r15+,r9
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21d2  mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21d4  mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21d6  mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21d8  mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d21da  rts; 0x8c0d21dc mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
