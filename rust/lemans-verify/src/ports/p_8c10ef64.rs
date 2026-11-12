//! 0x8c10ef64 — NUL-terminate a 16-byte field when its pointer differs from a
//! reference pointer.
//!
//! `r4` is saved in `r14`; when `r4 == r5` the function returns `r4`
//! unchanged (T=1 from `cmp/eq`, `bra` delay slot puts `r14` in `r0`).
//! Otherwise it calls strncpy (`0x8c011344`) with `r4`/`r5` untouched and
//! `r6 = 15` (delay slot of `jsr`), then stores a zero byte at `r14+15` and
//! returns `r14`. That call clobbers r0/r1/r3/r5/r7 and leaves T=1; the
//! remaining instructions are flag-neutral, so T stays 1 either way.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    let t = cpu.r[14] == cpu.r[5]; // cmp/eq r5,r14
    cpu.set_t(t);
    if t {
        cpu.r[0] = cpu.r[14]; // delay slot of `bra`
    } else {
        cpu.r[0] = mem.read_u32(0x8c10ef84).unwrap_or(0); // mov.l @(0x8c10ef84),r0
        cpu.r[6] = 15; // delay slot of `jsr`
        crate::registry::p_8c011344::run(cpu, mem);
        cpu.r[0] = 0; // mov #0,r0
        let _ = mem.write_u8(cpu.r[14].wrapping_add(15), cpu.r[0] as u8);
        cpu.r[0] = cpu.r[14]; // mov r14,r0
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
