//! 0x8c0e3a00 — store r4..r7 to four globals and 1 to a flag byte.
//!
//! ```text
//! mov.l r14,@-r15 / r13 / r12          ; prologue (stack window, excluded)
//! mov.l 0x8c0e3a24,r12                 ; r12 = 0x8c29b720
//! mov #1,r0
//! mov.l 0x8c0e3a28,r13                 ; r13 = 0x8c29b728
//! mov.l 0x8c0e3a2c,r14                 ; r14 = 0x8c29b730
//! mov.l 0x8c0e3a30,r3                  ; r3  = 0x8c29b738
//! mov.l 0x8c0e3a34,r1                  ; r1  = 0x8c29b740
//! mov.l r4,@r12 / r5,@r13 / r6,@r14 / r7,@r3
//! mov.b r0,@r1                         ; *0x8c29b740 = 1
//! mov.l @r15+,r12 / r13                ; epilogue: r12..r14 restored to INPUT values
//! rts
//! mov.l @r15+,r14                      ; delay slot
//! ```
//! Hidden side effects: r12/r13/r14 end as the caller's originals (popped),
//! but r3 = 0x8c29b738, r1 = 0x8c29b740 and r0 = 1 are left behind; five
//! stores (four dwords + one byte) land in the globals area.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s12 = cpu.r[12];
    let s13 = cpu.r[13];
    let s14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // 3x mov.l @-r15 (stack, excluded)
    cpu.r[12] = mem.read_u32(0x8c0e3a24).unwrap(); // 0x8c29b720
    cpu.r[0] = 1; // mov #1,r0
    cpu.r[13] = mem.read_u32(0x8c0e3a28).unwrap(); // 0x8c29b728
    cpu.r[14] = mem.read_u32(0x8c0e3a2c).unwrap(); // 0x8c29b730
    cpu.r[3] = mem.read_u32(0x8c0e3a30).unwrap(); // 0x8c29b738
    cpu.r[1] = mem.read_u32(0x8c0e3a34).unwrap(); // 0x8c29b740
    let _ = mem.write_u32(cpu.r[12], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[13], cpu.r[5]);
    let _ = mem.write_u32(cpu.r[14], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[3], cpu.r[7]);
    let _ = mem.write_u8(cpu.r[1], cpu.r[0] as u8); // mov.b r0,@r1
    cpu.r[12] = s12; // mov.l @r15+,r12
    cpu.r[13] = s13; // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(12); // rts delay slot: mov.l @r15+,r14
    cpu.r[14] = s14;
}
