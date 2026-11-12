//! 0x8c018f60 — conditional `free`: clear a global, then free `r4` if the low
//! half of `r5` is positive.
//!
//! ```text
//! mov.l r14,@-r15          ; save r14
//! sts.l pr,@-r15           ; save pr
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c018f7c         ; r4 == 0: nothing at all
//! mov.l 0x8c018f84,r3      ; r3 = 0x8c23da80
//! mov   #0,r1
//! exts.w r5,r0             ; r0 = sign-extended low half of r5
//! cmp/pl r0
//! bf.s  0x8c018f7c         ; r0 <= 0: skip the free
//! mov.l r1,@r3             ; delay slot: *0x8c23da80 = 0 (always runs)
//! mov.l 0x8c018f88,r0      ; r0 = 0x8c01032c (thunk to free)
//! jsr   @r0
//! nop
//! mov   r14,r0             ; 0x8c018f7c: return r4
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14          ; delay slot: restore r14
//! ```
//!
//! The global is cleared for every non-null `r4`, including the branch that
//! skips the free. Both exit branches have their delay slots execute. Note
//! `exts.w` looks only at the low 16 bits, so `r5 = 0xFFFF0001` takes the free
//! path while `r5 = 0x00008000` does not.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    let null = cpu.r[14] == 0; // tst r14,r14
    cpu.set_t(null);
    if !null {
        cpu.r[3] = mem.read_u32(0x8c018f84).unwrap_or(0x8c23da80); // mov.l 0x8c018f84,r3
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        let positive = (cpu.r[0] as i32) > 0; // cmp/pl r0
        cpu.set_t(positive);
        // bf.s 0x8c018f7c, delay slot always executes
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
        if positive {
            cpu.r[0] = mem.read_u32(0x8c018f88).unwrap_or(0x8c01032c); // mov.l 0x8c018f88,r0
            cpu.pr = 0x8c018f7c; // jsr @r0
            crate::registry::p_8c01032c::run(cpu, mem); // delay slot: nop
        }
    }

    // 0x8c018f7c
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
