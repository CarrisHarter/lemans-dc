//! 0x8c173868 — refresh the device table at [0x8c173900] from the current
//! 0x8c2b5260 slot when the table still needs it.
//!
//! Original: calls 0x8c1739f4 with `r4=4` (delay slot), saving the callee's
//! `r0` (the entry IMASK) in the `r15-4` slot, then loads the table pointer
//! into `r14`. `tst` + plain `bf` (no delay slot) skips to the SR merge when
//! `[table+24]` is nonzero — the table is already programmed. Otherwise it
//! calls 0x8c173954 with `r4=0` (delay slot of `bsr`, so the callee sees 0)
//! and `tst` + plain `bt` skips when that returns null. On the copy path the
//! first two words are AND-masked with `[0x8c173904]` (0x0fffffff), the last
//! two copied raw, then 5/1/1 are stamped at offsets 16/20/24. Every path
//! ends with `SR = (SR & 0xffffff0f) | ((saved & 15) << 4)` — the `mov.w`
//! mask 0xff0f sign-extends — restoring the entry IMASK, and the `rts` delay
//! slot pops the saved `r14`, so the entry `r14` is restored. Stack traffic
//! is invisible (stack window excluded), so `r15` is left untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let entry_pr = cpu.pr; // sts.l pr,@-r15: epilogue pops it
    let entry_r14 = cpu.r[14]; // mov.l r14,@-r15: rts delay slot pops it
    // jsr 0x8c1739f4; delay slot `mov #4,r4`; pr = 0x8c173874.
    cpu.r[3] = mem.read_u32(0x8c1738fc).unwrap_or(0);
    cpu.r[4] = 4; // delay slot
    cpu.pr = 0x8c173874;
    crate::registry::p_8c1739f4::run(cpu, mem);
    // 0x8c173874: the callee's rts lands here.
    cpu.r[14] = mem.read_u32(0x8c173900).unwrap_or(0);
    let saved_r0 = cpu.r[0]; // mov.l r0,@r15: stack slot, invisible
    cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(24)).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    // bf L8ac (no delay slot): skip when [table+24] != 0.
    if cpu.t() {
        // bsr 0x8c173954; delay slot `mov #0,r4`; pr = 0x8c173882.
        cpu.r[4] = 0; // delay slot: runs before the callee
        cpu.pr = 0x8c173882;
        crate::registry::p_8c173954::run(cpu, mem);
        // 0x8c173882:
        cpu.r[4] = cpu.r[0]; // mov r0,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        // bt L8ac (no delay slot): skip when the slot value is null.
        if !cpu.t() {
            let p = cpu.r[4];
            cpu.r[3] = 0;
            let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[3]);
            cpu.r[2] = mem.read_u32(p).unwrap_or(0);
            cpu.r[5] = mem.read_u32(0x8c173904).unwrap_or(0);
            cpu.r[2] &= cpu.r[5];
            let _ = mem.write_u32(cpu.r[14], cpu.r[2]);
            cpu.r[3] = mem.read_u32(p.wrapping_add(4)).unwrap_or(0);
            cpu.r[5] &= cpu.r[3];
            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[5]);
            cpu.r[2] = mem.read_u32(p.wrapping_add(8)).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[2]);
            cpu.r[3] = mem.read_u32(p.wrapping_add(12)).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[3]);
            cpu.r[2] = 5;
            let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[2]);
            cpu.r[4] = 1;
            let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[4]);
            let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[4]);
        }
    }
    // L8ac:
    cpu.r[0] = saved_r0; // mov.l @r15,r0
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[2] = mem.read_u16(0x8c1738e6).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] &= 15;
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.pr = entry_pr; // lds.l @r15+,pr
    cpu.r[14] = entry_r14; // rts delay slot: mov.l @r15+,r14
}
