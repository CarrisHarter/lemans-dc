//! 0x8c1261a8 — release a handle: if param_1 is non-null, clear the global at
//! 0x8c2ab5e0, and when the sign-extended low half of param_2 is positive,
//! free(param_1) through the 0x8c01032c thunk. Returns param_1.
//!
//! ```text
//! mov.l r14,@-r15      ; push r14
//! sts.l pr,@-r15       ; push pr
//! mov   r4,r14         ; r14 = param_1
//! tst   r14,r14        ; T = param_1 == 0
//! bt    0x8c1261c4     ; null: skip everything, still returns r14
//! mov.l 0x8c1261cc,r3  ; r3 = 0x8c2ab5e0 (literal pool)
//! mov   #0,r1
//! exts.w r5,r0         ; r0 = (int16)param_2
//! cmp/pl r0            ; T = (int16)param_2 > 0
//! bf.s  0x8c1261c4     ; not positive: skip the call …
//! mov.l r1,@r3         ; … but the delay slot always stores 0 to the global
//! mov.l 0x8c1261d0,r0  ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0            ; pr = 0x8c1261c4, r4/r5 unchanged
//! nop
//! mov   r14,r0         ; return param_1
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14      ; delay slot: restore the caller's r14
//! ```
//!
//! Only the low 16 bits of param_2 matter. The store to 0x8c2ab5e0 happens on
//! both non-null paths, including the one that skips the call, because it is a
//! delay-slot instruction. r14 and pr end at their entry values; T is left by
//! the tst on the null exit and by the callee otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1261a8: mov.l r14,@-r15 ; 0x8c1261aa: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1261ac: mov r4,r14 ; 0x8c1261ae: tst r14,r14
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c1261b0: bt 0x8c1261c4
    if cpu.r[14] != 0 {
        // 0x8c1261b2: mov.l 0x8c1261cc,r3 (0x8c2ab5e0)
        cpu.r[3] = mem.read_u32(0x8C1261CC).unwrap_or(0x8C2AB5E0);
        // 0x8c1261b4: mov #0,r1
        cpu.r[1] = 0;
        // 0x8c1261b6: exts.w r5,r0 ; 0x8c1261b8: cmp/pl r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c1261ba: bf.s 0x8c1261c4 ; 0x8c1261bc: mov.l r1,@r3 (delay slot)
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        if cpu.t() {
            // 0x8c1261be: mov.l 0x8c1261d0,r0 (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8C1261D0).unwrap_or(0x8C01032C);
            // 0x8c1261c0: jsr @r0 ; 0x8c1261c2: nop
            cpu.pr = 0x8C1261C4;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c1261c4: mov r14,r0 ; 0x8c1261c6: lds.l @r15+,pr
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1261c8: rts ; 0x8c1261ca: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
