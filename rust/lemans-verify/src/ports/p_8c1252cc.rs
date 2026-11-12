//! 0x8c1252cc - walk a circular list hanging off `[object+12]`.
//!
//! The sentinel is `**(object+12)`. Each step follows the current node's first
//! word until it returns to `*(object+12)`, calling 0x8c19b624 on `[node+8]+8`.
//! A nonzero `[object+24]` is then cleared by 0x8c1b67f4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[0]);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[4] = cpu.r[4].wrapping_add(12); // jsr delay
    cpu.pr = 0x8c1252e4;
    crate::registry::p_8c124d5c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[15].wrapping_add(8);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[4] = cpu.r[14].wrapping_add(12); // jsr delay
    cpu.pr = 0x8c1252fe;
    crate::registry::p_8c124d48::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
    if !cpu.t() {
        loop {
            cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
            cpu.r[4] = cpu.r[4].wrapping_add(8); // jsr delay
            cpu.pr = 0x8c125312;
            crate::registry::p_8c19b624::run(cpu, mem);
            cpu.r[0] = cpu.r[15].wrapping_add(4);
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[4] = cpu.r[15].wrapping_add(16); // after the push: frame+12
            cpu.r[5] = 0; // jsr delay
            cpu.pr = 0x8c125322;
            crate::registry::p_8c124d50::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            cpu.r[0] = cpu.r[15];
            cpu.r[15] = cpu.r[15].wrapping_sub(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
            cpu.r[4] = cpu.r[14].wrapping_add(12); // jsr delay
            cpu.pr = 0x8c125330;
            crate::registry::p_8c124d48::run(cpu, mem);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r0,r1
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(24)).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if !cpu.t() {
        cpu.pr = 0x8c125346;
        crate::registry::p_8c1b67f4::run(cpu, mem);
    }
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
