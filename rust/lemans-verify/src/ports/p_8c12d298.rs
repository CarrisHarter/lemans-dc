//! 0x8c12d298 — register an object: call 0x8c018fec(r4+12), publish the
//! pointer, tag byte +81, then zero the field block.
//!
//! ```text
//! push r14,r13; r14=r5; push pr; r13=r4
//! r0=[0x8c12d304] (0x8c018fec); jsr @r0; delay add #12,r4 (pr -> 0x8c12d2a8)
//! r0=[0x8c12d308] (0x8c2ab640); r1=0; [r0]=r13
//! [r13+81]=r14 (byte); [r13+4]=0; r0=64; [r13+8]=0
//! [r13+16,24,28,32,36,40,44,48,52,56,60]=0; [r13+r0=64]=0
//! r0=68 [r13+68]=0; r0=72 [r13+72]=0; [r13+0]=0
//! r0=76 [r13+76]=0; r0=84 [r13+84]=0; r0=80 byte [r13+80]=0
//! r0=88 [r13+88]=0; r0=92 [r13+92]=0; r0=100 [r13+100]=0
//! r0=104 [r13+104]=0; r0=96 [r13+96]=0
//! r0=r13; pop pr,r13; rts; delay pop r14
//! ```
//!
//! Straight line, no branches, T untouched. The jsr delay slot (`add #12,r4`)
//! runs before the callee, so 0x8c018fec publishes base+12. The zero stores
//! end +104 then +96 — out of order in the image, kept verbatim because the
//! write log is compared in order. Final state: r0=base, r1=0, r4=base+12;
//! r13/r14/r15/pr restored (stack traffic is excluded from the diff, so the
//! pushes/pops are elided and those registers are never touched).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    // jsr @r0 (0x8c018fec); delay slot runs first, pr would return to 0x8c12d2a8.
    cpu.r[4] = base.wrapping_add(12);
    crate::registry::p_8c018fec::run(cpu, mem);
    let slot = mem.read_u32(0x8c12d308).unwrap(); // 0x8c2ab640
    cpu.r[1] = 0;
    mem.write_u32(slot, base).unwrap();
    mem.write_u8(base.wrapping_add(81), cpu.r[5] as u8).unwrap();
    mem.write_u32(base.wrapping_add(4), 0).unwrap();
    cpu.r[0] = 64;
    mem.write_u32(base.wrapping_add(8), 0).unwrap();
    for off in [16u32, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60] {
        mem.write_u32(base.wrapping_add(off), 0).unwrap();
    }
    mem.write_u32(base.wrapping_add(cpu.r[0]), 0).unwrap(); // +64 via r0
    cpu.r[0] = 68;
    mem.write_u32(base.wrapping_add(68), 0).unwrap();
    cpu.r[0] = 72;
    mem.write_u32(base.wrapping_add(72), 0).unwrap();
    mem.write_u32(base, 0).unwrap(); // +0
    cpu.r[0] = 76;
    mem.write_u32(base.wrapping_add(76), 0).unwrap();
    cpu.r[0] = 84;
    mem.write_u32(base.wrapping_add(84), 0).unwrap();
    cpu.r[0] = 80;
    mem.write_u8(base.wrapping_add(80), 0).unwrap();
    cpu.r[0] = 88;
    mem.write_u32(base.wrapping_add(88), 0).unwrap();
    cpu.r[0] = 92;
    mem.write_u32(base.wrapping_add(92), 0).unwrap();
    cpu.r[0] = 100;
    mem.write_u32(base.wrapping_add(100), 0).unwrap();
    cpu.r[0] = 104;
    mem.write_u32(base.wrapping_add(104), 0).unwrap();
    cpu.r[0] = 96;
    mem.write_u32(base.wrapping_add(96), 0).unwrap();
    cpu.r[0] = base;
}
