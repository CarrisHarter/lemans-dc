//! 0x8c09138c — scan up to 46 bytes at r4+72 for a zero byte.
//! Returns 0 on zero byte found, 1 if none (rts delay slots).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    cpu.r[4] = cpu.r[4].wrapping_add(72);
    cpu.r[1] = 46;
    loop {
        let b = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[3] = b as u32; // extu.b (x2)
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if cpu.r[3] != 0 {
            cpu.r[0] = 0; // rts delay slot (nonzero byte -> 0)
            return;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.set_t(46 > (cpu.r[0] as i32)); // cmp/gt r0,r1: T=(r1>r0)
        if 46 > (cpu.r[0] as i32) {
            continue;
        }
        break;
    }
    cpu.r[0] = 1; // rts delay slot
}
