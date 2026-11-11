//! 0x8c0112c4 — strchr: find byte r5 in the string at r4.
//!
//! The `bra` delay slot sign-extends r5 into r4 (`exts.b r5,r4`), so the
//! original clobbers its own first argument — the C signature hides that.
//! Found: r0 = r6 = match address, T set by the winning `cmp/eq`.
//! Not found: r0 = 0 via the `rts` delay slot at 0x8c0112d0, T set by the
//! final `tst r2,r2` on the NUL.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[4] = cpu.r[5] as u8 as i8 as i32 as u32; // exts.b r5,r4 (bra delay slot)
    let mut r6 = s;
    loop {
        let c = mem.read_u8(r6).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[2] = c;
        if c == cpu.r[4] {
            cpu.set_t(true); // cmp/eq r4,r2 matched
            cpu.r[0] = r6;   // 0x8c0112da
            cpu.r[6] = r6;
            return;          // rts, delay slot nop
        }
        if c == 0 {
            cpu.set_t(true); // tst r2,r2 on the NUL
            cpu.r[6] = r6;
            cpu.r[0] = 0;    // delay slot of the rts at 0x8c0112ce
            return;
        }
        r6 = r6.wrapping_add(1); // 0x8c0112d2
    }
}
