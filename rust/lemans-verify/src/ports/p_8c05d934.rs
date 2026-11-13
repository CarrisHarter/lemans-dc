//! 0x8c05d934 — release the object at `[0x8c23de88]`, then clear that global.
//!
//! A null global still stores 0 back and returns 0. Otherwise
//! `0x8c05d9c0(object+16, -1)` releases the child at `[object+20]` (handle at
//! `node+16`) and `0x8c05d97c(object, -1)` releases the child at `[object+4]`
//! (handle at `node+12`). Count `-1` keeps either wrapper from freeing its
//! argument. This function then frees the object through `0x8c01032c` and
//! stores 0 at `0x8c23de88`. `r0` is 0. `r14` is restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[0] = rd(mem, 0x8c05d96c); // 0x8c23de88
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = rd(mem, cpu.r[0]);
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        cpu.r[4] = cpu.r[14].wrapping_add(16);
        cpu.set_t(cpu.r[4] == 0);
        if !cpu.t() {
            cpu.r[0] = rd(mem, 0x8c05d970); // 0x8c05d9c0
            cpu.pr = 0x8c05d94e;
            cpu.r[5] = (-1i32) as u32; // jsr delay
            crate::registry::p_8c05d9c0::run(cpu, mem);
        }
        cpu.set_t(cpu.r[14] == 0);
        if !cpu.t() {
            cpu.r[0] = rd(mem, 0x8c05d974); // 0x8c05d97c
            cpu.r[4] = cpu.r[14];
            cpu.pr = 0x8c05d95a;
            cpu.r[5] = (-1i32) as u32; // jsr delay
            crate::registry::p_8c05d97c::run(cpu, mem);
        }
        cpu.r[0] = rd(mem, 0x8c05d978); // 0x8c01032c
        cpu.pr = 0x8c05d960;
        cpu.r[4] = cpu.r[14]; // jsr delay
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.r[1] = rd(mem, 0x8c05d96c);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
