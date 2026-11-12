//! 0x8c0152fc — `if (*p != 0 && (p - q) >= 0x200) *g_slot = 0x450; return …`
//!
//! The pointer at 0x8c015320 (normally 0x8c23d8b8) is read at runtime, so
//! the port loads it from memory like the original. Hidden side effects:
//! the `rts` delay slot is `mov #1,r0`, so the store path returns 1 in r0
//! with r1 = 0x450, r2 = the slot pointer, r3 = 0x200 and r4 = p - q;
//! the early-out path keeps r3 = *p (sign-extended) and r4 untouched with
//! T = 1; the short path keeps T = 0. All paths leave r0 set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let b = mem.read_u8(cpu.r[4]).unwrap_or(0);
    let mut r3 = (b as i8 as i32) as u32; // mov.b @r4,r3 sign-extends
    cpu.r[3] = r3;
    cpu.set_t(r3 == 0); // tst r3,r3
    if r3 != 0 {
        r3 = 0x0200; // mov.w @0x8c01531a,r3
        let r4 = cpu.r[4].wrapping_sub(cpu.r[5]);
        let ge = (r4 as i32) >= (r3 as i32); // cmp/ge r3,r4
        cpu.set_t(ge);
        if ge {
            let r2 = mem.read_u32(0x8c015320).unwrap_or(0);
            let r1 = 0x0450u32; // mov.w @0x8c01531c,r1
            let _ = mem.write_u32(r2, r1);
            cpu.r[1] = r1;
            cpu.r[2] = r2;
            cpu.r[3] = r3;
            cpu.r[4] = r4;
            cpu.r[0] = 1; // rts delay slot: mov #1,r0
            return;
        }
        cpu.r[3] = r3;
        cpu.r[4] = r4;
    }
    cpu.r[0] = 0;
}
