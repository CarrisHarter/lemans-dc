//! 0x8c19ed28 - drain a circular list back to its sentinel, then self-link it.
//!
//! `[r13]` is the sentinel; the walk frees every node from `sentinel->next` until
//! it reaches the sentinel again (the `jsr` delay loads the next pointer *before*
//! the free). Then `sentinel->next = sentinel` and `sentinel->prev = sentinel`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
    cpu.r[14] = mem.read_u32(cpu.r[0]).unwrap_or(0); // bra delay: mov.l @r0,r14
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
        cpu.set_t(cpu.r[0] == cpu.r[14]); // cmp/eq r0,r14
        if cpu.t() {
            break;
        }
        cpu.r[4] = cpu.r[14]; // mov r14,r4
        cpu.r[0] = mem.read_u32(0x8c19ed54).unwrap_or(0x8c01032c); // mov.l 0x8c19ed54,r0
        cpu.r[14] = mem.read_u32(cpu.r[14]).unwrap_or(0); // jsr delay: mov.l @r14,r14
        cpu.pr = 0x8c19ed3e;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    let _ = mem.write_u32(cpu.r[0], cpu.r[0]); // mov.l r0,@r0
    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r0)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
