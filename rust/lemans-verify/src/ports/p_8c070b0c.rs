//! 0x8c070b0c — case-insensitive string compare; returns first diff (lowercased).
//!
//! Original side effects: r7 = -32, r3 = 90, r6 = 65, r0/r1 = last bytes
//! (lowercased), r4/r5 advanced, T = last tst/cmp; r0 = r0 - r1 in rts delay.

use sh4_oracle::{Cpu, Memory};

fn lower(b: u32, cpu: &mut Cpu) -> u32 {
    cpu.set_t(b >= cpu.r[6]); // cmp/ge r6,r0
    if b >= 65 {
        cpu.set_t(cpu.r[3] >= b); // cmp/ge r0,r3 (r0<=90)
        if 90 >= b {
            return b.wrapping_sub(cpu.r[7]); // sub r7,r0 (+32)
        }
    }
    b
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = (-32i32) as u32;
    cpu.r[3] = 90;
    cpu.r[6] = 65;
    loop {
        let a = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
        cpu.r[0] = a;
        cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay slot
        cpu.r[0] = lower(cpu.r[0], cpu);
        let b = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32;
        cpu.r[1] = b;
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay slot
        cpu.r[1] = lower(cpu.r[1], cpu);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            break;
        }
        cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r1,r0
        if cpu.r[1] != cpu.r[0] {
            break;
        }
    }
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay slot
}
