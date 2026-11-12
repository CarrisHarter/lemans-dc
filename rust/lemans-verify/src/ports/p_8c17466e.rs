//! 0x8c17466e — audio-heap free probe: emit an error unless the heap slot is live.
//!
//! ```text
//! r0=[0x8c1746d2].w (0x108); r5=[0x8c174700] (0x8c2b5528); push pr
//! r3=[r5+r0]; tst r3,r3; bf L682 (plain: no delay slot)
//! r4=[0x8c1746d4].w (0x1f7); r5=[0x8c174704] (str1); bra L68a; nop
//! L682: tst r4,r4          // entry r4 — r5 still holds the slot base here
//! bf L696 (plain); r5=[0x8c174708] (str2); r4=101
//! L68a: r3=[0x8c1746e0] (0x8c174564); jsr @r3; nop
//! pop pr; rts; delay r0=0
//! L696: r0=[0x8c1746d6].w (0x10c); r2=[r5+r0]; r0=1; [r4]=r2; pop pr; rts; nop
//! ```
//!
//! Three exits: slot word null -> emit(0x1f7, "not initialized") and return 0;
//! entry r4 null -> emit(101, "freeMemory is NULL") and return 0; otherwise
//! publish `[slot+0x10c]` through the caller's r4 pointer and return 1. The
//! emit call runs as the registered port, so its full register/memory effects
//! (slot store, strcpy, T) come along verbatim.
//!
//! Residue: r0=0/1 by path; T is the callee's exit flag on the emit paths and
//! clear on the store path (`tst r4,r4` with r4!=0); r15 and pr restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u16(0x8c1746d2).unwrap_or(0) as i16 as i32 as u32; // 0x108
    cpu.r[5] = mem.read_u32(0x8c174700).unwrap_or(0x8c2b5528);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r5),r3
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if cpu.t() {
        // Slot null: emit(0x1f7, str1). Entry r4 is unused on this path.
        cpu.r[4] = mem.read_u16(0x8c1746d4).unwrap_or(0) as i16 as i32 as u32; // 0x1f7
        cpu.r[5] = mem.read_u32(0x8c174704).unwrap_or(0x8c1e03f8);
        // bra L68a; delay nop
    } else {
        // `tst r4,r4` tests the ENTRY r4; r5 still holds the slot base.
        cpu.set_t(cpu.r[4] == 0);
        if cpu.t() {
            cpu.r[5] = mem.read_u32(0x8c174708).unwrap_or(0x8c1e0424);
            cpu.r[4] = 101; // mov #101,r4
        } else {
            // L696: live slot — publish [slot+0x10c] through the caller's pointer.
            cpu.r[0] = mem.read_u16(0x8c1746d6).unwrap_or(0) as i16 as i32 as u32; // 0x10c
            cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r5),r2
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[4], cpu.r[2]); // mov.l r2,@r4
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            return; // rts; delay nop — T stays clear from `tst r4,r4`
        }
    }
    // L68a: emit path shared by both error exits.
    cpu.r[3] = mem.read_u32(0x8c1746e0).unwrap_or(0x8c174564);
    cpu.pr = 0x8c174690; // jsr @r3
    // delay nop
    crate::registry::p_8c174564::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = 0; // rts delay slot: mov #0,r0
}
