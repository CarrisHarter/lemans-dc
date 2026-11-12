//! 0x8c018fc0 — conditional `free`: clear a global, then free `r4` when the
//! sign-extended low half of `r5` is positive.
//!
//! ```text
//! mov.l r14,@-r15          ; save r14
//! sts.l pr,@-r15           ; save pr
//! mov   r4,r14
//! tst   r14,r14
//! bt    0x8c018fdc         ; r4 == 0: nothing at all (BT is not delayed)
//! mov.l 0x8c018fe4,r3      ; r3 = 0x8c23da98
//! mov   #0,r1
//! exts.w r5,r0             ; r0 = sign-extended low half of r5
//! cmp/pl r0
//! bf.s  0x8c018fdc         ; r0 <= 0: skip the free
//! mov.l r1,@r3             ; delay slot: *0x8c23da98 = 0 (always runs)
//! mov.l 0x8c018fe8,r0      ; r0 = 0x8c01032c (thunk to free)
//! jsr   @r0
//! nop
//! mov   r14,r0             ; 0x8c018fdc: return r4
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14          ; delay slot: restore r14
//! ```
//!
//! `bt`/`bf` have no delay slot in SH-4; `bf.s` does, so the global is cleared
//! on every non-null `r4` — both the arm that frees and the arm that skips the
//! free. On the `r4 == 0` exit `r3` is never loaded and keeps its caller value.
//! `exts.w` looks only at the low 16 bits, so `r5 = 0xFFFF0001` frees while
//! `r5 = 0x00008000` does not.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c018fc0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c018fc2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // 0x8c018fc4: mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // 0x8c018fc6: tst r14,r14
    // 0x8c018fc8: bt 0x8c018fdc (not delayed: r3 is not loaded on this exit)
    if cpu.r[14] != 0 {
        // 0x8c018fca: mov.l 0x8c018fe4,r3
        cpu.r[3] = mem.read_u32(0x8c018fe4).unwrap_or(0x8c23da98);
        // 0x8c018fcc: mov #0,r1
        cpu.r[1] = 0;
        // 0x8c018fce: exts.w r5,r0
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32;
        // 0x8c018fd0: cmp/pl r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // 0x8c018fd2: bf.s 0x8c018fdc — delay slot always executes
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // 0x8c018fd4: mov.l r1,@r3
        if positive {
            // 0x8c018fd6: mov.l 0x8c018fe8,r0
            cpu.r[0] = mem.read_u32(0x8c018fe8).unwrap_or(0x8c01032c);
            // 0x8c018fd8: jsr @r0 ; 0x8c018fda: nop (delay slot)
            cpu.pr = 0x8c018fdc;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c018fdc: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c018fde: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c018fe0: rts ; 0x8c018fe2: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
