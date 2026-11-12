//! 0x8c172ee2 — dispatch a string copy or read back a stored slot.
//!
//! ```text
//! push pr; tst r4,r4; bf L734 (plain: no delay slot)
//! r4==0 side:  r2=[0x8c172f1c] (0x8c1725fc); r5=[0x8c172f30] (0x8c1df414)
//!              jsr @r2 ; delay: mov #101,r4 ; bra L746
//! L734: r2=[0x8c17301c] (0x8c20d450); r3=[r2]; tst r3,r3; bf L74c
//!       r3=[0x8c173024] (0x8c1725fc); r4=[0x8c173016].w (0x12f, sign-extended)
//!       r5=[0x8c173020] (0x8c1df39c); jsr @r3 ; delay: nop
//! L746: pop pr; rts ; delay: mov #0,r0
//! L74c: r3=[0x8c173028] (0x8c2b5264); mov #1,r0; r2=[r3]; [r4]=r2
//!       pop pr; rts ; delay: nop
//! ```
//!
//! The words at 0x8c172ef4..0x8c172f32 are a literal pool, not code. Both
//! call sites reach the ported 0x8c1725fc, so its full register/memory
//! effects (slot store, strcpy, T residue) come along verbatim; the delay
//! slots run before the callee sees the state. The L74c exit reads the slot
//! the callee wrote and returns 1, leaving T set by `tst r3,r3` (r3 != 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.t() {
        // r4 == 0: 0x8c1725fc(101, str@0x8c1df414).
        cpu.r[2] = mem.read_u32(0x8c172f1c).unwrap_or(0x8c1725fc);
        cpu.r[5] = mem.read_u32(0x8c172f30).unwrap_or(0x8c1df414);
        cpu.pr = 0x8c172ef0; // jsr @r2
        cpu.r[4] = 101; // delay slot: mov #101,r4
        crate::registry::p_8c1725fc::run(cpu, mem);
        // bra L746; delay nop
    } else {
        cpu.r[2] = mem.read_u32(0x8c17301c).unwrap_or(0x8c20d450);
        cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if !cpu.t() {
            // Flag set: [r4] = [0x8c2b5264], r0 = 1.
            cpu.r[3] = mem.read_u32(0x8c173028).unwrap_or(0x8c2b5264);
            cpu.r[0] = 1; // mov #1,r0
            cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[4], cpu.r[2]); // mov.l r2,@r4
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop
        }
        // Flag zero: 0x8c1725fc(0x12f, str@0x8c1df39c).
        cpu.r[3] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc);
        cpu.r[4] = mem.read_u16(0x8c173016).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[5] = mem.read_u32(0x8c173020).unwrap_or(0x8c1df39c);
        cpu.pr = 0x8c172f46; // jsr @r3
        // delay slot: nop
        crate::registry::p_8c1725fc::run(cpu, mem);
    }
    // L746: pop pr; rts; delay mov #0,r0.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0;
}
