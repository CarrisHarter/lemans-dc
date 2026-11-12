//! 0x8c0326d4 — tear down a container: stamp the vtable, release three
//! sub-objects, drop the slot block, and free the container when the count
//! is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14
//! mov.l r12,@-r15 ; mov r4,r13 ; sts.l pr,@-r15
//! tst r13,r13 ; bt exit                      ; null: r0 = 0, T = 1
//! r1 = [0x8c032744] ; [r13] = r1             ; 0x8c1e8a08 vtable stamp
//! r0 = [0x8c032748] ; jsr @r0 ; [r13+40]     ; 0x8c06c8e0 retire node
//! r0 = 0 ; [r13+40] = r0
//! r5 = 1 ; r0 = [0x8c03274c] ; jsr @r0 ; [r13+44]  ; 0x8c06cbc0 stamp+free
//! r0 = 0 ; [r13+44] = r0
//! r5 = 1 ; r0 = [0x8c032750] ; jsr @r0 ; [r13+36]  ; 0x8c0708c8 init+free
//! r0 = 0 ; [r13+36] = r0
//! r12 = [r13+4] ; tst r12,r12 ; bt done     ; empty slot: skip the releases
//! r0 = [0x8c032754] ; r4 = r12+4 ; jsr @r0 ; r5 = -1  ; 0x8c018bd8 handle+4
//! r0 = [0x8c032754] ; r4 = r12   ; jsr @r0 ; r5 = -1  ; 0x8c018bd8 handle
//! r0 = [0x8c032758] ; jsr @r0 ; r4 = r12                ; 0x8c01032c free slot
//! done: r1 = 0 ; r0 = exts.w(r14) ; cmp/pl r0
//! bf.s exit ; [r13+4] = r1                   ; delay: slot cleared always
//! r0 = [0x8c032758] ; jsr @r0 ; r4 = r13     ; 0x8c01032c free self
//! exit: r0 = r13 ; lds pr ; pop r12,r13 ; rts ; pop r14
//! ```
//!
//! The six `jsr` delay slots all run before their callee: the `mov.l`
//! argument loads for the five register-indirect calls and `mov #-1,r5` /
//! `mov r13,r4` for the handle releases and the self-free. `pr` is set for
//! every call (0x8c0326ee, 0x8c0326fa, 0x8c032706, 0x8c03271a, 0x8c032722,
//! 0x8c03272e, 0x8c032738) and restored from the frame in the epilogue, so
//! it ends with the seed value. On the null path `r0` is the null `r13`
//! and T stays set from the `tst`; on a live container T is the last
//! `cmp/pl` (or the callee's T when the self-free runs).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0326d4..0x8c0326de: save r14, r13, r12, pr; r14 = r5, r13 = r4.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c0326e0: tst r13,r13 ; 0x8c0326e2: bt exit.
    cpu.set_t(cpu.r[13] == 0);
    if cpu.r[13] != 0 {
        // 0x8c0326e4: vtable stamp.
        cpu.r[1] = mem.read_u32(0x8c032744).unwrap_or(0x8c1e8a08);
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]);

        // 0x8c0326e8: retire the node at [r13+40], then clear the field.
        cpu.r[0] = mem.read_u32(0x8c032748).unwrap_or(0x8c06c8e0);
        cpu.pr = 0x8c0326ee;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(40)).unwrap_or(0);
        crate::registry::p_8c06c8e0::run(cpu, mem);
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[13].wrapping_add(40), 0);

        // 0x8c0326f2: stamp and free [r13+44], then clear the field.
        cpu.r[5] = 1;
        cpu.r[0] = mem.read_u32(0x8c03274c).unwrap_or(0x8c06cbc0);
        cpu.pr = 0x8c0326fa;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(44)).unwrap_or(0);
        crate::registry::p_8c06cbc0::run(cpu, mem);
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[13].wrapping_add(44), 0);

        // 0x8c0326fe: release and free [r13+36], then clear the field.
        cpu.r[5] = 1;
        cpu.r[0] = mem.read_u32(0x8c032750).unwrap_or(0x8c0708c8);
        cpu.pr = 0x8c032706;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0);
        crate::registry::p_8c0708c8::run(cpu, mem);
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[13].wrapping_add(36), 0);

        // 0x8c03270a: r12 = [r13+4] ; 0x8c03270c: tst ; 0x8c03270e: bt done.
        cpu.r[12] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[12] == 0);
        if cpu.r[12] != 0 {
            // 0x8c032710: release the handle pair at slot+4.
            cpu.r[0] = mem.read_u32(0x8c032754).unwrap_or(0x8c018bd8);
            cpu.r[4] = cpu.r[12];
            cpu.r[4] = cpu.r[4].wrapping_add(4);
            cpu.pr = 0x8c03271a;
            cpu.r[5] = 0xffff_ffff; // delay: mov #-1,r5
            crate::registry::p_8c018bd8::run(cpu, mem);

            // 0x8c03271a: release the handle at the slot.
            cpu.r[0] = mem.read_u32(0x8c032754).unwrap_or(0x8c018bd8);
            cpu.r[4] = cpu.r[12];
            cpu.pr = 0x8c032722;
            cpu.r[5] = 0xffff_ffff; // delay: mov #-1,r5
            crate::registry::p_8c018bd8::run(cpu, mem);

            // 0x8c032722: free the slot block.
            cpu.r[0] = mem.read_u32(0x8c032758).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c032728;
            cpu.r[4] = cpu.r[12]; // delay: mov r12,r4
            crate::registry::p_8c01032c::run(cpu, mem);
        }

        // 0x8c032728: r1 = 0 ; 0x8c03272a: exts.w r14,r0 ; 0x8c03272c: cmp/pl.
        cpu.r[1] = 0;
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c03272e: bf.s exit ; delay always clears the slot field.
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[1]);
        if cpu.t() {
            // 0x8c032732: free the container itself.
            cpu.r[0] = mem.read_u32(0x8c032758).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c032738;
            cpu.r[4] = cpu.r[13]; // delay: mov r13,r4
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c032738: mov r13,r0 ; epilogue; rts delay pops r14.
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
