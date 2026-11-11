//! 0x8c02be1c — state-machine step: decrement a per-object counter.
//!
//! Hidden side effects the C does not show:
//! - the flag byte is loaded with `mov.b` (sign-extended) then `extu.b`ed
//!   twice, so r0 ends as the zero-extended byte on the early-exit path
//! - r14 is spilled/restored (net zero), but holds the counter base 0x3b8
//!   on the paths that continue; r6 = 1 is left behind
//! - the counter decrement is a `bra` delay slot (r7>1 path: -1) or a
//!   straight-line store (r7==1 path: -2), and T keeps the last cmp result
//! - the r7==1 path loads fr0/fr1 via [r4+4]+0x378 and r4+0x3a4 and leaves
//!   r3 = r4+0x3a4, r5 = 0x3a4, r1 = [r4+4]+0x378, r0 = 0x378

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let flag_base = mem.read_u32(0x8C02BE78).unwrap(); // 0x3b4
    let b = mem.read_u8(flag_base.wrapping_add(r4)).unwrap();
    cpu.r[0] = (b as u32) & 0xFF; // mov.b (sext) + extu.b + extu.b
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.t() {
        return; // bt -> rts (delay slot pops r14)
    }
    // r14 is spilled and restored by the rts delay slot, so it keeps its
    // original value on exit; the 0x3b8 counter base lives in a local.
    let r14 = mem.read_u32(0x8C02BE7C).unwrap(); // 0x3b8
    cpu.r[6] = 1; // mov #1,r6
    cpu.r[0] = r14;
    cpu.r[7] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
    cpu.set_t((cpu.r[7] as i32) > (cpu.r[6] as i32)); // cmp/gt r6,r7
    if cpu.t() {
        // r7 > 1: store r7, flag byte 1, counter -= 1 in the bra delay slot
        let base = mem.read_u32(0x8C02BE80).unwrap(); // 0x3bc
        cpu.r[0] = base;
        let _ = mem.write_u32(base.wrapping_add(r4), cpu.r[7]);
        let bbase = mem.read_u32(0x8C02BE84).unwrap(); // 0x3d0
        cpu.r[0] = bbase;
        let _ = mem.write_u8(bbase.wrapping_add(r4), cpu.r[6] as u8);
        cpu.r[0] = r14;
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
        cpu.r[1] = cpu.r[1].wrapping_add((-1i32) as u32);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]); // bra delay slot
        return;
    }
    // r7 <= 1
    cpu.r[0] = cpu.r[7];
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if !cpu.t() {
        return; // bf -> rts
    }
    // r7 == 1: compare two floats, then store and counter -= 2
    cpu.r[1] = mem.read_u32(r4.wrapping_add(4)).unwrap(); // mov.l @(4,r4),r1
    cpu.r[3] = r4;
    cpu.r[0] = mem.read_u32(0x8C02BE88).unwrap(); // 0x378
    cpu.r[5] = mem.read_u32(0x8C02BE8C).unwrap(); // 0x3a4
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]);
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr1
    cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
    if cpu.t() {
        return; // bt -> rts
    }
    let base = mem.read_u32(0x8C02BE80).unwrap(); // 0x3bc
    cpu.r[0] = base;
    let _ = mem.write_u32(base.wrapping_add(r4), cpu.r[7]);
    let bbase = mem.read_u32(0x8C02BE84).unwrap(); // 0x3d0
    cpu.r[0] = bbase;
    let _ = mem.write_u8(bbase.wrapping_add(r4), cpu.r[6] as u8);
    cpu.r[0] = r14;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
    cpu.r[1] = cpu.r[1].wrapping_add((-2i32) as u32);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]);
}
