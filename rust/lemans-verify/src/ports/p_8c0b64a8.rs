//! 0x8c0b64a8 — replace every occurrence of byte `*r6` in [`r4`,`r5`) with `*r7`.
//!
//! ```text
//! cmp/eq r5,r4 ; bt 0x8c0b64c2   ; plain bt: NO delay slot
//! loop:
//! mov.b @r4,r1 ; mov.b @r6,r0 ; exts.b r1,r1 ; exts.b r0,r0
//! cmp/eq r0,r1 ; bf 0x8c0b64bc    ; plain bf: NO delay slot
//! mov.b @r7,r0 ; mov.b r0,@r4
//! add #1,r4 ; cmp/eq r5,r4 ; bf loop
//! rts ; nop
//! ```
//! Hidden side effects: r4 ends at r5, r1 keeps the sign-extended last text
//! byte, r0 keeps the last compare operand (the replacement byte on a match),
//! and T ends set (final `cmp/eq r5,r4` is true). `mov.b` loads sign-extend.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
    if cpu.r[4] == cpu.r[5] {
        return; // bt 0x8c0b64c2 ; rts ; nop
    }
    loop {
        let r1 = mem.read_u8(cpu.r[4]).unwrap() as u8 as i8 as i32 as u32;
        let r0raw = mem.read_u8(cpu.r[6]).unwrap() as u8 as i8 as i32 as u32;
        let r1 = r1 as u8 as i8 as i32 as u32; // exts.b r1,r1
        let r0 = r0raw as u8 as i8 as i32 as u32; // exts.b r0,r0
        cpu.r[1] = r1;
        cpu.set_t(r0 == r1); // cmp/eq r0,r1
        if r0 == r1 {
            let r0 = mem.read_u8(cpu.r[7]).unwrap() as u8 as i8 as i32 as u32; // mov.b @r7,r0
            let _ = mem.write_u8(cpu.r[4], r0 as u8); // mov.b r0,@r4
            cpu.r[0] = r0;
        } else {
            cpu.r[0] = r0;
        }
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
        cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
        if cpu.r[4] == cpu.r[5] {
            break; // bf not taken ; rts ; nop
        }
    }
}
