//! 0x8c0b8fb4 — lazy one-time init guarded by a flag byte; returns the slot.
//!
//! ```text
//! mov.l 0x8c0b8fd0,r4   ; r4 = flag ptr (0x8c279f18)
//! mov.b @r4,r0 ; exts.b r0,r0
//! tst r0,r0 ; bf 0x8c0b8fca     ; skip if flag != 0
//! mov.l 0x8c0b8fd4,r3   ; r3 = slot ptr (0x8c279f20)
//! mov #0,r1 ; mov #1,r0
//! mov.b r1,@r3 ; mov.l r1,@(4,r3) ; mov.b r0,@r4
//! 0x8c0b8fca: mov.l 0x8c0b8fd4,r0 ; rts ; nop
//! ```
//! Hidden side effects: on the init path r3/r1/r0 are clobbered (r1 = 0,
//! r0 = 1 mid-function but reloaded as the slot pointer at the end), the
//! flag byte is set to 1, and T = (flag == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8C0B8FD0).unwrap();
    let mut r0 = mem.read_u8(cpu.r[4]).unwrap() as u8 as i8 as i32 as u32;
    r0 = r0 as u8 as i8 as i32 as u32; // exts.b r0,r0
    cpu.r[0] = r0;
    cpu.set_t(r0 == 0); // tst r0,r0
    if r0 == 0 {
        cpu.r[3] = mem.read_u32(0x8C0B8FD4).unwrap();
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[0] = 1; // mov #1,r0
        let _ = mem.write_u8(cpu.r[3], cpu.r[1] as u8); // mov.b r1,@r3
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[1]); // mov.l r1,@(4,r3)
        let _ = mem.write_u8(cpu.r[4], cpu.r[0] as u8); // mov.b r0,@r4
    }
    cpu.r[0] = mem.read_u32(0x8C0B8FD4).unwrap();
    // rts ; nop
}
