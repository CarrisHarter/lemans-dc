//! 0x8c0dfbec — clear a global, then free the caller's block if the flag is positive.
//!
//! ```text
//! mov.l r14,@-r15
//! sts.l pr,@-r15
//! mov   r4,r14          ; r14 = block
//! tst   r14,r14
//! bt    0x8c0dfc08
//! mov.l 0x8c0dfc10,r3   ; r3 = 0x8c27a928
//! mov   #0,r1
//! exts.w r5,r0          ; only the low half of the flag counts, signed
//! cmp/pl r0
//! bf.s  0x8c0dfc08
//! mov.l r1,@r3          ; delay slot: *0x8c27a928 = 0, runs on both paths
//! mov.l 0x8c0dfc14,r0   ; r0 = 0x8c01032c (free thunk at 0x8c159fc0)
//! jsr   @r0
//! nop
//! 0x8c0dfc08:
//! mov   r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14
//! ```
//!
//! The callee gets the untouched `r4`, so it frees the block itself; `pr` is
//! the address after the `jsr` and the delay slot is a `nop`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: push r14 and pr.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let t = cpu.r[14] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[3] = mem.read_u32(0x8c0dfc10).unwrap_or(0x8c27a928);
        cpu.r[1] = 0;
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // Delay slot of `bf.s`: the store executes whether or not we branch.
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c0dfc14).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c0dfc08; // jsr @r0 sets pr; delay slot is `nop`
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // Epilogue: return r14, pop pr and r14 (`rts` delay slot pops r14).
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
