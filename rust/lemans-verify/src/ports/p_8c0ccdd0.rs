//! 0x8c0ccdd0 — initialise an object graph and optionally tear it down.
//!
//! r4 = object, r5 = mode (only `exts.w` of it matters).
//!
//! ```text
//! push r14, r13, pr ; r14 = r5 ; r13 = r4
//! tst r13,r13 ; bt out
//! [r13+8] = 0x8c1ebe10
//! a = [r13] ; [a+4] = 0x8c1ebe10 + 0xd4
//! b = [a]   ; [b+4] = 0x8c1ebe10 + 0xb4
//! c = [b]   ; [c]   = 0x8c1ebe10 + 0x38
//! d = [r13+4] ; [d+24] = (r13+12) - d
//! b = [a]     ; [b+36] = (r13+40) - b
//! a = [r13]   ; [a+16] = (r13+80) - a
//! if (short)r14 != 0:
//!     teardown(r13+80, 0)         ; 0x8c0cd048
//!     destroy(r13+40, 0)          ; 0x8c07c100
//!     if r13+12 != 0: [r13+12] = 0x8c1e9ad4
//! if (short)r14 > 0: free(r13)    ; 0x8c01032c -> 0x8c159fc0
//! out: r0 = r13 ; pop pr, r13 ; rts ; pop r14
//! ```
//!
//! The `tst r0,r0` on `r13+12` at 0x8c0cce3e can only take its branch when
//! `r13 == 0xfffffff4`, whose earlier stores are outside mapped memory, so the
//! oracle never reaches that arm with a clean halt; the branch is still
//! transcribed.
//!
//! Paths and T:
//!  - `r4 == 0`: only push/pop; T = 1 from `tst r13,r13`; returns entry r13.
//!  - `(i16)r5 == 0`: stores only; no calls; T = 0 from `cmp/pl` on zero.
//!  - `(i16)r5 < 0`: stores, both calls, `[r13+12]` store, no free; T = 0.
//!  - `(i16)r5 > 0`: everything incl. the final free; T is `free`'s residue and
//!    r0 is overwritten with r13 at the exit.
//!
//! The callee residue in r0/r1/r2/r4-r7 (and T on the free path) is whatever the
//! ported callees leave; 0x8c0cd048 and 0x8c07c100 both preserve r13/r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0ccdd0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0ccdd2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0ccdd4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0ccdd6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0ccdd8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0ccdda: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0ccddc: bt 0x8c0cce52
    if cpu.r[13] != 0 {
        // 0x8c0ccdde: mov.l 0x8c0cce5c,r0
        cpu.r[0] = mem.read_u32(0x8c0cce5c).unwrap_or(0x8c1ebe10);
        // 0x8c0ccde0: mov r13,r6
        cpu.r[6] = cpu.r[13];
        // 0x8c0ccde2: mov r13,r5
        cpu.r[5] = cpu.r[13];
        // 0x8c0ccde4: mov.l r0,@(8,r13)
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[0]);
        // 0x8c0ccde6: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c0ccde8: mov.l 0x8c0cce60,r3
        cpu.r[3] = mem.read_u32(0x8c0cce60).unwrap_or(0xd4);
        // 0x8c0ccdea: add #12,r6
        cpu.r[6] = cpu.r[6].wrapping_add(12);
        // 0x8c0ccdec: mov.l @r13,r4
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0ccdee: add r3,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        // 0x8c0ccdf0: add #40,r5
        cpu.r[5] = cpu.r[5].wrapping_add(40);
        // 0x8c0ccdf2: mov.l r1,@(4,r4)
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[1]);
        // 0x8c0ccdf4: mov.l @r13,r4
        cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0ccdf6: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c0ccdf8: mov.l 0x8c0cce64,r3
        cpu.r[3] = mem.read_u32(0x8c0cce64).unwrap_or(0xb4);
        // 0x8c0ccdfa: add #56,r0
        cpu.r[0] = cpu.r[0].wrapping_add(56);
        // 0x8c0ccdfc: mov.l @r4,r7
        cpu.r[7] = mem.read_u32(cpu.r[4]).unwrap_or(0);
        // 0x8c0ccdfe: add r3,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        // 0x8c0cce00: mov.l r1,@(4,r7)
        let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[1]);
        // 0x8c0cce02: mov r13,r4
        cpu.r[4] = cpu.r[13];
        // 0x8c0cce04: mov.l @r13,r3
        cpu.r[3] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cce06: exts.w r14,r1
        cpu.r[1] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0cce08: add #80,r4
        cpu.r[4] = cpu.r[4].wrapping_add(80);
        // 0x8c0cce0a: mov.l @r3,r3
        cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c0cce0c: tst r1,r1
        cpu.set_t(cpu.r[1] == 0);
        // 0x8c0cce0e: mov.l @r3,r7
        cpu.r[7] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        // 0x8c0cce10: mov.l r0,@r7
        let _ = mem.write_u32(cpu.r[7], cpu.r[0]);
        // 0x8c0cce12: mov r4,r3
        cpu.r[3] = cpu.r[4];
        // 0x8c0cce14: mov.l @(4,r13),r0
        cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        // 0x8c0cce16: sub r0,r6
        cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[0]);
        // 0x8c0cce18: mov.l r6,@(24,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[6]);
        // 0x8c0cce1a: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cce1c: mov.l @r0,r0
        cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        // 0x8c0cce1e: sub r0,r5
        cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[0]);
        // 0x8c0cce20: mov.l r5,@(36,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(36), cpu.r[5]);
        // 0x8c0cce22: mov.l @r13,r0
        cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        // 0x8c0cce24: sub r0,r3
        cpu.r[3] = cpu.r[3].wrapping_sub(cpu.r[0]);
        // 0x8c0cce26: bt.s 0x8c0cce46
        let t = cpu.t();
        // delay slot 0x8c0cce28: mov.l r3,@(16,r0)
        let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[3]);
        if !t {
            // 0x8c0cce2a: mov.l 0x8c0cce68,r0
            cpu.r[0] = mem.read_u32(0x8c0cce68).unwrap_or(0x8c0cd048);
            // 0x8c0cce2c: jsr @r0 ; delay 0x8c0cce2e: mov #0,r5
            cpu.pr = 0x8c0cce30;
            cpu.r[5] = 0;
            crate::registry::p_8c0cd048::run(cpu, mem);

            // 0x8c0cce30: mov.l 0x8c0cce6c,r0
            cpu.r[0] = mem.read_u32(0x8c0cce6c).unwrap_or(0x8c07c100);
            // 0x8c0cce32: mov r13,r4
            cpu.r[4] = cpu.r[13];
            // 0x8c0cce34: add #40,r4
            cpu.r[4] = cpu.r[4].wrapping_add(40);
            // 0x8c0cce36: jsr @r0 ; delay 0x8c0cce38: mov #0,r5
            cpu.pr = 0x8c0cce3a;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);

            // 0x8c0cce3a: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c0cce3c: add #12,r0
            cpu.r[0] = cpu.r[0].wrapping_add(12);
            // 0x8c0cce3e: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c0cce40: bt 0x8c0cce46
            if cpu.r[0] != 0 {
                // 0x8c0cce42: mov.l 0x8c0cce70,r0
                cpu.r[0] = mem.read_u32(0x8c0cce70).unwrap_or(0x8c1e9ad4);
                // 0x8c0cce44: mov.l r0,@(12,r13)
                let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[0]);
            }
        }

        // 0x8c0cce46: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0cce48: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0cce4a: bf 0x8c0cce52
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0cce4c: mov.l 0x8c0cce74,r0
            cpu.r[0] = mem.read_u32(0x8c0cce74).unwrap_or(0x8c01032c);
            // 0x8c0cce4e: jsr @r0 ; delay 0x8c0cce50: mov r13,r4
            cpu.pr = 0x8c0cce52;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c0cce52: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0cce54: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0cce56: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0cce58: rts ; delay 0x8c0cce5a: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
