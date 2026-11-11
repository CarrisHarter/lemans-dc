//! 0x8c0443ec — write an 11-float (44-byte) struct at param_1 and return it.
//!
//! ```text
//! fldi1 fr0 ; fmov fr0,@r4          ; [r4+0]  = 1.0f
//! mov #1,r6 ; mov.l r6,@(4,r4)      ; [r4+4]  = 1
//! mov #0,r0 ; mov.l r0,@(8,r4)      ; [r4+8]  = 0
//! mov r4,r5 ; mov.l r6,@(12,r4)     ; [r4+12] = 1
//! mov r4,r3 ; mov.l r0,@(16,r4)     ; [r4+16] = 0
//! mov r4,r1 ; mov.l r0,@(20,r4)     ; [r4+20] = 0
//! add #32,r5 ; mov.l r6,@(24,r4)    ; [r4+24] = 1
//! mova 0x8c04441c,r0 ; mov.l r6,@(28,r4) ; [r4+28] = 1
//! add #36,r3 ; fmov @r0,fr0         ; fr0 = [0x8c04441c]
//! add #40,r1 ; fmov fr0,@r5         ; [r4+32] = fr0
//! mov r4,r0 ; fmov fr0,@r3          ; [r4+36] = fr0
//! rts ; fmov fr0,@r1                ; delay slot: [r4+40] = fr0
//! ```
//! Side effects the C does not show: r6 is left 1, r5 = r4+32, r3 = r4+36,
//! r1 = r4+40 (scratch pointers), r0 = r4 comes from `mov r4,r0` just
//! before `rts` (the return value), and fr0 ends as the literal-pool float,
//! not the 1.0 from `fldi1`. The final store at [r4+40] is the `rts` delay
//! slot. Ghidra's FPSCR.SZ branch is spurious: the oracle's `fmov.s` is a
//! fixed 32-bit transfer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u32(base, cpu.fr[0].to_bits()); // fmov fr0,@r4
    cpu.r[6] = 1; // mov #1,r6
    let _ = mem.write_u32(base.wrapping_add(4), cpu.r[6]); // mov.l r6,@(4,r4)
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(base.wrapping_add(8), cpu.r[0]); // mov.l r0,@(8,r4)
    cpu.r[5] = base; // mov r4,r5
    let _ = mem.write_u32(base.wrapping_add(12), cpu.r[6]); // mov.l r6,@(12,r4)
    cpu.r[3] = base; // mov r4,r3
    let _ = mem.write_u32(base.wrapping_add(16), cpu.r[0]); // mov.l r0,@(16,r4)
    cpu.r[1] = base; // mov r4,r1
    let _ = mem.write_u32(base.wrapping_add(20), cpu.r[0]); // mov.l r0,@(20,r4)
    cpu.r[5] = cpu.r[5].wrapping_add(32); // add #32,r5
    let _ = mem.write_u32(base.wrapping_add(24), cpu.r[6]); // mov.l r6,@(24,r4)
    cpu.r[0] = 0x8C04441C; // mova 0x8c04441c,r0 ((0x8c044408 & !3) + 4 + 4*4)
    let _ = mem.write_u32(base.wrapping_add(28), cpu.r[6]); // mov.l r6,@(28,r4)
    cpu.r[3] = cpu.r[3].wrapping_add(36); // add #36,r3
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // fmov @r0,fr0
    cpu.r[1] = cpu.r[1].wrapping_add(40); // add #40,r1
    let _ = mem.write_u32(cpu.r[5], cpu.fr[0].to_bits()); // fmov fr0,@r5
    cpu.r[0] = base; // mov r4,r0
    let _ = mem.write_u32(cpu.r[3], cpu.fr[0].to_bits()); // fmov fr0,@r3
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits()); // delay slot of `rts`: fmov fr0,@r1
}
