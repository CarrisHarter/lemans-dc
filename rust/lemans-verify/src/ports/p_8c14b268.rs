//! 0x8c14b268 — r4 = r4 >>a (-r5) (shad); r1 = (r4 & 0xff) - 127; fpul = r1;
//! fr0 = float(fpul); rts delay: fr0 *= fr1 where fr1 = [0x8c14b27c] = 0x3c010204.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5].wrapping_neg();
    let a = cpu.r[0] as i32;
    cpu.r[4] = if a >= 0 { cpu.r[4] << a } else { ((cpu.r[4] as i32) >> (-a)) as u32 }; // shad r0,r4
    cpu.r[1] = cpu.r[4] & 0xff; // extu.b r4,r1
    cpu.r[0] = 0x8C14_B27C; // mova 0x8c14b27c,r0
    cpu.r[1] = cpu.r[1].wrapping_sub(127); // add #-127,r1
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
    cpu.fr[1] = f32::from_bits(mem.read_u32(0x8c14b27c).unwrap_or(0));
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // rts delay: fmul fr1,fr0
}
