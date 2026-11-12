//! 0x8c172b64 — flag-gated call into the bounded strcpy dispatch.
//!
//! ```text
//! r5=[0x8c172c10] (0x8c20d430); push pr; r3=[r5]
//! tst r3,r3; bt Lstore (plain: no delay slot)
//! r3=[0x8c172bf8] (0x8c1725fc); r5=[0x8c172c04] (0x8c1df2bc)
//! jsr @r3; delay: r4=0
//! pop pr; rts; delay: r0=0
//! Lstore: r0=1; [r5]=r4; pop pr; rts; delay nop
//! ```
//!
//! The `bt` tests the `tst r3,r3` result: flag==0 takes the branch to the
//! store path (return 1), flag!=0 falls through to
//! `0x8c1725fc(0, 0x8c1df2bc)` and returns 0. The pool pointer 0x8c1df2bc
//! lands on a NUL pad byte just before "acIntInit…" (0x8c1df2c0), so the
//! callee copies an empty string. Note r3 keeps the (zero) flag on the
//! store path but is clobbered by the jsr-target load on the call path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c172c10).unwrap_or(0x8c20d430);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.t() {
        // Lstore (bt taken: flag was 0).
        cpu.r[0] = 1; // mov #1,r0
        let _ = mem.write_u32(cpu.r[5], cpu.r[4]); // mov.l r4,@r5
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // rts; delay nop
    } else {
        // Call path (flag nonzero).
        cpu.r[3] = mem.read_u32(0x8c172bf8).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c172c04).unwrap_or(0x8c1df2bc);
        cpu.pr = 0x8c172b76; // jsr @r3
        cpu.r[4] = 0; // delay slot: mov #0,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // rts; delay: r0=0
        cpu.r[0] = 0; // mov #0,r0
    }
}
