//! 0x8c21d398 — clear the 0x1af8-byte block at 0x8c2bae60 via memset
//! (0x8c010f88), then zero the three words at 0x8c2bc958, 0x8c2bc95c and
//! 0x8c2bc960.
//!
//! ```text
//! sts.l pr,@-r15
//! mov.l 0x8c21d4a8,r3 (0x8c010f88) ; mov.l 0x8c21d4a4,r4 (0x8c2bae60)
//! mov.w 0x8c21d48e,r6 (0x1af8)
//! jsr @r3 ; mov #0,r5 (delay slot)
//! mov.l 0x8c21d4b0,r3 (0x8c2bc95c) ; mov #0,r4
//! mov.l 0x8c21d4ac,r2 (0x8c2bc958) ; mov r4,r1
//! mov.l 0x8c21d4b4,r0 (0x8c2bc960)
//! mov.l r4,@r2 ; mov.l r4,@r3
//! lds.l @r15+,pr
//! rts ; mov.l r4,@r0 (delay slot)
//! ```
//!
//! The fill ends exactly at 0x8c2bc958, the first word zeroed afterwards. The
//! callee leaves T set (its last `cmp/hs` sees the counter at n), and the
//! post-call sequence is moves/stores only, so T exits set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c21d398: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c21d39a: mov.l 0x8c21d4a8,r3
    cpu.r[3] = mem.read_u32(0x8c21d4a8).unwrap_or(0); // 0x8c010f88 (memset)
    // 0x8c21d39c: mov.l 0x8c21d4a4,r4
    cpu.r[4] = mem.read_u32(0x8c21d4a4).unwrap_or(0); // 0x8c2bae60
    // 0x8c21d39e: mov.w 0x8c21d48e,r6
    cpu.r[6] = mem.read_u16(0x8c21d48e).unwrap_or(0) as i16 as i32 as u32; // 0x1af8
    // 0x8c21d3a0: jsr @r3 ; 0x8c21d3a2: mov #0,r5 (delay slot)
    cpu.pr = 0x8c21d3a4;
    cpu.r[5] = 0;
    crate::registry::p_8c010f88::run(cpu, mem);
    // 0x8c21d3a4: mov.l 0x8c21d4b0,r3
    cpu.r[3] = mem.read_u32(0x8c21d4b0).unwrap_or(0); // 0x8c2bc95c
    // 0x8c21d3a6: mov #0,r4
    cpu.r[4] = 0;
    // 0x8c21d3a8: mov.l 0x8c21d4ac,r2
    cpu.r[2] = mem.read_u32(0x8c21d4ac).unwrap_or(0); // 0x8c2bc958
    // 0x8c21d3aa: mov r4,r1
    cpu.r[1] = cpu.r[4];
    // 0x8c21d3ac: mov.l 0x8c21d4b4,r0
    cpu.r[0] = mem.read_u32(0x8c21d4b4).unwrap_or(0); // 0x8c2bc960
    // 0x8c21d3ae: mov.l r4,@r2
    let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
    // 0x8c21d3b0: mov.l r4,@r3
    let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
    // 0x8c21d3b2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c21d3b4: rts ; 0x8c21d3b6: mov.l r4,@r0 (delay slot)
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
}
