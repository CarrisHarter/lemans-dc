//! 0x8c090d5c — copy 0x90 bytes from `r4+0x178` to `r4+0x2c0`, then a
//! five-iteration delay loop.
//!
//! ```text
//! sts.l pr,@-r15
//! r0 = [0x8c090d80] = 0x2c0 ; r5 = r4
//! r1 = [0x8c090d84] = 0x178 ; r4 = r4 + 0x2c0
//! r6 = [0x8c090d88] = 0x90  ; r0 = [0x8c090d8c] = 0x8c010e84
//! jsr @r0 ; add r1,r5 (delay slot: src = r4 + 0x178)
//! r1 = 0 ; r0 = 10
//! loop: add #2,r1 ; cmp/gt r1,r0 ; bt loop
//! lds.l @r15+,pr ; rts ; nop
//! ```
//!
//! The memmove call always has `n = 0x90` and dst above src, so it runs its
//! backward arm; `dst|src|n` picks the byte/word/dword loop from the low bits
//! of `r4` (the two offsets are 4-aligned, so `r4 & 3` decides). The `bt` loop
//! is the only wrapper branch: r1 counts 2,4,6,8,10 and the tenth-unit compare
//! leaves T clear; no memory effects. The callee preserves r11-r13 through its
//! stack slots, so those end at their entry values, while r3 keeps the last
//! unit copied (the first source unit, sign-extended on the byte/word arms).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov.l @(0x8c090d80,pc),r0 ; 0x2c0
    cpu.r[0] = mem.read_u32(0x8c090d80).unwrap_or(0);
    // mov r4,r5
    cpu.r[5] = cpu.r[4];
    // mov.l @(0x8c090d84,pc),r1 ; 0x178
    cpu.r[1] = mem.read_u32(0x8c090d84).unwrap_or(0);
    // add r0,r4 ; dst = r4 + 0x2c0
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    // mov.l @(0x8c090d88,pc),r6 ; 0x90
    cpu.r[6] = mem.read_u32(0x8c090d88).unwrap_or(0);
    // mov.l @(0x8c090d8c,pc),r0 ; 0x8c010e84
    cpu.r[0] = mem.read_u32(0x8c090d8c).unwrap_or(0);
    // jsr @r0
    cpu.pr = 0x8c090d6e;
    // delay slot: add r1,r5 ; src = r4 + 0x178
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    crate::registry::p_8c010e84::run(cpu, mem); // memmove(dst, src, 0x90)

    // 0x8c090d6e: mov #0,r1 ; mov #10,r0
    cpu.r[1] = 0;
    cpu.r[0] = 10;
    // loop at 0x8c090d72: add #2,r1 ; cmp/gt r1,r0 ; bt 0x8c090d72
    loop {
        cpu.r[1] = cpu.r[1].wrapping_add(2);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r0
        if !cpu.t() {
            break;
        }
    }

    // 0x8c090d78: lds.l @r15+,pr ; rts ; nop
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
