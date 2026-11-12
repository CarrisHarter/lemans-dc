//! 0x8c215fac — bounded memcmp r4 vs r5 (r6 count): mismatch → r0=0, all equal → r0=1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let s = mem.read_u8(cpu.r[5]).unwrap_or(0);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[2] = s as i8 as i32 as u32; // mov.b is sign-extended
        let d = mem.read_u8(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = d as i8 as i32 as u32; // mov.b is sign-extended
        cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
        if cpu.r[2] != cpu.r[3] {
            cpu.r[0] = 0; // rts delay slot
            return;
        }
        let go = (cpu.r[6] as i32) > 0;
        cpu.set_t(go); // cmp/pl r6
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // bt.s delay slot runs regardless
        if go {
            continue;
        }
        break;
    }
    cpu.r[0] = 1; // rts delay slot
}
