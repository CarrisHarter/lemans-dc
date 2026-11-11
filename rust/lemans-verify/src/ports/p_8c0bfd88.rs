//! 0x8c0bfd88 — set_slot_if_empty.
//!
//! `r0=[r5+4] ; r0<<=2 ; r1=[r4+r0] ; tst r1,r1 ; bf done ; [r4+r0]=r5` —
//! installs `r5` into a table slot only when the slot is null.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0);
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    let empty = cpu.r[1] == 0;
    cpu.set_t(empty); // tst r1,r1
    if empty {
        let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]);
    }
}
