//! 0x8c0bdc80 — look up one 108-byte record and run the plane/ray test.
//!
//! ```text
//! push r14 ; push pr ; sub #8,r15
//! r0 = 108 ; mul.l r0,r7 ; r7 = macl        ; record index scaled
//! *(r15+4) = 0.0 ; *(r15+0) = 0.0           ; two float out-slots on the stack
//! r4 = *(r4+16) + r7                        ; record base table + index*108
//! push r15 (old) ; push r15+8               ; the two out-slot pointers
//! r7 = r6 ; r6 = r5 ; r5 = r4+20 ; r4 += 8  ; (delay slot) — callee args
//! jsr 0x8c0c0328
//! r15 += 8 ; extu.b r0,r0 ; tst r0,r0
//! r14 = -T ; r14 += 1 ; r0 = r14            ; return (callee result != 0)
//! r15 += 8 ; pop pr ; rts ; pop r14 (delay)
//! ```
//!
//! The two stack words hold the zero-initialised out-slots and the two pushed
//! words hold their addresses, which is what 0x8c0c0328 reads through `*r15`
//! and `*(r15+4)`. This function discards both stored floats; only the
//! callee's return value survives. `mul.l` leaves the scaled index in MACH/MACL
//! and `tst` leaves the final T bit; `r4..r7` end advanced by 4 by the callee.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15 ; add #-8,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    // mov #108,r0 ; mul.l r0,r7 ; sts macl,r7
    let p = (cpu.r[7] as i32 as i64) * 108i64;
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    cpu.r[0] = 108;
    cpu.r[7] = cpu.macl;
    // mov #4,r2 ; fldi0 fr0 ; add r15,r2 ; fmov fr0,@r2
    cpu.r[2] = cpu.r[15].wrapping_add(4);
    cpu.fr[0] = 0.0;
    let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits());
    // mov r15,r0 ; mov #0,r2 ; add r15,r2 ; fmov fr0,@r2
    cpu.r[0] = cpu.r[15];
    cpu.r[2] = cpu.r[15];
    let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits());
    // mov.l @(16,r4),r4 ; add r7,r4
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap();
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[7]);
    // mov.l r0,@-r15 ; mov r15,r0 ; add #8,r0 ; mov.l r0,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[0] = cpu.r[15];
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // mov r6,r7 ; mov r5,r6 ; mov r4,r5 ; mov.l 0x8c0bdccc,r0 ; add #20,r5
    cpu.r[7] = cpu.r[6];
    cpu.r[6] = cpu.r[5];
    cpu.r[5] = cpu.r[4].wrapping_add(20);
    cpu.r[0] = 0x8c0c0328; // literal at 0x8c0bdccc
    // jsr @r0 ; add #8,r4 (delay slot)
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.pr = 0x8c0bdcb6;
    crate::registry::p_8c0c0328::run(cpu, mem);
    // add #8,r15 ; extu.b r0,r0 ; tst r0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[0] &= 0xff;
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    // subc r14,r14 ; add #1,r14 ; mov r14,r0
    let (d, b1) = cpu.r[14].overflowing_sub(cpu.r[14]);
    let (d, b2) = d.overflowing_sub(cpu.t() as u32);
    cpu.r[14] = d;
    cpu.set_t(b1 || b2);
    cpu.r[14] = cpu.r[14].wrapping_add(1);
    cpu.r[0] = cpu.r[14];
    // add #8,r15 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14 (delay slot)
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
