//! 0x8c04ba78 — guarded dispatch: loads `r4 = [r4+0xa0]`; when nonzero
//! calls 0x8c104768 (byte at r4+32) and returns its value, else returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(0x8c04ba94).unwrap_or(0); // 0xa0
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.set_t(cpu.r[4] & cpu.r[4] == 0); // tst r4,r4
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c04ba98).unwrap_or(0); // 0x8c104768
        cpu.pr = 0x8c04ba88;
        crate::registry::p_8c104768::run(cpu, mem);
    } else {
        cpu.r[0] = 0;
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
