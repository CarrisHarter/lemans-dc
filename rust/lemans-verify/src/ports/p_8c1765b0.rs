//! 0x8c1765b0 - report whether `[r4+0x3dc] == 2` into `[r5]`.
//!
//! Twin of 0x8c176580 with `cmp/eq #2` instead of `tst`. The two emit exits use
//! their own ROM strings; the store path sets r0 = 1 and leaves T as the compare.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut emit = cpu.t(); // bf 0x8c1765bc
    if !emit {
        cpu.set_t(cpu.r[5] == 0); // tst r5,r5
        emit = cpu.t(); // bf 0x8c1765ce
    }
    if emit {
        cpu.r[5] = if cpu.r[4] == 0 {
            mem.read_u32(0x8c176644).unwrap_or(0x8c1e1954) // mov.l 0x8c176644,r5
        } else {
            mem.read_u32(0x8c176648).unwrap_or(0x8c1e197c) // mov.l 0x8c176648,r5
        };
        cpu.r[2] = mem.read_u32(0x8c176638).unwrap_or(0x8c174564); // mov.l 0x8c176638,r2
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c1765c8;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c1765cc
    } else {
        cpu.r[0] = mem.read_u16(0x8c17662e).unwrap_or(0x3dc) as i16 as i32 as u32; // mov.w (sign-extends)
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r0
        cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
        cpu.r[0] = cpu.t() as u32; // movt r0
        let _ = mem.write_u32(cpu.r[5], cpu.r[0]); // mov.l r0,@r5
        cpu.r[0] = 1; // mov #1,r0
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
