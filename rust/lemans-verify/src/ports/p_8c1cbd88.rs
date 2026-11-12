//! 0x8c1cbd88 — r5==1 → r0 = *(r4+44); r5==2 → r0 = *(r4+48); else
//! (r5==0 or any other value) → r0 = *(r4+40) (rts delay slots).
//!
//! T on exit: 1 on the three `bt` paths, 0 on the `bra` default path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] != 0 {
        cpu.r[0] = cpu.r[5]; // mov r5,r0
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.r[0] == 1 {
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(44)).unwrap_or(0); // rts delay
            return;
        }
        cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
        if cpu.r[0] == 2 {
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(48)).unwrap_or(0); // rts delay
            return;
        }
        // bra 0x8c1cbda2: T stays 0, default slot at +40.
    }
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
}
