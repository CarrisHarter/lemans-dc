//! 0x8c0a579c — build a 16-byte float record on the stack and hand it to two
//! empty stubs.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; fldi1 fr0 ; sts.l pr,@-r15 ; add #-16,r15
//! mov #0,r0 ; fmov fr0,@(r0,r15)            ; rec[0] = 1.0
//! mov #4,r2 ; mov.l 0x8c0a57d8,r0 ; fldi0 fr0
//! add r15,r2 ; fmov fr0,@r2                 ; rec[1] = 0.0
//! mov r15,r5 ; mov.l @r0,r14 ; mov #8,r2
//! add r15,r2 ; fmov fr0,@r2                 ; rec[2] = 0.0
//! mov r14,r4 ; mov #12,r2 ; mov.l 0x8c0a57dc,r0 ; add r15,r2
//! jsr @r0 ; fmov fr0,@r2                    ; rec[3] = 0.0 (delay slot)
//! mov.l 0x8c0a57e0,r0 ; mov r14,r4
//! jsr @r0 ; mov r15,r5                      ; delay slot
//! add #16,r15 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//! The literal at 0x8c0a57d8 is the data pointer 0x8c2ab268 (dereferenced with
//! `mov.l @r0,r14`), not a call target. The two jsr sites load 0x8c0fdc58 and
//! 0x8c0fdc5c, both `rts ; nop` stubs, so the surviving effects are the stack
//! scratch record, `fr0 = 0.0`, and the scratch registers (`r0` = the second
//! stub, `r2` = record+12, `r4` = 0, `r5` = record base); `r14`/`pr`/`r15` are
//! restored by the epilogue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; fldi1 fr0 ; sts.l pr,@-r15 ; add #-16,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.fr[0] = 1.0;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    // mov #0,r0 ; fmov fr0,@(r0,r15)
    cpu.r[0] = 0;
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[0]); // rec[0]
    // mov #4,r2 ; mov.l 0x8c0a57d8,r0 ; fldi0 fr0 ; add r15,r2 ; fmov fr0,@r2
    cpu.r[2] = 4;
    cpu.r[0] = mem.read_u32(0x8c0a57d8).unwrap_or(0); // 0x8c2ab268
    cpu.fr[0] = 0.0;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // rec[1]
    // mov r15,r5 ; mov.l @r0,r14 ; mov #8,r2 ; add r15,r2 ; fmov fr0,@r2
    cpu.r[5] = cpu.r[15];
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // rec[2]
    // mov r14,r4 ; mov #12,r2 ; mov.l 0x8c0a57dc,r0 ; add r15,r2
    cpu.r[4] = cpu.r[14];
    cpu.r[2] = 12;
    cpu.r[0] = mem.read_u32(0x8c0a57dc).unwrap_or(0); // 0x8c0fdc58
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // jsr @r0 ; fmov fr0,@r2 (delay slot)
    cpu.pr = 0x8c0a57c8;
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // rec[3]
    crate::registry::p_8c0fdc58::run(cpu, mem);
    // mov.l 0x8c0a57e0,r0 ; mov r14,r4
    cpu.r[0] = mem.read_u32(0x8c0a57e0).unwrap_or(0); // 0x8c0fdc5c
    cpu.r[4] = cpu.r[14];
    // jsr @r0 ; mov r15,r5 (delay slot)
    cpu.pr = 0x8c0a57d0;
    cpu.r[5] = cpu.r[15];
    crate::registry::p_8c0fdc5c::run(cpu, mem);
    // add #16,r15 ; lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
