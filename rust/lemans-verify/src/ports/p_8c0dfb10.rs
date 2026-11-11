//! 0x8c0dfb10 — byte[r4+0x194]=0; scan i=0..=[r4+0x190] for [r4+i*8]==r5;
//! on match: fr0=[r4+4+i*8], if !(fr4>fr0) fr4=fr0; r4+=4+i*8; rts delay *r4=fr4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x194;
    cpu.r[3] = 0;
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 0);
    cpu.r[0] = 0x190;
    cpu.r[6] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[1] = cpu.r[3];
    let mut i = 0u32;
    loop {
        cpu.set_t((cpu.r[6] as i32) > (i as i32)); // cmp/gt r3,r6: T=(r6>r3)
        if !((cpu.r[6] as i32) > (i as i32)) {
            cpu.r[1] = cpu.r[6];
            return;
        }
        cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[5]); // cmp/eq r5,r0
        if cpu.r[0] == cpu.r[5] {
            let idx = (i << 2) << 1; // r3*8
            cpu.r[0] = cpu.r[4].wrapping_add(4).wrapping_add(idx);
            cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0));
            cpu.set_t(cpu.fr[4] > cpu.fr[0]); // fcmp/gt fr0,fr4: T=(fr4>fr0)
            if !(cpu.fr[4] > cpu.fr[0]) {
                cpu.fr[4] = cpu.fr[0];
            }
            cpu.r[4] = cpu.r[4].wrapping_add(4).wrapping_add(idx);
            let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits()); // rts delay slot
            return;
        }
        cpu.r[1] = cpu.r[1].wrapping_add(8);
        i = i.wrapping_add(1);
    }
}
