//! 0x8c082ca0 — construct the vtable-0x8c1e9b50 object and its view block.
//!
//! Original stores, in instruction order (pointer comes from `r4`, `param_2`
//! from `r5`):
//!
//! ```text
//! +0   u32 0x8c1e9b50 (vtable)   +4   f32 0.0
//! +8   u8  1                     +12  u32 param_2
//! +16  u32 0x8c0822fc (code ptr, stored only; never called)
//! +20..+35   four words copied from 0x8c1d0f28
//! +36  f32 1.0                   +40  f32 0.0
//! +44  f32 12.0 (literal 0x8c082d24)
//! +48  u32 0                     +52  u32 0
//! +56..+103  twelve-word rect table written by 0x8c0820b4
//! ```
//!
//! The one call is `jsr @r1` at 0x8c082cf8, whose `r1` came from the literal
//! pool at 0x8c082d2c (0x8c0820b4). The delay slot computes `r4 = obj + 56`;
//! the float arguments are `fr4 = fr5 = 0.0`, `fr6 = 640.0` (0x8c082d34),
//! `fr7 = 480.0` (0x8c082d30). `mov.b` writes one byte at +8.
//!
//! The pseudocode's `if (FPSCR_SZ)` arms are the decompiler's ambiguity about
//! `fmov` width, not a branch: the disassembly has no conditional transfer and
//! the oracle is single-precision. `tst r3,r3` after the call clears T because
//! 0x8c0822fc != 0.
//!
//! Register residues: r0 = r1 = obj, r3 = 0x8c0822fc, and the callee's
//! leftovers r4 = obj+68, r5 = obj+64, r6 = obj+92, r7 = obj+88; r14 and pr
//! are restored from the stack, r2/r8-r13 untouched. FPU: fr0 = 0.0, fr4 =
//! fr5 = 0.0, fr6 = 640.0, fr7 = 480.0, fr1/fr2 are the callee's last
//! quotients, fr12..fr15 restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c082ca0 mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c082ca2 sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c082ca4 add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c082ca6 mov.l r4,@r15 — the object pointer lives in this slot
    let obj = cpu.r[4];
    let param_2 = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15], obj);
    // 0x8c082ca8 mov r5,r14
    cpu.r[14] = param_2;
    // 0x8c082caa mov.l @r15,r1
    cpu.r[1] = obj;
    // 0x8c082cac fldi0 fr5
    cpu.fr[5] = 0.0;
    // 0x8c082cae mov.l 0x8c082d20,r0 — vtable 0x8c1e9b50
    cpu.r[0] = mem.read_u32(0x8C08_2D20).unwrap_or(0);
    // 0x8c082cb0 fldi1 fr0
    cpu.fr[0] = 1.0;
    // 0x8c082cb2 mov #0,r4
    cpu.r[4] = 0;
    // 0x8c082cb4 mov.l r0,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    // 0x8c082cb6 fmov fr5,fr4
    cpu.fr[4] = cpu.fr[5];
    // 0x8c082cb8 mov.l @r15,r1 ; 0x8c082cba mova 0x8c082d24,r0 ;
    // 0x8c082cbc add #40,r1 ; 0x8c082cbe fmov fr5,@r1
    cpu.r[1] = obj;
    cpu.r[0] = 0x8C08_2D24;
    cpu.r[1] = cpu.r[1].wrapping_add(40);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[5]);
    // 0x8c082cc0 mov.l @r15,r3 ; 0x8c082cc2 mov.l 0x8c082d28,r5 ;
    // 0x8c082cc4 add #36,r3 ; 0x8c082cc6 mov.l 0x8c082d2c,r1
    cpu.r[3] = obj;
    cpu.r[5] = mem.read_u32(0x8C08_2D28).unwrap_or(0);
    cpu.r[3] = cpu.r[3].wrapping_add(36);
    cpu.r[1] = mem.read_u32(0x8C08_2D2C).unwrap_or(0);
    // 0x8c082cc8 fmov fr0,@r3 — 1.0
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    // 0x8c082cca mov.l @r15,r3 ; 0x8c082ccc fmov @r0,fr0 ;
    // 0x8c082cce add #44,r3 ; 0x8c082cd0 fmov fr0,@r3
    cpu.r[3] = obj;
    cpu.fr[0] = mem.read_f32(0x8C08_2D24).unwrap_or(0.0); // 12.0
    cpu.r[3] = cpu.r[3].wrapping_add(44);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    // 0x8c082cd2 mova 0x8c082d30,r0 ; 0x8c082cd4 mov.l @r15,r3 ;
    // 0x8c082cd6 mov.l r4,@(48,r3)
    cpu.r[0] = 0x8C08_2D30;
    cpu.r[3] = obj;
    let _ = mem.write_u32(cpu.r[3].wrapping_add(48), cpu.r[4]);
    // 0x8c082cd8 mov.l @r15,r3 ; 0x8c082cda mov.l r4,@(52,r3)
    cpu.r[3] = obj;
    let _ = mem.write_u32(cpu.r[3].wrapping_add(52), cpu.r[4]);
    // 0x8c082cdc..0x8c082cee — copy 0x8c1d0f28[0..4] to obj+20..+32
    cpu.r[6] = obj;
    cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(20);
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[3]);
    cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(8), cpu.r[4]);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(12), cpu.r[3]);
    // 0x8c082cf0 fmov @r0,fr7 — 480.0
    cpu.fr[7] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c082cf2 mov.l @r15,r4 ; 0x8c082cf4 mova 0x8c082d34,r0 ;
    // 0x8c082cf6 fmov @r0,fr6 — 640.0
    cpu.r[4] = obj;
    cpu.r[0] = 0x8C08_2D34;
    cpu.fr[6] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c082cf8 jsr @r1 — 0x8c0820b4(obj+56, fr4=fr5=0, fr6=640, fr7=480)
    cpu.pr = 0x8C08_2CFC;
    cpu.r[4] = cpu.r[4].wrapping_add(56); // delay slot 0x8c082cfa add #56,r4
    crate::registry::p_8c0820b4::run(cpu, mem);

    // 0x8c082cfc mov.l 0x8c082d38,r3 — 0x8c0822fc
    cpu.r[3] = mem.read_u32(0x8C08_2D38).unwrap_or(0);
    // 0x8c082cfe mov #1,r0
    cpu.r[0] = 1;
    // 0x8c082d00 mov.l @r15,r1
    cpu.r[1] = obj;
    // 0x8c082d02 fldi0 fr0
    cpu.fr[0] = 0.0;
    // 0x8c082d04 tst r3,r3 — nonzero, so T clears
    cpu.set_t((cpu.r[3] & cpu.r[3]) == 0);
    // 0x8c082d06 mov.l r14,@(12,r1)
    let _ = mem.write_u32(cpu.r[1].wrapping_add(12), cpu.r[14]);
    // 0x8c082d08 mov.l @r15,r1 ; 0x8c082d0a mov.b r0,@(8,r1)
    cpu.r[1] = obj;
    let _ = mem.write_u8(cpu.r[1].wrapping_add(8), cpu.r[0] as u8);
    // 0x8c082d0c mov.l @r15,r0 ; 0x8c082d0e add #4,r0 ;
    // 0x8c082d10 fmov fr0,@r0
    cpu.r[0] = obj;
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    // 0x8c082d12 mov.l @r15,r0 ; 0x8c082d14 mov.l r3,@(16,r0)
    cpu.r[0] = obj;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[3]);
    // 0x8c082d16 mov.l @r15,r0
    cpu.r[0] = obj;
    // 0x8c082d18 add #4,r15 ; 0x8c082d1a lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c082d1c rts ; delay slot 0x8c082d1e mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
