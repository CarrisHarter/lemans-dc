//! 0x8c011c50 — return 1 if the first r5 bytes at r4 are all zero, else 0.
//!
//! Original side effects: r6 = 0 then counts, r2 = last byte, r4 advanced,
//! r0 = result, T = last cmp (cmp/pl on entry, cmp/ge r5,r6 in the loop).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    cpu.r[6] = 0; // bf.s delay slot: mov #0,r6
    if (cpu.r[5] as i32) <= 0 {
        cpu.r[0] = 1;
        return;
    }
    loop {
        let b = mem.read_u8(cpu.r[4]).unwrap_or(0); // mov.b @r4,r2
        cpu.r[2] = b as i8 as i32 as u32;
        cpu.set_t(b == 0); // tst r2,r2
        if b != 0 {
            cpu.r[0] = 0; // rts delay slot: mov #0,r0
            return;
        }
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.set_t((cpu.r[6] as i32) >= (cpu.r[5] as i32)); // cmp/ge r5,r6
        cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay slot
        if (cpu.r[6] as i32) >= (cpu.r[5] as i32) {
            break;
        }
    }
    cpu.r[0] = 1;
}
