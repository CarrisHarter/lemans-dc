//! 0x8c074c08 — flag 0x08 variant: `fr0 = [ [rec+8] + 32 ]` if set, else 0.
//!
//! Same shape as 0x8c074bbc (r0 = record, r1 = object) with immediate
//! `and #8,r0` and the value at object+32. Side effects the C hides:
//! r5 = r5<<2, r4 = base+16, r0 = flags & 8, r1 = [rec+8]+32 (or [rec+8]),
//! r3 = 0xe8/0xf8, exit T = (flags & 8) == 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let shifted = cpu.r[5] << 2; // shll2 r5
    cpu.r[5] = shifted;
    let mut r3 = mem.read_u32(0x8c074c4c).unwrap_or(0); // 0xe8
    let mut idx = mem
        .read_u32(base.wrapping_add(r3).wrapping_add(shifted))
        .unwrap_or(0);
    if mem.read_u32(base.wrapping_add(8)).unwrap_or(0) == 3 {
        r3 = mem.read_u32(0x8c074c50).unwrap_or(0); // 0xf8
        idx = mem
            .read_u32(base.wrapping_add(r3).wrapping_add(shifted))
            .unwrap_or(0);
    }
    cpu.r[3] = r3;
    cpu.r[1] = idx;
    cpu.r[0] = idx; // mov r1,r0
    if idx == 5 {
        cpu.fr[0] = 0.0; // rts delay slot: fldi0 fr0
        cpu.set_t(true);
        return;
    }
    let rec = mem
        .read_u32(base.wrapping_add(16).wrapping_add(idx << 2))
        .unwrap_or(0);
    cpu.r[4] = base.wrapping_add(16); // add #16,r4
    cpu.r[0] = rec; // mov.l @(r0,r4),r0
    let obj = mem.read_u32(rec.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r0),r1
    cpu.r[1] = obj;
    let flags = mem.read_u32(obj.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r1),r0
    cpu.r[0] = flags & 8; // and #8,r0
    let t = cpu.r[0] == 0; // tst r0,r0
    cpu.set_t(t);
    if t {
        cpu.fr[0] = 0.0; // fldi0 fr0
    } else {
        cpu.r[1] = obj.wrapping_add(32); // add #32,r1
        cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap_or(0)); // fmov @r1,fr0
    }
}
