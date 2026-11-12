//! 0x8c06cbc0 — stamp an object with the type table, then free it.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! tst r14,r14 ; bt ret                 ; null: r0 = 0, T = 1
//! mov.l 0x8c06cbe4,r1                  ; r1 = 0x8c1e94e8
//! exts.w r5,r0 ; cmp/pl r0             ; T = (int16)r5 > 0
//! bf.s ret ; [r14] = r1                ; the store is the delay slot, always runs
//! mov.l 0x8c06cbe8,r0 ; jsr @r0 ; nop  ; r0 = 0x8c01032c; free(r4)
//! ret: mov r14,r0 ; lds.l pr ; rts ; mov.l r14
//! ```
//!
//! `r5`'s sign-extended low half decides whether the object is freed; the type
//! stamp happens on both non-null paths. `r0` is `r14` at every exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let t = cpu.r[14] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[1] = mem.read_u32(0x8c06cbe4).unwrap_or(0x8c1e94e8);
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let pl = (cpu.r[0] as i32) > 0;
        cpu.set_t(pl);
        // Delay slot of `bf.s`: executed on both outcomes.
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        if pl {
            cpu.r[0] = mem.read_u32(0x8c06cbe8).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c06cbda;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
