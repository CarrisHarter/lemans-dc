//! 0x8c0796b8 — free a handle's inner block and maybe the handle itself.
//!
//! ```text
//! mov.l r14,@-r15
//! mov.l r13,@-r15
//! mov   r5,r14          ; r14 = caller's short count
//! sts.l pr,@-r15
//! mov   r4,r13          ; r13 = handle
//! tst   r13,r13
//! bt    0x8c0796de      ; null handle: return it untouched
//! mov.l @(12,r13),r4    ; r4 = inner block
//! tst   r4,r4
//! bt    0x8c0796d2
//! mov.l 0x8c0796e8,r0   ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0             ; free(inner block)
//! nop
//! 0x8c0796d2:
//! exts.w r14,r0         ; r0 = sign-extended low half of the count
//! cmp/pl r0
//! bf    0x8c0796de
//! mov.l 0x8c0796e8,r0   ; r0 = 0x8c01032c
//! jsr   @r0             ; free(handle)
//! mov   r13,r4          ; delay slot
//! 0x8c0796de:
//! mov   r13,r0
//! lds.l @r15+,pr
//! mov.l @r15+,r13
//! rts
//! mov.l @r15+,r14       ; delay slot
//! ```
//!
//! Hidden state: the frame (three pushes, filtered as stack writes but real
//! for r15), the per-path T residue (`tst` on the null exit, `cmp/pl` on the
//! non-positive exit, the last callee's flags after a `free`), the free-list
//! stores, and whatever `free` leaves in r0..r6. The callee's own residue is
//! delegated to its port.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[15].wrapping_sub(4), cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15].wrapping_sub(4), cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15].wrapping_sub(4), cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[13] = cpu.r[4];
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] != 0 {
            cpu.r[0] = mem.read_u32(0x8c0796e8).unwrap_or(0x8c01032c);
            crate::registry::p_8c01032c::run(cpu, mem); // jsr @r0 ; nop
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive); // cmp/pl r0
        if positive {
            cpu.r[0] = mem.read_u32(0x8c0796e8).unwrap_or(0x8c01032c);
            cpu.r[4] = cpu.r[13]; // delay slot of `jsr`
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
