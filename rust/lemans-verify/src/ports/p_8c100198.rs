//! 0x8c100198 — bump a cursor: count++, offset += 4, wrap from base table.
//!
//! ```text
//! mov.l @(4,r4),r0 ; add #1,r0 ; mov.l r0,@(4,r4)   ; count++
//! mov.l @(8,r4),r3 ; mov.l @r4,r1                   ; r3 = offset, r1 = base
//! add #4,r3 ; mov.l r3,@(8,r4)                      ; offset += 4
//! mov.l @r1,r0 ; mov.l @(12,r1),r1 ; shll2 r0 ; add r0,r1
//! cmp/eq r3,r1 ; bf 0x8c1001b8                     ; new offset != base[3]+4*base[0]
//! mov.l @r4,r0 ; mov.l @(12,r0),r0 ; mov.l r0,@(8,r4) ; offset = base[3]
//! 0x8c1001b8: rts ; mov r4,r0                       ; delay slot: return param
//! ```
//! Hidden side effects: r1 = base[12] + 4*base[0] on BOTH paths, r3 = old
//! offset+4, r0 = r4 always (the rts delay slot overwrites whatever the
//! branch path left), T = the cmp/eq result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap(); // mov.l @(4,r4),r0
    cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]); // mov.l r0,@(4,r4)
    cpu.r[3] = mem.read_u32(cpu.r[4] + 8).unwrap(); // mov.l @(8,r4),r3
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r1
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]); // mov.l r3,@(8,r4)
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap(); // mov.l @r1,r0
    cpu.r[1] = mem.read_u32(cpu.r[1] + 12).unwrap(); // mov.l @(12,r1),r1
    cpu.r[0] = cpu.r[0] << 2; // shll2 r0
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
    cpu.set_t(cpu.r[3] == cpu.r[1]); // cmp/eq r3,r1
    if cpu.r[3] == cpu.r[1] {
        // bf not taken
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
        cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap(); // mov.l @(12,r0),r0
        let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // mov.l r0,@(8,r4)
    }
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
