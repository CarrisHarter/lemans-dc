//! 0x8c173954 — load the current 0x8c2b5260 slot, optionally clear it and
//! bump its mod-64 counter, and set SR IMASK from the 0x8c1739f4(15) result.
//!
//! Original: spills `r4`, calls 0x8c1739f4 with `r4=15` (delay slot), then
//! computes the slot address `table+0xb4+[table+0xac]*4`. The `bt.s` delay
//! slot `mov.l @r6,r6` runs on BOTH paths: with a null saved `r4` (tst true,
//! branch taken) the clear block is skipped and the counter is untouched;
//! with a non-null saved `r4` the slot is zeroed and `[table+0xac]` is
//! incremented mod 64. Then `SR = (SR & mask) | ((r5 & 15) << 4)` where the
//! `mov.w` mask literal 0xff0f sign-extends to 0xffffff0f (OR-only on the
//! IMASK field, same as sibling 0x8c17390c), and the `rts` delay slot moves
//! the loaded slot value into `r0`. The stack spill is invisible (stack
//! window excluded), so `r15` is left untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c1739b8).unwrap_or(0);
    let saved_r4 = cpu.r[4];
    cpu.r[4] = 15; // delay slot of `jsr`
    crate::registry::p_8c1739f4::run(cpu, mem);
    cpu.r[5] = cpu.r[0]; // mov r0,r5: callee result also lives in r5
    let r5 = cpu.r[5];
    cpu.r[0] = mem.read_u16(0x8c1739b4).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[4] = mem.read_u32(0x8c1739bc).unwrap_or(0);
    cpu.r[3] = mem.read_u16(0x8c1739b0).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[2] = saved_r4; // mov.l @r15,r2: the spill the oracle pushes
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
    cpu.r[6] = cpu.r[6].wrapping_shl(2);
    cpu.set_t(cpu.r[2] == 0); // tst r2,r2
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[3]);
    let take = cpu.t(); // bt.s: taken when the saved r4 was null
    cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0); // delay slot: runs on both paths
    if !take {
        cpu.r[0] = mem.read_u16(0x8c1739b4).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[1] = 0;
        cpu.r[2] = mem.read_u16(0x8c1739b0).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[4]);
        cpu.r[3] = cpu.r[3].wrapping_shl(2);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
        cpu.r[1] = mem.read_u16(0x8c1739b4).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[4]);
        cpu.r[0] &= 63;
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    }
    cpu.r[3] = cpu.sr; // stc sr,r3
    cpu.r[0] = r5;
    cpu.r[2] = mem.read_u16(0x8c1739b2).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] &= 15;
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[3] &= cpu.r[2];
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[0] |= cpu.r[3];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[0] = cpu.r[6]; // rts delay slot: mov r6,r0
}
