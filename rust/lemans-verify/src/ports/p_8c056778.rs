//! 0x8c056778 — free `*(r4 + 8)` unless it is NULL.
//!
//! ```text
//! sts.l pr,@-r15
//! mov.l @(8,r4),r5        ; r5 = block pointer
//! tst   r5,r5
//! bt    0x8c0567a2        ; NULL: epilogue, T = 1 from tst
//! mov.l @(4,r4),r0        ; r0 = count
//! r1 = r0*0x38 + r5       ; shll2/shll/sub/shll2/shll/add
//! cmp/hi r5,r1
//! bf.s  0x8c05679c        ; r1 <= r5: no walk (delay slot: r3 = r5)
//! loop: add #56,r3
//!       cmp/hi r3,r1
//!       bt loop
//! 0x8c05679c: mov.l 0x8c0567a8,r0  ; r0 = 0x8c01032c (free thunk)
//! jsr @r0
//! mov r5,r4               ; delay slot: free's argument
//! 0x8c0567a2: lds.l @r15+,pr
//! rts
//! nop
//! ```
//!
//! The 0x38-stride walk only writes r1/r3; free overwrites both before it
//! returns, so on the non-NULL path the observable state is entirely the
//! callee's, including T. The NULL path returns with r0..r4 untouched, r5 = 0
//! and T = 1. pr and r15 are restored by the frame, so both leave with their
//! seed values. The `bf.s` delay slot runs on both outcomes; the loop branch
//! (`bt`) is a plain branch, so its `add #56,r3` executes before comparing.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c056778: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05677a: mov.l @(8,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c05677c: tst r5,r5
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        // 0x8c056780: mov.l @(4,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        // 0x8c056782: mov r0,r1
        cpu.r[1] = cpu.r[0];
        // 0x8c056784: shll2 r1
        cpu.r[1] = cpu.r[1] << 2;
        // 0x8c056786: shll r1
        cpu.set_t(cpu.r[1] >> 31 != 0);
        cpu.r[1] = cpu.r[1] << 1;
        // 0x8c056788: sub r0,r1
        cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]);
        // 0x8c05678a: shll2 r1
        cpu.r[1] = cpu.r[1] << 2;
        // 0x8c05678c: shll r1
        cpu.set_t(cpu.r[1] >> 31 != 0);
        cpu.r[1] = cpu.r[1] << 1;
        // 0x8c05678e: add r5,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]);
        // 0x8c056790: cmp/hi r5,r1
        cpu.set_t(cpu.r[1] > cpu.r[5]);
        // 0x8c056792: bf.s 0x8c05679c ; 0x8c056794: mov r5,r3 (delay slot)
        cpu.r[3] = cpu.r[5];
        if cpu.r[1] > cpu.r[5] {
            loop {
                // 0x8c056796: add #56,r3
                cpu.r[3] = cpu.r[3].wrapping_add(0x38);
                // 0x8c056798: cmp/hi r3,r1
                let t = cpu.r[1] > cpu.r[3];
                cpu.set_t(t);
                // 0x8c05679a: bt 0x8c056796
                if !t {
                    break;
                }
            }
        }
        // 0x8c05679c: mov.l 0x8c0567a8,r0
        cpu.r[0] = mem.read_u32(0x8c0567a8).unwrap_or(0x8c01032c);
        // 0x8c05679e: jsr @r0 ; 0x8c0567a0: mov r5,r4 (delay slot)
        cpu.pr = 0x8c0567a2;
        cpu.r[4] = cpu.r[5];
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c0567a2: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0567a4: rts ; 0x8c0567a6: nop (delay slot)
}
