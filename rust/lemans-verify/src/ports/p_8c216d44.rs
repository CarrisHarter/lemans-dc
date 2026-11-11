//! 0x8c216d44 — lower-to-upper.
//!
//! When the sign-extended low byte of `r4` lies in `[97, 122]` subtracts 32
//! from `r4`. Returns the (possibly adjusted) `r4`; `r3`/`r5` keep the bounds
//! and the character, `r1` becomes 122 whenever the second compare ran.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[3] = 97;
    cpu.r[5] = cpu.r[4] as i8 as i32 as u32;
    let ge = (cpu.r[5] as i32) >= (cpu.r[3] as i32);
    cpu.set_t(ge);
    if ge {
        cpu.r[1] = 122;
        let gt = (cpu.r[5] as i32) > (cpu.r[1] as i32);
        cpu.set_t(gt);
        if !gt {
            cpu.r[4] = cpu.r[4].wrapping_sub(32);
        }
    }
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
