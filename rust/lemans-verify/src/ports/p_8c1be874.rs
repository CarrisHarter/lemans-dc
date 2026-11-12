//! 0x8c1be874 — guarded free that also clears a global, returns p.
//!
//! `tst` on r4 gives the null exit (T=1, nothing stored). Non-null: r3 = the
//! global address from the pool (0x8c26c230), r1 = 0, `exts.w r5` + `cmp/pl`,
//! and the `bf.s` delay slot stores the zero byte at r3 **on both outcomes** —
//! the byte is cleared even when the free is skipped. Only a positive
//! sign-extended low half of r5 reaches the free thunk at 0x8c01032c.
//! `mov r14,r0` after the call overwrites the callee's r0 with the saved r4;
//! `pr` and r14 are restored from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = cpu.r[4]; // mov r4,r14 (pushed, so r14 ends at its entry value)
    cpu.set_t(saved == 0); // tst r14,r14
    if saved != 0 {
        cpu.r[3] = mem.read_u32(0x8c1be898).unwrap_or(0x8c26c230);
        cpu.r[1] = 0; // mov #0,r1
        let count = (cpu.r[5] as u16 as i16) as i32; // exts.w r5,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        let _ = mem.write_u8(cpu.r[3], cpu.r[1] as u8); // bf.s delay slot: both outcomes
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c1be89c).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c1be890;
            crate::registry::p_8c01032c::run(cpu, mem); // jsr @r0 ; nop
        }
    }
    cpu.r[0] = saved; // mov r14,r0
}
