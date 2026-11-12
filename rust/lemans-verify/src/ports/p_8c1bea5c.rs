//! 0x8c1bea5c - copy a run of 92-byte records from `r4` up to `r5` into `r6`.
//!
//! Walks the source from `r4` and the destination from `r6` in 92-byte steps
//! until the source reaches `r5`. Each step copies the first 64 bytes with
//! 0x8c010e84 and then the seven words at `+64..+88` one at a time, in the order
//! 80, 84, 72, 88, 76, 64, 68. A null destination skips the copy but still
//! advances both cursors. Returns the final destination cursor.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.set_t(cpu.r[12] == cpu.r[13]); // cmp/eq r13,r12
    cpu.r[14] = cpu.r[6]; // bt.s delay: mov r6,r14
    if !cpu.t() {
        loop {
            cpu.set_t((cpu.r[14] & cpu.r[14]) == 0); // tst r14,r14
            if !cpu.t() {
                cpu.r[0] = mem.read_u32(0x8c1bead8).unwrap_or(0x8c010e84); // mov.l 0x8c1bead8,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[14]; // mov r14,r4
                cpu.r[6] = 64; // jsr delay: mov #64,r6
                cpu.pr = 0x8c1bea7c;
                crate::registry::p_8c010e84::run(cpu, mem);
                cpu.r[0] = cpu.r[12].wrapping_add(80); // mov r12,r0 / add #80,r0
                cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r1
                cpu.r[3] = cpu.r[12].wrapping_add(84); // mov r12,r3 / add #84,r3
                cpu.r[0] = 80; // mov #80,r0
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]); // mov.l r1,@(r0,r14)
                cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r3
                cpu.r[0] = 84; // mov #84,r0
                cpu.r[1] = cpu.r[12]; // mov r12,r1
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3]); // mov.l r3,@(r0,r14)
                cpu.r[1] = cpu.r[1].wrapping_add(72); // add #72,r1
                cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r1
                cpu.r[0] = 72; // mov #72,r0
                cpu.r[3] = cpu.r[12]; // mov r12,r3
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]); // mov.l r1,@(r0,r14)
                cpu.r[3] = cpu.r[3].wrapping_add(88); // add #88,r3
                cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r3
                cpu.r[0] = 88; // mov #88,r0
                cpu.r[1] = cpu.r[12]; // mov r12,r1
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3]); // mov.l r3,@(r0,r14)
                cpu.r[1] = cpu.r[1].wrapping_add(76); // add #76,r1
                cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r1
                cpu.r[0] = 76; // mov #76,r0
                cpu.r[3] = cpu.r[12]; // mov r12,r3
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]); // mov.l r1,@(r0,r14)
                cpu.r[3] = cpu.r[3].wrapping_add(64); // add #64,r3
                cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r3
                cpu.r[0] = 64; // mov #64,r0
                cpu.r[1] = cpu.r[12]; // mov r12,r1
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3]); // mov.l r3,@(r0,r14)
                cpu.r[1] = cpu.r[1].wrapping_add(68); // add #68,r1
                cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0); // mov.l @r1,r1
                cpu.r[0] = 68; // mov #68,r0
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]); // mov.l r1,@(r0,r14)
            }
            cpu.r[12] = cpu.r[12].wrapping_add(92); // add #92,r12
            cpu.set_t(cpu.r[12] == cpu.r[13]); // cmp/eq r13,r12
            cpu.r[14] = cpu.r[14].wrapping_add(92); // bf.s delay: add #92,r14
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
