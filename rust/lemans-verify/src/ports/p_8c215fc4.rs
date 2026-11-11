//! 0x8c215fc4 — word copy r4→r5: if r6>=0 r6+=3; r6>>=1; copy words while r6>0 (delay r6-=1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t((cpu.r[6] as i32) >= 0); // cmp/pz r6
    if (cpu.r[6] as i32) >= 0 {
        cpu.r[6] = cpu.r[6].wrapping_add(3); // bt delay slot
    }
    cpu.r[6] = ((cpu.r[6] as i32) >> 1) as u32; // shar r6
    loop {
        let go = (cpu.r[6] as i32) > 0;
        cpu.set_t(go); // cmp/pl r6
        if !go {
            break;
        }
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // bt.s delay slot
    }
}
