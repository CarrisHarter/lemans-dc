//! 0x8c0fd168 — init a 16-byte node: id from *0x8c202fa8, zero, byte from r5, next.
//!
//! ```text
//! add #-4,r15 ; mov.l r4,@r15          ; save param (stack, excluded)
//! mov.l 0x8c0fd190,r0                  ; r0 = 0x8c202fa8
//! mov #0,r4                            ; r4 = 0  (clobbers the argument!)
//! mov.l @r15,r6                        ; r6 = param
//! mov.l @r0,r0                         ; r0 = *0x8c202fa8
//! mov r6,r3 ; mov r6,r1
//! mov.l r0,@r6                         ; node[0] = id
//! add #8,r3 ; mov.l r4,@(4,r6)         ; node[1] = 0
//! add #4,r1 ; mov.b @r5,r0 ; mov.b r0,@r3  ; node[2] (byte at +8) = *r5
//! mov.l r4,@(12,r6)                    ; node[3] = 0 ...
//! mov r6,r0 ; mov.l r1,@(12,r6)        ; ... immediately overwritten with param+4
//! rts ; add #4,r15                     ; delay slot restores r15
//! ```
//! Hidden side effects: r4 = 0 (argument destroyed), r6 = param, r3 = param+8,
//! r1 = param+4, r0 = param (return), and TWO stores hit node+12 (0, then
//! param+4) — both are recorded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let param = cpu.r[4]; // mov.l r4,@r15
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[6] = param; // mov.l @r15,r6
    cpu.r[0] = mem.read_u32(0x8c0fd190).unwrap(); // 0x8c202fa8
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // mov.l @r0,r0
    cpu.r[3] = cpu.r[6];
    cpu.r[1] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]); // mov.l r0,@r6
    cpu.r[3] = cpu.r[3].wrapping_add(8); // add #8,r3
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[4]); // mov.l r4,@(4,r6)
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r0
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8); // mov.b r0,@r3
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[4]); // mov.l r4,@(12,r6)
    cpu.r[0] = cpu.r[6]; // mov r6,r0
    let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[1]); // mov.l r1,@(12,r6)
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot: add #4,r15
}
