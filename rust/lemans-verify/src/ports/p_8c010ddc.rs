//! 0x8c010ddc — restore a saved CPU context, returning the incoming r5.
//!
//! This is the tail of a context switch: it records the incoming r5 in the
//! global at 0x8c23d890, then reloads FR0-FR15, FPUL, FPSCR, GBR, MACH, MACL
//! and the integer frame (r1-r15 and PR) from the block addressed by r4.
//! Control leaves through the restored PR; r0 ends up holding the recorded r5,
//! or 1 when that value was zero.
//!
//! Frame layout, relative to the entry r4:
//!
//! ```text
//! +0x00 r15   +0x04 r14   +0x08 PR   +0x0c r13   +0x10 r12   +0x14 r11
//! +0x18 r10   +0x1c r9    +0x20 r8   +0x24 r7    +0x28 r6    +0x2c r5
//! +0x30 r4    +0x34 r3    +0x38 r2   +0x3c r1
//! +0x44 MACH  +0x48 MACL  +0x4c GBR
//! +0x50 FR0   ...         +0x8c FR15 +0x90 FPUL  +0x94 FPSCR
//! ```
//!
//! The only store is the 4-byte write of r5 to 0x8c23d890. `cmp/eq #0,r0`
//! sets T exactly when the recorded r5 was zero, and both arms of that `bf`
//! leave that T in place (the zero arm also overwrites r0 with 1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l 0x8c010e4c,r0 ; mov.l r5,@r0
    cpu.r[0] = mem.read_u32(0x8c010e4c).unwrap_or(0x8c23d890);
    let _ = mem.write_u32(cpu.r[0], cpu.r[5]);

    // add #80,r4 ; fmov @r4+,fr0 .. fr14 ; fmov @r4,fr15
    cpu.r[4] = cpu.r[4].wrapping_add(80);
    for i in 0..15 {
        cpu.fr[i] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
    }
    cpu.fr[15] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);

    // mov.l @(4,r4),r0 ; lds r0,fpul
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.fpul = cpu.r[0];
    // mov.l @(8,r4),r0 ; lds r0,fpscr
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.fpscr = cpu.r[0];

    // add #-88,r4 ; ldc r0,gbr ; lds r0,macl ; lds r0,mach
    cpu.r[4] = cpu.r[4].wrapping_sub(88);
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
    cpu.gbr = cpu.r[0];
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(20)).unwrap_or(0);
    cpu.macl = cpu.r[0];
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
    cpu.mach = cpu.r[0];

    // mov.l @(8,r4),r1 ; mov.l @(4,r4),r2 ; mov.l @r4,r3
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);

    // add #-52,r4 ; restore r5..r13, then r14
    cpu.r[4] = cpu.r[4].wrapping_sub(52);
    for i in 5..14 {
        let disp = (44 - 4 * (i - 5)) as u32;
        cpu.r[i] = mem.read_u32(cpu.r[4].wrapping_add(disp)).unwrap_or(0);
    }
    cpu.r[14] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);

    // mov r4,r0 ; mov.l @(48,r0),r4 ; mov.l @r0,r15 ; mov.l @(8,r0),r0 ; lds r0,pr
    cpu.r[0] = cpu.r[4];
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(48)).unwrap_or(0);
    cpu.r[15] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
    cpu.pr = cpu.r[0];

    // mov.l 0x8c010e4c,r0 ; mov.l @r0,r0 ; cmp/eq #0,r0 ; bf 0x8c010e48 ;
    // mov #1,r0 ; rts ; nop
    cpu.r[0] = mem.read_u32(0x8c010e4c).unwrap_or(0x8c23d890);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let zero = cpu.r[0] == 0;
    cpu.set_t(zero);
    if zero {
        cpu.r[0] = 1;
    }
}
