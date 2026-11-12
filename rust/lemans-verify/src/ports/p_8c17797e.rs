//! 0x8c17797e - map an enum in `r4` to a small code stored at `[r5]`.
//!
//! With `r5` non-null: `r4 == 0` stores 4, `r4 == 1` stores 2, `r4 == 2` and
//! `r4 == 3` store 1, and anything else emits str@0x8c1e2214 with code 0x322 and
//! returns 0. The mapped cases return 1. `r5 == 0` emits str@0x8c1e21e0 with
//! code 101 and returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[2] = mem.read_u32(0x8c1779a4).unwrap_or(0x8c174564); // mov.l 0x8c1779a4,r2
        cpu.r[5] = mem.read_u32(0x8c1779c4).unwrap_or(0x8c1e21e0); // mov.l 0x8c1779c4,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c17798c;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c1779f4
    } else {
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if cpu.t() {
            cpu.r[2] = 4; // mov #4,r2
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // bra delay: mov.l r2,@r5
            cpu.r[0] = 1; // at 0x8c1779f6
        } else {
            cpu.r[0] = cpu.r[4]; // mov r4,r0
            cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
            if cpu.t() {
                cpu.r[3] = 2; // mov #2,r3
                let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // bra delay: mov.l r3,@r5
                cpu.r[0] = 1;
            } else {
                cpu.r[0] = cpu.r[4]; // mov r4,r0
                cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
                cpu.r[6] = 1; // bt.s delay: mov #1,r6
                let mut mapped = cpu.t();
                if !mapped {
                    cpu.r[0] = cpu.r[4]; // mov r4,r0
                    cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
                    mapped = cpu.t();
                }
                if mapped {
                    let _ = mem.write_u32(cpu.r[5], cpu.r[6]); // bra delay: mov.l r6,@r5
                    cpu.r[0] = 1;
                } else {
                    cpu.r[3] = mem.read_u32(0x8c177a9c).unwrap_or(0x8c174564); // mov.l 0x8c177a9c,r3
                    cpu.r[4] = mem.read_u16(0x8c177a84).unwrap_or(0x322) as i16 as i32 as u32; // mov.w
                    cpu.r[5] = mem.read_u32(0x8c177a98).unwrap_or(0x8c1e2214); // mov.l 0x8c177a98,r5
                    cpu.pr = 0x8c1779f0; // jsr delay is nop
                    crate::registry::p_8c174564::run(cpu, mem);
                    cpu.r[0] = 0; // rts delay at 0x8c1779f4
                }
            }
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
