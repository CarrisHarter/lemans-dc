//! 0x8c056bcc — clear a pointer, then free it when the signed halfword
//! argument is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15     ; save r14 and pr
//! mov   r4,r14
//! tst   r14,r14 ; bt 0x8c056be6        ; r4 == 0: no store, no free
//! mov.l 0x8c056bf0,r1                  ; r1 = 0x8c1e8f88
//! exts.w r5,r0
//! cmp/pl r0 ; bf.s 0x8c056be6          ; r5 low half <= 0: no free
//! mov.l r1,@r14                        ; delay slot: store runs on BOTH outcomes
//! mov.l 0x8c056bf4,r0 ; jsr @r0 ; nop  ; free(r4) at 0x8c01032c
//! mov r14,r0
//! lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! The store of the 0x8c1e8f88 sentinel happens whenever `r4 != 0`, because it
//! sits in the `bf.s` delay slot. `r5` is only inspected as an `exts.w`
//! halfword, so its upper bits never matter. `jsr` overwrites `pr` with
//! 0x8c056be6, but the epilogue reloads the caller's `pr` from the stack, so
//! the function returns with `r0 = r4` and `pr`/`r14`/`r15` restored; the final
//! T is the last flag-setting instruction: the entry `tst` on the null path,
//! `cmp/pl` on the no-free path, or whatever free leaves.
//!
//! Literals are read from the image rather than hardcoded, as the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c056bcc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c056bce: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c056bd0: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c056bd2: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c056bd4: bt 0x8c056be6
    if !cpu.t() {
        // 0x8c056bd6: mov.l 0x8c056bf0,r1
        cpu.r[1] = mem.read_u32(0x8c056bf0).unwrap_or(0); // 0x8c1e8f88
        // 0x8c056bd8: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as u16 as i16 as i32 as u32;
        // 0x8c056bda: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c056bdc: bf.s 0x8c056be6; 0x8c056bde: mov.l r1,@r14 (delay)
        let free_it = cpu.t();
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        if free_it {
            // 0x8c056be0: mov.l 0x8c056bf4,r0
            cpu.r[0] = mem.read_u32(0x8c056bf4).unwrap_or(0); // 0x8c01032c
            // 0x8c056be2: jsr @r0; 0x8c056be4: nop (delay slot)
            cpu.pr = 0x8c056be6;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c056be6: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c056be8: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c056bea: rts; 0x8c056bec: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
