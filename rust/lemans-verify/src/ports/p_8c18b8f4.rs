//! 0x8c18b8f4 — insert a 20-byte record into a sorted run, dropping the tail.
//!
//! `[object]` is the count and `[object+4]` the base. Records are 20 bytes.
//! The walk starts at the last slot and compares it with `r5` through
//! `0x8c085480` (1 when the slot's halfwords are greater). A greater slot is
//! shifted one place toward the end, except the shift that would land past
//! the count is skipped, so the old last record is dropped. A slot that is
//! not greater ends the walk: if it is the original count the index is
//! returned unwritten, otherwise `r5` is copied into that hole. `0x8c10ef64`
//! sees the two `+12` pointers before the halfwords, `+8` and `+16` move.
//! `r0` is the hole index. `r8`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn mul20(v: u32) -> u32 {
    v.wrapping_shl(2).wrapping_add(v).wrapping_shl(2)
}

fn copy_record(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32, pr: u32) {
    cpu.r[5] = rd(mem, src.wrapping_add(12));
    cpu.r[0] = rd(mem, 0x8c18b9c4); // 0x8c10ef64
    cpu.pr = pr;
    cpu.r[4] = rd(mem, dst.wrapping_add(12)); // jsr delay
    crate::registry::p_8c10ef64::run(cpu, mem);
    let word8 = rd(mem, src.wrapping_add(8));
    let _ = mem.write_u32(dst.wrapping_add(8), word8);
    for off in [0u32, 2, 4, 6] {
        let half = mem.read_u16(src.wrapping_add(off)).unwrap_or(0);
        let _ = mem.write_u16(dst.wrapping_add(off), half);
    }
    let word16 = rd(mem, src.wrapping_add(16));
    let _ = mem.write_u32(dst.wrapping_add(16), word16);
}

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
    cpu.r[9] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[8] = cpu.r[5];

    cpu.r[11] = rd(mem, cpu.r[9]);
    cpu.set_t((cpu.r[11] as i32) > 0);
    let mut tail = true;
    if cpu.t() {
        cpu.r[10] = mul20(cpu.r[11]);
        loop {
            cpu.r[1] = cpu.r[11].wrapping_add((-1i32) as u32);
            cpu.r[12] = mul20(cpu.r[1]);
            cpu.r[5] = rd(mem, cpu.r[9].wrapping_add(4));
            cpu.r[0] = rd(mem, 0x8c18b9c0); // 0x8c085480
            cpu.r[4] = cpu.r[8];
            cpu.pr = 0x8c18b92e;
            cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[12]); // jsr delay
            crate::registry::p_8c085480::run(cpu, mem);
            cpu.r[0] &= 0xff;
            cpu.set_t(cpu.r[0] == 0);
            if cpu.t() {
                cpu.r[0] = rd(mem, cpu.r[9]);
                cpu.set_t(cpu.r[0] == cpu.r[11]);
                if !cpu.t() {
                    break;
                }
                cpu.r[0] = cpu.r[11]; // bra delay
                tail = false;
                break;
            }
            cpu.r[0] = rd(mem, cpu.r[9]);
            cpu.set_t((cpu.r[0] as i32) > (cpu.r[11] as i32));
            if cpu.t() {
                cpu.r[13] = rd(mem, cpu.r[9].wrapping_add(4));
                cpu.r[14] = cpu.r[13].wrapping_add(cpu.r[12]);
                cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[10]);
                cpu.set_t(cpu.r[14] == cpu.r[13]);
                if !cpu.t() {
                    let dst = cpu.r[13];
                    let src = cpu.r[14];
                    copy_record(cpu, mem, dst, src, 0x8c18b94e);
                }
            }
            cpu.r[11] = cpu.r[11].wrapping_sub(1);
            cpu.set_t((cpu.r[11] as i32) > 0);
            let cont = cpu.t();
            cpu.r[10] = cpu.r[10].wrapping_sub(20); // bt.s delay
            if !cont {
                break;
            }
        }
    }
    if tail {
        cpu.r[10] = mul20(cpu.r[11]);
        cpu.r[0] = rd(mem, cpu.r[9].wrapping_add(4));
        cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[0]);
        cpu.set_t(cpu.r[8] == cpu.r[10]);
        if !cpu.t() {
            let dst = cpu.r[10];
            let src = cpu.r[8];
            copy_record(cpu, mem, dst, src, 0x8c18b992);
        }
        cpu.r[0] = cpu.r[11];
    }

    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = rd(mem, cpu.r[15]);
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
