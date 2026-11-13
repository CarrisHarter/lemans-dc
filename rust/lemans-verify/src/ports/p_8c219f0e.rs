//! 0x8c219f0e - dispatch r4 to one of ten field stores.
//!
//! r5 is the value. 0..9 each call a store and return 0; anything else
//! returns -1 without a call. The matched cmp/eq leaves T set, and none of
//! the stores touch T, so a hit ends with T set. The miss falls out of
//! cmp/eq #9 with T clear. r14 is restored either way.

use sh4_oracle::{Cpu, Memory};

fn store(cpu: &mut Cpu, mem: &mut Memory, pool: u32, ret: u32, f: fn(&mut Cpu, &mut Memory)) {
    cpu.r[3] = mem.read_u32(pool).unwrap_or(0);
    cpu.pr = ret;
    cpu.r[4] = cpu.r[14]; // jsr delay
    f(cpu, mem);
    cpu.r[0] = 0;
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[0] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.set_t(cpu.r[0] == 1);
    cpu.r[14] = cpu.r[5]; // bt.s delay
    if cpu.r[0] == 1 {
        store(cpu, mem, 0x8c21a05c, 0x8c219f48, crate::registry::p_8c21b0a2::run);
    } else if {
        cpu.set_t(cpu.r[0] == 2);
        cpu.r[0] == 2
    } {
        store(cpu, mem, 0x8c21a060, 0x8c219f52, crate::registry::p_8c21b06e::run);
    } else if {
        cpu.set_t(cpu.r[0] == 4);
        cpu.r[0] == 4
    } {
        store(cpu, mem, 0x8c21a064, 0x8c219f5c, crate::registry::p_8c21b0f6::run);
    } else if {
        cpu.set_t(cpu.r[0] == 3);
        cpu.r[0] == 3
    } {
        store(cpu, mem, 0x8c21a068, 0x8c219f66, crate::registry::p_8c21b0b6::run);
    } else if {
        cpu.set_t(cpu.r[0] == 6);
        cpu.r[0] == 6
    } {
        store(cpu, mem, 0x8c21a06c, 0x8c219f70, crate::registry::p_8c21b0e2::run);
    } else if {
        cpu.set_t(cpu.r[0] == 5);
        cpu.r[0] == 5
    } {
        store(cpu, mem, 0x8c21a070, 0x8c219f7a, crate::registry::p_8c21b08a::run);
    } else if {
        cpu.set_t(cpu.r[0] == 0);
        cpu.r[0] == 0
    } {
        store(cpu, mem, 0x8c21a074, 0x8c219f84, crate::registry::p_8c21b10a::run);
    } else if {
        cpu.set_t(cpu.r[0] == 7);
        cpu.r[0] == 7
    } {
        store(cpu, mem, 0x8c21a078, 0x8c219f8e, crate::registry::p_8c21b12e::run);
    } else if {
        cpu.set_t(cpu.r[0] == 8);
        cpu.r[0] == 8
    } {
        store(cpu, mem, 0x8c21a07c, 0x8c219f98, crate::registry::p_8c21b142::run);
    } else if {
        cpu.set_t(cpu.r[0] == 9);
        cpu.r[0] == 9
    } {
        store(cpu, mem, 0x8c21a080, 0x8c219fa2, crate::registry::p_8c21b17c::run);
    } else {
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 0xffff_ffff;
        cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    epilogue(cpu, mem);
}
