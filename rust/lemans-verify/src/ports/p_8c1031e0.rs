//! 0x8c1031e0 - apply one constant at five object offsets.
//!
//! `r5 == 1` loads 45.0; every other r5 loads 35.0. Both paths call
//! 0x8c16ea8c at +76, +0xc0, +0x134, +0x21c and +0x290, with r5 forced to 1.
//! The last call adds 0x290 to the saved object pointer; the epilogue
//! restores the caller's r14.

use sh4_oracle::{Cpu, Memory};

fn call(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.pr = ret;
    crate::registry::p_8c16ea8c::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    let ones = if cpu.t() {
        false
    } else {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        cpu.t()
    };
    if ones {
        cpu.r[0] = 0x8c103298;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[5] = 1;
        cpu.r[4] = cpu.r[4].wrapping_add(76); // jsr delay
        call(cpu, mem, 0x8c103200);
        cpu.r[0] = 0x8c103298;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0xc0);
        cpu.r[5] = 1; // jsr delay
        call(cpu, mem, 0x8c103210);
        cpu.r[0] = 0x8c103298;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0x134);
        cpu.r[5] = 1;
        call(cpu, mem, 0x8c103220);
        cpu.r[0] = 0x8c103298;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0x21c);
        cpu.r[5] = 1;
        call(cpu, mem, 0x8c103230);
        cpu.r[0] = 0x8c103298;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[5] = 1;
        cpu.r[1] = 0x290;
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[1]);
        cpu.r[4] = cpu.r[14]; // jsr delay
        call(cpu, mem, 0x8c103240);
    } else {
        cpu.r[0] = 0x8c1032b0;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(76);
        cpu.r[5] = 1; // jsr delay
        call(cpu, mem, 0x8c103252);
        cpu.r[0] = 0x8c1032b0;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0xc0);
        cpu.r[5] = 1;
        call(cpu, mem, 0x8c103262);
        cpu.r[0] = 0x8c1032b0;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0x134);
        cpu.r[5] = 1;
        call(cpu, mem, 0x8c103272);
        cpu.r[0] = 0x8c1032b0;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[4] = cpu.r[14].wrapping_add(0x21c);
        cpu.r[5] = 1;
        call(cpu, mem, 0x8c103282);
        cpu.r[0] = 0x8c1032b0;
        cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.r[5] = 1;
        cpu.r[1] = 0x290;
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[1]);
        cpu.r[4] = cpu.r[14];
        call(cpu, mem, 0x8c103292);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
