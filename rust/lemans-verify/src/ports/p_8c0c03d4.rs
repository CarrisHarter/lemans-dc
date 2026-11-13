//! 0x8c0c03d4 — drain each 108-byte record, then free the array.
//!
//! `r4` is the object. A null `[object+8]` returns with T set and does not
//! free. Otherwise the end pointer is `base + sext(count) * 108`. While the
//! end is unsigned-above the cursor, `0x8c0be110(cursor, -1)` drains that
//! record and does not free it. The original base is then freed through
//! `0x8c01032c`, including when the count is 0 or the scaled end wraps below
//! the base. `r11`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = rd(mem, cpu.r[14].wrapping_add(8));
    cpu.set_t(cpu.r[12] == 0);
    if !cpu.t() {
        cpu.r[1] = rd(mem, cpu.r[14].wrapping_add(4));
        cpu.r[0] = 108;
        cpu.r[13] = cpu.r[12];
        let prod = (cpu.r[1] as i32 as i64).wrapping_mul(cpu.r[0] as i32 as i64);
        cpu.mach = (prod >> 32) as u32;
        cpu.macl = prod as u32;
        cpu.r[1] = cpu.macl;
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[1]);
        cpu.set_t(cpu.r[13] > cpu.r[12]); // cmp/hi r12, r13
        if cpu.t() {
            cpu.r[11] = (-1i32) as u32;
            loop {
                cpu.r[0] = rd(mem, 0x8c0c0418); // 0x8c0be110
                cpu.r[5] = cpu.r[11];
                cpu.pr = 0x8c0c0400;
                cpu.r[4] = cpu.r[12]; // jsr delay
                crate::registry::p_8c0be110::run(cpu, mem);
                cpu.r[12] = cpu.r[12].wrapping_add(108);
                cpu.set_t(cpu.r[13] > cpu.r[12]); // cmp/hi r12, r13
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = rd(mem, 0x8c0c041c); // 0x8c01032c
        cpu.pr = 0x8c0c040c;
        cpu.r[4] = rd(mem, cpu.r[14].wrapping_add(8)); // jsr delay
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
