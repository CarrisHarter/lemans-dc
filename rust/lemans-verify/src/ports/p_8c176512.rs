//! 0x8c176512 - flag reporter: `[r5] = ([r4+0x3dc] == 3 || [r4+0x3dc] == 0)`.
//!
//! Three exits. `r4 == 0` and `r4 != 0 && r5 == 0` both emit through
//! 0x8c174564 with code 101 (different ROM strings) and return 0. Otherwise the
//! field at `+0x3dc` is read once, `cmp/eq #3` short-circuits to r3 = 1, a zero
//! field also yields r3 = 1, anything else r3 = 0; `[r5] = r3` and r0 = 1.
//! T ends as that same predicate, since `mov #1,r0` and the store leave it alone.
//!
//! The window 0x8c17651e..0x8c17654e is an inline literal pool, not code.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] == 0 {
        cpu.r[5] = mem.read_u32(0x8c17654c).unwrap_or(0x8c1e18cc); // mov.l 0x8c17654c,r5
        emit(cpu, mem);
    } else {
        cpu.set_t(cpu.r[5] == 0); // tst r5,r5
        if cpu.r[5] == 0 {
            cpu.r[5] = mem.read_u32(0x8c176634).unwrap_or(0x8c1e18f0); // mov.l 0x8c176634,r5
            emit(cpu, mem);
        } else {
            cpu.r[0] = mem.read_u16(0x8c17662e).unwrap_or(0x3dc) as i16 as i32 as u32; // mov.w (sign-extends)
            cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r0
            cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
            cpu.r[6] = cpu.r[0]; // bt.s delay: mov r0,r6
            if cpu.t() {
                cpu.r[3] = 1; // mov #1,r3
            } else {
                cpu.set_t(cpu.r[6] == 0); // tst r6,r6
                if cpu.t() {
                    cpu.r[3] = 1; // mov #1,r3
                } else {
                    cpu.r[3] = 0; // bra delay: mov #0,r3
                }
            }
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[5], cpu.r[3]); // mov.l r3,@r5
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn emit(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c176638).unwrap_or(0x8c174564); // mov.l 0x8c176638,r2
    cpu.r[4] = 101; // jsr delay: mov #101,r4
    cpu.pr = 0x8c17655c;
    crate::registry::p_8c174564::run(cpu, mem);
    cpu.r[0] = 0; // rts delay at 0x8c176560
}
