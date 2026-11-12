//! 0x8c18c608 - walk a cursor chain, resetting each record, then set a flag byte.
//!
//! `r13 = r14 + 144`; the cursor `[r14+152]` is spilled to `[r15]`. While that
//! cursor differs from `r13`, the loop reloads it, adds 12, passes `[r0+8]` to
//! 0x8c19b624 and then hands `r15` to 0x8c05e8cc, which advances the cursor.
//! On exit byte `[r14]` is cleared to 0 (the 0x8c18c5c4 twin sets it to 1).
//!
//! Only the already-advanced path is pinned: a spec seeds `[r14+152] = r14+144`
//! so the walk exits before its first iteration. The loop body needs a
//! 0x8c05e8cc cursor chain that no fixture in the corpus builds yet.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[13] = cpu.r[14].wrapping_add(116); // mov r14,r13 ; add #116,r13
    cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0); // mov.l @(36,r13),r0
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@r15
    cpu.r[13] = cpu.r[13].wrapping_add(28); // bra delay: add #28,r13
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
        cpu.set_t(cpu.r[0] == cpu.r[13]); // cmp/eq r13,r0
        if cpu.t() {
            break;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r0),r4
        cpu.r[0] = mem.read_u32(0x8c18c644).unwrap_or(0x8c19b624); // mov.l 0x8c18c644,r0
        cpu.pr = 0x8c18c628; // jsr delay is nop
        crate::registry::p_8c19b634::run(cpu, mem);
        cpu.r[0] = mem.read_u32(0x8c18c648).unwrap_or(0x8c05e8cc); // mov.l 0x8c18c648,r0
        cpu.r[4] = cpu.r[15]; // jsr delay: mov r15,r4
        cpu.pr = 0x8c18c62e;
        crate::registry::p_8c05e8cc::run(cpu, mem);
    }
    cpu.r[0] = 0; // mov #0,r0 (this twin clears the byte)
    let _ = mem.write_u8(cpu.r[14], cpu.r[0] as u8); // mov.b r0,@r14
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
