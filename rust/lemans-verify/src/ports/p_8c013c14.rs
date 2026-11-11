//! 0x8c013c14 — guarded float transform.
//!
//! Builds flag bits from `fr4`: 8 = negative, 4 = zero, 2 = infinity,
//! 1 = NaN. NaN or infinity return the NaN constant; a negative input is
//! negated first. Then it calls 0x8c01471c(`r15`, 1.0) and, unless the result
//! exceeds 1.0, dispatches on `[r15]` (the callee's int result) through two
//! more helpers, negating the running `fr5` on the 3/4/1/2 paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[0] = 4;
    cpu.fr[5] = cpu.fr[4];
    cpu.r[4] = 0;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[5]);
    cpu.r[3] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
    let pz = (cpu.r[3] as i32) >= 0;
    cpu.set_t(pz);
    cpu.r[5] = 8; // delay slot of `bt.s`
    if !pz {
        cpu.r[4] = cpu.r[5];
    }
    cpu.fr[3] = 0.0;
    cpu.set_t(cpu.fr[5] == cpu.fr[3]);
    if cpu.fr[5] != cpu.fr[3] {
        cpu.r[1] = mem.read_u32(cpu.r[15] + 4).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c013cf8).unwrap_or(0);
        cpu.r[2] = mem.read_u32(0x8c013cfc).unwrap_or(0);
        cpu.r[1] = cpu.r[1] & cpu.r[3];
        let eq = cpu.r[1] == cpu.r[2];
        cpu.set_t(eq);
        if eq {
            cpu.r[0] = 2;
            cpu.r[4] = cpu.r[4] | cpu.r[0];
        } else {
            cpu.set_t(cpu.fr[5] == cpu.fr[5]);
            if cpu.fr[5] != cpu.fr[5] {
                cpu.r[0] = 1;
                cpu.r[4] = cpu.r[4] | cpu.r[0];
            }
        }
    } else {
        cpu.r[2] = 4;
        cpu.r[4] = cpu.r[4] | cpu.r[2];
    }
    cpu.r[2] = 3;
    cpu.r[0] = cpu.r[4];
    let naninf = (cpu.r[0] & cpu.r[2]) != 0;
    cpu.set_t(!naninf);
    if naninf {
        cpu.r[1] = mem.read_u32(0x8c013d00).unwrap_or(0);
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        finish(cpu, mem);
        return;
    }
    cpu.r[5] = 8;
    let neg = (cpu.r[0] & cpu.r[5]) != 0;
    cpu.set_t(!neg);
    if neg {
        cpu.fr[4] = -cpu.fr[4];
    }
    cpu.r[3] = mem.read_u32(0x8c013d08).unwrap_or(0);
    cpu.r[4] = cpu.r[15];
    cpu.r[2] = mem.read_u32(0x8c013d04).unwrap_or(0);
    cpu.fr[5] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // delay slot of `jsr`
    crate::registry::p_8c01471c::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c013d04).unwrap_or(0);
    cpu.fr[4] = cpu.fr[0];
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    let gt = cpu.fr[4] > cpu.fr[3];
    cpu.set_t(gt);
    if gt {
        cpu.r[1] = mem.read_u32(0x8c013d0c).unwrap_or(0);
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        finish(cpu, mem);
        return;
    }
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = cpu.r[0] & 7;
    let odd = (cpu.r[0] & 1) != 0;
    cpu.set_t(odd);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // delay slot of `bt.s`
    if odd {
        cpu.r[3] = mem.read_u32(0x8c013d10).unwrap_or(0);
        cpu.fr[3] = cpu.fr[4];
        cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        cpu.fr[4] = cpu.fr[4] + cpu.fr[3];
    }
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    if cpu.r[0] == 0 || cpu.r[0] == 7 {
        cpu.r[3] = mem.read_u32(0x8c013d14).unwrap_or(0);
        crate::registry::p_8c011d1c::run(cpu, mem);
        cpu.fr[4] = cpu.fr[0];
        cpu.fr[5] = cpu.fr[0];
    } else if cpu.r[0] == 1 || cpu.r[0] == 2 {
        cpu.r[3] = mem.read_u32(0x8c013d18).unwrap_or(0);
        crate::registry::p_8c011d3c::run(cpu, mem);
        cpu.fr[4] = cpu.fr[0];
        cpu.fr[5] = cpu.fr[0];
        cpu.fr[5] = -cpu.fr[5];
    } else if cpu.r[0] == 3 || cpu.r[0] == 4 {
        cpu.r[3] = mem.read_u32(0x8c013d14).unwrap_or(0);
        crate::registry::p_8c011d1c::run(cpu, mem);
        cpu.fr[5] = cpu.fr[0];
        cpu.fr[4] = cpu.fr[0];
        cpu.fr[5] = -cpu.fr[5];
    } else if cpu.r[0] == 5 || cpu.r[0] == 6 {
        cpu.r[3] = mem.read_u32(0x8c013d18).unwrap_or(0);
        crate::registry::p_8c011d3c::run(cpu, mem);
        cpu.fr[5] = cpu.fr[0];
    } else {
        cpu.fr[5] = cpu.fr[0];
    }
    cpu.fr[0] = cpu.fr[5];
    finish(cpu, mem);
}

fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
