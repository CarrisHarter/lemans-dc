//! 0x8c227440 — gated bit insert.
//!
//! Accepts (r0=0) when `([r4+4] & 0x179f8) != 0` and either
//! `([r4+4] & 0x101f8) == 0` or `r5 != 1`; otherwise returns 10. On
//! acceptance it clears `0xffff9fff` (`mov.w 0x9fff` is sign-extended) in
//! the word at `r4 + 8*r5 + 8` and inserts `r6 & 3` at bit 13. Final T is
//! `shll r7`'s carry, i.e. bit 31 of `r5`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x179f8;
    cpu.r[7] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let t1 = (cpu.r[7] & cpu.r[2]) == 0;
    cpu.set_t(t1);
    if t1 {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[1] = 0x101f8;
    let t2 = (cpu.r[7] & cpu.r[1]) == 0;
    cpu.set_t(t2);
    if !t2 {
        cpu.r[0] = cpu.r[5]; // `mov r5,r0`
        let eq = cpu.r[0] == 1;
        cpu.set_t(eq); // `cmp/eq #1,r0`
        if eq {
            cpu.r[0] = 10; // delay slot of `rts`
            return;
        }
    }
    cpu.r[7] = cpu.r[5];
    let t = (cpu.r[7] >> 31) & 1 != 0;
    cpu.r[7] = cpu.r[7] << 1;
    cpu.set_t(t); // `shll r7`
    cpu.r[3] = cpu.r[4];
    cpu.r[7] = cpu.r[7] << 2;
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    cpu.r[1] = 0xffff9fff; // `mov.w 0x9fff` sign-extends
    cpu.r[7] = cpu.r[7].wrapping_add(8);
    cpu.r[0] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[2] = 3;
    cpu.r[6] = cpu.r[6] & cpu.r[2];
    cpu.r[3] = 13;
    cpu.r[0] = cpu.r[0] & cpu.r[1];
    cpu.r[6] = cpu.r[6] << cpu.r[3]; // shad
    cpu.r[6] = cpu.r[6] | cpu.r[0];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[7], cpu.r[6]);
}
