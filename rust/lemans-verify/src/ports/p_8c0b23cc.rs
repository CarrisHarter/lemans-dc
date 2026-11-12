//! 0x8c0b23cc — classify an optional object's state byte into a 0/2/6 code.
//!
//! `r4` is an optional object. Its byte at `r4+15` (the last byte of the
//! eight-byte block at `r4+8`, fetched through 0x8c05f51c which returns
//! `r4+8`) drives the code: 1 -> 6, 2..=127 -> 2, 0 or negative -> fall
//! through with 0. A zero code then consults the optional `r5`: when the f32
//! at `r5+16` is strictly below 1.0 the code becomes 2, otherwise it stays 0.
//!
//! Side effects: saves `r14`/`r13`/`pr` and an eight-byte frame on the stack
//! window; copies the eight bytes at `r4+8` to that frame; `r4 += 8` (callee);
//! `r13` walks to `r5+16` on the float arm; `fr1` := `[r5+16]`, `fr0` := 1.0;
//! returns the code in `r0`; leaves `T` from the last compare executed.
//! Ghidra's FPSCR.SZ arm around the float load does not exist in the machine
//! code (the oracle is single-precision-only), so no mode switch is modelled.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[13] = cpu.r[5];
    cpu.r[14] = 0; // delay slot of `bt.s`, executes on both arms
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c0b2438).unwrap_or(0); // 0x8c05f51c
        cpu.pr = 0x8c0b23e2; // return address after the `jsr`
        crate::registry::p_8c05f51c::run(cpu, mem); // r4 += 8; r0 = r4
        cpu.r[1] = cpu.r[0];
        for off in 0..8u32 {
            cpu.r[0] = mem.read_u8(cpu.r[1].wrapping_add(off)).unwrap_or(0) as u32;
            let _ = mem.write_u8(cpu.r[15].wrapping_add(off), cpu.r[0] as u8);
        }
        cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(7)).unwrap_or(0) as u32;
        cpu.r[1] = cpu.r[0] as i8 as i32 as u32; // exts.b r0,r1
        cpu.set_t((cpu.r[1] as i32) > 0); // cmp/pl r1
        if cpu.t() {
            cpu.r[0] = 1;
            cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32)); // cmp/gt r0,r1
            if cpu.t() {
                cpu.r[14] = 2; // delay slot of `bra`
            } else {
                cpu.r[14] = 6;
            }
        }
    }
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.t() {
        cpu.set_t(cpu.r[13] == 0); // tst r13,r13
        if !cpu.t() {
            cpu.r[13] = cpu.r[13].wrapping_add(16);
            cpu.fr[1] = mem.read_f32(cpu.r[13]).unwrap_or(0.0); // fmov @r13,fr1
            cpu.fr[0] = 1.0; // fldi1 fr0
            cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
            if cpu.t() {
                cpu.r[14] = 2;
            }
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
