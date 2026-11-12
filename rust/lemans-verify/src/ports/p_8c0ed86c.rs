//! 0x8c0ed86c — release four owned pointers, then re-zero the struct.
//!
//! Calls 0x8c0102cc on `[r4+4]`, `[r4+16]`, `[r4+52]` and `[r4+56]` (each
//! pointer loaded in the `jsr` delay slot), then 0x8c0ed80c on the struct
//! itself. Only the callee residue and the zeroed object are observable; r15,
//! r14 and pr come back at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14]; // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    for (offset, ret) in [(4u32, 0x8c0ed878u32), (16, 0x8c0ed87e), (52, 0x8c0ed884), (56, 0x8c0ed88a)] {
        cpu.r[0] = mem.read_u32(0x8c0ed898).unwrap_or(0x8c0102cc); // mov.l 0x8c0ed898,r0
        let arg = mem.read_u32(cpu.r[14].wrapping_add(offset)).unwrap_or(0); // jsr delay slot
        cpu.r[4] = arg;
        cpu.pr = ret;
        crate::registry::p_8c0102cc::run(cpu, mem);
    }
    cpu.r[0] = mem.read_u32(0x8c0ed89c).unwrap_or(0x8c0ed80c); // mov.l 0x8c0ed89c,r0
    cpu.r[4] = cpu.r[14]; // jsr delay slot
    cpu.pr = 0x8c0ed890;
    crate::registry::p_8c0ed80c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(8); // lds.l @r15+,pr ; rts delay: @r15+,r14
    cpu.r[14] = saved14;
}
