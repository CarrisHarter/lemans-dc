//! 0x8c152140 - stamp a vtable word, run the 0x8c1514f8 init, then free when positive.
//!
//! Null r13 exits after the prologue (T=1). Otherwise `[r13] = 0x8c1ef498` lands
//! in the `jsr` delay slot and 0x8c1514f8 runs with `r5 = 0` (so it never frees
//! the object itself). A positive sign-extended low half of r14 then frees r13.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[1] = mem.read_u32(0x8c152170).unwrap_or(0x8c1ef498); // mov.l 0x8c152170,r1
        cpu.r[5] = 0; // mov #0,r5
        cpu.r[0] = mem.read_u32(0x8c152174).unwrap_or(0x8c1514f8); // mov.l 0x8c152174,r0
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]); // jsr delay: mov.l r1,@r13
        cpu.pr = 0x8c152158;
        crate::registry::p_8c1514f8::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c152178).unwrap_or(0x8c01032c); // mov.l 0x8c152178,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c152164;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
