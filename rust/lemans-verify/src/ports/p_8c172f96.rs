//! 0x8c172f96 — store 64 through r4 when the driver flag is set, else report.
//!
//! ```text
//! push pr; tst r4,r4; bf taken iff r4 != 0 (plain bf: no delay slot)
//! r4 == 0:  jsr 0x8c1725fc(101, "acSystemGetIntArrayStartOffset - interruptArrayLength is NULL")
//!            pr = 0x8c172fa4, delay slot sets r4 = 101; bra join
//! r4 != 0:  r3 = [0x8c20d450]; tst r3,r3; bf taken iff r3 != 0
//!   flag set:   [r4] = 64; pop pr; rts; r0 = 1 (T stays 0)
//!   flag clear: jsr 0x8c1725fc(0x12f, "acSystemGetIntArrayStartOffset - Driver not installed")
//!            pr = 0x8c172fba, delay slot nop
//! join: pop pr; rts; delay slot mov #0,r0
//! ```
//!
//! Both reporter sides share one tail: the callee's T survives (bra/lds/rts
//! touch no flags) while the delay slot forces r0 = 0. The `mov.w` pool word
//! (0x12f) is sign-extended by the load. The callees run as the registered
//! port, so their full register/memory effects come along verbatim.
//!
//! Residue: r0 = 0 on the reporter paths, 1 on the store path; r12-r15 and pr
//! restored; T is the callee's exit flag on the reporter paths, 0 after a store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.t() {
        // r4 == 0: report the NULL interrupt-array length.
        cpu.r[2] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c173034).unwrap_or(0x8c1df488);
        cpu.pr = 0x8c172fa4; // jsr @r2
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        // bra join; delay nop
    } else {
        cpu.r[2] = mem.read_u32(0x8c17301c).unwrap_or(0x8c20d450);
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r3
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if !cpu.t() {
            // Flag set: store 64 through r4 and return 1.
            cpu.r[2] = 64; // mov #64,r2
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[4], cpu.r[2]); // mov.l r2,@r4
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop
        }
        // Flag clear: report the missing driver.
        cpu.r[3] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc);
        cpu.r[4] = mem.read_u16(0x8c173016).unwrap_or(0) as i16 as i32 as u32; // mov.w: sign-extends
        cpu.r[5] = mem.read_u32(0x8c173020).unwrap_or(0x8c1df39c);
        cpu.pr = 0x8c172fba; // jsr @r3
        // delay slot: nop
        crate::registry::p_8c1725fc::run(cpu, mem);
    }
    // join: pop pr; rts; delay mov #0,r0.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0;
}
