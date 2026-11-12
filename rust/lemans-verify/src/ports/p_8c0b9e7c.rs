//! 0x8c0b9e7c — clear a 36-byte state block, call the global array clearer,
//! then publish a byte flag and return it.
//!
//! ```text
//! push r14 ; sts.l pr,@-r15 ; r14 = r4 ; r1 = 0
//! [r14+20] = 0 ; r4 = [0x8c0b9eb0] (0x8c27a0b8)
//! [r14+4]  = 0 ; r0 = [0x8c0b9eb4] (0x8c0ba554)
//! [r14+24] = 0 ; [r14+8] = 0 ; [r14+28] = 0 ; [r14+12] = 0 ; [r14+32] = 0
//! jsr @r0 ; delay: [r14+16] = 0
//! r1 = [0x8c0b9eb0] (0x8c27a0b8) ; r0 = 1
//! mov.b r0,@r1 ; mov.b @r1,r0 ; extu.b r0,r0
//! mov.b r0,@r14 ; mov.b @r14,r0 ; extu.b r0,r0
//! lds.l @r15+,pr ; rts ; delay: mov.l @r15+,r14
//! ```
//!
//! The callee 0x8c0ba554 saves r12/r13/r14, clears four u32 slots at
//! [0x8c27a028] and [0x8c27a098] and four 24-byte structs at [0x8c27a038],
//! and leaves T clear (last `cmp/gt` is 4 > 4). Nothing later in this routine
//! touches T. The caller then writes byte 1 to 0x8c27a0b8, reads it back
//! (`mov.b` sign-extends, `extu.b` re-truncates), stores it at [r14] and
//! returns it, so final r0 = 1 and r1 = 0x8c27a0b8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b9e7c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0b9e7e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0b9e80: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0b9e82: mov #0,r1
    cpu.r[1] = 0;
    // 0x8c0b9e84: mov.l r1,@(20,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[1]);
    // 0x8c0b9e86: mov.l 0x8c0b9eb0,r4
    cpu.r[4] = mem.read_u32(0x8c0b9eb0).unwrap_or(0); // 0x8c27a0b8
    // 0x8c0b9e88: mov.l r1,@(4,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    // 0x8c0b9e8a: mov.l 0x8c0b9eb4,r0
    cpu.r[0] = mem.read_u32(0x8c0b9eb4).unwrap_or(0); // 0x8c0ba554
    // 0x8c0b9e8c: mov.l r1,@(24,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[1]);
    // 0x8c0b9e8e: mov.l r1,@(8,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    // 0x8c0b9e90: mov.l r1,@(28,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(28), cpu.r[1]);
    // 0x8c0b9e92: mov.l r1,@(12,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]);
    // 0x8c0b9e94: mov.l r1,@(32,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(32), cpu.r[1]);
    // 0x8c0b9e96: jsr @r0 ; 0x8c0b9e98: mov.l r1,@(16,r14) (delay slot)
    cpu.pr = 0x8c0b9e9a;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]);
    crate::registry::p_8c0ba554::run(cpu, mem);
    // 0x8c0b9e9a: mov.l 0x8c0b9eb0,r1
    cpu.r[1] = mem.read_u32(0x8c0b9eb0).unwrap_or(0); // 0x8c27a0b8
    // 0x8c0b9e9c: mov #1,r0
    cpu.r[0] = 1;
    // 0x8c0b9e9e: mov.b r0,@r1
    let _ = mem.write_u8(cpu.r[1], cpu.r[0] as u8);
    // 0x8c0b9ea0: mov.b @r1,r0
    cpu.r[0] = mem.read_u8(cpu.r[1]).unwrap_or(0) as i8 as i32 as u32;
    // 0x8c0b9ea2: extu.b r0,r0
    cpu.r[0] &= 0xFF;
    // 0x8c0b9ea4: mov.b r0,@r14
    let _ = mem.write_u8(cpu.r[14], cpu.r[0] as u8);
    // 0x8c0b9ea6: mov.b @r14,r0
    cpu.r[0] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32;
    // 0x8c0b9ea8: extu.b r0,r0
    cpu.r[0] &= 0xFF;
    // 0x8c0b9eaa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b9eac: rts ; 0x8c0b9eae: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
