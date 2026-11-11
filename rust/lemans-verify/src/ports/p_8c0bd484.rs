//! 0x8c0bd484 — zero a 0x90-byte struct at r4 (36 words, offsets 0..=140),
//! then stamp the literal at 0x8c0bd528 (0x000186a0) into words 12..=23
//! (byte offsets 48..=92).
//!
//! CFG: straight-line, no branches. Prologue pushes r14/r13 and the epilogue
//! pops them, so r13/r14/r15 are net unchanged and no T bit is touched.
//!
//! Hidden side effects the C does not show: r0 := r4 (rts delay slot),
//! r1 := literal, r3 := r4+120, r5 := r4+72, r6 := 72, r7 := 68.
//! Store order: zeros at 0..=60, 64, 68, 72, 76..=92, 96..=120, 124..=140;
//! then the literal at 60, 64, 52, 56, 48, 68, 72, 76, 80, 84, 88, 92.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    // zero stores, in the original's exact order
    for off in (0..=60u32).step_by(4) {
        let _ = mem.write_u32(base + off, 0);
    }
    for off in [64u32, 68, 72] {
        let _ = mem.write_u32(base + off, 0);
    }
    for off in (76..=92u32).step_by(4) {
        let _ = mem.write_u32(base + off, 0);
    }
    for off in (96..=120u32).step_by(4) {
        let _ = mem.write_u32(base + off, 0);
    }
    for off in (124..=140u32).step_by(4) {
        let _ = mem.write_u32(base + off, 0);
    }
    // literal stores, original order
    let v = mem.read_u32(0x8C0BD528).unwrap();
    for off in [60u32, 64, 52, 56, 48, 68, 72, 76, 80, 84, 88, 92] {
        let _ = mem.write_u32(base + off, v);
    }
    cpu.r[0] = base; // rts delay slot: mov r4,r0
    cpu.r[1] = v;
    cpu.r[3] = base + 120;
    cpu.r[5] = base + 72;
    cpu.r[6] = 72;
    cpu.r[7] = 68;
}
