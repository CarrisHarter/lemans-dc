//! 0x8c227620 — insert_bits_181ff.
//!
//! Returns 10 when `([r4+4] & 0x181ff) == 0`; otherwise clears
//! `0xffffff3f` (sign-extended `mov.w 0xff3f`) in the word at `r4` and
//! inserts `r5 & 3` at bit 6.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[3] = 98815;
    let t = (cpu.r[3] & cpu.r[2]) == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[3] = 3;
    cpu.r[6] = cpu.r[4];
    cpu.r[5] = cpu.r[5] & cpu.r[3];
    cpu.r[2] = 0xffffff3f; // `mov.w` sign-extends 0xff3f
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[1] = cpu.r[1] & cpu.r[2];
    cpu.r[5] = cpu.r[5] | cpu.r[1];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);

}
