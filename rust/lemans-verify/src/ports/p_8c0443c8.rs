//! 0x8c0443c8 — conditionally free a block and return the block pointer.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15   ; save
//! mov r4,r14                          ; r14 = param_1
//! tst r14,r14 ; bt 0x8c0443de         ; param_1 == 0 -> skip
//! exts.w r5,r0 ; cmp/pl r0 ; bf ...   ; (int16)param_2 <= 0 -> skip
//! mov.l 0x8c0443e8,r0 ; jsr @r0 ; nop ; free(param_1)
//! out: mov r14,r0                     ; return param_1
//! lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! Only the low 16 bits of `r5` are examined, sign-extended by `exts.w`;
//! `r5` itself is not modified, and the loaded `r0`/`exts.w` result are
//! overwritten by the final `mov r14,r0`. On the call arm the returned `T`
//! is whatever the callee leaves (the epilogue sets no flags); on the two
//! early exits it is 1 (from `tst`) when `param_1 == 0`, else 0 (from
//! `cmp/pl`). `r15`, `r14` and `pr` are restored by the epilogue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0443c8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0443ca: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0443cc: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0443ce: tst r14,r14 ; 0x8c0443d0: bt 0x8c0443de
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // 0x8c0443d2: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as u16 as i16 as i32 as u32;
        // 0x8c0443d4: cmp/pl r0 ; 0x8c0443d6: bf 0x8c0443de
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            // 0x8c0443d8: mov.l 0x8c0443e8,r0 (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0443E8).unwrap_or(0x8C01032C);
            // 0x8c0443da: jsr @r0 ; 0x8c0443dc: nop (delay slot)
            cpu.pr = 0x8C0443DE;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0443de: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0443e0: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0443e2: rts ; 0x8c0443e4: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
