//! 0x8c231720 — build a 128-entry quantization table at 0x8c2c8850 from the
//! f32 table at `r4` (read descending): entry[i] = ((ftrc(t[i]*255) & mask)
//! << 8) | (ftrc(t[127]*255) & mask), mask = 0x80+i... precisely mask = r7 =
//! 0x80+r6(old), then sets bit 2 of `[0x8c2c8a50]`. Returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c231784; // mova
    cpu.r[6] = mem.read_u16(0x8c231780).unwrap_or(0) as i16 as i32 as u32; // 0x80
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // 255.0
    cpu.r[0] = mem.read_u16(0x8c231782).unwrap_or(0) as i16 as i32 as u32; // 0x1fc
    cpu.r[7] = cpu.r[6];
    cpu.r[7] = cpu.r[7].wrapping_add(127);
    cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[3];
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4]; // fmul fr4,fr2
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
    cpu.r[5] = cpu.fpul; // sts fpul,r5
    cpu.r[5] &= cpu.r[7];
    loop {
        cpu.r[6] = cpu.r[6].wrapping_sub(1);
        cpu.r[2] = cpu.r[5];
        cpu.r[1] = cpu.r[6];
        cpu.r[1] <<= 2; // shll2
        cpu.r[0] = cpu.r[1];
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0.0);
        cpu.r[0] = mem.read_u32(0x8c231788).unwrap_or(0); // 0x8c2c8850
        cpu.fr[3] = cpu.fr[3] * cpu.fr[4]; // fmul fr4,fr3
        cpu.fpul = cpu.fr[3] as i32 as u32; // ftrc fr3,fpul
        cpu.r[5] = cpu.fpul; // sts fpul,r5
        cpu.r[5] &= cpu.r[7];
        cpu.r[3] = cpu.r[5];
        cpu.r[3] <<= 8; // shll8
        cpu.r[3] |= cpu.r[2];
        cpu.r[2] = cpu.r[3] & 0xffff; // extu.w r3,r2
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[2]);
        cpu.set_t(cpu.r[6] & cpu.r[6] == 0); // tst r6,r6
        if cpu.t() {
            break;
        }
    }
    cpu.r[3] = mem.read_u32(0x8c23178c).unwrap_or(0); // 0x8c2c8a50
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] |= 4;
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    cpu.r[0] = 0; // rts delay slot
}
