//! 0x8c172ea4 — dispatch an int-array-start-offset error through 0x8c1725fc.
//!
//! ```text
//! push pr; tst r4,r4; bt L_null (plain: no delay slot)
//! r2=[r4]; tst r2,r2; bf L_nonempty (plain)
//! L_null: 0x8c1725fc(101, "acSystemGetIntArrayStartOffset - ... is NULL"); bra L_join
//! L_nonempty: r2=[0x8c20d450]; tst r2,r2; bf L_slot (plain)
//!   0x8c1725fc(0x12f, "acSystemGetIntArrayStartOffset - Driver not installed")
//! L_join: pop pr; rts; delay mov #0,r0
//! L_slot: [r4]=[0x8c2b5280]; pop pr; rts; delay nop (r0 stays 1)
//! ```
//!
//! A null `r4` or a zero word at `[r4]` reports the NULL-pointer message with
//! code 101; a nonzero `[r4]` checks the installed flag at `0x8c20d450` —
//! clear reports the not-installed message with code 0x12f, set copies the
//! table slot at `0x8c2b5280` into `[r4]` and returns 1. Both message paths
//! return 0 via the `rts` delay slot. The callee runs as the registered port,
//! so its full register/memory effects (including its exit T) come along
//! verbatim; the slot path leaves T clear from its `tst`.
//!
//! Residue: r0 = 0/1 by path; r4/r5 hold the last call's arguments on the
//! message paths; r2/r3 are scratch; r15 and pr restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    // tst r4,r4; bt L_null — r4==0 takes the NULL-message path.
    let mut null_path = cpu.r[4] == 0;
    cpu.set_t(null_path);
    if !null_path {
        cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r2
        null_path = cpu.r[2] == 0; // tst r2,r2; bf L_nonempty
        cpu.set_t(null_path);
    }
    if null_path {
        // L_null: 0x8c1725fc(101, str@0x8c1df3d4).
        cpu.r[3] = mem.read_u32(0x8c172f1c).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c172f28).unwrap_or(0x8c1df3d4);
        cpu.pr = 0x8c172eb8; // jsr @r3
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        // bra L_join; delay nop
    } else {
        // L_nonempty: probe the installed flag.
        cpu.r[3] = mem.read_u32(0x8c172f14).unwrap_or(0x8c20d450);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r2
        cpu.set_t(cpu.r[2] == 0); // tst r2,r2; bf L_slot
        if !cpu.t() {
            // L_slot: [r4]=[0x8c2b5280], r0=1.
            cpu.r[2] = mem.read_u32(0x8c172f2c).unwrap_or(0x8c2b5280);
            cpu.r[0] = 1; // mov #1,r0
            cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r3
            let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop
        }
        // Flag clear: 0x8c1725fc(0x12f, str@0x8c1df39c).
        cpu.r[2] = mem.read_u32(0x8c172f1c).unwrap_or(0x8c1725fc);
        cpu.r[4] = mem.read_u16(0x8c172ef6).unwrap_or(0) as i16 as i32 as u32; // 0x12f
        cpu.r[5] = mem.read_u32(0x8c172f20).unwrap_or(0x8c1df39c);
        cpu.pr = 0x8c172ece; // jsr @r2
        // delay nop
        crate::registry::p_8c1725fc::run(cpu, mem);
        // falls into L_join
    }
    // L_join: pop pr; rts; delay mov #0,r0.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0;
}
