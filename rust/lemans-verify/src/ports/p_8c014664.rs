//! 0x8c014664 — atan2(fr5, fr4): calls the atan kernel (0x8c014480) on
//! fr4/fr5 (jsr delay slot `fdiv fr3,fr4`), then adjusts by ±π from the
//! sign bits of the spilled inputs: fr5 negative adds -π or +π depending
//! on the fr4 sign; fr5 non-negative returns the kernel result as-is.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = cpu.r[15];
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[0] = 4;
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
    cpu.r[3] = mem.read_u32(0x8c0146ac).unwrap_or(0); // 0x8c014480
    let _ = mem.write_f32(cpu.r[15], cpu.fr[5]);
    cpu.fr[4] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[5];
    cpu.fr[4] = cpu.fr[4] / cpu.fr[3]; // jsr delay slot: fdiv fr3,fr4
    cpu.pr = 0x8c01467c;
    crate::registry::p_8c014480::run(cpu, mem);
    cpu.r[2] = cpu.r[15];
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(0x8c0146b0).unwrap_or(0); // 0x80000000
    cpu.set_t(cpu.r[4] & cpu.r[3] == 0); // tst r4,r3
    cpu.fr[4] = cpu.fr[0]; // bt.s delay slot: always runs
    if !cpu.t() {
        cpu.r[1] = cpu.r[15];
        cpu.r[1] = cpu.r[1].wrapping_add(4);
        cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.set_t(cpu.r[3] & cpu.r[4] == 0); // tst r3,r4
        if !cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c0146b4).unwrap_or(0); // 0x8c1ccfe8
            cpu.fr[3] = cpu.fr[4];
            cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // bra delay slot: -pi
        } else {
            cpu.r[3] = mem.read_u32(0x8c0146b8).unwrap_or(0); // 0x8c1ccfb8
            cpu.fr[3] = cpu.fr[4];
            cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // +pi
        }
        cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // fadd fr3,fr4
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[0] = cpu.fr[4]; // rts delay slot
}
