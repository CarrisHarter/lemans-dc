//! 0x8c172e2e — range_check_global.
//!
//! Returns 1 when `r4` lies in `[[0x8c2b5260]+4, +4+[+8])`, else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r6 = mem.read_u32(0x8c172f00).unwrap_or(0);
    let r5 = mem.read_u32(r6 + 4).unwrap_or(0);
    cpu.r[6] = r6;
    cpu.r[5] = r5;
    let t = cpu.r[4] >= r5;
    cpu.set_t(t);
    if t {
        let r0 = mem.read_u32(r6 + 8).unwrap_or(0);
        let hi = r0.wrapping_add(r5);
        cpu.r[0] = r0;
        let t2 = cpu.r[4] >= hi;
        cpu.set_t(t2);
        if !t2 {
            cpu.r[0] = 1; // delay slot of `rts`
            return;
        }
    }
    cpu.r[0] = 0;
}
