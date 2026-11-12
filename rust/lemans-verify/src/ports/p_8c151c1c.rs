//! 0x8c151c1c - construct the object at r4: sub-init, vtable, float fields.
//!
//! Calls 0x8c15217c(arg) with the entry r4, then stamps `[arg] = 0x8c1ef44c`
//! plus +0.0 at arg+88/arg+92, builds (1.0, 0.0, 0.0) in the 12-byte frame,
//! copies the frame words to arg+96/arg+100/arg+104, and zeroes arg+108/arg+112.
//! Straight line, no branches. The frame slots live in the stack window but the
//! reloads must observe them, so the frame goes through real memory.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14

    cpu.r[0] = mem.read_u32(0x8c151c78).unwrap_or(0x8c15217c);
    cpu.pr = 0x8c151c2a; // jsr delay: nop
    crate::registry::p_8c15217c::run(cpu, mem);

    cpu.r[3] = mem.read_u32(0x8c151c7c).unwrap_or(0x8c1ef44c);
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[3]); // mov.l r3,@r14
    cpu.r[1] = cpu.r[1].wrapping_add(88); // add #88,r1
    cpu.fr[1] = 0.0; // fldi0 fr1
    let _ = mem.write_u32(cpu.r[1], cpu.fr[1].to_bits()); // fmov fr1,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(92); // add #92,r0
    cpu.r[2] = 0; // mov #0,r2
    let _ = mem.write_u32(cpu.r[0], cpu.fr[1].to_bits()); // fmov fr1,@r0
    cpu.fr[0] = 1.0; // fldi1 fr0
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits()); // fmov fr0,@r2
    cpu.r[6] = cpu.r[14]; // mov r14,r6
    cpu.r[2] = 4; // mov #4,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[1].to_bits()); // fmov fr1,@r2
    cpu.r[3] = cpu.r[14]; // mov r14,r3
    cpu.r[2] = 8; // mov #8,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2
    let _ = mem.write_u32(cpu.r[2], cpu.fr[1].to_bits()); // fmov fr1,@r2
    cpu.r[5] = cpu.r[15]; // mov r15,r5
    cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r4
    cpu.r[6] = cpu.r[6].wrapping_add(96); // add #96,r6
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r5),r0
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.r[3] = cpu.r[3].wrapping_add(108); // add #108,r3
    let _ = mem.write_u32(cpu.r[6], cpu.r[4]); // mov.l r4,@r6
    cpu.r[1] = cpu.r[1].wrapping_add(112); // add #112,r1
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r6)
    cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r5),r4
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[4]); // mov.l r4,@(8,r6)
    let _ = mem.write_u32(cpu.r[3], cpu.fr[1].to_bits()); // fmov fr1,@r3
    let _ = mem.write_u32(cpu.r[1], cpu.fr[1].to_bits()); // fmov fr1,@r1

    cpu.r[15] = cpu.r[15].wrapping_add(12); // add #12,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
