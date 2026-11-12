//! 0x8c0b92f4 — append `len` bytes to the global buffer.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; r14 = len
//! r3 = [0x8c26d0d0] (used) ; r1 = used + len ; r0 = [0x8c26d0c0] (capacity)
//! cmp/ge r1,r0 ; bf fail
//! r5 = src ; r6 = len ; r4 = [0x8c26d0d8] (base) ; r0 = 0x8c07093c
//! jsr @r0 ; add r3,r4            ; copy(base + used, src, len)
//! r3 = 0x8c26d0d0 ; r1 = *r3 + len ; *r3 = r1 (bra delay slot) ; r0 = 0
//! pop pr, r14 ; rts
//! fail: r0 = -1 ; pop pr, r14 ; rts
//! ```
//!
//! Returns 0 after advancing the cursor, or -1 without touching the buffer
//! when `cmp/ge` says `capacity < used + len` (signed). The copy is
//! 0x8c07093c, whose delay-slot semantics also leave `r5`/`r6` at `src + len`
//! and `r2` sign-extended from the last byte (or unchanged for `len == 0`).

use sh4_oracle::{Cpu, Memory};

const CAPACITY: u32 = 0x8c26d0c0;
const USED: u32 = 0x8c26d0d0;
const BASE: u32 = 0x8c26d0d8;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    let len = cpu.r[5];
    cpu.r[14] = len; // mov r5,r14
    let used = mem.read_u32(USED).unwrap_or(0);
    cpu.r[3] = used; // mov.l [0x8c26d0d0],r3
    let capacity = mem.read_u32(CAPACITY).unwrap_or(0);
    cpu.r[1] = used.wrapping_add(len); // mov r3,r1 ; add r14,r1
    cpu.r[0] = capacity; // mov.l [0x8c26d0c0],r0
    let fits = (capacity as i32) >= (cpu.r[1] as i32); // cmp/ge r1,r0
    cpu.set_t(fits);

    if fits {
        let base = mem.read_u32(BASE).unwrap_or(0);
        cpu.r[5] = cpu.r[4]; // mov r4,r5 (src)
        cpu.r[6] = len; // mov r14,r6 (len)
        cpu.r[4] = base; // mov.l [0x8c26d0d8],r4
        cpu.r[0] = 0x8c07093c; // mov.l 0x8c0b9338,r0
        cpu.pr = 0x8c0b9318; // jsr @r0
        cpu.r[4] = cpu.r[4].wrapping_add(used); // delay slot: add r3,r4
        crate::registry::p_8c07093c::run(cpu, mem);

        cpu.r[3] = USED; // mov.l 0x8c0b932c,r3
        cpu.r[0] = 0; // mov #0,r0
        cpu.r[1] = mem.read_u32(USED).unwrap_or(0).wrapping_add(len); // mov.l @r3,r1 ; add r14,r1
        let _ = mem.write_u32(USED, cpu.r[1]); // bra delay slot: mov.l r1,@r3
    } else {
        cpu.r[0] = u32::MAX; // mov #-1,r0
    }

    // Epilogue: lds.l @r15+,pr ; rts (delay slot: mov.l @r15+,r14)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
