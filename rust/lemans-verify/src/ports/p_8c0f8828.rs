//! 0x8c0f8828 — dispatch on the low byte of r5.
//!
//! The pushed `pr` is restored before `rts`, so the entry value comes back.
//! `tst` is the only flag-setting instruction and both tails (`mov.l`/`add`)
//! leave the flags alone, so T at exit is `(r5 & 0xff) == 0`.
//!
//! `r5 & 0xff != 0` calls 0x8c14fe10 on the object at `r4 + 44`; zero calls
//! 0x8c14fe68. The `add #44,r4` at 0x8c0f8834/0x8c0f883e is the jsr delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0f8828: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0f882a: extu.b r5,r0 ; 0x8c0f882c: tst r0,r0
    cpu.r[0] = cpu.r[5] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    // 0x8c0f882e: bt 0x8c0f883a
    if cpu.t() {
        // 0x8c0f883a: mov.l 0x8c0f884c,r0 (0x8c14fe68)
        cpu.r[0] = mem.read_u32(0x8c0f884c).unwrap_or(0x8c14fe68);
        // 0x8c0f883c: jsr @r0 ; 0x8c0f883e: add #44,r4 (delay)
        cpu.r[4] = cpu.r[4].wrapping_add(44);
        cpu.pr = 0x8c0f8840;
        crate::registry::p_8c14fe68::run(cpu, mem);
    } else {
        // 0x8c0f8830: mov.l 0x8c0f8848,r0 (0x8c14fe10)
        cpu.r[0] = mem.read_u32(0x8c0f8848).unwrap_or(0x8c14fe10);
        // 0x8c0f8832: jsr @r0 ; 0x8c0f8834: add #44,r4 (delay)
        cpu.r[4] = cpu.r[4].wrapping_add(44);
        cpu.pr = 0x8c0f8836;
        crate::registry::p_8c14fe10::run(cpu, mem);
        // 0x8c0f8836: bra 0x8c0f8840 ; 0x8c0f8838: nop
    }
    // 0x8c0f8840: lds.l @r15+,pr ; 0x8c0f8842: rts ; 0x8c0f8844: nop
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
