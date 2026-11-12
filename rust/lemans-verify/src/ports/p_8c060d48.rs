//! 0x8c060d48 — construct an object at `r4`: base init, vtable, two fields.
//!
//! ```text
//! push r14, r13, r12, pr
//! r14 = r7 ; r13 = r6 ; r12 = r4
//! call 0x8c056bf8 with r4 unchanged   ; writes 0x8c1e8f88 to [r4]
//! [r12]   = 0x8c1e9198  (literal pool @0x8c060d7c)
//! [r12+4] = 0
//! [r12]   = 0x8c1e91fc  (literal pool @0x8c060d80)
//! [r12+8] = r14 (r7)
//! r0 = r12
//! [r12+12] = r13 (r6)
//! pop pr, r12, r13, r14; rts
//! ```
//! Contract: `r4` is the object, `r7` lands at +8 and `r6` at +12; `r5` is
//! forwarded to the callee, which ignores it. Returns `r4` in `r0`;
//! callee-saved r12/r13/r14/pr are restored. The decompiled C hides the order:
//! the callee's store to `[r4]` is overwritten twice (0x8c1e9198, then
//! 0x8c1e91fc), so all three stores stay in `Memory::writes` in that order.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[7]; // mov r7,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]); // mov.l r12,@-r15
    cpu.r[13] = cpu.r[6]; // mov r6,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[0] = mem.read_u32(0x8c060d78).unwrap_or(0); // 0x8c056bf8
    cpu.pr = 0x8c060d5c; // jsr return address (delay slot: nop)
    crate::registry::p_8c056bf8::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c060d7c).unwrap_or(0); // 0x8c1e9198
    let _ = mem.write_u32(cpu.r[12], cpu.r[0]); // mov.l r0,@r12
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[12].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r12)
    cpu.r[0] = mem.read_u32(0x8c060d80).unwrap_or(0); // 0x8c1e91fc
    let _ = mem.write_u32(cpu.r[12], cpu.r[0]); // mov.l r0,@r12 (overwrites)
    let _ = mem.write_u32(cpu.r[12].wrapping_add(8), cpu.r[14]); // mov.l r14,@(8,r12)
    cpu.r[0] = cpu.r[12]; // mov r12,r0
    let _ = mem.write_u32(cpu.r[12].wrapping_add(12), cpu.r[13]); // mov.l r13,@(12,r12)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r12
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
