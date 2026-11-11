//! 0x8c1730f6 — increment_global_n_times.
//!
//! Increments the counter at `0x8c2b5520` `r4` times (at least once). The
//! decrement is the `bf.s` delay slot, so it runs on the final iteration too.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r1 = mem.read_u32(0x8c17315c).unwrap_or(0);
    let mut r2;
    loop {
        r2 = mem.read_u32(r1).unwrap_or(0);
        r2 = r2.wrapping_add(1);
        let _ = mem.write_u32(r1, r2);
        let t = cpu.r[4] == 0;
        cpu.set_t(t);
        cpu.r[4] = cpu.r[4].wrapping_sub(1); // delay slot of `bf.s`
        if t {
            break;
        }
    }
    cpu.r[1] = r1;
    cpu.r[2] = r2;
}
