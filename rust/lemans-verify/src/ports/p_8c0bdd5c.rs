//! 0x8c0bdd5c — release three slots, then free the object on a positive count.
//!
//! A null object returns 0 with T set. Otherwise `0x8c043a68(object+20)` frees
//! `[object+28]` when that word is set, and `0x8c01e460(object+20, -1)` does
//! not. `0x8c0c03d4(object+8)` drains and frees the array at `[object+16]`,
//! then `0x8c0bdddc(object+8, 0)` declines to free `object+8`.
//! `0x8c018bd8(object+4, -1)` releases the handle. A positive low half of the
//! saved count frees the object through `0x8c01032c`. `r0` is the object.
//! `r13`/`r14` are restored.

use sh4_oracle::{Cpu, Memory};

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
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.set_t(cpu.r[13] == 0); // dead second tst
        cpu.r[4] = cpu.r[4].wrapping_add(20);
        cpu.set_t(cpu.r[4] == 0);
        if !cpu.t() {
            cpu.set_t(cpu.r[4] == 0); // dead second tst
            cpu.r[0] = rd(mem, 0x8c0bddc4); // 0x8c043a68
            cpu.pr = 0x8c0bdd7e;
            crate::registry::p_8c043a68::run(cpu, mem);
            cpu.r[0] = rd(mem, 0x8c0bddc8); // 0x8c01e460
            cpu.r[4] = cpu.r[13].wrapping_add(20);
            cpu.pr = 0x8c0bdd88;
            cpu.r[5] = (-1i32) as u32; // jsr delay
            crate::registry::p_8c01e460::run(cpu, mem);
        }
        cpu.r[4] = cpu.r[13].wrapping_add(8);
        cpu.set_t(cpu.r[4] == 0);
        if !cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0bddcc); // 0x8c0c03d4
            cpu.pr = 0x8c0bdd96;
            crate::registry::p_8c0c03d4::run(cpu, mem);
            cpu.r[4] = cpu.r[13].wrapping_add(8);
            cpu.set_t(cpu.r[4] == 0);
            if !cpu.t() {
                cpu.r[0] = rd(mem, 0x8c0bddd0); // 0x8c0bdddc
                cpu.pr = 0x8c0bdda4;
                cpu.r[5] = 0; // jsr delay
                crate::registry::p_8c0bdddc::run(cpu, mem);
            }
        }
        cpu.r[0] = rd(mem, 0x8c0bddd4); // 0x8c018bd8
        cpu.r[4] = cpu.r[13].wrapping_add(4);
        cpu.pr = 0x8c0bddae;
        cpu.r[5] = (-1i32) as u32; // jsr delay
        crate::registry::p_8c018bd8::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0bddd8); // 0x8c01032c
            cpu.pr = 0x8c0bddba;
            cpu.r[4] = cpu.r[13]; // jsr delay
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
