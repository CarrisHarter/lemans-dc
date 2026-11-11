//! 0x8c11004c — r0 = ((1 shld r5) & *(r4+44)) == 0 ? 1 : 0 (subc/add trick, rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4] + 44).unwrap_or(0);
    cpu.r[1] = 1;
    let a = cpu.r[5] as i32;
    cpu.r[1] = if a >= 0 { 1u32 << a } else { 1u32 >> (-a) }; // shld r5,r1
    cpu.set_t(cpu.r[1] & cpu.r[3] == 0); // tst r1,r3
    // subc r0,r0 ; add #1,r0  =>  r0 = 0 if T else 1
    cpu.r[0] = if cpu.r[1] & cpu.r[3] == 0 { 0 } else { 1 };
}
