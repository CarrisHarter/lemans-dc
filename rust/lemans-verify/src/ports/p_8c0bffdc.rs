//! 0x8c0bffdc — install a vtable, release owned resources, optionally free self.
//!
//! ```text
//! 8c0bffdc  mov.l r14,@-r15        ; save r14
//! 8c0bffde  mov.l r13,@-r15        ; save r13
//! 8c0bffe0  mov   r5,r14           ; r14 = (short) count argument
//! 8c0bffe2  sts.l pr,@-r15         ; save pr
//! 8c0bffe4  mov   r4,r13           ; r13 = object
//! 8c0bffe6  tst   r13,r13
//! 8c0bffe8  bt    0x8c0c0024       ; null object: return null, T=1
//! 8c0bffea  mov.l 0x8c0c0030,r0    ; r0 = 0x8c1eb43c (vtable)
//! 8c0bffec  mov   #0,r1
//! 8c0bffee  mov.l r0,@r13          ; *object = vtable
//! 8c0bfff0  mov   r13,r0
//! 8c0bfff2  add   #16,r0
//! 8c0bfff4  tst   r0,r0
//! 8c0bfff6  bt.s  0x8c0c0018       ; object == -16: skip the body
//! 8c0bfff8  mov.l r1,@(12,r13)     ; delay slot: object[3] = 0
//! 8c0bfffa  tst   r0,r0            ; (same test again, unreachable as taken)
//! 8c0bfffc  bt    0x8c0c0018
//! 8c0bfffe  mov.l @(24,r13),r4
//! 8c0c0000  tst   r4,r4
//! 8c0c0002  bt    0x8c0c000a
//! 8c0c0004  mov.l 0x8c0c0034,r0    ; r0 = 0x8c01032c (free thunk)
//! 8c0c0006  jsr   @r0              ; free(object[6])
//! 8c0c0008  nop
//! 8c0c000a  mov   r13,r4
//! 8c0c000c  add   #16,r4
//! 8c0c000e  tst   r4,r4
//! 8c0c0010  bt    0x8c0c0018
//! 8c0c0012  mov.l 0x8c0c0038,r0    ; r0 = 0x8c01e078
//! 8c0c0014  jsr   @r0              ; 0x8c01e078(object + 16, 0)
//! 8c0c0016  mov   #0,r5            ; delay slot
//! 8c0c0018  exts.w r14,r0          ; r0 = (int16) argument
//! 8c0c001a  cmp/pl r0
//! 8c0c001c  bf    0x8c0c0024       ; count <= 0: keep the object
//! 8c0c001e  mov.l 0x8c0c0034,r0    ; r0 = 0x8c01032c (free thunk)
//! 8c0c0020  jsr   @r0              ; free(object)
//! 8c0c0022  mov   r13,r4           ; delay slot
//! 8c0c0024  mov   r13,r0           ; return the object
//! 8c0c0026  lds.l @r15+,pr
//! 8c0c0028  mov.l @r15+,r13
//! 8c0c002a  rts
//! 8c0c002c  mov.l @r15+,r14        ; delay slot
//! ```
//!
//! `r13` and `r14` are saved, so both end at their entry values even though the
//! body assigns the object to `r13` and the count to `r14`; the returned `r0`
//! is the entry `r4`. `r14` holds the whole `r5`, but only its low half is
//! sign-extended by the `exts.w`. T ends as the `cmp/pl` set it on the
//! non-positive path, and as the last callee left it when `free(object)` runs.
//! The `0x8c0c0018` landing point skips the `mov #0,r5` delay slot, so `r5`
//! keeps its incoming value when the body is skipped.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0bffdc: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0bffde: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0bffe0: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0bffe2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0bffe4: mov r4,r13
    cpu.r[13] = cpu.r[4];

    // 0x8c0bffe6: tst r13,r13 ; 0x8c0bffe8: bt 0x8c0c0024
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        // 0x8c0bffea: mov.l 0x8c0c0030,r0
        cpu.r[0] = mem.read_u32(0x8c0c0030).unwrap_or(0x8c1eb43c);
        // 0x8c0bffec: mov #0,r1
        cpu.r[1] = 0;
        // 0x8c0bffee: mov.l r0,@r13
        let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
        // 0x8c0bfff0: mov r13,r0 ; 0x8c0bfff2: add #16,r0
        cpu.r[0] = cpu.r[13].wrapping_add(16);
        // 0x8c0bfff4: tst r0,r0 ; 0x8c0bfff6: bt.s 0x8c0c0018
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c0bfff8: mov.l r1,@(12,r13) — delay slot, runs either way
        let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[1]);
        if !cpu.t() {
            // 0x8c0bfffa: tst r0,r0 ; 0x8c0bfffc: bt 0x8c0c0018
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                // 0x8c0bfffe: mov.l @(24,r13),r4
                cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(24)).unwrap_or(0);
                // 0x8c0c0000: tst r4,r4 ; 0x8c0c0002: bt 0x8c0c000a
                cpu.set_t(cpu.r[4] == 0);
                if !cpu.t() {
                    // 0x8c0c0004: mov.l 0x8c0c0034,r0
                    cpu.r[0] = mem.read_u32(0x8c0c0034).unwrap_or(0x8c01032c);
                    // 0x8c0c0006: jsr @r0 ; delay slot is a nop
                    cpu.pr = 0x8c0c000a;
                    crate::registry::p_8c01032c::run(cpu, mem);
                }
                // 0x8c0c000a: mov r13,r4 ; 0x8c0c000c: add #16,r4
                cpu.r[4] = cpu.r[13].wrapping_add(16);
                // 0x8c0c000e: tst r4,r4 ; 0x8c0c0010: bt 0x8c0c0018
                cpu.set_t(cpu.r[4] == 0);
                if !cpu.t() {
                    // 0x8c0c0012: mov.l 0x8c0c0038,r0
                    cpu.r[0] = mem.read_u32(0x8c0c0038).unwrap_or(0x8c01e078);
                    // 0x8c0c0014: jsr @r0 ; 0x8c0c0016: mov #0,r5 (delay slot)
                    cpu.r[5] = 0;
                    cpu.pr = 0x8c0c0018;
                    crate::registry::p_8c01e078::run(cpu, mem);
                }
            }
        }
        // 0x8c0c0018: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c0c001a: cmp/pl r0 ; 0x8c0c001c: bf 0x8c0c0024
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            // 0x8c0c001e: mov.l 0x8c0c0034,r0
            cpu.r[0] = mem.read_u32(0x8c0c0034).unwrap_or(0x8c01032c);
            // 0x8c0c0020: jsr @r0 ; 0x8c0c0022: mov r13,r4 (delay slot)
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c0c0024;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0c0024: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0c0026: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c0028: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c002a: rts ; 0x8c0c002c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
