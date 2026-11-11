//! 0x8c01b748 — classify and dispatch.
//!
//! Maps `r5` (1..4) to an index, calls 0x8c074d58(`word[pool]`, idx, 8, 0),
//! and when that returns non-zero walks a table from `[r4+72]`; for the first
//! non-zero entry at `[entry+0x15c]` it runs a byte scan, records global
//! state, and calls 0x8c19f554(`[r12+104]`, 1) after tagging `[r12+84]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c01b814).unwrap_or(0);
    cpu.r[14] = 0;
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[0] = cpu.r[13];
    let eq1 = cpu.r[0] == 1;
    cpu.set_t(eq1);
    if eq1 {
        cpu.r[14] = 0; // stays 0
    } else {
        let eq2 = cpu.r[0] == 2;
        cpu.set_t(eq2);
        if eq2 {
            cpu.r[14] = 1;
        } else {
            let eq3 = cpu.r[0] == 3;
            cpu.set_t(eq3);
            if eq3 {
                cpu.r[14] = 2;
            } else {
                let eq4 = cpu.r[0] == 4;
                cpu.set_t(eq4);
                cpu.r[14] = if eq4 { 3 } else { 0 };
            }
        }
    }
    cpu.r[0] = mem.read_u32(0x8c01b818).unwrap_or(0);
    cpu.r[5] = cpu.r[14];
    cpu.r[7] = 0;
    cpu.r[6] = 8; // delay slot of `jsr`
    crate::registry::p_8c074d58::run(cpu, mem);
    cpu.r[0] = cpu.r[0] & 0xff;
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    if t {
        finish(cpu, mem);
        return;
    }
    cpu.r[0] = cpu.r[12];
    cpu.r[0] = cpu.r[0].wrapping_add(72);
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[13] = cpu.r[13] << 2;
    cpu.r[4] = 0xfffffffc;
    cpu.r[0] = cpu.r[13];
    cpu.r[5] = mem.read_u32(cpu.r[3] + 8).unwrap_or(0);
    cpu.r[1] = 0x15c;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[4]);
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3] + 4).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[0] = cpu.r[1];
    cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    let t2 = cpu.r[0] == 0;
    cpu.set_t(t2);
    if t2 {
        // probe the byte scan path head
    } else {
        cpu.r[0] = 0xa6;
        cpu.r[7] = 0;
        cpu.r[12] = cpu.r[7];
        cpu.r[0] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0) as u32;
        cpu.r[6] = 1;
        cpu.r[4] = cpu.r[0] & 0xffff;
        loop {
            cpu.r[5] = mem.read_u32(cpu.r[3] + 8).unwrap_or(0);
            cpu.r[0] = cpu.r[12];
            cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[3] + 4).unwrap_or(0);
            cpu.r[5] = cpu.r[5] << 2;
            cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
            cpu.r[0] = cpu.r[1];
            cpu.r[0] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0) as u32;
            cpu.r[0] = cpu.r[0] & 0xff;
            cpu.r[0] = cpu.r[0] & 0xff;
            let tz = cpu.r[0] == 0;
            cpu.set_t(tz);
            if tz {
                cpu.r[6] = 0;
            } else {
                cpu.r[12] = cpu.r[12].wrapping_add(4);
                cpu.r[7] = cpu.r[7].wrapping_add(1);
            }
            let gt = (cpu.r[4] as i32) > (cpu.r[7] as i32);
            cpu.set_t(gt);
            if !gt {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[6] & 0xff;
    let t3 = cpu.r[0] == 0;
    cpu.set_t(t3);
    if t3 {
        finish(cpu, mem);
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c01b828).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let t4 = cpu.r[1] == 0;
    cpu.set_t(t4);
    if t4 {
        finish(cpu, mem);
        return;
    }
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[1] + 4, cpu.r[0] as u8);
    finish(cpu, mem);
}

fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
