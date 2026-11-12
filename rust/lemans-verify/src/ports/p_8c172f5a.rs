//! 0x8c172f5a — report an interrupt-array/driver error through the bounded-copy helper.
//!
//! ```text
//! push pr; tst r4,r4; bt L66 (plain: no delay slot)
//! r2=[r4]; tst r2,r2; bf L72
//! L66: r3=[0x8c173024] (0x8c1725fc); r5=[0x8c17302c] (0x8c1df450 "... is NULL")
//!      jsr @r3; delay mov #101,r4; bra L84
//! L72: r3=[0x8c17301c] (0x8c20d450); r2=[r3]; tst r2,r2; bf L8a
//!      r2=[0x8c173024] (0x8c1725fc); r4=[0x8c173016].w (0x12f, sign-extended)
//!      r5=[0x8c173020] (0x8c1df39c "... not installed"); jsr @r2; delay nop
//! L84: pop pr; rts; delay mov #0,r0
//! L8a: r3=[0x8c173030] (0x8c2b5288); r0=1; [r4]=r3; pop pr; rts; delay nop
//! ```
//!
//! Null r4 (or a null pointee) reports "...interruptArray is NULL" with code
//! 101; a live pointer with a clear flag at 0x8c20d450 reports "...Driver not
//! installed" with code 0x12f. When the flag is set the slot exit stores the
//! TABLE ADDRESS itself (r3, not a loaded word) at [r4] and returns 1. The
//! jsr paths return 0 via the rts delay slot. The callees run as the
//! registered 0x8c1725fc port, so their full register/memory effects come
//! along verbatim.
//!
//! Residue: T is the callee's exit flag on the jsr paths, 0 on the slot path
//! (last test saw a nonzero flag); r15/pr restored; r0 is 0 or 1.

use sh4_oracle::{Cpu, Memory};

/// L66: jsr 0x8c1725fc with r4=101 (delay slot) and r5 = "... is NULL".
/// The trailing `bra L84` has no register effect.
fn jsr_interrupt_array(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc);
    cpu.r[5] = mem.read_u32(0x8c17302c).unwrap_or(0x8c1df450);
    cpu.pr = 0x8c172f6e; // jsr @r3
    cpu.r[4] = 101; // delay slot: mov #101,r4
    crate::registry::p_8c1725fc::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    // bt L66 (plain: no delay slot)
    if cpu.t() {
        jsr_interrupt_array(cpu, mem);
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r2
        cpu.set_t(cpu.r[2] == 0); // tst r2,r2
        // bf L72
        if cpu.t() {
            jsr_interrupt_array(cpu, mem); // falls through to L66
        } else {
            cpu.r[3] = mem.read_u32(0x8c17301c).unwrap_or(0x8c20d450);
            cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r2
            cpu.set_t(cpu.r[2] == 0); // tst r2,r2
            // bf L8a
            if !cpu.t() {
                // Slot exit: store the table address itself at [r4].
                cpu.r[3] = mem.read_u32(0x8c173030).unwrap_or(0x8c2b5288);
                cpu.r[0] = 1; // mov #1,r0
                let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
                cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                return; // rts; delay nop
            }
            cpu.r[2] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc);
            cpu.r[4] = mem.read_u16(0x8c173016).unwrap_or(0) as i16 as i32 as u32; // 0x12f
            cpu.r[5] = mem.read_u32(0x8c173020).unwrap_or(0x8c1df39c);
            cpu.pr = 0x8c172f84; // jsr @r2
            // delay slot: nop
            crate::registry::p_8c1725fc::run(cpu, mem);
        }
    }
    // L84: bra target of the L66 side, fallthrough of the L7a side.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0; // delay slot of rts: mov #0,r0
}
