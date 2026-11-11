//! 0x8c019678 — `if ((*g + 0x28) == 3) flag_base[r4] = 1;`
//!
//! Hidden side effects: r0 is clobbered with the loaded field (or with the
//! flag base 0xac on the taken path), r1 is set to 1, and T carries the
//! `cmp/eq #3` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c01968c).unwrap_or(0);
    let p = mem.read_u32(g).unwrap_or(0);
    let field = mem.read_u32(p + 0x28).unwrap_or(0);
    cpu.set_t(field == 3);
    cpu.r[0] = field;
    if field == 3 {
        let base = mem.read_u32(0x8c019690).unwrap_or(0);
        cpu.r[0] = base;
        cpu.r[1] = 1;
        // `mov.b r1,@(r0,r4)` writes one byte
        let _ = mem.write_u8(base.wrapping_add(cpu.r[4]), 1);
    }
}
