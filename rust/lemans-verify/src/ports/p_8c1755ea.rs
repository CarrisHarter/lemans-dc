//! 0x8c1755ea — range_check_descriptor.
//!
//! Returns 1 when `r4` lies in `[[0x8c2b5528+0x15c4], [0x8c2b5528+0x15c8]]` and the
//! descriptor's `+0x15cc` word is non-zero; else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x15cc;
    cpu.r[5] = mem.read_u32(0x8c175624).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    let t = cpu.r[3] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[0] = 0x15c4;
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
        let t2 = cpu.r[4] >= cpu.r[3];
        cpu.set_t(t2);
        if t2 {
            cpu.r[0] = 0x15c8;
            cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
            let t3 = cpu.r[4] > cpu.r[1];
            cpu.set_t(t3);
            if !t3 {
                cpu.r[0] = 1; // delay slot of `rts`
                return;
            }
        }
    }
    cpu.r[0] = 0;
}
