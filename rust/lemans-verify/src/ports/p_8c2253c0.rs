//! 0x8c2253c0 - 1 when two strings have the same capped length and the same bytes.
//!
//! Both lengths come from 0x8c225392 (capped at 12). A length of 0 is a match
//! without entering the compare loop. The `bf.s` delay slots advance both
//! pointers even on the exiting iteration.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bsr delay
    cpu.pr = 0x8c2253ce;
    crate::registry::p_8c225392::run(cpu, mem);
    cpu.r[14] = cpu.r[0];
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bsr delay
    cpu.pr = 0x8c2253d4;
    crate::registry::p_8c225392::run(cpu, mem);
    cpu.r[4] = cpu.r[0];
    cpu.set_t(cpu.r[4] == cpu.r[14]); // cmp/eq r4,r14
    let mut matched = false;
    if cpu.t() {
        cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
        cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl r14
        cpu.r[6] = 0;
        cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // bf.s delay
        if cpu.t() {
            loop {
                cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
                cpu.r[2] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
                cpu.set_t(cpu.r[3] == cpu.r[2]); // cmp/eq r3,r2
                if !cpu.t() {
                    break;
                }
                cpu.r[6] = cpu.r[6].wrapping_add(1);
                cpu.set_t((cpu.r[6] as i32) >= (cpu.r[14] as i32)); // cmp/ge r14,r6
                cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay
                cpu.r[5] = cpu.r[5].wrapping_add(1); // inner bf.s delay, always
                if cpu.t() {
                    matched = true;
                    break;
                }
            }
        } else {
            matched = true;
        }
    }
    cpu.r[0] = u32::from(matched);
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
