//! 0x8c05f254 — store `r6` into `r4 + (r5<<2)`, then merge a state code into
//! the low nibble of the byte at `r4+4`.
//!
//! CFG: straight-line up to `tst r0,r0 ; bt 0x8c05f286`, where `r0` is the
//! zero-extended byte at 0x8c1f76e0 (image value 1). The `bt` has no delay
//! slot; when it is taken the byte at `r4+4` is left alone and `r0` keeps that
//! global byte. Otherwise the routine calls 0x8c0b23cc with `r4 = r6` and
//! `r5 = 0` (the `mov #0,r5` delay slot), masks the returned 0/2/6 code to
//! four bits and ORs it under the preserved high nibble of `[r4+4]`. The
//! `mov.b @(4,r14),r0` load sign-extends, so a negative old byte leaves the
//! upper 24 bits set in both `r1` (masked with 0xfffffff0) and the final `r0`;
//! only the low byte is stored back.
//!
//! Side effects: `r14` and `pr` are pushed on the stack window and popped by
//! the epilogue; `r14` holds the object base during the body; `r5` is shifted
//! left by two (no `T` change); `r6`/`r13` are preserved (the callee restores
//! its `r13`/`r14`/`pr`); `r4` ends as the code on the call path; `r1`/`r3`
//! are clobbered only on the call path; `T` is the `tst` result on the skip
//! path and whatever the callee's last compare left on the call path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[6]); // mov.l r6,@(r0,r14)
    cpu.r[0] = mem.read_u32(0x8c05f28c).unwrap_or(0); // mov.l @0x8c05f28c,r0 -> 0x8c1f76e0
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32; // mov.b @r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0 (twice)
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if !cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c05f290).unwrap_or(0); // mov.l @0x8c05f290,r0 -> 0x8c0b23cc
        cpu.r[4] = cpu.r[6]; // mov r6,r4
        cpu.r[5] = 0; // mov #0,r5 (jsr delay slot)
        cpu.pr = 0x8c05f274; // return address after the jsr
        crate::registry::p_8c0b23cc::run(cpu, mem);
        cpu.r[4] = cpu.r[0]; // mov r0,r4
        cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(4)).unwrap_or(0) as i8 as i32 as u32;
        // mov.b @(4,r14),r0 sign-extends.
        cpu.r[3] = 0xfffffff0; // mov #-16,r3
        cpu.r[1] = cpu.r[0] & cpu.r[3]; // and r3,r1
        cpu.r[0] = cpu.r[4] & 0x0f; // and #15,r0
        cpu.r[0] |= cpu.r[1]; // or r1,r0
        let _ = mem.write_u8(cpu.r[14].wrapping_add(4), cpu.r[0] as u8); // mov.b r0,@(4,r14)
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
