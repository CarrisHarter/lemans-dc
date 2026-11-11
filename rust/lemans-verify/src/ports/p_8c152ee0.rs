//! 0x8c152ee0 — initialise a 92-byte struct at r4.
//!
//! Straight-line. Saves r14..r9, reserves 44 bytes of scratch at r15, stores
//! the float literal at 0x8c152fc4 to *r4, copies three words from the table
//! at 0x8c1d3f68 to r4+16..+24, writes 1.0f to r4+4/r4+60/r4+72/r4+76/r4+36,
//! zeroes r4+8..+12, r4+28..+32, r4+52..+68, r4+80..+88, then restores
//! r9..r14. No T-setting instruction, so T is preserved.
//!
//! Final leaked registers: r0 = r4_entry, r1 = r4+36, r2 = r3 = r15_scratch+16,
//! r4 = 0.0f bits (mem[r15_scratch+12]), r5 = r4+80, r6 = 0 (mem[r15_scratch+8]),
//! r7 = r4+72; fr0 = 0.0, fr1 = 1.0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14..r9,@-r15 ; add #-44,r15
    for n in [14usize, 13, 12, 11, 10, 9] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let v = cpu.r[n];
        let _ = mem.write_u32(cpu.r[15], v);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(44);
    let sp = cpu.r[15];

    let fr0_init = mem.read_f32(0x8c152fc4).unwrap_or(0.0); // mova 0x8c152fc4,r0 ; fmov @r0,fr0
    cpu.fr[0] = fr0_init;
    let r4 = cpu.r[4];
    let _ = mem.write_f32(r4, cpu.fr[0]); // fmov fr0,@r4

    let g = mem.read_u32(0x8c152fc8).unwrap_or(0); // 0x8c1d3f68
    let v0 = mem.read_u32(g).unwrap_or(0);
    let v1 = mem.read_u32(g.wrapping_add(4)).unwrap_or(0);
    let v2 = mem.read_u32(g.wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(16), v0);
    let _ = mem.write_u32(r4.wrapping_add(20), v1);
    let _ = mem.write_u32(r4.wrapping_add(24), v2);

    // scratch: fr1 = 1.0 at sp+32, fr0 = 0.0 at sp+36/+40/+0/+4/+20/+24/+8/+12/+16
    let _ = mem.write_f32(sp.wrapping_add(32), 1.0); // fldi1 fr1 ; fmov fr1,@r2
    cpu.fr[0] = 0.0; // fldi0 fr0
    let _ = mem.write_f32(sp.wrapping_add(36), 0.0);
    let _ = mem.write_f32(sp.wrapping_add(40), 0.0);
    let _ = mem.write_u32(r4.wrapping_add(4), 1.0f32.to_bits()); // mov.l r14,@r11 (r14 = mem[sp+32])
    let _ = mem.write_u32(r4.wrapping_add(8), 0); // mov.l r0,@(4,r11) (r0 = mem[sp+36])
    let _ = mem.write_u32(r4.wrapping_add(12), 0); // mov.l r3,@(8,r11) (r3 = mem[sp+40])
    let _ = mem.write_f32(sp, 0.0);
    let _ = mem.write_f32(sp.wrapping_add(4), 0.0);
    let _ = mem.write_u32(r4.wrapping_add(28), 0); // mov.l r11,@r9 (r11 = mem[sp])
    let _ = mem.write_u32(r4.wrapping_add(32), 0); // mov.l r10,@(4,r9) (r10 = mem[sp+4])
    let _ = mem.write_f32(sp.wrapping_add(20), 0.0);
    let _ = mem.write_f32(sp.wrapping_add(24), 0.0);
    let _ = mem.write_f32(sp.wrapping_add(28), 1.0);
    let _ = mem.write_u32(r4.wrapping_add(52), 0); // mov.l r10,@r12 (mem[sp+20])
    let _ = mem.write_u32(r4.wrapping_add(56), 0); // mov.l r9,@(4,r12) (mem[sp+24])
    let _ = mem.write_u32(r4.wrapping_add(60), 1.0f32.to_bits()); // mov.l r9,@(8,r12) (mem[sp+28])
    let _ = mem.write_u32(r4.wrapping_add(64), 0); // mov.l r14,@(r0,r4), r0 = 64, r14 = 0
    let _ = mem.write_u32(r4.wrapping_add(68), 0); // r0 = 68
    let _ = mem.write_f32(r4.wrapping_add(72), 1.0); // fmov fr1,@r7
    let _ = mem.write_f32(r4.wrapping_add(76), 1.0); // fmov fr1,@r6
    let _ = mem.write_f32(sp.wrapping_add(8), 0.0);
    let _ = mem.write_f32(sp.wrapping_add(12), 0.0);
    let _ = mem.write_f32(sp.wrapping_add(16), 0.0);
    let _ = mem.write_u32(r4.wrapping_add(80), 0); // mov.l r6,@r5 (r6 = mem[sp+8])
    let _ = mem.write_u32(r4.wrapping_add(84), 0); // mov.l r4,@(4,r5) (r4 = mem[sp+12])
    let _ = mem.write_u32(r4.wrapping_add(88), 0); // mov.l r3,@(8,r5) (r3 = mem[sp+16])
    let _ = mem.write_f32(r4.wrapping_add(36), 1.0); // fmov fr1,@r1

    // epilogue: add #44,r15 ; pop r9..r13 ; rts ; delay pop r14
    cpu.r[15] = cpu.r[15].wrapping_add(44);
    for n in [9usize, 10, 11, 12, 13, 14] {
        let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[n] = v;
    }

    cpu.r[3] = 0; // mov.l @(8,r3),r3 with r3 = sp+8: mem[sp+16] = 0.0f bits
    cpu.r[0] = r4; // mov r4,r0 (before r4 is reloaded from mem[sp+12])
    cpu.r[1] = r4.wrapping_add(36);
    cpu.r[2] = sp.wrapping_add(16);
    // (r3 set above: mem[sp+16] = 0)
    cpu.r[4] = 0; // mov.l @(4,r3),r4 with r3 = sp+8: mem[sp+12] = 0.0f bits
    cpu.r[5] = r4.wrapping_add(80);
    cpu.r[6] = 0; // mem[sp+8]
    cpu.r[7] = r4.wrapping_add(72);
    cpu.fr[0] = 0.0;
    cpu.fr[1] = 1.0;
}
