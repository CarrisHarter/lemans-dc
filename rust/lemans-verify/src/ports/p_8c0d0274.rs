//! 0x8c0d0274 — reset six indexed fields of a block.
//!
//! `r14 = 0; if (0 <= r5 && 4 > r5) r14 = fn_8c0c0ecc([0x8c27a238], r5);`
//! then six calls to 0x8c16eadc with `(*r4 + {0x4c,0xc0,0x134,0x1a8,0x21c,0x290})`
//! as the base and `r14` as the value. The callee adds its own 0x5c and stores
//! through the pointer held at that field. `r4` is re-read from `*r13` before
//! every call; `r13` holds the argument pointer. T at exit is the last compare
//! in this function — the callees touch no flags.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.set_t((cpu.r[5] as i32) >= 0); // cmp/pz r5
    cpu.r[14] = 0; // delay slot of `bf.s`
    if cpu.t() {
        cpu.r[0] = 4;
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r0
        if cpu.t() {
            cpu.r[0] = 0x8c27a238; // `mov.l 0x8c0d02e4,r0` loads the pool constant
            cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0);
            crate::registry::p_8c0c0ecc::run(cpu, mem);
            cpu.r[14] = cpu.r[0];
        }
    }
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0x4c);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0xc0);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0x134);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0x1a8);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0x21c);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0).wrapping_add(0x290);
    cpu.r[5] = cpu.r[14];
    crate::registry::p_8c16eadc::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
