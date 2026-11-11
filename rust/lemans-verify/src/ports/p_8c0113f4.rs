//! 0x8c0113f4 — bounded substring search.
//!
//! Takes `haystack` (r4) and `needle` (r5): `r11 = strlen(hay) - strlen(need) + 1`
//! guards the search (non-positive means no match, and the needle must be
//! non-empty). Then it memcmps (0x8c010e50) the needle against each haystack
//! offset from 0 upward and returns the first matching address, else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c011468).unwrap_or(0);
    cpu.r[9] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[4] = cpu.r[5]; // delay slot of `jsr`
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c011468).unwrap_or(0);
    cpu.r[10] = cpu.r[0];
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `jsr`
    crate::registry::p_8c0112f4::run(cpu, mem);
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[10]);
    cpu.r[11] = cpu.r[0];
    cpu.r[11] = cpu.r[11].wrapping_add(1);
    let pl = (cpu.r[11] as i32) > 0;
    cpu.set_t(pl);
    if !pl {
        cpu.r[0] = 0;
        epilogue(cpu, mem);
        return;
    }
    cpu.r[3] = 0;
    let hi = cpu.r[10] > cpu.r[3];
    cpu.set_t(hi);
    if !hi {
        cpu.r[0] = 0;
        epilogue(cpu, mem);
        return;
    }
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[12] = 0;
    cpu.r[8] = mem.read_u32(0x8c01146c).unwrap_or(0);
    loop {
        cpu.r[3] = cpu.r[12];
        let hs = cpu.r[3] >= cpu.r[11];
        cpu.set_t(hs);
        cpu.r[13] = cpu.r[4];
        cpu.r[14] = cpu.r[4]; // delay slot of `bt.s`
        if hs {
            break;
        }
        cpu.r[5] = cpu.r[9];
        cpu.r[6] = cpu.r[10];
        cpu.r[4] = cpu.r[14]; // delay slot of `jsr`
        crate::registry::p_8c010e50::run(cpu, mem);
        let t = cpu.r[0] == 0;
        cpu.set_t(t);
        if t {
            cpu.r[0] = cpu.r[13];
            epilogue(cpu, mem);
            return;
        }
        cpu.r[12] = cpu.r[12].wrapping_add(1);
        let hs2 = cpu.r[12] >= cpu.r[11];
        cpu.set_t(hs2);
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        cpu.r[14] = cpu.r[14].wrapping_add(1); // delay slot of `bf.s`
        if hs2 {
            break;
        }
    }
    cpu.r[0] = 0;
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
