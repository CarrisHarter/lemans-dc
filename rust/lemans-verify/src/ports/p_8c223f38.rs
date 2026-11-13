//! 0x8c223f38 — copy 12 bytes once a record scan hits `0x33` or `0xcc`.
//!
//! The record is `0x8c2c02b8 + 0x148*r4` (the product is signed). A clear byte
//! at `+2`, a failed `0x8c222430` (which then runs `0x8c222c3a`), a failed
//! `0x8c222470`, or a clear byte at `+8` returns `0xffffff01`. A negative
//! result from `0x8c220f96` returns `0xffffff03`. Otherwise `0x8c2259a4`
//! supplies the scan base and the loop steps 32 bytes until the count at
//! `[record+28]+48`. No marker returns `0xffffff05`. A marker copies 12 bytes
//! from four past the marker onto the pointer passed in `r5`, plants a 0,
//! and returns 0. `r12`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn hw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn epilogue(cpu: &mut Cpu, mem: &Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let frame = cpu.r[15];
    cpu.r[14] = hw(mem, 0x8c223fee); // 0x148
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = word(mem, 0x8c223ffc); // 0x8c2c02b8
    let product = (cpu.r[14] as i32 as i64) * (cpu.r[13] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    let _ = mem.write_u32(frame, cpu.r[5]);
    cpu.r[14] = cpu.macl;
    let _ = mem.write_u32(frame.wrapping_add(4), cpu.r[14]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[3]);
    let record = cpu.r[14];
    cpu.r[0] = mem.read_u8(record.wrapping_add(2)).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[3] = word(mem, 0x8c224000); // 0x8c222430
        cpu.r[4] = cpu.r[13];
        cpu.pr = 0x8c223f5e;
        crate::registry::p_8c222430::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            cpu.r[3] = word(mem, 0x8c224004); // 0x8c222c3a
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c223f68;
            crate::registry::p_8c222c3a::run(cpu, mem);
            cpu.r[0] = hw(mem, 0x8c223ff0); // 0xff01
            epilogue(cpu, mem);
            return;
        }
        cpu.r[2] = word(mem, 0x8c224008); // 0x8c222470
        cpu.r[4] = cpu.r[13];
        cpu.pr = 0x8c223f72;
        crate::registry::p_8c222470::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[2] = word(mem, frame.wrapping_add(4));
            cpu.r[3] = word(mem, 0x8c223ffc);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            cpu.r[0] = mem.read_u8(cpu.r[2].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                cpu.r[3] = word(mem, 0x8c223ff4); // 0x8c220f96
                cpu.r[4] = cpu.r[13];
                cpu.pr = 0x8c223f8e;
                crate::registry::p_8c220f96::run(cpu, mem);
                cpu.set_t((cpu.r[0] as i32) >= 0);
                if cpu.t() {
                    scan(cpu, mem, record, frame);
                    return;
                }
                cpu.r[0] = hw(mem, 0x8c223fea); // 0xff03
                epilogue(cpu, mem);
                return;
            }
        }
    }
    cpu.r[0] = hw(mem, 0x8c223ff0); // 0xff01
    epilogue(cpu, mem);
}

fn scan(cpu: &mut Cpu, mem: &mut Memory, record: u32, frame: u32) {
    cpu.r[3] = word(mem, 0x8c22400c); // 0x8c2259a4
    cpu.r[12] = 0;
    cpu.r[5] = cpu.r[12];
    let _ = mem.write_u32(record.wrapping_add(24), cpu.r[12]);
    cpu.r[4] = cpu.r[13];
    cpu.pr = 0x8c223fa4;
    crate::registry::p_8c2259a4::run(cpu, mem);
    cpu.r[5] = cpu.r[12];
    cpu.r[7] = cpu.r[12];
    cpu.r[4] = cpu.r[0];
    cpu.r[13] = 1; // bra delay
    let marker = hw(mem, 0x8c223ff2); // 0xcc
    loop {
        cpu.r[2] = word(mem, record.wrapping_add(28));
        cpu.r[3] = word(mem, cpu.r[2].wrapping_add(48));
        cpu.set_t((cpu.r[5] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r5
        if cpu.t() {
            break;
        }
        cpu.r[1] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[6] = cpu.r[1] & 0xff;
        cpu.r[0] = cpu.r[6];
        cpu.set_t(cpu.r[0] == 51);
        if !cpu.t() {
            cpu.r[2] = marker;
            cpu.set_t(cpu.r[2] == cpu.r[6]);
            if !cpu.t() {
                cpu.r[3] = word(mem, record.wrapping_add(24));
                cpu.r[4] = cpu.r[4].wrapping_add(32);
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                cpu.r[3] = cpu.r[3].wrapping_add(1);
                let _ = mem.write_u32(record.wrapping_add(24), cpu.r[3]);
                continue;
            }
        }
        cpu.r[7] = cpu.r[13];
        break;
    }
    cpu.set_t(cpu.r[7] == 0);
    if cpu.t() {
        cpu.r[0] = hw(mem, 0x8c223fec); // 0xff05
        epilogue(cpu, mem);
        return;
    }
    cpu.r[5] = cpu.r[4].wrapping_add(4);
    cpu.r[4] = word(mem, frame);
    cpu.r[13] = cpu.r[4].wrapping_add(12);
    loop {
        cpu.set_t(cpu.r[4] >= cpu.r[13]); // cmp/hs r13,r4
        if cpu.t() {
            break;
        }
        cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
    }
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[13], cpu.r[12] as u8);
    cpu.r[3] = word(mem, record.wrapping_add(24));
    cpu.r[3] = cpu.r[3].wrapping_add(1);
    let _ = mem.write_u32(record.wrapping_add(24), cpu.r[3]);
    epilogue(cpu, mem);
}
