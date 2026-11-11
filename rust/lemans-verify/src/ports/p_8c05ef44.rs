//! 0x8c05ef44 — conditional flag-byte update on the struct at r4.
//!
//! r0 = r4+28 ; fr0 = 1.0 (fldi1) ; fcmp/eq fr0,fr4 -> T = (fr4 == 1.0).
//! The bf.s delay slot `fmov fr4,@r0` ALWAYS stores fr4 to [r4+28].
//! T=1 path (fr4 == 1.0): byte [r4+4] = ((b & 0x8f) | 0) & 0xf0 | 0.
//! T=0 path: byte [r4+4] = ((b & 0x8f) | 0x20) & 0xf0 | 0x02.
//! Each path stores the byte twice (reload between and/or pairs, final
//! store in the rts delay slot), so the write log has 2 byte writes.
//! mov.b loads sign-extend into r0, which survives the and/or chain.
//!
//! Leftovers the C does not show: r0 = final sign-extended masked byte,
//! r3 = 0xffffff8f, r1 = 0xfffffff0, fr0 = 1.0, the float store to
//! [r4+28], and T set by fcmp/eq.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let r0 = base.wrapping_add(28); // mov r4,r0 ; add #28,r0
    cpu.fr[0] = 1.0; // fldi1 fr0
    let eq = cpu.fr[4] == cpu.fr[0];
    cpu.set_t(eq); // fcmp/eq fr0,fr4
    let _ = mem.write_f32(r0, cpu.fr[4]); // bf.s delay slot: fmov fr4,@r0 (always)
    // mov.b @(4,r4),r0 — sign-extends
    cpu.r[0] = mem.read_u8(base.wrapping_add(4)).unwrap() as i8 as i32 as u32;
    cpu.r[3] = 0xFFFFFF8F; // mov #-113,r3
    cpu.r[1] = 0xFFFFFFF0; // mov #-16,r1
    if eq {
        // 0x8c05ef50..: and r3,r0 ; or #0,r0 ; store ; reload ; and r1,r0 ; or #0,r0 ; rts ; store
        cpu.r[0] &= cpu.r[3];
        cpu.r[0] |= 0;
        let _ = mem.write_u8(base.wrapping_add(4), cpu.r[0] as u8);
        cpu.r[0] = mem.read_u8(base.wrapping_add(4)).unwrap() as i8 as i32 as u32;
        cpu.r[0] &= cpu.r[1];
        cpu.r[0] |= 0;
        let _ = mem.write_u8(base.wrapping_add(4), cpu.r[0] as u8); // rts delay slot
    } else {
        // 0x8c05ef66..: and r3,r0 ; or #32,r0 ; store ; reload ; and r1,r0 ; or #2,r0 ; rts ; store
        cpu.r[0] &= cpu.r[3];
        cpu.r[0] |= 0x20;
        let _ = mem.write_u8(base.wrapping_add(4), cpu.r[0] as u8);
        cpu.r[0] = mem.read_u8(base.wrapping_add(4)).unwrap() as i8 as i32 as u32;
        cpu.r[0] &= cpu.r[1];
        cpu.r[0] |= 0x02;
        let _ = mem.write_u8(base.wrapping_add(4), cpu.r[0] as u8); // rts delay slot
    }
}
