//! 0x8c0d8ef0 — bounds-check r5 against *(r4+76)-1 (return 0 if out of range);
//! if byte at r4+68 == 1 return **(r4+64); else return *(*(r4+64) + r5*4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t((cpu.r[5] as i32) >= 0); // cmp/pz r5
    if (cpu.r[5] as i32) < 0 {
        cpu.r[0] = 0; // rts delay slot at 0x8c0d8f02
        return;
    }
    cpu.r[0] = cpu.r[4].wrapping_add(76);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_sub(1);
    let n = cpu.r[0];
    cpu.set_t((cpu.r[5] as i32) > (n as i32)); // cmp/gt r0,r5: T=(r5>r0)
    if (cpu.r[5] as i32) > (n as i32) {
        cpu.r[0] = 0; // rts delay slot
        return;
    }
    cpu.r[0] = cpu.r[4].wrapping_add(68);
    let b = mem.read_u8(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = b as u32; // extu.b x2
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    cpu.r[4] = cpu.r[4].wrapping_add(64);
    if cpu.r[0] == 1 {
        cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay slot
        return;
    }
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
}
