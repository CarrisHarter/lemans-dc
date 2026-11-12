//! 0x8c19c9b8 - find a node whose string matches r5.
//!
//! Inits the object, then walks the circular list at `object+4`. A match
//! stores `[node+8]` at `object+20`. A miss leaves that word 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[14] = cpu.r[5];
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = cpu.r[6]; // jsr delay
    cpu.pr = 0x8c19c9ca;
    crate::registry::p_8c19d39c::run(cpu, mem);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[0] = mem.read_u32(0x8c19ca6c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(20), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[1]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c19c9e6;
    crate::registry::p_8c19ca80::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    cpu.r[0] = cpu.r[15].wrapping_add(4);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
    cpu.pr = 0x8c19ca02;
    crate::registry::p_8c19ca78::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == cpu.r[1]);
    if !cpu.t() {
        loop {
            cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.r[6] = cpu.r[14];
            cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
            cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // bra delay
            loop {
                cpu.r[3] = mem.read_u8(cpu.r[6]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[0] = mem.read_u8(cpu.r[7]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[3] &= 0xff;
                cpu.r[1] = cpu.r[3] & 0xff;
                cpu.r[0] &= 0xff;
                cpu.r[0] &= 0xff;
                cpu.r[6] = cpu.r[6].wrapping_add(1);
                cpu.set_t(cpu.r[0] == cpu.r[1]);
                cpu.r[7] = cpu.r[7].wrapping_add(1); // bt.s delay
                if !cpu.t() {
                    cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
                    break;
                }
                cpu.r[0] = cpu.r[3] & 0xff;
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    cpu.r[1] = 0; // bra delay
                    break;
                }
            }
            cpu.set_t(cpu.r[1] == 0);
            if cpu.t() {
                cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(20), cpu.r[4]); // bra delay
                break;
            }
            cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
            cpu.r[0] = cpu.r[15].wrapping_add(4);
            let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[1]);
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
            cpu.pr = 0x8c19ca52;
            crate::registry::p_8c19ca78::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == cpu.r[1]);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
