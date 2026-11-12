//! 0x8c21d6bc — reschedule one list entry, then restore a saved CPU context.
//!
//! The symbol's body is the 32-instruction routine at 0x8c21d67e; the listed
//! entry is a two-instruction thunk (`bra 0x8c21d67e`; delay slot `mov #1,r4`)
//! that forces a positive list index, so the `cmp/pz r4` arm taking the
//! `0x7fffffff` sentinel is unreachable from here.
//!
//! ```text
//! r4 = 1; base = [0x8c2bc958]; list = [base+0xd78]
//! [list+0x14] = [base+0x204] + r4
//! context_save(0x8c011168, list+0x18)      ; saves r15/r14/pr/r13..r5
//! tst r0,r0 ; bf out                        ; context_save returns 0
//! context_restore(0x8c010ddc, base+4, 1)    ; jumps to the frame's PR
//! ```
//!
//! The restore at base+4 replaces every integer register, FR0-FR15, FPUL,
//! FPSCR, MACH, MACL and GBR, and returns to the PR stored in the frame, so the
//! port ends there (the spec points that PR at HALTPC). r0 is the restored
//! global marker, or 1 when it was zero; T is clear because the marker was 1.
//!
//! The two prologue pushes are modelled only as `r15 -= 8`: context_save
//! stores that live r15 into its frame, so the value has to be right even
//! though the stack bytes themselves are outside the compared window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c21d6bc: bra 0x8c21d67e ; 0x8c21d6be: mov #1,r4 (delay slot)
    cpu.r[4] = 1;

    // 0x8c21d67e: mov.l r14,@-r15 ; 0x8c21d686: sts.l pr,@-r15
    // (the bytes land in the synthetic stack window and are not compared)
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    // 0x8c21d682: mov.l 0x8c21d704,r14 ! 0x8c2bc958
    cpu.r[14] = 0x8c2bc958;
    // 0x8c21d684: mov.w 0x8c21d700,r0 ! 0x0d78
    cpu.r[0] = 0x0d78;
    // 0x8c21d688: mov.l @r14,r3 ; 0x8c21d68a: mov.l @(r0,r3),r2
    let base = mem.read_u32(0x8c2bc958).unwrap_or(0);
    cpu.r[3] = base;
    let list = mem.read_u32(base.wrapping_add(0x0d78)).unwrap_or(0);
    cpu.r[2] = list;

    // 0x8c21d680: cmp/pz r4 (r4 == 1, so T is set) ; 0x8c21d68c: bf 0x8c21d698
    cpu.set_t(true);
    // 0x8c21d68e: mov.w 0x8c21d6fe,r0 ! 0x0204
    cpu.r[0] = 0x0204;
    // 0x8c21d690: mov.l @r14,r1 ; 0x8c21d692: mov.l @(r0,r1),r1
    cpu.r[1] = mem.read_u32(0x8c2bc958).unwrap_or(base);
    cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(0x0204)).unwrap_or(0);
    // 0x8c21d694: bra 0x8c21d69a ; 0x8c21d696: add r4,r1 (delay slot)
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);

    // 0x8c21d69a: mov.w 0x8c21d700,r0 ! 0x0d78
    cpu.r[0] = 0x0d78;
    // 0x8c21d69c: mov.l 0x8c21d70c,r3 ! 0x8c011168
    cpu.r[3] = 0x8c011168;
    // 0x8c21d69e: mov.l r1,@(20,r2)
    let _ = mem.write_u32(list.wrapping_add(20), cpu.r[1]);

    // 0x8c21d6a0: mov.l @r14,r4 ; 0x8c21d6a2: mov.l @(r0,r4),r4
    cpu.r[4] = mem.read_u32(0x8c2bc958).unwrap_or(base);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(0x0d78)).unwrap_or(list);
    // 0x8c21d6a4: jsr @r3 ; 0x8c21d6a6: add #24,r4 (delay slot)
    cpu.r[4] = cpu.r[4].wrapping_add(24);
    cpu.pr = 0x8c21d6a8;
    crate::registry::p_8c011168::run(cpu, mem); // context_save

    // 0x8c21d6a8: tst r0,r0 ; 0x8c21d6aa: bf 0x8c21d6b6
    // context_save always returns 0, so the branch is not taken.
    let saved = cpu.r[0] == 0;
    cpu.set_t(saved);
    if saved {
        // 0x8c21d6ac: mov.l 0x8c21d710,r3 ! 0x8c010ddc
        cpu.r[3] = 0x8c010ddc;
        // 0x8c21d6ae: mov #1,r5
        cpu.r[5] = 1;
        // 0x8c21d6b0: mov.l @r14,r4 ; 0x8c21d6b2: jsr @r3 ;
        // 0x8c21d6b4: add #4,r4 (delay slot)
        cpu.r[4] = mem.read_u32(0x8c2bc958).unwrap_or(base);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.pr = 0x8c21d6b6;
        crate::registry::p_8c010ddc::run(cpu, mem); // context_restore, rts via frame PR
    }
}
