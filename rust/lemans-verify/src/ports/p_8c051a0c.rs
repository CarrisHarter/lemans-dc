//! 0x8c051a0c — move slot r5 into slot r4 across two parallel tables, then
//! re-point whatever the r4 entry references at offset 12 back at the new slot.
//!
//! ```text
//! mov.l 0x8c051a54,r13   ; r13 = 0x1d4 (table A base)
//! mov   #-1,r4
//! mov.l 0x8c051a58,r7    ; r7 = 0x1d8 (table B base)
//! A[r4] = A[r5]; B[r4] = B[r5]; A[r5] = -1; B[r5] = 0
//! jsr @0x8c04a578       ; delay slot: r13 = A[old r4]
//! r1 = *r0; tst r1,r1; bt end
//! shll2 r13
//! r1 = *(r13 + r1); r0 = *(r13 + r1); tst r0,r0; bt end
//! *(r0 + 12) = old r4
//! end: restore pr, r13, r14; rts (delay slot restores r14)
//! ```
//! Hidden side effects: r4 is clobbered to -1 up front; r5 keeps the caller's
//! value on the flag-set path but the callee overwrites it with 0x8c04b018 on
//! the flag-zero path; r13 ends as `A[r4]*4`, r0/r1 keep the tail-chain loads,
//! and the delay slot of `jsr` runs before the callee. T is the result of the
//! last executed `tst` (chain load), not the first.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[13] = mem.read_u32(0x8c051a54).unwrap_or(0); // 0x1d4
    cpu.r[4] = 0xffff_ffff; // mov #-1,r4
    cpu.r[7] = mem.read_u32(0x8c051a58).unwrap_or(0); // 0x1d8
    cpu.r[0] = cpu.r[13];
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[1]);
    cpu.r[0] = cpu.r[7];
    cpu.r[6] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c051a5c).unwrap_or(0); // 0x8c04a578
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[6]);
    cpu.r[0] = cpu.r[13];
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[4]);
    cpu.r[0] = cpu.r[7];
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[3]);
    cpu.r[0] = cpu.r[13];
    // jsr @r1; delay slot runs before the callee and clobbers r13.
    cpu.pr = 0x8c051a38;
    cpu.r[13] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[14])).unwrap_or(0);
    crate::registry::p_8c04a578::run(cpu, mem);
    cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
    if !cpu.t() {
        cpu.r[13] <<= 2; // shll2 r13 (no T change)
        cpu.r[0] = cpu.r[13];
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            let _ = mem.write_u32(cpu.r[0].wrapping_add(12), cpu.r[14]);
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts delay slot restores r14 before the return lands.
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
