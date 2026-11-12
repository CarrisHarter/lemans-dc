//! 0x8c06cbec — initialise an object: install vtable `0x8c1e94e8`, clear
//! +12..+24 through 0x8c0b4ff0, then store 0.5f at +4 and +8.
//!
//! ```text
//! mov.l r14,@-r15 / sts.l pr,@-r15   ; save frame
//! r14 = r4; r4 += 12
//! jsr  0x8c0b4ff0                    ; zeroes +12, +16, +20
//! mov.l r1,@r14                      ; delay slot: *this = vtable
//! *(f32 *)(this + 24) = 0.0
//! fr0 = *(f32 *)0x8c06cc24           ; 0.5
//! *(f32 *)(this + 4) = fr0; *(f32 *)(this + 8) = fr0
//! return this
//! ```
//!
//! Leftovers: r0=this, r1=this+8, r3=this+4, r4=this+12, fr0=0.5. r14/r15 and
//! pr are restored by the epilogue. The vtable store sits in the `jsr` delay
//! slot, so it lands before any callee write.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4];
    cpu.r[1] = mem.read_u32(0x8c06cc1c).unwrap_or(0x8c1e94e8);
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.pr = 0x8c06cbfc; // jsr 0x8c0b4ff0
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]); // delay slot: *this = vtable
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = cpu.r[14].wrapping_add(24);
    cpu.fr[0] = 0.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[3] = cpu.r[14];
    cpu.r[0] = 0x8c06cc24; // mova
    cpu.fr[0] = mem.read_f32(0x8c06cc24).unwrap_or(0.0);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[1] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = cpu.r[1].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[14];

    // lds.l @r15+,pr ; rts ; delay: mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
