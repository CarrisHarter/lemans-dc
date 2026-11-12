//! 0x8c165b08 — initialise an object header and free it when the signed
//! halfword flag is positive.
//!
//! ```text
//! 8c165b08  mov.l r14,@-r15        push r14
//! 8c165b0a  mov.l r13,@-r15        push r13
//! 8c165b0c  mov   r4,r14           r14 = object (returned pointer)
//! 8c165b0e  sts.l pr,@-r15         push pr
//! 8c165b10  mov   r5,r13           r13 = flag argument
//! 8c165b12  tst   r14,r14
//! 8c165b14  bt    0x8c165b34       NULL object: return it
//! 8c165b16  mov.l 0x8c165b40,r0    r0 = 0x8c1efe74
//! 8c165b18  tst   r14,r14
//! 8c165b1a  bt.s  0x8c165b28       never taken here (r14 != 0)
//! 8c165b1c  mov.l r0,@r14          delay: *object = 0x8c1efe74 on BOTH paths
//! 8c165b1e  mov.l 0x8c165b44,r0    r0 = 0x8c1e9198
//! 8c165b20  mov.l r0,@r14          *object = 0x8c1e9198
//! 8c165b22  mov.l 0x8c165b48,r0    r0 = 0x8c056bcc
//! 8c165b24  jsr   @r0
//! 8c165b26  mov   #0,r5            delay: count 0, so 0x8c056bcc never frees
//! 8c165b28  exts.w r13,r0          only the low half of the flag matters
//! 8c165b2a  cmp/pl r0
//! 8c165b2c  bf    0x8c165b34       flag <= 0: keep the object
//! 8c165b2e  mov.l 0x8c165b4c,r0    r0 = 0x8c01032c (free thunk)
//! 8c165b30  jsr   @r0
//! 8c165b32  mov   r14,r4           delay: free(object)
//! 8c165b34  mov   r14,r0           return the object
//! 8c165b36  lds.l @r15+,pr
//! 8c165b38  mov.l @r15+,r13
//! 8c165b3a  rts
//! 8c165b3c  mov.l @r15+,r14       delay
//! ```
//!
//! Both 0x8c1efe74 and 0x8c1e9198 are stored at *object; the second store wins
//! before 0x8c056bcc runs, and that routine overwrites the header again with
//! its 0x8c1e8f88 sentinel (its own free branch is skipped because r5 = 0).
//! Only `exts.w`'s low half of the flag decides the final free, e.g.
//! r5 = 0xffff0001 is positive while r5 = 0x00010000 is zero. The free path
//! ends with free's register residue in r1/r3..r6 and its T; the other paths
//! end with `tst`'s or `cmp/pl`'s flag. r14 and r13 come back from the frame and
//! r0 always ends as the saved object, so the caller-visible pointer is r4.
//!
//! Literals are read from the image rather than hardcoded, as the original
//! does, and every register the prologue saves is restored from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c165b08: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c165b0a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c165b0c: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c165b0e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c165b10: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c165b12: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c165b14: bt 0x8c165b34
    if cpu.r[14] != 0 {
        // 0x8c165b16: mov.l 0x8c165b40,r0
        cpu.r[0] = mem.read_u32(0x8c165b40).unwrap_or(0x8c1efe74);
        // 0x8c165b18: tst r14,r14
        cpu.set_t(cpu.r[14] == 0);
        // 0x8c165b1a: bt.s 0x8c165b28; 0x8c165b1c: mov.l r0,@r14 (delay)
        let skip_clear = cpu.t();
        let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
        if !skip_clear {
            // 0x8c165b1e: mov.l 0x8c165b44,r0
            cpu.r[0] = mem.read_u32(0x8c165b44).unwrap_or(0x8c1e9198);
            // 0x8c165b20: mov.l r0,@r14
            let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
            // 0x8c165b22: mov.l 0x8c165b48,r0
            cpu.r[0] = mem.read_u32(0x8c165b48).unwrap_or(0x8c056bcc);
            // 0x8c165b24: jsr @r0; 0x8c165b26: mov #0,r5 (delay slot)
            cpu.pr = 0x8c165b28;
            cpu.r[5] = 0;
            crate::registry::p_8c056bcc::run(cpu, mem);
        }
        // 0x8c165b28: exts.w r13,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[13] as u16 as i16 as i32 as u32;
        // 0x8c165b2a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c165b2c: bf 0x8c165b34
        if cpu.t() {
            // 0x8c165b2e: mov.l 0x8c165b4c,r0
            cpu.r[0] = mem.read_u32(0x8c165b4c).unwrap_or(0x8c01032c);
            // 0x8c165b30: jsr @r0; 0x8c165b32: mov r14,r4 (delay slot)
            cpu.pr = 0x8c165b34;
            cpu.r[4] = cpu.r[14];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c165b34: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c165b36: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c165b38: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c165b3a: rts; 0x8c165b3c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
