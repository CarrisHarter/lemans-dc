//! 0x8c226de0 — *(0x8c2c6600+0x82c)=30; if r4<=0 { r4=-r4; *(..)=r4+1 }; r4-=1; *(0x8c2c6600+0x7f4)=r4; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x82c;
    cpu.r[3] = 30;
    cpu.r[5] = 0x8c2c6600;
    cpu.set_t((cpu.r[4] as i32) > 0); // cmp/pl
    let _ = mem.write_u32(cpu.r[0] + cpu.r[5], cpu.r[3]);
    if cpu.r[4] as i32 <= 0 {
        cpu.r[4] = cpu.r[4].wrapping_neg();
        cpu.r[2] = cpu.r[4].wrapping_add(1);
        let _ = mem.write_u32(cpu.r[0] + cpu.r[5], cpu.r[2]);
    }
    cpu.r[0] = 0x7f4;
    cpu.r[4] = cpu.r[4].wrapping_sub(1);
    let _ = mem.write_u32(cpu.r[0] + cpu.r[5], cpu.r[4]);
    cpu.r[0] = 0; // rts delay
}
