//! 0x8c074b68 — double/float getter: `fr0 = [ [rec+8] + 48 ]` if flag 0x80.
//!
//! idx from r5; table base+0xe8 (base+0xf8 when [base+8]==3); idx==5 -> fr0=0.
//! flags = [[rec+8]+8]; if flags & 0x80, fr0 = f32 at [rec+8]+48, else fldi0.
//! Side effects the C hides: r5 = r5<<2, r4 = base+16, r3 = [rec+8]+48 (or
//! [rec+8], or 0xe8/0xf8 on the idx5 path), r0 = 0x80, r1 = flags & 0x80,
//! exit T = (flags & 0x80) == 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let shifted = cpu.r[5] << 2; // shll2 r5
    cpu.r[5] = shifted;
    let mut r3 = mem.read_u32(0x8c074bb0).unwrap_or(0); // 0xe8
    let mut idx = mem
        .read_u32(base.wrapping_add(r3).wrapping_add(shifted))
        .unwrap_or(0);
    if mem.read_u32(base.wrapping_add(8)).unwrap_or(0) == 3 {
        r3 = mem.read_u32(0x8c074bb4).unwrap_or(0); // 0xf8
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
    let obj = mem.read_u32(rec.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r1),r3
    cpu.r[3] = obj;
    let mask = mem.read_u32(0x8c074bb8).unwrap_or(0); // 0x80
    cpu.r[0] = mask;
    let flags = mem.read_u32(obj.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r3),r1
    cpu.r[1] = flags & mask; // and r0,r1
    let t = cpu.r[1] == 0; // tst r1,r1
    cpu.set_t(t);
    if t {
        cpu.fr[0] = 0.0; // fldi0 fr0
    } else {
        cpu.r[3] = obj.wrapping_add(48); // add #48,r3
        cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap_or(0)); // fmov @r3,fr0
    }
}
