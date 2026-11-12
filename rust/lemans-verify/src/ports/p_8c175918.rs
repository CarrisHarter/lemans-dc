//! 0x8c175918 — `amSoundIsLooping(loopFlag, theSound)` guard + flag probe.
//!
//! ```text
//! push pr; tst r5,r5; bf L92a (plain: no delay slot)
//! r5 == 0:  jsr 0x8c174564(101, "amSoundIsLooping - loopFlag is NULL")
//!            pr = 0x8c175926, delay slot sets r4 = 101; bra join
//! L92a: tst r4,r4; r3 = 0; bf.s L940; delay [r5] = r3   // store runs both ways
//! r4 == 0:  jsr 0x8c174564(101, "amSoundIsLooping - theSound is NULL")
//!            pr = 0x8c17593a, delay slot sets r4 = 101
//! join: pop pr; rts; delay slot mov #0,r0
//! L940: r0 = [r4+32]; cmp/eq #1,r0; r0 = 1; movt r2; [r5] = r2; pop pr; rts; nop
//! ```
//!
//! Three exits: r5 == 0 reports through the engine reporter and returns 0;
//! r4 == 0 zeroes [r5] via the bf.s delay slot, then reports and returns 0;
//! otherwise [r5] = ([r4+32] == 1) and the return is 1. The `mov #1,r0`
//! between `cmp/eq` and `movt` does not touch T, so movt still sees the
//! compare result. Both reporter sides share one tail: the callee's T
//! survives (bra/lds/rts touch no flags) while the delay slot forces r0 = 0.
//! The callee runs as the registered port, so its full register/memory
//! effects come along verbatim.
//!
//! Residue: r0 = 0 on the reporter paths, 1 on the probe path; r12-r15 and pr
//! restored; T is the callee's exit flag on the reporter paths,
//! ([r4+32] == 1) on the probe path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.t() {
        // r5 == 0: plain bf falls through to the loopFlag-NULL report.
        cpu.r[2] = mem.read_u32(0x8c175970).unwrap_or(0x8c174564);
        cpu.r[5] = mem.read_u32(0x8c17596c).unwrap_or(0x8c1e0d90);
        cpu.pr = 0x8c175926; // jsr @r2
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c174564::run(cpu, mem);
        // bra 0x8c17593a; delay nop
    } else {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        cpu.r[3] = 0; // mov #0,r3
        // bf.s 0x8c175940 — the delay-slot store runs whether it branches or not.
        let to_l940 = !cpu.t();
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // mov.l r3,@r5
        if to_l940 {
            // r4 != 0: [r5] = ([r4+32] == 1), return 1.
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(32)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
            cpu.r[0] = 1; // mov #1,r0 (does not touch T)
            cpu.r[2] = u32::from(cpu.t()); // movt r2
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // mov.l r2,@r5
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop
        }
        // r4 == 0: theSound-NULL report (the [r5] = 0 store above already ran).
        cpu.r[3] = mem.read_u32(0x8c175970).unwrap_or(0x8c174564);
        cpu.r[5] = mem.read_u32(0x8c175974).unwrap_or(0x8c1e0db4);
        cpu.pr = 0x8c17593a; // jsr @r3
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c174564::run(cpu, mem);
    }
    // join: pop pr; rts; delay mov #0,r0.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0;
}
