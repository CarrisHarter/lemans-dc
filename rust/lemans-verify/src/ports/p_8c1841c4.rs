//! 0x8c1841c4 - release an array of records, then the container.
//!
//! A null container returns 0. Otherwise the global at the pool pointer is
//! cleared and every non-null slot is released (0x8c184c20, 0x8c018bd8,
//! then free). The array itself is freed when non-null, 0x8c01e078 runs
//! with a zero count, and a positive entry r5 frees the container.

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
    cpu.r[11] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[10] = cpu.r[4];
    cpu.set_t(cpu.r[10] == 0);
    if cpu.r[10] != 0 {
        cpu.r[1] = mem.read_u32(0x8c18424c).unwrap_or(0);
        cpu.r[0] = 0;
        cpu.r[12] = (-1i32) as u32;
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        cpu.r[14] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
        loop {
            cpu.r[4] = mem.read_u32(cpu.r[10].wrapping_add(8)).unwrap_or(0);
            cpu.r[1] = mem.read_u32(cpu.r[10].wrapping_add(4)).unwrap_or(0);
            cpu.r[0] = cpu.r[4];
            cpu.r[1] = cpu.r[1].wrapping_shl(2);
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
            cpu.set_t(cpu.r[14] == cpu.r[0]);
            if cpu.t() {
                break;
            }
            cpu.r[13] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.set_t(cpu.r[13] == 0);
            if cpu.r[13] != 0 {
                cpu.r[4] = cpu.r[13];
                cpu.r[5] = cpu.r[12];
                cpu.r[4] = cpu.r[4].wrapping_add(4);
                cpu.pr = 0x8c1841f4;
                crate::registry::p_8c184c20::run(cpu, mem);
                cpu.r[5] = cpu.r[12];
                cpu.r[4] = cpu.r[13];
                cpu.pr = 0x8c1841fc;
                crate::registry::p_8c018bd8::run(cpu, mem);
                cpu.r[4] = cpu.r[13];
                cpu.pr = 0x8c184202;
                crate::registry::p_8c01032c::run(cpu, mem);
            }
            cpu.r[14] = cpu.r[14].wrapping_add(4);
        }
        cpu.set_t(cpu.r[10] == 0);
        if cpu.r[10] != 0 {
            cpu.set_t(cpu.r[10] == 0);
            if cpu.r[10] != 0 {
                cpu.set_t(cpu.r[4] == 0);
                if cpu.r[4] != 0 {
                    cpu.pr = 0x8c184224;
                    crate::registry::p_8c01032c::run(cpu, mem);
                }
                cpu.set_t(cpu.r[10] == 0);
                if cpu.r[10] != 0 {
                    cpu.r[4] = cpu.r[10];
                    cpu.r[5] = 0;
                    cpu.pr = 0x8c184230;
                    crate::registry::p_8c01e078::run(cpu, mem);
                }
            }
        }
        cpu.r[0] = cpu.r[11] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            cpu.r[4] = cpu.r[10];
            cpu.pr = 0x8c18423c;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[10];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
