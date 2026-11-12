//! 0x8c172b3c — emit a message when a flag word is set, else rotate two globals.
//!
//! ```text
//! r6=[0x8c172c08] (0x8c20d42c); push pr; r3=[r6]; tst r3,r3
//! bt L54 (plain: no delay slot)
//! r3=[0x8c172c0c] (0x8c174564); r5=[0x8c172c04] (0x8c1df2bc)
//! jsr @r3; delay: r4=0; pop pr; rts; delay: r0=0
//! L54: r5=[0x8c172c00] (0x8c20d428); r0=1; r2=[r5]
//! [r6]=r2; [r5]=r4; pop pr; rts; delay nop
//! ```
//!
//! The `tst`/`bt` tests the flag at 0x8c20d42c: nonzero falls through to the
//! emit call `0x8c174564(0, str@0x8c1df2bc)` and returns 0; zero branches to
//! the swap path which rotates the globals ([0x8c20d42c]=[0x8c20d428],
//! [0x8c20d428]=r4) and returns 1. The callee runs as the registered port,
//! so its full register/memory effects come along verbatim.
//!
//! Residue: r0=0/1 per path; r6=0x8c20d42c and r5=pool word on both paths;
//! r3/r4 clobbered on the call path (r4=0 in the jsr delay slot); r2=[old
//! 0x8c20d428] on the swap path; pr restored. T is the callee's exit flag
//! on the call path, 1 (from `tst 0,0`) on the swap path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(0x8c172c08).unwrap_or(0x8c20d42c);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // push pr
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap_or(0); // mov.l @r6,r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    // bt L54 — plain branch, no delay slot; taken when the flag is zero.
    if !cpu.t() {
        // Flag set: emit via 0x8c174564(0, str@0x8c1df2bc), return 0.
        cpu.r[3] = mem.read_u32(0x8c172c0c).unwrap_or(0x8c174564);
        cpu.r[5] = mem.read_u32(0x8c172c04).unwrap_or(0x8c1df2bc);
        cpu.pr = 0x8c172b4e; // jsr @r3
        cpu.r[4] = 0; // delay slot: mov #0,r4
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // rts; delay slot: mov #0,r0
        cpu.r[0] = 0;
    } else {
        // Flag clear: rotate the globals, return 1.
        cpu.r[5] = mem.read_u32(0x8c172c00).unwrap_or(0x8c20d428);
        cpu.r[0] = 1; // mov #1,r0
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r2
        let _ = mem.write_u32(cpu.r[6], cpu.r[2]); // mov.l r2,@r6
        let _ = mem.write_u32(cpu.r[5], cpu.r[4]); // mov.l r4,@r5
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        // rts; delay slot: nop
    }
}
