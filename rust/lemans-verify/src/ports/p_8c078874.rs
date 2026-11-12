//! 0x8c078874 — release a reference on a handle, then free the handle itself
//! when the caller's count argument asks for it.
//!
//! ```text
//! 8c078874  mov.l r14,@-r15      push r14
//! 8c078876  mov.l r13,@-r15      push r13
//! 8c078878  mov   r5,r14         r14 = count arg
//! 8c07887a  sts.l pr,@-r15       push pr
//! 8c07887c  mov   r4,r13         r13 = handle
//! 8c07887e  tst   r13,r13
//! 8c078880  bt    0x8c078894     NULL handle: nothing to release
//! 8c078882  mov.l 0x8c0788a0,r0  0x8c018bd8 (release helper)
//! 8c078884  jsr   @r0
//! 8c078886  mov   #-1,r5         delay: the helper's count is always -1
//! 8c078888  exts.w r14,r0        r0 = sext16(count arg)
//! 8c07888a  cmp/pl r0
//! 8c07888c  bf    0x8c078894     count <= 0: keep the handle
//! 8c07888e  mov.l 0x8c0788a4,r0  0x8c01032c (free thunk)
//! 8c078890  jsr   @r0
//! 8c078892  mov   r13,r4         delay: free(handle)
//! 8c078894  mov   r13,r0         return the handle
//! 8c078896  lds.l @r15+,pr
//! 8c078898  mov.l @r15+,r13
//! 8c07889a  rts
//! 8c07889c  mov.l @r15+,r14      delay
//! ```
//!
//! The count the release helper receives is its own `short` parameter and it is
//! always -1 here: the delay slot at 0x8c078886 overwrites r5 after r14 has
//! already captured the caller's argument. So the helper's trailing
//! `free(handle)` arm never runs, and the caller's count is only read by the
//! `exts.w`/`cmp/pl` pair — the low 16 bits decide whether the handle itself is
//! freed. Both callees are delegated; r13/r14 are restored from the frame, so
//! the residues are the callee's.
//!
//! T ends as 1 on the NULL-handle early exit (`tst`), 0 after the `cmp/pl`
//! miss, and the free thunk's last flag when the handle is actually freed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c078874: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c078876: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c078878: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c07887a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07887c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c07887e: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c078880: bt 0x8c078894
    if cpu.r[13] != 0 {
        // 0x8c078882: mov.l 0x8c0788a0,r0
        cpu.r[0] = mem.read_u32(0x8c0788a0).unwrap_or(0); // 0x8c018bd8
        // 0x8c078884: jsr @r0 ; 0x8c078886: mov #-1,r5 (delay)
        cpu.pr = 0x8c078888;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);

        // 0x8c078888: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c07888a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c07888c: bf 0x8c078894
        if (cpu.r[0] as i32) > 0 {
            // 0x8c07888e: mov.l 0x8c0788a4,r0
            cpu.r[0] = mem.read_u32(0x8c0788a4).unwrap_or(0); // 0x8c01032c
            // 0x8c078890: jsr @r0 ; 0x8c078892: mov r13,r4 (delay)
            cpu.pr = 0x8c078894;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c078894: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c078896: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c078898: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c07889a: rts ; 0x8c07889c: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
