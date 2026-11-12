//! 0x8c082154 — if (ptr != 0 && (short)len > 0) free(ptr); return ptr;
//!
//! ```text
//! push r14, pr
//! r14 = r4
//! tst   r14,r14
//! bt    0x8c08216a         ; ptr == 0 -> no call
//! exts.w r5,r0
//! cmp/pl r0
//! bf    0x8c08216a         ; (short)r5 <= 0 -> no call
//! mov.l 0x8c082174,r0      ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! nop                      ; delay slot
//! 0x8c08216a:
//! mov   r14,r0             ; return the original r4
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14          ; delay slot
//! ```
//!
//! r4 is pushed to the stack before it is copied into r14, so the return value
//! is the untouched argument. The T bit at exit is the last flag set on the
//! taken path: 1 when r4 == 0 (`tst`), 0 when r4 != 0 and the length check
//! failed (`cmp/pl`), and whatever `free` left when the call happens. Only the
//! nonzero/positive path touches r0..r13 and the FPU; `free` is called with
//! r4 unchanged and r5 still holding the caller's length.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if (cpu.r[0] as i32) > 0 {
            cpu.r[0] = mem.read_u32(0x8c082174).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c08216a; // jsr @r0 return address
            crate::registry::p_8c01032c::run(cpu, mem); // delay slot is a nop
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
