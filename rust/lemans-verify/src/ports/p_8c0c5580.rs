//! 0x8c0c5580 — construct an "effects/skidsand0" object from three sources.
//!
//! ```text
//! push r14,r13,r12,r11,pr; r15 -= 8
//! r13 = param_4; r12 = param_3
//! [r15] = param_1; r11 = param_2; r14 = param_1
//! [r15+4] = param_1                       ; spill for the call arguments
//! call 0x8c05d378(param_1, param_2, param_3, param_4)
//! *param_1 = 0x8c1eb940
//! call 0x8c05d274(param_1, 1, 1)
//! *param_1 = 0x8c1e8b54
//! [param_1+56] = param_4[0]; [param_1+60] = param_4[1]
//! [param_1+64] = param_4[0]; [param_1+68] = param_4[1]
//! *param_1 = 0x8c1eb798
//! call 0x8c011344(param_1+72, param_2, 64)   ; strncpy
//! [param_1+88..104] = param_3[0..4]
//! return param_1
//! ```
//!
//! Faithful details: the first callee's store of its own vtable is kept in the
//! write log even though four later stores replace it; `r3` ends as
//! `param_1 + 88`; the pointers in r11..r14 are restored from the stack, so
//! they end at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: r14, r13, r12, r11, pr, then an 8-byte frame.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[7]; // mov r7,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[6]; // mov r6,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[11] = cpu.r[5]; // mov r5,r11
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r14
    cpu.r[0] = mem.read_u32(0x8c0c5600).unwrap_or(0); // call target 0x8c05d378
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[14]);
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // delay slot
    crate::registry::p_8c05d378::run(cpu, mem);

    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[6] = 1;
    cpu.r[0] = mem.read_u32(0x8c0c5604).unwrap_or(0); // 0x8c1eb940
    cpu.r[5] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c0c5608).unwrap_or(0); // call target 0x8c05d274
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // delay slot
    crate::registry::p_8c05d274::run(cpu, mem);

    cpu.r[0] = mem.read_u32(0x8c0c560c).unwrap_or(0); // 0x8c1e8b54
    cpu.r[4] = cpu.r[14].wrapping_add(56); // add #56,r4
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // *param_1 = 0x8c1e8b54
    cpu.r[5] = cpu.r[11]; // mov r11,r5
    cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0); // param_4[0]
    cpu.r[14] = cpu.r[14].wrapping_add(64); // add #64,r14
    cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0); // param_4[1]
    cpu.r[6] = 64;
    cpu.r[1] = mem.read_u32(0x8c0c5610).unwrap_or(0); // 0x8c1eb798
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r4
    cpu.r[0] = mem.read_u32(0x8c0c5614).unwrap_or(0); // call target 0x8c011344
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[3]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // param_1
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // *param_1 = 0x8c1eb798
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0).wrapping_add(72); // delay slot: add #72,r4
    crate::registry::p_8c011344::run(cpu, mem);

    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c0c5618).unwrap_or(0); // 0x88
    cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0); // param_3[0]
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0); // param_3[1]
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0); // param_3[2]
    cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(12)).unwrap_or(0); // param_3[3]
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // return value

    // Epilogue: drop the frame, then pop pr and the saved registers.
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
