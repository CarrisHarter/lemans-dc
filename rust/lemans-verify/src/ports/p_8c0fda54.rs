//! 0x8c0fda54 — copy four floats from r5+0x30..0x3c to the pointer at @r15.
//!
//! ```text
//! mov.l @r15,r3            ; dst (second arg passed on the stack)
//! mov r5,r0 ; add #48,r0 ; fmov @r0,fr0     ; fr0 = src[0x30]
//! mov r5,r1 ; add #52,r1 ; fmov fr0,@r3     ; dst[0] = fr0
//! mov r5,r0 ; fmov @r1,fr0                  ; fr0 = src[0x34]
//! add #60,r5                                ; r5 = r5 + 0x3c  <-- clobbers arg
//! add #56,r0 ; mov r3,r1 ; add #4,r1
//! fmov fr0,@r1                              ; dst[1] = fr0
//! fmov @r0,fr0                              ; fr0 = src[0x38]
//! mov r3,r0 ; add #8,r0 ; fmov fr0,@r0      ; dst[2] = fr0
//! add #12,r3 ; fmov @r5,fr0                 ; fr0 = src[0x3c] (r5 now +0x3c)
//! rts ; fmov fr0,@r3                        ; delay slot: dst[3] = fr0
//! ```
//! The decompiled SZ-bit branch is NOT in the code: four unconditional single
//! fmovs. Hidden side effects: r5 ends at src+0x3c, r0 = dst+8, r1 = dst+4,
//! r3 = dst+12, fr0 = src[0x3c]; stores land in order dst+0,+4,+8,+12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r3
    cpu.r[0] = cpu.r[5].wrapping_add(48); // add #48,r0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
    cpu.r[1] = cpu.r[5].wrapping_add(52); // add #52,r1
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
    cpu.r[5] = cpu.r[5].wrapping_add(60); // add #60,r5
    cpu.r[0] = cpu.r[0].wrapping_add(56); // add #56,r0
    cpu.r[1] = cpu.r[3].wrapping_add(4); // add #4,r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
    cpu.r[0] = cpu.r[3].wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[3] = cpu.r[3].wrapping_add(12); // add #12,r3
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr0 (r5 = orig+0x3c)
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // rts delay slot: fmov fr0,@r3
}
