//! 0x8c0d43c0 — destroy an array of 0x24-byte handle slots.
//!
//! ```text
//! 8c0d43c0  mov.l r14,@-r15       push r14
//! 8c0d43c2  mov.l r13,@-r15       push r13
//! 8c0d43c4  mov.l r12,@-r15       push r12
//! 8c0d43c6  mov.l r11,@-r15       push r11
//! 8c0d43c8  sts.l pr,@-r15        push pr
//! 8c0d43ca  mov   r4,r11          r11 = descriptor
//! 8c0d43cc  mov.l @(8,r11),r13    r13 = descriptor->array
//! 8c0d43ce  tst   r13,r13
//! 8c0d43d0  bt    0x8c0d43fa      array == NULL: nothing to do
//! 8c0d43d2  mov.l @(4,r11),r0     r0 = descriptor->count
//! 8c0d43d4  mov   r0,r14          r14 = count
//! 8c0d43d6  shll2 r14             count*4
//! 8c0d43d8  shll  r14             count*8
//! 8c0d43da  add   r0,r14          count*9
//! 8c0d43dc  shll2 r14             count*36
//! 8c0d43de  add   r13,r14         r14 = array + count*0x24 (end)
//! 8c0d43e0  cmp/hi r13,r14        T = end > array (unsigned)
//! 8c0d43e2  bf    0x8c0d43f4      empty/wrapped range: free the array
//! 8c0d43e4  mov   #-1,r12         r12 = 0xffffffff
//! 8c0d43e6  mov.l 0x8c0d4408,r0   r0 = 0x8c0d357c
//! 8c0d43e8  mov   r12,r5          r5 = -1
//! 8c0d43ea  jsr   @r0
//! 8c0d43ec  mov   r13,r4          delay: release slot at r13
//! 8c0d43ee  add   #36,r13         next slot
//! 8c0d43f0  cmp/hi r13,r14
//! 8c0d43f2  bt    0x8c0d43e6      still inside: loop
//! 8c0d43f4  mov.l 0x8c0d440c,r0   r0 = 0x8c01032c (free thunk)
//! 8c0d43f6  jsr   @r0
//! 8c0d43f8  mov.l @(8,r11),r4     delay: free(descriptor->array)
//! 8c0d43fa  lds.l @r15+,pr        epilogue
//! 8c0d43fc  mov.l @r15+,r11
//! 8c0d43fe  mov.l @r15+,r12
//! 8c0d4400  mov.l @r15+,r13
//! 8c0d4402  rts
//! 8c0d4404  mov.l @r15+,r14       delay
//! ```
//!
//! Each 0x24-byte slot is itself a handle for 0x8c0d357c, called with count -1
//! (i.e. "release one reference, never free the handle outright"). The range
//! test is an unsigned `cmp/hi`, so a count whose 36-byte stride wraps below
//! the array pointer skips the loop but still frees the array.
//!
//! The array pointer is re-read from the descriptor for the final free, so a
//! callee that stored into the descriptor would be visible here. r11..r14 and
//! pr are pushed by the prologue and popped by the epilogue, so they end at
//! their entry values; on the `array == NULL` exit T is 1 from the `tst`, on
//! every other exit the last flag-setting instruction is inside `free`.

use sh4_oracle::{Cpu, Memory};

/// 0x8c0d4408 holds 0x8c0d357c, 0x8c0d440c holds 0x8c01032c.
const RELEASE_HANDLE_PTR: u32 = 0x8c0d4408;
const FREE_THUNK_PTR: u32 = 0x8c0d440c;
const RELEASE_HANDLE: u32 = 0x8c0d357c;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d43c0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0d43c2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0d43c4: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0d43c6: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c0d43c8: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0d43ca: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c0d43cc: mov.l @(8,r11),r13
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
    // 0x8c0d43ce: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0d43d0: bt 0x8c0d43fa
    if cpu.r[13] != 0 {
        // 0x8c0d43d2: mov.l @(4,r11),r0
        let count = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
        // 0x8c0d43d4..0x8c0d43de: r14 = array + count*0x24
        cpu.r[14] = count.wrapping_mul(0x24).wrapping_add(cpu.r[13]);
        // 0x8c0d43e0: cmp/hi r13,r14
        cpu.set_t(cpu.r[14] > cpu.r[13]);
        // 0x8c0d43e2: bf 0x8c0d43f4
        if cpu.r[14] > cpu.r[13] {
            // 0x8c0d43e4: mov #-1,r12
            cpu.r[12] = 0xffff_ffff;
            loop {
                // 0x8c0d43e6: mov.l 0x8c0d4408,r0
                cpu.r[0] = mem.read_u32(RELEASE_HANDLE_PTR).unwrap_or(RELEASE_HANDLE);
                // 0x8c0d43e8: mov r12,r5
                cpu.r[5] = cpu.r[12];
                // 0x8c0d43ea: jsr @r0 ; 0x8c0d43ec: mov r13,r4 (delay slot)
                cpu.pr = 0x8c0d43ee;
                cpu.r[4] = cpu.r[13];
                crate::registry::p_8c0d357c::run(cpu, mem);
                // 0x8c0d43ee: add #36,r13
                cpu.r[13] = cpu.r[13].wrapping_add(0x24);
                // 0x8c0d43f0: cmp/hi r13,r14
                cpu.set_t(cpu.r[14] > cpu.r[13]);
                // 0x8c0d43f2: bt 0x8c0d43e6
                if !(cpu.r[14] > cpu.r[13]) {
                    break;
                }
            }
        }
        // 0x8c0d43f4: mov.l 0x8c0d440c,r0
        cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
        // 0x8c0d43f6: jsr @r0 ; 0x8c0d43f8: mov.l @(8,r11),r4 (delay slot)
        cpu.pr = 0x8c0d43fa;
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    // 0x8c0d43fa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d43fc: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d43fe: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d4400: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d4402: rts ; 0x8c0d4404: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
