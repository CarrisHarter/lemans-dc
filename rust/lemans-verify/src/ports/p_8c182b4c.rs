//! 0x8c182b4c — tear down an object, then free it when the count is positive.
//!
//! A null object returns 0 with T set and does not consult the count. Otherwise
//! a non-zero child word is released by `0x8c183654`. `0x8c182bd8(object+4, 0)`
//! and `0x8c182bb4(object, 0)` both decline to free because the count argument
//! is zero; their register residue still lands. A positive low half of the
//! caller's count then frees the object through `0x8c01032c`. `r0` is the
//! object; `r13` and `r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[5];
    cpu.r[13] = cpu.r[4];
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.set_t(cpu.r[13] == 0);
        cpu.set_t(cpu.r[13] == 0);
        cpu.r[0] = rd(mem, cpu.r[13].wrapping_add(4));
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[0] = cpu.r[13].wrapping_add(4);
            cpu.r[5] = rd(mem, cpu.r[0]);
            cpu.r[0] = rd(mem, 0x8c182ba4);
            cpu.pr = 0x8c182b74;
            crate::registry::p_8c183654::run(cpu, mem);
        }
        cpu.r[4] = cpu.r[13];
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.set_t(cpu.r[4] == 0);
        if !cpu.t() {
            cpu.r[0] = rd(mem, 0x8c182ba8);
            cpu.pr = 0x8c182b82;
            cpu.r[5] = 0; // jsr delay slot
            crate::registry::p_8c182bd8::run(cpu, mem);
        }
        cpu.set_t(cpu.r[13] == 0);
        if !cpu.t() {
            cpu.r[0] = rd(mem, 0x8c182bac);
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c182b8e;
            cpu.r[5] = 0; // jsr delay slot
            crate::registry::p_8c182bb4::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c182bb0);
            cpu.pr = 0x8c182b9a;
            cpu.r[4] = cpu.r[13]; // jsr delay slot
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
