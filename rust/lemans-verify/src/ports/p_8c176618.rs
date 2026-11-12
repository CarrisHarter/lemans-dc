//! 0x8c176618 - validate three required fields, then stamp two state words.
//!
//! Null `r14` emits str@0x8c1e1a20 with code 101 and returns 0, as does any of
//! `[r14+0x3f8]`, `[r14+0x344]` or `[r14+0x354]` being zero. Otherwise, when
//! `[r14+0x3c0]` is zero, 0x8c17678e runs on `(r14, str@0x8c177646)`; then
//! `[r14+0x3e0] = 2`, `[r14+0x3dc] = 1` and the function returns 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    let mut result = 0u32;
    if !cpu.t() {
        let mut ok = true;
        for off in [0x3f8u32, 0x344, 0x354] {
            cpu.r[0] = off; // mov.w 0x8c176724/26/28,r0
            cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r14),r3
            cpu.set_t(cpu.r[3] == 0); // tst r3,r3
            if cpu.t() {
                ok = false;
                break;
            }
        }
        if ok {
            cpu.r[0] = 0x3c0; // mov.w 0x8c17672a,r0
            cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r14),r2
            cpu.set_t(cpu.r[2] == 0); // tst r2,r2
            if cpu.t() {
                cpu.r[5] = mem.read_u32(0x8c176740).unwrap_or(0x8c177646); // mov.l 0x8c176740,r5
                cpu.r[4] = cpu.r[14]; // bsr delay: mov r14,r4
                cpu.pr = 0x8c17668e;
                crate::registry::p_8c17678e::run(cpu, mem);
            }
            cpu.r[0] = 0x3e0; // mov.w 0x8c17672c,r0
            cpu.r[2] = 2; // mov #2,r2
            cpu.r[4] = 1; // mov #1,r4
            let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[2]); // mov.l r2,@(r0,r14)
            cpu.r[0] = cpu.r[0].wrapping_sub(4); // add #-4,r0
            let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[4]); // mov.l r4,@(r0,r14)
            cpu.r[0] = cpu.r[4]; // mov r4,r0
            result = 1;
        }
    }
    if result == 0 {
        // Two distinct reporter strings: the null path uses 0x8c1e1a20, the
        // missing-field path uses 0x8c1e1a44.
        let (pool_fn, pool_str) = if cpu.r[14] == 0 {
            (0x8c176638u32, 0x8c176654u32)
        } else {
            (0x8c17673c, 0x8c176738)
        };
        cpu.r[2] = mem.read_u32(pool_fn).unwrap_or(0x8c174564); // mov.l pool,r2
        cpu.r[5] = mem.read_u32(pool_str).unwrap_or(0x8c1e1a20); // mov.l pool,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c176678;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c17667a
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
