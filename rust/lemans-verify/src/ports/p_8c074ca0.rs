//! 0x8c074ca0 — `return (flag_at(param1, idx) & mask) != 0;`
//!
//! ```text
//! mov.l r14,@-r15
//! mov.l 0x8c074cec,r7   ; r7 = 0xe8
//! shll2 r5              ; r5 = param2*4  (leftover!)
//! mov r4,r3 ; mov r5,r0 ; add r7,r3 ; mov.l @(r0,r3),r7   ; r3 left = p+0xe8
//! mov.l @(8,r4),r0 ; cmp/eq #3,r0
//! bf.s … ; mov #0,r14   ; delay slot runs on BOTH paths (bf taken when != 3)
//!   (==3): mov r4,r3 ; add r7,r3 (r3 = p+0xf8) ; reload idx from *(p+0xf8+param2*4)
//! mov r7,r0 ; cmp/eq #5,r0 ; bt exit   ; idx == 5 → r0 ends 0, T=1
//! full path: r5 = *(p+0x10+idx*4); r4 += 0x28; r6 <<= 2;
//!   r3 = *(p+0x28+idx*4); r4 = *(r5+8); r0 = *(r3+param3*4); r3 = *(r4+0x10)
//!   tst r0,r3 ; subc r1,r1 ; add #1,r1 ; mov r1,r14
//! exit: mov r14,r0 ; rts ; mov.l @r15+,r14   ; delay slot restores r14!
//! ```
//! Hidden side effects the C does not show: r4 left at param1+0x28 (full path),
//! r3 left at param1+0xe8/0xf8 (early-exit paths), r5/r6/r7 left shifted by 2,
//! r1 = result, T from the last tst/cmp, and r14 is callee-saved — the rts
//! delay slot restores the caller's r14, so only r0 carries the result out.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let saved_r14 = cpu.r[14]; // pushed at entry, popped by the rts delay slot
    let mut r7 = mem.read_u32(0x8C074CEC).unwrap(); // 0xe8
    cpu.r[5] <<= 2; // shll2 r5
    cpu.r[3] = base.wrapping_add(r7); // mov r4,r3 ; add r7,r3 — leftover
    r7 = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[5])).unwrap();
    cpu.r[7] = r7;
    let r0 = mem.read_u32(base.wrapping_add(8)).unwrap(); // mov.l @(8,r4),r0
    cpu.set_t(r0 == 3); // cmp/eq #3,r0
    cpu.r[14] = 0; // delay slot of bf.s, both paths
    if r0 == 3 {
        let off = mem.read_u32(0x8C074CF0).unwrap(); // 0xf8
        cpu.r[3] = base.wrapping_add(off); // mov r4,r3 ; add r7,r3
        r7 = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[5])).unwrap();
        cpu.r[7] = r7;
    }
    cpu.r[0] = r7; // mov r7,r0
    if r7 == 5 {
        cpu.set_t(true); // cmp/eq #5,r0, bt taken
        cpu.r[0] = cpu.r[14]; // mov r14,r0 at exit
        cpu.r[14] = saved_r14; // mov.l @r15+,r14
        return;
    }
    let idx4 = r7 << 2; // shll2 r7
    cpu.r[7] = idx4;
    let r5 = mem.read_u32(base.wrapping_add(0x10).wrapping_add(idx4)).unwrap();
    cpu.r[5] = r5; // mov.l @(r0,r3),r5 with r3 = base+0x10, r0 = idx4
    cpu.r[4] = base.wrapping_add(0x28); // add #40,r4 — leftover
    cpu.r[6] <<= 2; // shll2 r6 — leftover
    let r3 = mem.read_u32(cpu.r[4].wrapping_add(idx4)).unwrap(); // r0 still idx4
    let r4 = mem.read_u32(r5.wrapping_add(8)).unwrap();
    cpu.r[4] = r4; // mov.l @(8,r5),r4
    let r0 = mem.read_u32(r3.wrapping_add(cpu.r[6])).unwrap();
    cpu.r[0] = r0; // mov.l @(r0,r3),r0 with r0 = param3*4
    let r3 = mem.read_u32(r4.wrapping_add(0x10)).unwrap();
    cpu.r[3] = r3; // mov.l @(16,r4),r3
    let t = r0 & r3 == 0; // tst r0,r3
    cpu.set_t(t);
    let r1 = if t { 0u32 } else { 1 }; // subc r1,r1 ; add #1,r1
    cpu.r[1] = r1;
    cpu.r[14] = r1; // mov r1,r14
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[14] = saved_r14; // restored by the rts delay slot
}
