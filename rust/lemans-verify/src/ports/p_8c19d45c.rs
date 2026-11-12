//! 0x8c19d45c - decrement a refcount and, at zero, clear two bits on the target.
//!
//! `[r14] -= 1`; while the result is non-zero the function returns. At zero it
//! calls 0x8c05d274 twice on `[r14+4]` — first with `r5 = 1` (set) then with
//! `r5 = 2` (clear) — and both calls pass `r6 = 0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r0
    cpu.r[0] = cpu.r[0].wrapping_sub(1); // add #-1,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // mov.l r0,@r14
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r4
        cpu.r[6] = 0; // mov #0,r6
        cpu.r[0] = mem.read_u32(0x8c19d488).unwrap_or(0x8c05d274); // mov.l 0x8c19d488,r0
        cpu.r[5] = 1; // jsr delay: mov #1,r5
        cpu.pr = 0x8c19d478;
        crate::registry::p_8c05d274::run(cpu, mem);
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r4
        cpu.r[6] = 0; // mov #0,r6
        cpu.r[0] = mem.read_u32(0x8c19d488).unwrap_or(0x8c05d274); // mov.l 0x8c19d488,r0
        cpu.r[5] = 2; // jsr delay: mov #2,r5
        cpu.pr = 0x8c19d482;
        crate::registry::p_8c05d274::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
