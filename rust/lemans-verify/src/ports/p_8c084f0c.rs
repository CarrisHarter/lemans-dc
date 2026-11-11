//! 0x8c084f0c — fill loop: replicate the 32-byte record at r6 into every
//! 32-byte slot of [r4, r5).
//!
//! Per iteration, from the fixed source r6: byte at +0, dwords at +4,+8,+12,
//! +16,+20,+24 (via fmov), word at +28; destination r4 advances by 32.
//! All `bt`/`bf` are non-delayed, so the loop body is skipped cleanly when
//! r4 == 0 (`tst r4,r4; bt`) while r4 still advances.
//!
//! Side effects the decompiled C does not show:
//!   * r8..r14 are pushed in the prologue and popped in the epilogue (the last
//!     pop is the `rts` delay slot), so their final values are the seeds; the
//!     port keeps the scratch pointers in locals instead;
//!   * r1 = r6+4 and r3 = r6+20 are written *before* the loop, so they are
//!     clobbered even when the body never runs (the r4==r5 short-circuit
//!     skips them, but the r4==0 null path does not);
//!   * r7 ends at dst+20 and r0 at the sign-extended source word at r6+28 of
//!     the last executed iteration; fr0 holds the source dword at r6+24;
//!   * T is always 1 on exit: both the initial `cmp/eq r5,r4` short-circuit
//!     and the loop-exit `cmp/eq r5,r4` set it;
//!   * r15 is net unchanged (seven pushes, seven pops) — stack window writes
//!     are excluded from comparison anyway.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4 (0x8c084f1a)
    if cpu.r[4] == cpu.r[5] {
        return; // bt 0x8c084f7a
    }
    let src = cpu.r[6];
    cpu.r[1] = src.wrapping_add(4); // 0x8c084f1e + 0x8c084f2a
    let r10 = src.wrapping_add(8);
    let r12 = src.wrapping_add(12);
    let r14 = src.wrapping_add(16);
    cpu.r[3] = src.wrapping_add(20); // 0x8c084f26 + 0x8c084f32
    let r8 = src.wrapping_add(24);
    let r1 = cpu.r[1];
    let r3 = cpu.r[3];
    let mut r4 = cpu.r[4];
    loop {
        cpu.set_t(r4 == 0); // tst r4,r4 (0x8c084f36)
        if r4 != 0 {
            let mut r7 = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32; // mov.b @r6,r7 (0x8c084f3a)
            r7 &= 0xFF; // extu.b r7,r7 (0x8c084f40)
            let _ = mem.write_u8(r4, r7 as u8); // mov.b r7,@r4 (0x8c084f42)
            cpu.fr[0] = mem.read_f32(r1).unwrap_or(0.0); // fmov @r1,fr0 (0x8c084f46)
            let _ = mem.write_f32(r4.wrapping_add(4), cpu.fr[0]); // 0x8c084f4c
            cpu.fr[0] = mem.read_f32(r10).unwrap_or(0.0); // 0x8c084f50
            let _ = mem.write_f32(r4.wrapping_add(8), cpu.fr[0]); // 0x8c084f56
            cpu.fr[0] = mem.read_f32(r12).unwrap_or(0.0); // 0x8c084f5a
            let _ = mem.write_f32(r4.wrapping_add(12), cpu.fr[0]); // 0x8c084f60
            cpu.fr[0] = mem.read_f32(r14).unwrap_or(0.0); // 0x8c084f64
            let _ = mem.write_f32(r4.wrapping_add(16), cpu.fr[0]); // 0x8c084f66
            cpu.fr[0] = mem.read_f32(r3).unwrap_or(0.0); // 0x8c084f68
            let _ = mem.write_f32(r4.wrapping_add(20), cpu.fr[0]); // 0x8c084f6a
            cpu.fr[0] = mem.read_f32(r8).unwrap_or(0.0); // 0x8c084f6c
            let _ = mem.write_f32(r4.wrapping_add(24), cpu.fr[0]); // 0x8c084f6e
            let w = mem.read_u16(src.wrapping_add(28)).unwrap_or(0) as i16 as i32 as u32; // 0x8c084f70
            let _ = mem.write_u16(r4.wrapping_add(28), w as u16); // 0x8c084f72
            cpu.r[0] = w;
            cpu.r[7] = r4.wrapping_add(20); // mov r4,r7 + add #20,r7 (0x8c084f4e/0x8c084f5e)
        }
        r4 = r4.wrapping_add(32); // 0x8c084f74
        cpu.r[4] = r4;
        cpu.set_t(r4 == cpu.r[5]); // cmp/eq r5,r4 (0x8c084f76)
        if r4 == cpu.r[5] {
            break; // bf not taken -> epilogue
        }
    }
    // Epilogue pops r8..r13 and the rts delay slot pops r14 (all seeds); r15 net 0.
}
