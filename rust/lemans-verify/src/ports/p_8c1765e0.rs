//! 0x8c1765e0 - store `r5` at `[r4+0x3f8]` when it is 32-aligned, else report.
//!
//! Three exits: `r4 == 0` or `r5 == 0` emits str@0x8c1e19a4 with code 101 and
//! returns 0; `(r5 & 31) != 0` emits str@0x8c1e19e0 with code 0x1f5 and returns
//! 0; otherwise `[r4+0x3f8] = r5` and r0 = 1 with T left set by `tst r5,r2`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut emit1 = cpu.t(); // bt 0x8c1765ea
    if !emit1 {
        cpu.set_t(cpu.r[5] == 0); // tst r5,r5
        emit1 = cpu.t(); // bf 0x8c1765f6 -> r5 != 0 goes to the alignment test
    }
    if emit1 {
        cpu.r[3] = mem.read_u32(0x8c176638).unwrap_or(0x8c174564); // mov.l 0x8c176638,r3
        cpu.r[5] = mem.read_u32(0x8c17664c).unwrap_or(0x8c1e19a4); // mov.l 0x8c17664c,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c1765f2;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c17660a
    } else {
        cpu.r[2] = 31; // mov #31,r2
        cpu.set_t((cpu.r[5] & cpu.r[2]) == 0); // tst r5,r2
        if cpu.t() {
            cpu.r[0] = mem.read_u16(0x8c176632).unwrap_or(0x3f8) as i16 as i32 as u32; // mov.w (sign-extends)
            let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // mov.l r5,@(r0,r4)
            cpu.r[0] = 1; // mov #1,r0
        } else {
            cpu.r[2] = mem.read_u32(0x8c176638).unwrap_or(0x8c174564); // mov.l 0x8c176638,r2
            cpu.r[4] = mem.read_u16(0x8c176630).unwrap_or(0x1f5) as i16 as i32 as u32; // mov.w 0x1f5
            cpu.r[5] = mem.read_u32(0x8c176650).unwrap_or(0x8c1e19e0); // mov.l 0x8c176650,r5
            cpu.pr = 0x8c176606; // jsr delay is nop
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0; // rts delay at 0x8c17660a
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
