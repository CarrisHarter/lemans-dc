//! 0x8c128830 — rewrite bits 13..14 and 9..7 of the halfword at `r5+4`.
//!
//! `[r4+8] == 0` clears bits 5 and 6 of the high byte; `== 1` sets bit 5 and
//! clears bit 6. Bits 9..7 become 3, 2, 1 or 0 when the two flags are both
//! set, only the first, only the second, or neither. Each flag is forced by
//! `[r4+12]` / `[r4+16]` when that word is 0 or 1, otherwise it comes from
//! bits 9..7 of the halfword (1, 2 or 3). `r11`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn sext16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = word(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn lookup(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = word(mem, 0x8c128934); // 0x8c05f294
    cpu.r[4] = cpu.r[12];
    crate::registry::p_8c05f294::run(cpu, mem);
}

fn patch_high_byte(cpu: &mut Cpu, mem: &mut Memory, or_bit: u32) {
    cpu.pr = 0x8c12884a;
    lookup(cpu, mem);
    cpu.r[3] = cpu.r[0];
    cpu.r[0] = mem.read_u8(cpu.r[3].wrapping_add(1)).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[1] = (-97i32) as u32; // 0xffffff9f
    cpu.r[0] &= cpu.r[1];
    cpu.r[0] |= or_bit;
    let _ = mem.write_u8(cpu.r[3].wrapping_add(1), cpu.r[0] as u8);
}

fn store_field(cpu: &mut Cpu, mem: &mut Memory, bits: u32) {
    cpu.pr = 0x8c1288d2;
    lookup(cpu, mem);
    cpu.r[4] = sext16(mem, cpu.r[0]);
    cpu.r[3] = word(mem, 0x8c128938); // 0xfffffc7f
    cpu.r[1] = word(mem, bits);
    cpu.r[4] &= cpu.r[3];
    cpu.r[4] |= cpu.r[1];
    let _ = mem.write_u16(cpu.r[0], cpu.r[4] as u16); // bra delay
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[11] = cpu.r[4];

    cpu.r[0] = word(mem, cpu.r[11].wrapping_add(8));
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        patch_high_byte(cpu, mem, 0);
    } else {
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            patch_high_byte(cpu, mem, 32);
        }
    }

    cpu.r[14] = 0;
    cpu.r[4] = cpu.r[12];
    cpu.r[13] = cpu.r[14]; // jsr delay
    cpu.pr = 0x8c128878;
    cpu.r[0] = word(mem, 0x8c128934);
    crate::registry::p_8c05f294::run(cpu, mem);
    cpu.r[3] = sext16(mem, cpu.r[0]);
    cpu.r[1] = 22;
    cpu.r[0] = (-29i32) as u32;
    cpu.r[3] <<= 22; // shld r1, r3 (r1 is 22)
    let amount = cpu.r[0] as i32; // -29
    cpu.r[3] = ((cpu.r[3] as i32) >> ((-amount) as u32 & 31)) as u32; // shad
    cpu.r[0] = cpu.r[3] as i16 as i32 as u32; // exts.w
    cpu.set_t(cpu.r[0] == 1);
    if cpu.t() {
        cpu.r[13] = 1;
    } else {
        cpu.set_t(cpu.r[0] == 2);
        if cpu.t() {
            cpu.r[14] = 1;
        } else {
            cpu.set_t(cpu.r[0] == 3);
            if cpu.t() {
                cpu.r[14] = 1;
                cpu.r[13] = cpu.r[14];
            }
        }
    }

    cpu.r[0] = word(mem, cpu.r[11].wrapping_add(12));
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[14] = 0;
    } else {
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            cpu.r[14] = 1;
        }
    }
    cpu.r[0] = word(mem, cpu.r[11].wrapping_add(16));
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[13] = 0;
    } else {
        cpu.set_t(cpu.r[0] == 1);
        if cpu.t() {
            cpu.r[13] = 1;
        }
    }

    cpu.r[0] = cpu.r[14] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[13] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            store_field(cpu, mem, 0x8c12893c); // 0x180
        } else {
            // 0x8c1288e0 tests r14 again, so T is clear on this path.
            cpu.r[0] = cpu.r[14] & 0xff;
            cpu.set_t(cpu.r[0] == 0);
            store_field(cpu, mem, 0x8c128940); // 0x100
        }
    } else {
        cpu.r[0] = cpu.r[14] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            store_field(cpu, mem, 0x8c128940);
        } else {
            cpu.r[0] = cpu.r[13] & 0xff;
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                store_field(cpu, mem, 0x8c128944); // 0x80
            } else {
                cpu.pr = 0x8c12891a;
                lookup(cpu, mem);
                cpu.r[3] = cpu.r[0];
                cpu.r[0] = sext16(mem, cpu.r[3]);
                cpu.r[1] = word(mem, 0x8c128938);
                cpu.r[0] &= cpu.r[1];
                let _ = mem.write_u16(cpu.r[3], cpu.r[0] as u16);
            }
        }
    }

    cpu.pr = pop(cpu, mem);
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
