//! 0x8c06c3b8 — next record of a 44-byte array.
//!
//! `r4` points at a cursor object: count at 0x28, base pointer at 0x2c,
//! current index at 0x30.
//!
//! ```text
//!  mov.l @(48,r4),r3   r3 = index (also the array element index)
//!  mov.l @(40,r4),r0   r0 = count
//!  cmp/ge r0,r3        T = signed(index >= count)
//!  bf  0x8c06c3c4      if index < count, compute
//!  rts ; mov #0,r0     else return 0, index untouched
//! ```
//!
//! The compute path bumps the index and returns `base + index*44`. The product
//! is the one `mul.l` leaves: low word in `macl`, high word in `mach`, then
//! `sts macl,r3` keeps the low word in `r3`.
//!
//! Residue: the early exit leaves T = 1 from `cmp/ge`; the compute path leaves
//! T = 0. `r1` ends as the new index, and only the compute path writes memory.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l @(48,r4),r3 ; mov.l @(40,r4),r0 ; cmp/ge r0,r3
    let index = mem.read_u32(cpu.r[4].wrapping_add(48)).unwrap_or(0);
    let count = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0);
    cpu.r[3] = index;
    let t = (index as i32) >= (count as i32);
    cpu.set_t(t);
    if t {
        cpu.r[0] = 0; // delay slot of `rts`
        return;
    }

    // mov r3,r1 ; mov #44,r0 ; mul.l r0,r3
    cpu.r[1] = cpu.r[3];
    cpu.r[0] = 44;
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[3] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;

    // add #1,r1 ; mov.l r1,@(48,r4) ; sts macl,r3
    cpu.r[1] = cpu.r[1].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(48), cpu.r[1]);
    cpu.r[3] = cpu.macl;

    // mov.l @(44,r4),r0 ; add r3,r0 ; rts ; nop
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(44)).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
}
