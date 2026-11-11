//! 0x8c085434 — lexicographic compare of 4 halfwords, high field first;
//! returns (a[i] > b[i]) for the first differing field i, signed.
//!
//! Original side effects the C does not show: r0 = T as 0/1 (movt in the
//! rts delay slot), not the loaded halfword; r1 = sign-extended a[i] of the
//! field that decided the result; T = that final cmp/gt.

use sh4_oracle::{Cpu, Memory};

fn exts_w(v: u16) -> u32 {
    (v as i16 as i32) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for off in [6u32, 4, 2] {
        cpu.r[1] = exts_w(mem.read_u16(cpu.r[4] + off).unwrap_or(0));
        cpu.r[0] = exts_w(mem.read_u16(cpu.r[5] + off).unwrap_or(0));
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
        if cpu.r[0] == cpu.r[1] {
            continue; // bt to next field
        }
        cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/gt r0,r1
        cpu.r[0] = cpu.t() as u32; // movt r0 (rts delay slot)
        return;
    }
    cpu.r[1] = exts_w(mem.read_u16(cpu.r[4]).unwrap_or(0));
    cpu.r[0] = exts_w(mem.read_u16(cpu.r[5]).unwrap_or(0));
    cpu.set_t(cpu.r[1] > cpu.r[0]); // cmp/gt r0,r1
    cpu.r[0] = cpu.t() as u32; // movt r0
}
