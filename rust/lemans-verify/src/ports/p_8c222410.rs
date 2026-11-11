//! 0x8c222410 — compare_512_bytes.
//!
//! Compares 512 bytes at `r4` and `r5`; returns -1 when any byte differs, else 0.
//! The counter increments in the `bt.s` delay slot, so it advances on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0x200;
    cpu.r[6] = 0;
    cpu.r[0] = u32::MAX;
    cpu.r[7] = 0;
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
        let eq = cpu.r[2] == cpu.r[3];
        cpu.set_t(eq);
        cpu.r[6] = cpu.r[6].wrapping_add(1); // delay slot of `bt.s`
        if !eq {
            cpu.r[7] = cpu.r[0];
        }
        let ge = (cpu.r[6] as i32) >= (cpu.r[1] as i32);
        cpu.set_t(ge);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[5] = cpu.r[5].wrapping_add(1); // delay slot of `bf.s`
        if ge {
            break;
        }
    }
    cpu.r[0] = cpu.r[7]; // delay slot of `rts`
}
