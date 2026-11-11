//! 0x8c026a50 — build a 16-byte header at param_1 and return it.
//!
//! Layout written: { *PTR_DAT_8c026a78, 0, *(u8 *)param_2, param_1+4 } —
//! note the +0xc field is stored twice: first 0, then r1 = base+4.
//!
//! Side effects the C hides: the struct base is saved on the stack and
//! recovered into r6, then copied into r1/r3, so those end as base,
//! base+4 and base+8; r4 is zeroed (the 0 stored at +4 comes from r4 after
//! `mov #0,r4`); r0 passes through the byte load before ending as base;
//! r15 nets out (push plus the rts delay-slot pop).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let ptr = mem.read_u32(0x8c026a78).unwrap_or(0); // PTR_DAT_8c026a78
    let global = mem.read_u32(ptr).unwrap_or(0);
    cpu.r[6] = base; // 0x8c026a58 mov.l @r15,r6 (saved r4)
    cpu.r[4] = 0; // 0x8c026a56 mov #0,r4
    cpu.r[3] = base.wrapping_add(8); // mov r6,r3 ; add #8,r3
    cpu.r[1] = base.wrapping_add(4); // mov r6,r1 ; add #4,r1
    let _ = mem.write_u32(base, global); // 0x8c026a60
    let _ = mem.write_u32(base.wrapping_add(4), 0); // 0x8c026a64
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = b as i8 as i32 as u32; // 0x8c026a68 mov.b @r5,r0 (sign-extended)
    let _ = mem.write_u8(base.wrapping_add(8), b); // 0x8c026a6a
    let _ = mem.write_u32(base.wrapping_add(12), 0); // 0x8c026a6c
    cpu.r[0] = base; // 0x8c026a6e mov r6,r0
    let _ = mem.write_u32(base.wrapping_add(12), cpu.r[1]); // 0x8c026a70
}
