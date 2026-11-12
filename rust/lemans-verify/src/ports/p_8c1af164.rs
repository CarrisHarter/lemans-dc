//! 0x8c1af164 — free the block pointer stored at `arg+8` when it is non-NULL.
//!
//! ```text
//! sts.l pr,@-r15          ; push pr
//! mov.l @(8,r4),r5        ; r5 = arg->ptr
//! tst   r5,r5
//! bt    0x8c1af18a        ; NULL: skip the call
//! mov.l @(4,r4),r0        ; r0 = arg->count
//! mov   r5,r1
//! shll2 r0 ; shll2 r0 ; shll r0   ; r0 = count * 32
//! add   r0,r1             ; r1 = ptr + count*32
//! cmp/hi r5,r1
//! bf.s  0x8c1af184        ; delay: mov r5,r3
//! add   #32,r3            ; 0x8c1af17e: r3 += 32
//! cmp/hi r3,r1
//! bt    0x8c1af17e        ; keep stepping r3 until it reaches r1
//! mov.l 0x8c1af190,r0     ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0               ; delay: mov r5,r4
//! lds.l @r15+,pr
//! rts
//! nop
//! ```
//!
//! The `bf.s`/`bt` loop only advances `r3`; nothing reads it before the call
//! clobbers it, so it exists solely to keep the step count honest. Both branch
//! directions leave T clear, but the callee's flags replace it anyway.
//!
//! The NULL path exits with `tst`'s T set and r0/r1/r3/r4 as they entered; the
//! call path passes `r5` through the jsr delay slot as the argument, so `r4`
//! ends as the callee left it. `pr` is restored from the frame on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c1af164: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c1af166: mov.l @(8,r4),r5
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c1af168: tst r5,r5
    cpu.set_t(cpu.r[5] == 0);
    // 0x8c1af16a: bt 0x8c1af18a
    if cpu.r[5] != 0 {
        // 0x8c1af16c: mov.l @(4,r4),r0
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        // 0x8c1af16e: mov r5,r1
        cpu.r[1] = cpu.r[5];
        // 0x8c1af170/172/174: shll2 r0 ; shll2 r0 ; shll r0
        cpu.r[0] = cpu.r[0].wrapping_shl(2).wrapping_shl(2).wrapping_shl(1);
        // 0x8c1af176: add r0,r1
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        // 0x8c1af178: cmp/hi r5,r1
        cpu.set_t(cpu.r[1] > cpu.r[5]);
        // 0x8c1af17a: bf.s 0x8c1af184 ; 0x8c1af17c: mov r5,r3 (delay slot)
        cpu.r[3] = cpu.r[5];
        if cpu.t() {
            // 0x8c1af17e: add #32,r3 ; 0x8c1af180: cmp/hi r3,r1 ; 0x8c1af182: bt 0x8c1af17e
            loop {
                cpu.r[3] = cpu.r[3].wrapping_add(32);
                cpu.set_t(cpu.r[1] > cpu.r[3]);
                if !cpu.t() {
                    break;
                }
            }
        }
        // 0x8c1af184: mov.l 0x8c1af190,r0
        cpu.r[0] = mem.read_u32(0x8c1af190).unwrap_or(0x8c01032c);
        // 0x8c1af186: jsr @r0 ; 0x8c1af188: mov r5,r4 (delay slot)
        cpu.r[4] = cpu.r[5];
        cpu.pr = 0x8c1af18a;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c1af18a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c1af18c: rts ; 0x8c1af18e: nop (delay slot)
}
