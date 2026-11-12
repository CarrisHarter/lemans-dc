//! 0x8c05e042 — byte-compare loop at r4 vs r5, entered at the loop body.
//!
//! Original: load both bytes (`mov.b` sign-extends), zero-extend them into
//! r3/r0/r1, advance both pointers, then `cmp/eq` sets T. Unequal bytes return
//! `d - s` from the `rts` delay slot (T=0); equal bytes branch to the shared
//! tail at 0x8c05e038, which returns 0 when the byte was NUL (T=1) and
//! otherwise re-enters this body.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let d = mem.read_u8(cpu.r[4]).unwrap_or(0);
        let s = mem.read_u8(cpu.r[5]).unwrap_or(0);
        cpu.r[3] = d as u32; // mov.b @r4,r3 then extu.b r3,r3
        cpu.r[0] = d as u32; // extu.b r3,r0
        cpu.r[1] = s as u32; // extu.b r1,r1, twice
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(d == s); // cmp/eq r1,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay slot, always runs
        if d != s {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay slot: sub r1,r0
            return;
        }
        // bt.s 0x8c05e038
        cpu.r[0] = cpu.r[3] & 0xff; // extu.b r3,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            continue; // bf 0x8c05e042
        }
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
}
