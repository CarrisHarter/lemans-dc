//! 0x8c175c72 — `amSoundIsPlaying(sound)`.
//!
//! ```text
//! if (sound == 0) {
//!     report(101, "amSoundIsPlaying - theSound is NULL");  // 0x8c174564
//!     return 0;
//! }
//! return [sound + 0x38] != 0;      // r3 = field, T = (field == 0)
//! ```
//!
//! Both T-flags are observable: the null path leaves T from the reporter's
//! strcpy byte tail, the non-null path leaves T = `[sound+0x38] == 0`.
//!
//! The null path is the engine's error reporter 0x8c174564: it stores the
//! status word 101 at 0x8c2b5590 (0x8c2b5528 + 0x68), then formats r5 into
//! the fixed buffer at 0x8c2b552c through strlen (0x8c0112f4) plus either
//! strcpy (0x8c016484, r0 = dst / r1 = src) or, when the message is at least
//! 0xff bytes, strncpy (0x8c011344) and a NUL terminator. The three callees
//! are registered ports, so their residue (r0/r1/r2, the restored r3/r4 and
//! the write log) is inherited exactly. The reporter's own pushes are inside
//! the synthetic stack window and net to zero on r15; its epilogue restores
//! r12/r13/r14 and pr, so this port saves those explicitly.

use sh4_oracle::{Cpu, Memory};

/// 0x8c174564 — the engine's error reporter, inlined.
fn report(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [cpu.r[12], cpu.r[13], cpu.r[14]];
    let msg = cpu.r[5]; // mov r5,r13
    cpu.r[13] = msg;
    let msg_null = msg == 0;
    cpu.set_t(msg_null); // tst r13,r13
    cpu.r[0] = 0x104; // mov.w 0x8c1745da,r0
    cpu.r[14] = 0x8c2b5528; // mov.l 0x8c1745e0,r14
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[4]); // delay slot
    let limit = 0xffu32; // mov.w 0x8c1745dc,r12

    if msg_null {
        cpu.r[1] = 0x8c1e0348; // mov.l 0x8c1745e4,r1
    } else {
        cpu.r[2] = 0x8c0112f4; // mov.l 0x8c1745e8,r2
        cpu.r[12] = limit;
        cpu.r[4] = msg; // delay slot: mov r13,r4
        crate::registry::p_8c0112f4::run(cpu, mem);
        let long = cpu.r[0] >= cpu.r[12]; // cmp/hs r12,r0
        cpu.set_t(long);
        if long {
            cpu.r[2] = 0x8c011344; // mov.l 0x8c1745f0,r2
            cpu.r[4] = cpu.r[14];
            cpu.r[6] = cpu.r[12];
            cpu.r[5] = cpu.r[13];
            cpu.r[4] = cpu.r[14].wrapping_add(4); // delay slot: add #4,r4
            crate::registry::p_8c011344::run(cpu, mem);
            cpu.r[3] = cpu.r[14].wrapping_add(4);
            cpu.r[2] = 0;
            cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[3]);
            let _ = mem.write_u8(cpu.r[12], 0);
            cpu.r[12] = saved[0];
            cpu.r[13] = saved[1];
            cpu.r[14] = saved[2];
            return;
        }
        cpu.r[1] = cpu.r[13]; // mov r13,r1
    }
    cpu.r[2] = 0x8c016484; // mov.l 0x8c1745ec,r2
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[0] = cpu.r[14].wrapping_add(4); // delay slot: add #4,r0
    crate::registry::p_8c016484::run(cpu, mem);
    cpu.r[12] = saved[0];
    cpu.r[13] = saved[1];
    cpu.r[14] = saved[2];
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let sound = cpu.r[4];
    cpu.set_t(sound == 0); // tst r4,r4
    if sound == 0 {
        cpu.r[2] = 0x8c174564; // mov.l 0x8c175cd4,r2
        cpu.r[5] = 0x8c1e11f4; // mov.l 0x8c175ce0,r5
        cpu.r[4] = 101; // delay slot: mov #101,r4
        report(cpu, mem);
        cpu.r[0] = 0; // bra 0x8c175c90 ; mov #0,r0 (delay)
        return;
    }
    let playing = mem.read_u32(sound.wrapping_add(0x38)).unwrap_or(0); // mov.l @(56,r4),r3
    cpu.r[3] = playing;
    cpu.set_t(playing == 0); // tst r3,r3
    cpu.r[0] = u32::from(playing != 0);
}
