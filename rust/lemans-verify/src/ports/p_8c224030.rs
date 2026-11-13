//! 0x8c224030 — locate a marker in a `0x148`-byte record and copy 12 bytes.
//!
//! `record = 0x8c2c02b8 + sext(r4) * 0x148` (`mul.l` fills both MAC halves).
//! Byte `+2` clear, or either gate call returning zero, returns sign-extended
//! `0xff01`. A negative result from `0x8c220f96` returns `0xff03`. The scan
//! walks 32-byte steps from `0x8c2259a4` while the cursor is below
//! `[record+28]+48`, accepting byte `0x33` or `0xcc`. No match returns
//! `0xff05`. A match copies the 12 bytes at `marker+4` into `r5` and plants a
//! NUL, returning 0.
//!
//! `r13` and `r14` are restored. The destination and the product sit in the
//! 8-byte frame.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

fn sext16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    let value = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    cpu.r[14] = sext16(mem, 0x8c2240f6); // 0x148
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = rd(mem, 0x8c224100);
    let product = (cpu.r[14] as i32 as i64) * (cpu.r[13] as i32 as i64); // mul.l r14, r13
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    cpu.r[14] = cpu.macl;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[14]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[3]);
    cpu.r[0] = sext8(mem, cpu.r[14].wrapping_add(2));
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c2240f8); // 0xffffff01
        epilogue(cpu, mem);
        return;
    }

    cpu.r[3] = rd(mem, 0x8c224104);
    cpu.pr = 0x8c224054;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c222430::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[3] = rd(mem, 0x8c224108);
        cpu.pr = 0x8c22405e;
        cpu.r[4] = cpu.r[13];
        crate::registry::p_8c222c3a::run(cpu, mem);
        cpu.r[0] = sext16(mem, 0x8c2240f8);
        epilogue(cpu, mem);
        return;
    }

    cpu.r[2] = rd(mem, 0x8c22410c);
    cpu.pr = 0x8c224068;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c222470::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c2240f8);
        epilogue(cpu, mem);
        return;
    }

    cpu.r[2] = rd(mem, cpu.r[15].wrapping_add(4));
    cpu.r[3] = rd(mem, 0x8c224100);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
    cpu.r[0] = sext8(mem, cpu.r[2].wrapping_add(8));
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c2240f8);
        epilogue(cpu, mem);
        return;
    }

    cpu.r[3] = rd(mem, 0x8c224110);
    cpu.pr = 0x8c224084;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c220f96::run(cpu, mem);
    cpu.set_t((cpu.r[0] as i32) >= 0); // cmp/pz
    if !cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c2240fa); // 0xffffff03
        epilogue(cpu, mem);
        return;
    }

    cpu.r[3] = rd(mem, 0x8c224114);
    cpu.r[5] = rd(mem, cpu.r[14].wrapping_add(24));
    cpu.pr = 0x8c224096;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c2259a4::run(cpu, mem);
    cpu.r[5] = rd(mem, cpu.r[14].wrapping_add(24));
    cpu.r[7] = 0;
    cpu.r[4] = cpu.r[0];
    cpu.r[1] = 1;
    loop {
        cpu.r[2] = rd(mem, cpu.r[14].wrapping_add(28));
        cpu.r[3] = rd(mem, cpu.r[2].wrapping_add(48));
        cpu.set_t(cpu.r[5] >= cpu.r[3]); // cmp/ge, unsigned
        if cpu.t() {
            break;
        }
        cpu.r[3] = rd(mem, cpu.r[14].wrapping_add(24));
        cpu.r[3] = cpu.r[3].wrapping_add(1);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[3]);
        cpu.r[13] = sext8(mem, cpu.r[4]);
        cpu.r[6] = cpu.r[13] & 0xff;
        cpu.r[0] = cpu.r[6];
        cpu.set_t(cpu.r[0] == 51);
        if !cpu.t() {
            cpu.r[3] = sext16(mem, 0x8c2240fc); // 0xcc
            cpu.set_t(cpu.r[6] == cpu.r[3]);
            if !cpu.t() {
                cpu.r[4] = cpu.r[4].wrapping_add(32);
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                continue;
            }
        }
        cpu.r[7] = cpu.r[1];
        break;
    }

    cpu.set_t(cpu.r[7] == 0);
    if cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c2240fe); // 0xffffff05
        epilogue(cpu, mem);
        return;
    }

    cpu.r[5] = cpu.r[4];
    cpu.r[4] = rd(mem, cpu.r[15]);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.r[14] = cpu.r[4];
    cpu.r[14] = cpu.r[14].wrapping_add(12);
    loop {
        cpu.set_t(cpu.r[4] >= cpu.r[14]); // cmp/hs
        if cpu.t() {
            break;
        }
        cpu.r[3] = sext8(mem, cpu.r[5]);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
    }
    cpu.r[3] = 0;
    cpu.r[0] = cpu.r[3];
    let _ = mem.write_u8(cpu.r[14], cpu.r[3] as u8);
    epilogue(cpu, mem);
}
