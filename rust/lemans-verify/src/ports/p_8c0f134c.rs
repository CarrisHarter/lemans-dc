//! 0x8c0f134c — forward dword copy loop: copy [r4,r5) to r6, return end dst.
//!
//! ```text
//! cmp/eq r5,r4 ; bt 0x8c0f1360      ; empty range: r0 = r6, T = 1
//! 0x8c0f1350: tst r6,r6 ; bt 0x8c0f1358   ; NULL dst: skip the copy
//! mov.l @r4,r0 ; mov.l r0,@r6
//! 0x8c0f1358: add #4,r4 ; cmp/eq r5,r4
//! bf.s 0x8c0f1350
//! add #4,r6                         ; delay slot: runs EVERY iteration, taken or not
//! 0x8c0f1360: rts
//! mov r6,r0                         ; delay slot: return value
//! ```
//! Hidden side effects: r0 keeps the last dword copied (untouched when r6==0),
//! r4 ends == r5, r6 advanced 4 per iteration INCLUDING the final one, and the
//! final T is 1 (the loop only exits through cmp/eq true).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
    if cpu.r[4] != cpu.r[5] {
        // bt not taken -> loop head at 0x8c0f1350
        loop {
            cpu.set_t(cpu.r[6] == 0); // tst r6,r6
            if cpu.r[6] != 0 {
                // bt not taken
                cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
                let _ = mem.write_u32(cpu.r[6], cpu.r[0]); // mov.l r0,@r6
            }
            cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
            cpu.set_t(cpu.r[4] == cpu.r[5]); // cmp/eq r5,r4
            cpu.r[6] = cpu.r[6].wrapping_add(4); // bf.s delay slot: add #4,r6, always
            if cpu.r[4] == cpu.r[5] {
                break; // bf.s not taken -> fall through to rts
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // rts delay slot: mov r6,r0
}
