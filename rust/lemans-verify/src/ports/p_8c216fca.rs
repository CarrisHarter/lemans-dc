//! 0x8c216fca - 1-based index of a matching record, or 0.
//!
//! Checksums the string at r4. Scans `[r5]` records of stride 44 starting at
//! `r5+16`. Byte +9 must equal the checksum's low byte. `r6 == 1` compares
//! the string at +10 with 0x8c216dbc; otherwise with 0x8c216dde. A non-positive
//! count returns 0.

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
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = cpu.r[5];
    cpu.r[14] = cpu.r[14].wrapping_add(16);
    cpu.r[12] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]);
    cpu.r[11] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[4] = cpu.r[12]; // bsr delay
    cpu.pr = 0x8c216fe6;
    crate::registry::p_8c216e3e::run(cpu, mem);
    cpu.set_t((cpu.r[11] as i32) > 0); // cmp/pl r11
    cpu.r[10] = cpu.r[0];
    cpu.r[13] = 0; // bf.s delay
    let mut result = 0u32;
    if cpu.t() {
        loop {
            cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(9)).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[3] = cpu.r[10] & 0xff;
            cpu.r[0] &= 0xff;
            cpu.set_t(cpu.r[3] == cpu.r[0]);
            if cpu.t() {
                cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.set_t(cpu.r[0] == 1);
                cpu.r[5] = cpu.r[14];
                cpu.r[5] = cpu.r[5].wrapping_add(10);
                cpu.r[4] = cpu.r[12];
                if cpu.t() {
                    cpu.pr = 0x8c217006;
                    crate::registry::p_8c216dbc::run(cpu, mem);
                    cpu.set_t(cpu.r[0] == 1);
                    if cpu.t() {
                        result = cpu.r[13].wrapping_add(1);
                        break;
                    }
                } else {
                    cpu.pr = 0x8c217016;
                    crate::registry::p_8c216dde::run(cpu, mem);
                    cpu.set_t(cpu.r[0] == 1);
                    if cpu.t() {
                        result = cpu.r[13].wrapping_add(1);
                        break;
                    }
                }
            }
            cpu.r[13] = cpu.r[13].wrapping_add(1);
            cpu.set_t((cpu.r[13] as i32) >= (cpu.r[11] as i32)); // cmp/ge r11,r13
            cpu.r[14] = cpu.r[14].wrapping_add(44); // bf.s delay
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = result;
    cpu.r[15] = cpu.r[15].wrapping_add(4);
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
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
