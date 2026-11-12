//! 0x8c027260 — reset two walker objects through 0x8c050fa8.
//!
//! ```text
//! push r14; push pr
//! r14 = r4
//! r0 = *(0x8c027278)            // 0x8c050fa8
//! jsr @r0 ; r4 = *(r14+0x2c)    // delay slot: first walker
//! r0 = *(0x8c027278)
//! jsr @r0 ; r4 = *(r14+0x30)    // delay slot: second walker
//! pop pr; rts ; pop r14
//! ```
//!
//! The container's +0x2c/+0x30 fields point at the two walker structs; the
//! callee (0x8c050fa8) is a separate port and is called once per pointer.
//! The target's own instructions touch neither T nor fr: the composed exit T
//! is the callee's trailing `shll` result (always 0), r0 ends as the second
//! walker's `e8[0x1c]`, and r14/r15/pr return to their entry values (r14 was
//! pushed before being repurposed as the container).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14

    cpu.r[0] = mem.read_u32(0x8c027278).unwrap_or(0); // literal -> 0x8c050fa8
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(0x2c)).unwrap_or(0); // delay slot
    crate::registry::p_8c050fa8::run(cpu, mem);

    cpu.r[0] = mem.read_u32(0x8c027278).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(0x30)).unwrap_or(0); // delay slot
    crate::registry::p_8c050fa8::run(cpu, mem);

    // T is the callee's: p_8c050fa8 models its final `shll r0` (T = 0), so no
    // compensation is needed here.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
