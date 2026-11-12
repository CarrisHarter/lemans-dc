//! 0x8c0c5f5c — construct a transform object from six inputs and clamp its
//! stored extents: `out[+80] = min(max(fr4, fr6), fr5)`.
//!
//! Object layout, all offsets from `r4` (`r12` after the prologue):
//! ```text
//! [0]      = 0x8c1e8b54                 ; vtable/type tag (written twice)
//! [+56]    = u32 from [r5+0]            ; copied pair
//! [+60]    = u32 from [r5+4]
//! [+64]    = u32 from [r6+0]
//! [+68]    = u32 from [r6+4]
//! [+72]    = f32 fr4                    ; lower extent
//! [+76]    = f32 fr5                    ; upper extent
//! [+80]    = f32 fr6                    ; value, then overwritten by clamp
//! ```
//! Calls 0x8c05d378 (`*r4 = 0x8c1e9148` and zero fields) and 0x8c05d274
//! (set bit 0 of `[r4+0x10]`), then writes the type tag 0x8c1e8b54 and
//! 0x8c1eb828 over the callee's state.
//!
//! Clamp: `fr0 = [+72]`, `fr1 = [+80]`, `T = fr0 > fr1`; when taken `fr1 = fr0`.
//! So `fr1 = max(fr4, fr6)`. Then `fr0 = [+76]`, `fr1 = [+80]`, `T = fr1 > fr0`;
//! when taken `fr1 = fr0`, so `fr1 = min(max(fr4,fr6), fr5)`. Both comparisons
//! are strict `fcmp/gt`, so equality takes the not-taken arm (value unchanged,
//! T = 0). A NaN operand clears T on either comparison. The final `T` is the
//! second comparison's result; it is part of the observable state.
//!
//! ```text
//! fmov fr15,@-r15 ; fmov fr14,@-r15 ; fmov fr13,@-r15
//! fmov fr5,fr14 ; mov.l r14,@-r15 ; fmov fr4,fr13 ; mov.l r13,@-r15
//! mov r6,r14 ; mov.l r12,@-r15 ; mov r5,r13 ; sts.l pr,@-r15
//! add #-4,r15 ; mov r4,r12 ; fmov fr6,fr15 ; mov.l r12,@r15
//! jsr 0x8c05d378 (delay: mov.l @r15,r4)
//! r1 = [r15] ; r6 = 1 ; r5 = 1 ; [r1] = 0x8c1eb940
//! jsr 0x8c05d274 (delay: mov.l @r15,r4)
//! ... field stores ...
//! fcmp/gt / conditional fr1 = fr0 (max), fcmp/gt / conditional (min)
//! epilogue: add #4,r15 ; lds.l pr ; pop r12,r13,r14,fr13,fr14
//! rts (delay: fmov @r15+,fr15) ; r0 = r12
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c5f5c: fmov fr15,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    // 0x8c0c5f5e: fmov fr14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    // 0x8c0c5f60: fmov fr13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    // 0x8c0c5f62: fmov fr5,fr14
    cpu.fr[14] = cpu.fr[5];
    // 0x8c0c5f64: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c5f66: fmov fr4,fr13
    cpu.fr[13] = cpu.fr[4];
    // 0x8c0c5f68: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0c5f6a: mov r6,r14
    cpu.r[14] = cpu.r[6];
    // 0x8c0c5f6c: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0c5f6e: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c0c5f70: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0c5f72: add #-4,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    // 0x8c0c5f74: mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c0c5f76: fmov fr6,fr15
    cpu.fr[15] = cpu.fr[6];
    // 0x8c0c5f78: mov.l r12,@r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0c5f7a: mov.l 0x8c0c6004,r0 (0x8c05d378) ; 0x8c0c5f7c: jsr @r0
    cpu.r[0] = mem.read_u32(0x8c0c6004).unwrap_or(0);
    cpu.pr = 0x8c0c5f80;
    // 0x8c0c5f7e: mov.l @r15,r4 (delay slot)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    crate::registry::p_8c05d378::run(cpu, mem);
    // 0x8c0c5f80: mov.l @r15,r1
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 0x8c0c5f82: mov #1,r6
    cpu.r[6] = 1;
    // 0x8c0c5f84: mov.l 0x8c0c6008,r0 (0x8c1eb940)
    cpu.r[0] = mem.read_u32(0x8c0c6008).unwrap_or(0);
    // 0x8c0c5f86: mov r6,r5
    cpu.r[5] = cpu.r[6];
    // 0x8c0c5f88: mov.l r0,@r1
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    // 0x8c0c5f8a: mov.l 0x8c0c600c,r0 (0x8c05d274) ; 0x8c0c5f8c: jsr @r0
    cpu.r[0] = mem.read_u32(0x8c0c600c).unwrap_or(0);
    cpu.pr = 0x8c0c5f90;
    // 0x8c0c5f8e: mov.l @r15,r4 (delay slot)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    crate::registry::p_8c05d274::run(cpu, mem);
    // 0x8c0c5f90: mov.l 0x8c0c6010,r0 (0x8c1e8b54)
    cpu.r[0] = mem.read_u32(0x8c0c6010).unwrap_or(0);
    // 0x8c0c5f92: mov r12,r4
    cpu.r[4] = cpu.r[12];
    // 0x8c0c5f94: mov r12,r7
    cpu.r[7] = cpu.r[12];
    // 0x8c0c5f96: mov.l r0,@r12
    let _ = mem.write_u32(cpu.r[12], cpu.r[0]);
    // 0x8c0c5f98: mov r12,r3
    cpu.r[3] = cpu.r[12];
    // 0x8c0c5f9a: mov.l @r13,r1
    cpu.r[1] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0c5f9c: add #56,r4
    cpu.r[4] = cpu.r[4].wrapping_add(56);
    // 0x8c0c5f9e: mov.l @(4,r13),r0
    cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c5fa0: add #64,r7
    cpu.r[7] = cpu.r[7].wrapping_add(64);
    // 0x8c0c5fa2: add #72,r3
    cpu.r[3] = cpu.r[3].wrapping_add(72);
    // 0x8c0c5fa4: mov.l r1,@r4
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    // 0x8c0c5fa6: mov.l r0,@(4,r4)
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]);
    // 0x8c0c5fa8: mov r12,r1
    cpu.r[1] = cpu.r[12];
    // 0x8c0c5faa: mov.l @r14,r6
    cpu.r[6] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    // 0x8c0c5fac: mov r12,r0
    cpu.r[0] = cpu.r[12];
    // 0x8c0c5fae: mov.l @(4,r14),r5
    cpu.r[5] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c5fb0: add #76,r1
    cpu.r[1] = cpu.r[1].wrapping_add(76);
    // 0x8c0c5fb2: mov.l 0x8c0c6014,r4 (0x8c1eb828)
    cpu.r[4] = mem.read_u32(0x8c0c6014).unwrap_or(0);
    // 0x8c0c5fb4: add #80,r0
    cpu.r[0] = cpu.r[0].wrapping_add(80);
    // 0x8c0c5fb6: mov.l r6,@r7
    let _ = mem.write_u32(cpu.r[7], cpu.r[6]);
    // 0x8c0c5fb8: mov.l r5,@(4,r7)
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[5]);
    // 0x8c0c5fba: mov.l r4,@r12
    let _ = mem.write_u32(cpu.r[12], cpu.r[4]);
    // 0x8c0c5fbc: fmov fr13,@r3
    let _ = mem.write_f32(cpu.r[3], cpu.fr[13]);
    // 0x8c0c5fbe: fmov fr14,@r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[14]);
    // 0x8c0c5fc0: fmov fr15,@r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[15]);
    // 0x8c0c5fc2: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    // 0x8c0c5fc4: fmov @r0,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0c5fc6: fcmp/gt fr1,fr0 -> T = fr0 > fr1
    cpu.set_t(cpu.fr[0] > cpu.fr[1]);
    // 0x8c0c5fc8: bt 0x8c0c5fce (else 0x8c0c5fca: bra 0x8c0c5fd0)
    if cpu.t() {
        // 0x8c0c5fce: fmov fr0,fr1
        cpu.fr[1] = cpu.fr[0];
    }
    // 0x8c0c5fd0: mov r12,r1 ; 0x8c0c5fd2: add #80,r1
    cpu.r[1] = cpu.r[12].wrapping_add(80);
    // 0x8c0c5fd4: mov r12,r0
    cpu.r[0] = cpu.r[12];
    // 0x8c0c5fd6: fmov fr1,@r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    // 0x8c0c5fd8: add #76,r0
    cpu.r[0] = cpu.r[0].wrapping_add(76);
    // 0x8c0c5fda: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0c5fdc: fmov @r1,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    // 0x8c0c5fde: fcmp/gt fr0,fr1 -> T = fr1 > fr0
    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
    // 0x8c0c5fe0: bt 0x8c0c5fe6 (else 0x8c0c5fe2: bra 0x8c0c5fe8)
    if cpu.t() {
        // 0x8c0c5fe6: fmov fr0,fr1
        cpu.fr[1] = cpu.fr[0];
    }
    // 0x8c0c5fe8: mov r12,r0 ; 0x8c0c5fea: add #80,r0
    cpu.r[0] = cpu.r[12].wrapping_add(80);
    // 0x8c0c5fec: fmov fr1,@r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[1]);
    // 0x8c0c5fee: mov r12,r0
    cpu.r[0] = cpu.r[12];
    // 0x8c0c5ff0: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ff2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ff4: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ff6: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ff8: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ffa: fmov @r15+,fr13
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ffc: fmov @r15+,fr14
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c5ffe: rts ; 0x8c0c6000: fmov @r15+,fr15 (delay slot)
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
