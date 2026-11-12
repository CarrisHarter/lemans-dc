//! 0x8c177c3c - map an enum in `r4` to a small code stored at `[r5]`.
//!
//! With `r5` non-null: `r4 == 16` stores 0, `r4 == 8` stores 1, `r4 == 4` stores 3,
//! and anything else stores 0 before emitting str@0x8c1e238c with code 0x324 and
//! returning 0. The three mapped cases return 1. The `r5 == 0` fall-through path
//! stores through a null pointer in the original, so it is not reachable.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = 0; // bf.s delay: mov #0,r14
    if cpu.r[5] != 0 {
        cpu.r[0] = cpu.r[4]; // mov r4,r0
        cpu.set_t(cpu.r[0] == 16); // cmp/eq #16,r0
        if cpu.t() {
            let _ = mem.write_u32(cpu.r[5], cpu.r[14]); // bra delay: mov.l r14,@r5
            cpu.r[0] = 1; // at 0x8c177cb4
        } else {
            cpu.set_t(cpu.r[0] == 8); // cmp/eq #8,r0
            if cpu.t() {
                cpu.r[2] = 1; // mov #1,r2
                let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
                cpu.r[0] = 1;
            } else {
                cpu.r[0] = cpu.r[4]; // mov r4,r0
                cpu.set_t(cpu.r[0] == 4); // cmp/eq #4,r0
                if cpu.t() {
                    cpu.r[3] = 3; // mov #3,r3
                    let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                    cpu.r[0] = 1;
                } else {
                    let _ = mem.write_u32(cpu.r[5], cpu.r[14]); // mov.l r14,@r5
                    cpu.r[3] = mem.read_u32(0x8c177d70).unwrap_or(0x8c174564); // mov.l 0x8c177d70,r3
                    cpu.r[4] = mem.read_u16(0x8c177d60).unwrap_or(0x324) as i16 as i32 as u32; // mov.w
                    cpu.r[5] = mem.read_u32(0x8c177d6c).unwrap_or(0x8c1e238c); // mov.l 0x8c177d6c,r5
                    cpu.pr = 0x8c177cac; // jsr delay is nop
                    crate::registry::p_8c174564::run(cpu, mem);
                    cpu.r[0] = cpu.r[14]; // mov r14,r0 at 0x8c177cae
                }
            }
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
