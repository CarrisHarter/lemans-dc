//! 0x8c1a4a68 — count trailing zero bits of r4 (r0 = r3 at exit, rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[1] = 1;
    cpu.r[3] = 0;
    loop {
        cpu.r[0] = cpu.r[4] & 1;
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            break; // bf not taken: exit with r0 = r3 (rts delay)
        }
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.r[4] == 0 {
            break;
        }
        cpu.r[4] = cpu.r[4] >> 1; // shlr r4
        cpu.r[1] = cpu.r[1] << 1; // shll r1
        cpu.r[3] = cpu.r[3].wrapping_add(1);
    }
    cpu.r[0] = cpu.r[3]; // rts delay slot
}
