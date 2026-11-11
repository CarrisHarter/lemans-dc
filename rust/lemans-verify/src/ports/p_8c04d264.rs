//! 0x8c04d264 — linear search: return the index of `param_2` in the int array
//! at `*(int*)(param_1+4)`, or -1 if `param_1->count` (at +0xc) entries hold no match.
//!
//! ```text
//! mov.l @(12,r4),r6
//! mov   #0,r3
//! bra   0x8c04d27c
//! mov   r3,r0          ; delay slot: r0 = 0 (byte offset)
//! 0x8c04d26c:
//! mov.l @(4,r4),r1
//! mov.l @(r0,r1),r1
//! cmp/eq r5,r1
//! bf    0x8c04d278
//! rts
//! mov   r3,r0          ; delay slot: return index
//! 0x8c04d278:
//! add   #4,r0
//! add   #1,r3
//! 0x8c04d27c:
//! cmp/hi r3,r6
//! bt    0x8c04d26c
//! rts
//! mov   #-1,r0         ; delay slot: not found
//! ```
//! Hidden side effects: r6 holds the count, r3 the running index, r1 the last
//! element loaded, r0 the byte offset; the T bit is 1 on a match and 0 on the
//! -1 exit because it is whatever the last cmp left behind.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(0xc)).unwrap();
    cpu.r[3] = 0;
    cpu.r[0] = 0; // delay slot of `bra`
    loop {
        cpu.set_t(cpu.r[6] > cpu.r[3]); // cmp/hi r3,r6 (unsigned)
        if cpu.r[6] <= cpu.r[3] {
            cpu.r[0] = 0xFFFF_FFFF; // delay slot of `rts`: mov #-1,r0
            return;
        }
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
        cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(cpu.r[0])).unwrap();
        cpu.set_t(cpu.r[1] == cpu.r[5]); // cmp/eq r5,r1
        if cpu.r[1] == cpu.r[5] {
            cpu.r[0] = cpu.r[3]; // delay slot of `rts`
            return;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(4); // add #4,r0
        cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
    }
}
