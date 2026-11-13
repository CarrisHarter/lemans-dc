//! 0x8c1767b2 - clear a 1020-byte record and stamp sixteen 52-byte slots.
//!
//! A null object or a null string reports code 101 and returns 0. A string
//! longer than 80 reports code 0x458 and returns 0. Otherwise memset clears
//! 0x3fc bytes, strcpy plants the string at +0x364, and the loop writes
//! 127/63/0x666 into each slot. cmp/hs against 0x340 leaves T set and r0 = 1.
//! The bf.s delay store runs on the iteration that stops the loop.

use sh4_oracle::{Cpu, Memory};

fn report(cpu: &mut Cpu, mem: &mut Memory, code_addr: u32, text_addr: u32, ret: u32) {
    cpu.r[3] = mem.read_u32(0x8c17689c).unwrap_or(0x8c174564);
    if code_addr == 0 {
        cpu.r[5] = mem.read_u32(text_addr).unwrap_or(0);
        cpu.r[4] = 101; // jsr delay
    } else {
        cpu.r[1] = cpu.r[3];
        cpu.r[4] = mem.read_u16(code_addr).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[5] = mem.read_u32(text_addr).unwrap_or(0);
    }
    cpu.pr = ret;
    crate::registry::p_8c174564::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.set_t(cpu.r[14] == 0);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    let null_obj = cpu.t();
    cpu.r[13] = cpu.r[5]; // bt.s delay
    if null_obj || cpu.r[13] == 0 {
        if !null_obj {
            cpu.set_t(cpu.r[13] == 0);
        }
        report(cpu, mem, 0, 0x8c1768a0, 0x8c1767cc);
        cpu.r[0] = 0;
    } else {
        cpu.set_t(false);
        cpu.r[2] = mem.read_u32(0x8c1768a4).unwrap_or(0x8c0112f4);
        cpu.pr = 0x8c1767d6;
        cpu.r[4] = cpu.r[13];
        crate::registry::p_8c0112f4::run(cpu, mem);
        cpu.r[3] = 80;
        cpu.set_t(cpu.r[0] > cpu.r[3]); // cmp/hi
        if cpu.t() {
            report(cpu, mem, 0x8c176888, 0x8c1768a8, 0x8c1767e6);
            cpu.r[0] = 0;
        } else {
            cpu.r[3] = mem.read_u32(0x8c1768ac).unwrap_or(0x8c010f88);
            cpu.r[5] = 0;
            cpu.r[6] = mem.read_u16(0x8c17688a).unwrap_or(0) as i16 as i32 as u32;
            cpu.pr = 0x8c1767f4;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c010f88::run(cpu, mem);
            cpu.r[2] = mem.read_u32(0x8c1768b0).unwrap_or(0x8c016484);
            cpu.r[1] = cpu.r[13];
            cpu.r[0] = mem.read_u16(0x8c17688c).unwrap_or(0) as i16 as i32 as u32;
            cpu.pr = 0x8c1767fe;
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[14]);
            crate::registry::p_8c016484::run(cpu, mem);
            cpu.r[13] = mem.read_u16(0x8c17688e).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[6] = 63;
            cpu.r[5] = mem.read_u16(0x8c176890).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[4] = 0;
            cpu.r[7] = 127;
            loop {
                cpu.r[3] = cpu.r[14].wrapping_add(cpu.r[4]);
                cpu.r[2] = cpu.r[14];
                let _ = mem.write_u32(cpu.r[3].wrapping_add(32), cpu.r[7]);
                cpu.r[3] = cpu.r[14];
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
                let _ = mem.write_u32(cpu.r[2].wrapping_add(36), cpu.r[6]);
                cpu.r[2] = cpu.r[14];
                let _ = mem.write_u32(cpu.r[3].wrapping_add(40), cpu.r[5]);
                cpu.r[3] = cpu.r[14];
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
                let _ = mem.write_u32(cpu.r[2].wrapping_add(44), cpu.r[5]);
                cpu.r[2] = cpu.r[14];
                cpu.r[4] = cpu.r[4].wrapping_add(52);
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(48), cpu.r[5]);
                let _ = mem.write_u32(cpu.r[2].wrapping_add(32), cpu.r[7]);
                cpu.r[2] = cpu.r[14];
                cpu.r[3] = cpu.r[14];
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(36), cpu.r[6]);
                cpu.r[3] = cpu.r[14];
                let _ = mem.write_u32(cpu.r[2].wrapping_add(40), cpu.r[5]);
                cpu.r[2] = cpu.r[14];
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
                cpu.r[4] = cpu.r[4].wrapping_add(52);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(44), cpu.r[5]);
                let cont = cpu.r[4] < cpu.r[13];
                cpu.set_t(cpu.r[4] >= cpu.r[13]);
                let _ = mem.write_u32(cpu.r[2].wrapping_add(48), cpu.r[5]);
                if !cont {
                    break;
                }
            }
            cpu.r[0] = 1;
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
