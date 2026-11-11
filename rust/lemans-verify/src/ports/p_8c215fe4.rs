//! 0x8c215fe4 — byte copy r4→r5 while r6>0 (delay r6-=1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let go = (cpu.r[6] as i32) > 0;
        cpu.set_t(go); // cmp/pl r6
        if !go {
            break;
        }
        let b = mem.read_u8(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = b as u32;
        let _ = mem.write_u8(cpu.r[5], b);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // bt.s delay slot
    }
}
