//! 0x8c0b9598 — initialise a 72-byte object header and clear its fields.
//!
//! Stores the vtable `0x8c1eb144` at `[r4+68]` (literal pool at 0x8c0b9600),
//! a provisional type `5` at `[r4]`, zeroes the integer fields at +4…+24,
//! the single-precision fields at +28…+52, then calls 0x8c010f88 (memset) to
//! zero six bytes at `r4+60`. The call's delay slot stores `4` at `[r4+56]`,
//! so that store lands before the memset's byte stores. On return the float
//! fields at +36…+48 are zeroed, `[r4]` is overwritten with the second
//! argument (`r14`), and `r4` is returned in `r0`.
//!
//! Register side effects: `r0`/`r13` end as the object, `r1 = r4+40`,
//! `r3 = r4+44`, `r4 = r4+48`, `r5 = 0`, `r6 = 6`, `r7 = 6` (memset's counter),
//! `fr0 = +0.0` and T set (memset's last `cmp/hs`). `r13`/`r14` are restored
//! from the stack; all FMOVs are single-precision stores, so they write four
//! bytes regardless of FPSCR.SZ (the oracle has no double-precision FMOV).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[5];
    cpu.r[13] = cpu.r[4];

    cpu.r[1] = mem.read_u32(0x8c0b9600).unwrap_or(0); // 0x8c1eb144
    cpu.r[0] = 68;
    cpu.r[5] = 0;
    let _ = mem.write_u32(cpu.r[13].wrapping_add(68), cpu.r[1]);
    cpu.fr[0] = 0.0; // fldi0
    cpu.r[0] = 5;
    let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
    cpu.r[1] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[5]);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[5]);
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[1] = 4;
    let _ = mem.write_u32(cpu.r[13].wrapping_add(24), cpu.r[5]);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[13].wrapping_add(20), cpu.r[5]);
    cpu.r[0] = cpu.r[0].wrapping_add(52);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[5]);
    cpu.r[4] = cpu.r[4].wrapping_add(60);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[5]);
    cpu.r[6] = 6;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[0] = mem.read_u32(0x8c0b9604).unwrap_or(0); // 0x8c010f88 (memset)
    let _ = mem.write_u32(cpu.r[13].wrapping_add(56), cpu.r[1]); // delay slot of `jsr`
    cpu.pr = 0x8c0b95da;
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4 = object+60, 0, 6)

    cpu.r[4] = cpu.r[13];
    cpu.r[4] = cpu.r[4].wrapping_add(48);
    cpu.fr[0] = 0.0; // fldi0
    cpu.r[3] = cpu.r[13];
    cpu.r[1] = cpu.r[13];
    let _ = mem.write_f32(cpu.r[4], cpu.fr[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(44);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = cpu.r[1].wrapping_add(40);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(36);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    let _ = mem.write_u32(cpu.r[13], cpu.r[14]);
    cpu.r[0] = cpu.r[13];

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
