//! 0x8c074d58 — same shape as 0x8c074cf4, but the mask lives at offset 0x18.
//!
//! Hidden side effects: gate-closed path leaves r0 = 0, T = 1; full path
//! leaves r4 = param1+0x28, r3 = base+0xe8/0xf8 on early exit, r5/r6/r7
//! shifted by 2, r1 = result; r14 is restored by the rts delay slot.
//! Mask load is `mov.l @(24,r4),r3`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let saved_r14 = cpu.r[14];
    let gate_ptr = mem.read_u32(0x8C074DB0).unwrap(); // 0x108
    let mut r0 = mem.read_u8(base.wrapping_add(gate_ptr)).unwrap() as u32;
    r0 &= 0xFF; // extu.b twice
    cpu.r[0] = r0;
    cpu.set_t(r0 == 0); // tst r0,r0
    cpu.r[14] = 0; // bt.s delay slot, both paths
    if r0 == 0 {
        cpu.r[0] = cpu.r[14];
        cpu.r[14] = saved_r14;
        return;
    }
    let mut r7 = mem.read_u32(0x8C074DB4).unwrap(); // 0xe8
    cpu.r[5] <<= 2; // shll2 r5
    cpu.r[3] = base.wrapping_add(r7); // mov r4,r3 ; add r7,r3 — leftover
    r7 = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[5])).unwrap();
    cpu.r[7] = r7;
    let r0 = mem.read_u32(base.wrapping_add(8)).unwrap();
    cpu.set_t(r0 == 3); // cmp/eq #3,r0
    if r0 == 3 {
        let off = mem.read_u32(0x8C074DB8).unwrap(); // 0xf8
        cpu.r[3] = base.wrapping_add(off);
        r7 = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[5])).unwrap();
        cpu.r[7] = r7;
    }
    cpu.r[0] = r7; // mov r7,r0
    if r7 == 5 {
        cpu.set_t(true);
        cpu.r[0] = cpu.r[14];
        cpu.r[14] = saved_r14;
        return;
    }
    let idx4 = r7 << 2; // shll2 r7
    cpu.r[7] = idx4;
    let r5 = mem.read_u32(base.wrapping_add(0x10).wrapping_add(idx4)).unwrap();
    cpu.r[5] = r5;
    cpu.r[4] = base.wrapping_add(0x28); // add #40,r4 — leftover
    cpu.r[6] <<= 2; // shll2 r6 — leftover
    let r3 = mem.read_u32(cpu.r[4].wrapping_add(idx4)).unwrap();
    let r4 = mem.read_u32(r5.wrapping_add(8)).unwrap();
    cpu.r[4] = r4;
    let r0 = mem.read_u32(r3.wrapping_add(cpu.r[6])).unwrap();
    cpu.r[0] = r0;
    let r3 = mem.read_u32(r4.wrapping_add(0x18)).unwrap(); // mask at +0x18
    cpu.r[3] = r3;
    let t = r0 & r3 == 0; // tst r0,r3
    cpu.set_t(t);
    let r1 = if t { 0u32 } else { 1 }; // subc r1,r1 ; add #1,r1
    cpu.r[1] = r1;
    cpu.r[14] = r1;
    cpu.r[0] = cpu.r[14];
    cpu.r[14] = saved_r14;
}
