//! 0x8c07fe0c — destroy a ring container, then free the container when asked.
//!
//! ```text
//! 8c07fe0c  mov.l r14,@-r15        push r14
//! 8c07fe0e  mov.l r13,@-r15        push r13
//! 8c07fe10  mov   r4,r14           r14 = container
//! 8c07fe12  sts.l pr,@-r15         push pr
//! 8c07fe14  mov   r5,r13           r13 = free flag (short in the low half)
//! 8c07fe16  tst   r14,r14
//! 8c07fe18  bt    0x8c07fe52       NULL container: return NULL
//! 8c07fe1a  mov.l 0x8c07fe5c,r0    r0 = 0x8c0802f0 (ring destroy)
//! 8c07fe1c  jsr   @r0              ring.destroy(container); r4 untouched
//! 8c07fe1e  nop
//! 8c07fe20  mov   r14,r0
//! 8c07fe22  add   #4,r0            r0 = container+4
//! 8c07fe24  tst   r0,r0
//! 8c07fe26  bt    0x8c07fe46       container == -4 (unmapped): skip to the flag test
//! 8c07fe28  tst   r0,r0            compiler kept the duplicate test
//! 8c07fe2a  bt    0x8c07fe46
//! 8c07fe2c  mov.l @(16,r14),r4     r4 = item array
//! 8c07fe2e  tst   r4,r4
//! 8c07fe30  bt    0x8c07fe38       NULL array: keep it
//! 8c07fe32  mov.l 0x8c07fe60,r0    r0 = 0x8c01032c (free thunk)
//! 8c07fe34  jsr   @r0              free(item array)
//! 8c07fe36  nop
//! 8c07fe38  mov   r14,r4
//! 8c07fe3a  add   #4,r4            r4 = container+4
//! 8c07fe3c  tst   r4,r4
//! 8c07fe3e  bt    0x8c07fe46       container == -4: skip
//! 8c07fe40  mov.l 0x8c07fe64,r0    r0 = 0x8c01e078 (release when count > 0)
//! 8c07fe42  jsr   @r0              release(container+4, 0): count 0, no-op
//! 8c07fe44  mov   #0,r5            delay
//! 8c07fe46  exts.w r13,r0          only the low half of the flag matters
//! 8c07fe48  cmp/pl r0
//! 8c07fe4a  bf    0x8c07fe52       flag <= 0: keep the container
//! 8c07fe4c  mov.l 0x8c07fe60,r0    r0 = 0x8c01032c (free thunk)
//! 8c07fe4e  jsr   @r0
//! 8c07fe50  mov   r14,r4           delay: free(container)
//! 8c07fe52  mov   r14,r0           return the container
//! 8c07fe54  lds.l @r15+,pr
//! 8c07fe56  mov.l @r15+,r13
//! 8c07fe58  rts
//! 8c07fe5a  mov.l @r15+,r14        delay
//! ```
//!
//! A NULL container returns immediately with T still set by the entry `tst`.
//! Otherwise the ring at `container+4` is destroyed first (`capacity` at `+4`,
//! `count` at `+8`, `start` at `+12`, items at `+16`), then the item array is
//! freed when non-NULL, then the release helper is handed `container+4` with a
//! zero count, which always takes its early exit. Finally the sign-extended low
//! half of the flag is tested: positive frees the container itself.
//!
//! The `(container+4) != 0` tests can only fail for `container == 0xfffffffc`,
//! an unmapped address, but they are ported as written. Every `jsr` writes pr
//! with its return address; the callees save and restore it and the epilogue
//! reloads the seeded pr from the frame. `bf` and `bt` here have no delay slots;
//! only the `rts` does, and it pops r14.

use sh4_oracle::{Cpu, Memory};

/// 0x8c07fe5c holds 0x8c0802f0, 0x8c07fe60 holds 0x8c01032c and 0x8c07fe64 holds 0x8c01e078.
const DESTROY_PTR: u32 = 0x8c07fe5c;
const FREE_THUNK_PTR: u32 = 0x8c07fe60;
const RELEASE_PTR: u32 = 0x8c07fe64;
const DESTROY: u32 = 0x8c0802f0;
const FREE_THUNK: u32 = 0x8c01032c;
const RELEASE: u32 = 0x8c01e078;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c07fe0c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07fe0e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c07fe10: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c07fe12: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07fe14: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c07fe16: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c07fe18: bt 0x8c07fe52
    if cpu.r[14] != 0 {
        // 0x8c07fe1a: mov.l 0x8c07fe5c,r0
        cpu.r[0] = mem.read_u32(DESTROY_PTR).unwrap_or(DESTROY);
        // 0x8c07fe1c: jsr @r0 ; 0x8c07fe1e: nop (delay)
        cpu.pr = 0x8c07fe20;
        crate::registry::p_8c0802f0::run(cpu, mem);

        // 0x8c07fe20: mov r14,r0 ; 0x8c07fe22: add #4,r0
        cpu.r[0] = cpu.r[14].wrapping_add(4);
        // 0x8c07fe24: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c07fe26: bt 0x8c07fe46
        if !cpu.t() {
            // 0x8c07fe28: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c07fe2a: bt 0x8c07fe46
            if !cpu.t() {
                // 0x8c07fe2c: mov.l @(16,r14),r4
                cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
                // 0x8c07fe2e: tst r4,r4
                cpu.set_t(cpu.r[4] == 0);
                // 0x8c07fe30: bt 0x8c07fe38
                if cpu.r[4] != 0 {
                    // 0x8c07fe32: mov.l 0x8c07fe60,r0
                    cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
                    // 0x8c07fe34: jsr @r0 ; 0x8c07fe36: nop (delay)
                    cpu.pr = 0x8c07fe38;
                    crate::registry::p_8c01032c::run(cpu, mem);
                }
                // 0x8c07fe38: mov r14,r4 ; 0x8c07fe3a: add #4,r4
                cpu.r[4] = cpu.r[14].wrapping_add(4);
                // 0x8c07fe3c: tst r4,r4
                cpu.set_t(cpu.r[4] == 0);
                // 0x8c07fe3e: bt 0x8c07fe46
                if cpu.r[4] != 0 {
                    // 0x8c07fe40: mov.l 0x8c07fe64,r0
                    cpu.r[0] = mem.read_u32(RELEASE_PTR).unwrap_or(RELEASE);
                    // 0x8c07fe42: jsr @r0 ; 0x8c07fe44: mov #0,r5 (delay)
                    cpu.pr = 0x8c07fe46;
                    cpu.r[5] = 0;
                    crate::registry::p_8c01e078::run(cpu, mem);
                }
            }
        }
        // 0x8c07fe46: exts.w r13,r0
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32;
        // 0x8c07fe48: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07fe4a: bf 0x8c07fe52
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07fe4c: mov.l 0x8c07fe60,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c07fe4e: jsr @r0 ; 0x8c07fe50: mov r14,r4 (delay)
            cpu.pr = 0x8c07fe52;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07fe52: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c07fe54: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07fe56: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07fe58: rts ; 0x8c07fe5a: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
