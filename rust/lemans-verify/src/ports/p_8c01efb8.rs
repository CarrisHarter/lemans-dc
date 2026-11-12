//! 0x8c01efb8 — wait out a size-derived span, then free `[r4+8]`.
//!
//! ```text
//! sts.l pr,@-r15
//! r5 = [r4+8]                     ; block to free
//! if (r5 == 0) goto epilogue      ; T = 1
//! r1 = [r4+4] * 0x104 + r5        ; u32, wrapping
//! T = r1 > r5                     ; cmp/hi r5,r1 (unsigned)
//! r3 = r5                         ; bf.s delay slot, runs either way
//! if (T) for (;;) { r0 = 0x104; r3 += 0x104; T = r1 > r3; if (!T) break; }
//!                                 ; the loop body is add r0,r3; cmp/hi; bt
//! jsr @0x8c01032c                 ; delay slot: r4 = r5
//! epilogue: lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! The span compares unsigned, so a wrapped `r1` (below `r5`) takes the
//! `bf.s` path without iterating. When the loop does run it stops at the first
//! `r3 >= r1`; `T` is then the last `cmp/hi` result, and the callee's `T` and
//! register residue pass through the epilogue unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let t = cpu.r[5] == 0; // tst r5,r5
    cpu.set_t(t);
    if !t {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = cpu.r[0];
        cpu.r[1] <<= 2;
        cpu.r[1] <<= 2;
        cpu.r[1] <<= 2;
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.r[1] <<= 2; // r1 = 0x104 * [r4+4]
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
        let mut hi = cpu.r[1] > cpu.r[5]; // cmp/hi r5,r1
        cpu.set_t(hi);
        cpu.r[3] = cpu.r[5]; // bf.s delay slot
        if hi {
            cpu.r[0] = mem.read_u32(0x8c01efec).unwrap_or(0x104);
            loop {
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // add r0,r3
                hi = cpu.r[1] > cpu.r[3]; // cmp/hi r3,r1
                cpu.set_t(hi);
                if !hi {
                    break; // bt not taken
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c01eff0).unwrap_or(0x8c01032c);
        cpu.pr = 0x8c01efe4; // jsr @r0
        cpu.r[4] = cpu.r[5]; // delay slot
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
