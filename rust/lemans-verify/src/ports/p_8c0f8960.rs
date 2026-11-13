//! 0x8c0f8960 — stamp the object, maybe drain a shared list, then release two slots.
//!
//! A null object returns 0 with T set. Otherwise `[object] = 0x8c1ed000` and
//! the counter at `0x8c2ab018` is decremented. When that hits zero, the list at
//! `0x8c2ab030` is walked: `0x8c0f8a10` starts the cursor, each step frees
//! `[cursor+16]` and `0x8c05e8cc` advances the cursor saved on the stack, then
//! `0x8c0f98f0` clears the list header. Either way `0x8c14fe94(object+44, -1)`,
//! `0x8c018bd8(object+36, -1)` and `0x8c018bd8(object+4, -1)` run. A positive
//! low half of the count frees the object. `r0` is the object. `r12`–`r14` restored.

use sh4_oracle::{Cpu, Memory};

const VTABLE_PTR: u32 = 0x8c0f89ec;
const COUNTER_PTR: u32 = 0x8c0f89f0;
const LIST_PTR: u32 = 0x8c0f89f4;
const ADVANCE_PTR: u32 = 0x8c0f89f8;
const FREE_PTR: u32 = 0x8c0f89fc;
const UNLINK_PTR: u32 = 0x8c0f8a00;
const CLEAR_PTR: u32 = 0x8c0f8a04;
const STAMP_PTR: u32 = 0x8c0f8a08;
const RELEASE_PTR: u32 = 0x8c0f8a0c;

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // one scratch slot

    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.r[0] = rd(mem, VTABLE_PTR);
        let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
        cpu.r[1] = rd(mem, COUNTER_PTR);
        cpu.r[0] = rd(mem, cpu.r[1]);
        cpu.r[0] = cpu.r[0].wrapping_sub(1);
        cpu.set_t(cpu.r[0] == 0);
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // bf.s delay, always
        if cpu.t() {
            cpu.r[4] = rd(mem, LIST_PTR);
            cpu.r[0] = rd(mem, ADVANCE_PTR);
            cpu.pr = 0x8c0f898a;
            crate::registry::p_8c0f8a10::run(cpu, mem);
            cpu.r[12] = rd(mem, LIST_PTR);
            cpu.r[0] = rd(mem, cpu.r[0]);
            cpu.r[12] = cpu.r[12].wrapping_add(4);
            let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // bra delay
            loop {
                cpu.r[0] = rd(mem, cpu.r[15]);
                cpu.set_t(cpu.r[12] == cpu.r[0]);
                if cpu.t() {
                    break;
                }
                cpu.r[0] = cpu.r[0].wrapping_add(12);
                cpu.r[4] = rd(mem, cpu.r[0].wrapping_add(4));
                cpu.r[0] = rd(mem, FREE_PTR);
                cpu.pr = 0x8c0f899e;
                crate::registry::p_8c01032c::run(cpu, mem);
                cpu.r[0] = rd(mem, UNLINK_PTR);
                cpu.pr = 0x8c0f89a4;
                cpu.r[4] = cpu.r[15]; // jsr delay
                crate::registry::p_8c05e8cc::run(cpu, mem);
            }
            cpu.r[4] = rd(mem, LIST_PTR);
            cpu.r[0] = rd(mem, CLEAR_PTR);
            cpu.pr = 0x8c0f89b2;
            crate::registry::p_8c0f98f0::run(cpu, mem);
        }
        cpu.r[0] = rd(mem, STAMP_PTR);
        cpu.r[4] = cpu.r[13].wrapping_add(44);
        cpu.pr = 0x8c0f89bc;
        cpu.r[5] = (-1i32) as u32;
        crate::registry::p_8c14fe94::run(cpu, mem);
        cpu.r[0] = rd(mem, RELEASE_PTR);
        cpu.r[4] = cpu.r[13].wrapping_add(36);
        cpu.pr = 0x8c0f89c6;
        cpu.r[5] = (-1i32) as u32;
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = rd(mem, RELEASE_PTR);
        cpu.r[4] = cpu.r[13].wrapping_add(4);
        cpu.pr = 0x8c0f89d0;
        cpu.r[5] = (-1i32) as u32;
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            cpu.r[0] = rd(mem, FREE_PTR);
            cpu.pr = 0x8c0f89dc;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
