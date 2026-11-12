//! 0x8c01a6ec — free a block when its length argument is positive.
//!
//! ```text
//! mov.l r14,@-r15      ; save r14
//! sts.l pr,@-r15       ; save pr
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c01a702     ; null block: return it unchanged, T=1
//! exts.w r5,r0         ; r0 = (int16)r5 (sign-extended word)
//! cmp/pl r0
//! bf    0x8c01a702     ; length <= 0: return the block, T=0
//! mov.l 0x8c01a70c,r0  ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0            ; pr = 0x8c01a702; delay slot is a nop
//! nop
//! 0x8c01a702:
//! mov   r14,r0         ; r0 = block (also clobbers the callee's r0)
//! lds.l @r15+,pr
//! rts                  ; delay slot:
//! mov.l @r15+,r14
//! ```
//!
//! The two early exits leave different T residues (1 for the null block, 0 for
//! a non-positive length); the called path keeps whatever T the callee leaves
//! (free ends its bin walk with cmp/hi false). r0 is always `r14`, so the
//! callee's r0 is dead, but r1/r3/r4/r5/r6 and every store are the callee's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];

    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    if !null {
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c01a70c).unwrap_or(0);
            // `jsr @r0`, delay slot `nop`.
            cpu.pr = 0x8c01a702;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
