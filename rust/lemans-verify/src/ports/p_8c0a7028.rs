//! 0x8c0a7028 — clear then set bits of `*(u32*)(*(u32*)(param_1 + 0x18c) + 16)`.
//!
//! Saves `r14`/`r13`/`pr`, loads the object pointer from `param_1 + 0x18c`,
//! and if it is non-null calls 0x8c05d274 twice: first with `r5 = 2` (clear the
//! bit-1 mask), then — reloading the pointer, which the first call may have
//! altered — with `r5 = 1` (set bit 0). `param_2` (r5 at entry) is forwarded in
//! r6 both times. Returns with the callee's final r0/r1/T, r4 as last reloaded.
//!
//! ```text
//! mov.l r14,@-r15   ; mov.l r13,@-r15
//! mov   r5,r14      ; sts.l pr,@-r15   ; mov r4,r13
//! mov.l 0x18c,r0    ; mov.l @(r0,r13),r4
//! tst   r4,r4       ; bt 0x8c0a704e
//! mov.l @0x8c0a705c,r0 ; mov r14,r6 ; jsr @r0 ; mov #2,r5
//! mov.l 0x18c,r0    ; mov r14,r6 ; mov.l @(r0,r13),r4
//! mov.l @0x8c0a705c,r0 ; jsr @r0 ; mov #1,r5
//! lds.l @r15+,pr    ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! The target literal at 0x8c0a705c is 0x8c05d274; the offset literal at
//! 0x8c0a7058 is 0x18c. The second pointer load matters: a callee store can
//! land on `param_1 + 0x18c` itself.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[0] = 0x0000_018c;
    cpu.r[4] = mem
        .read_u32(cpu.r[13].wrapping_add(cpu.r[0]))
        .unwrap_or(0);
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[0] = 0x8c05_d274; // literal at 0x8c0a705c
        cpu.r[6] = cpu.r[14];
        cpu.r[5] = 2; // delay slot of `jsr`
        crate::registry::p_8c05d274::run(cpu, mem);
        cpu.r[0] = 0x0000_018c;
        cpu.r[6] = cpu.r[14];
        cpu.r[4] = mem
            .read_u32(cpu.r[13].wrapping_add(cpu.r[0]))
            .unwrap_or(0); // reload: the first call may have stored here
        cpu.r[0] = 0x8c05_d274; // literal at 0x8c0a705c
        cpu.r[5] = 1; // delay slot of `jsr`
        crate::registry::p_8c05d274::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
