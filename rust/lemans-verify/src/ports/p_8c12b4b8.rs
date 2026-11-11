//! 0x8c12b4b8 — count elements: returns how many times base+4k != end, i.e. the
//! element count at `(*param + 0x20) + 4` when `+0x28` is the array base.
//!
//! Original:
//!   mov.l @r4,r5; add #32,r5; mov.l @(8,r5),r4; mov.l @(4,r5),r3; mov r4,r1;
//!   shll2 r3; add r3,r1; cmp/eq r4,r1; bt.s done; mov #0,r0;
//! loop: mov.l @(4,r5),r1; add #4,r4; mov.l @(8,r5),r3; shll2 r1; add r1,r3;
//!   cmp/eq r4,r3; bf.s loop; add #1,r0; done: rts; nop
//!
//! Side effects the C does not show: r5 := *r4 + 32 sticks; r4/r3/r1 are
//! clobbered (r4 ends at base+4*count, r3 at the same end, r1 at 4*count);
//! r0 is incremented in the bf.s delay slot, so it runs on every iteration
//! including the last; T is left at 1 (both exits come from a taken/untaken
//! cmp/eq with equality); shll2 leaves T alone. The header words at r5+4/r5+8
//! are re-read every iteration.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r5
    cpu.r[5] = cpu.r[5].wrapping_add(32); // add #32,r5
    cpu.r[4] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0); // mov.l @(8,r5),r4
    cpu.r[3] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0); // mov.l @(4,r5),r3
    cpu.r[1] = cpu.r[4]; // mov r4,r1
    cpu.r[3] = cpu.r[3] << 2; // shll2 r3 (T untouched)
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1
    cpu.set_t(cpu.r[1] == cpu.r[4]); // cmp/eq r4,r1
    cpu.r[0] = 0; // bt.s delay slot: mov #0,r0 (runs taken or not)
    if cpu.r[1] == cpu.r[4] {
        return; // bt.s taken -> rts; nop
    }
    loop {
        cpu.r[1] = mem.read_u32(cpu.r[5] + 4).unwrap_or(0); // mov.l @(4,r5),r1
        cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
        cpu.r[3] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0); // mov.l @(8,r5),r3
        cpu.r[1] = cpu.r[1] << 2; // shll2 r1 (T untouched)
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]); // add r1,r3
        cpu.set_t(cpu.r[3] == cpu.r[4]); // cmp/eq r4,r3
        cpu.r[0] = cpu.r[0].wrapping_add(1); // bf.s delay slot: add #1,r0 (always)
        if cpu.r[3] == cpu.r[4] {
            return; // bf.s not taken -> fall through to rts; nop
        }
    }
}
