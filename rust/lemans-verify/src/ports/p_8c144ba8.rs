//! 0x8c144ba8 — walk the linked list at [r4+4] comparing each node's
//! unsigned key at +12 against [r5]: nodes with key > [r5] are followed via
//! +0, others via +4 while remembering the last such node in r3. At the end,
//! if r3 != 0 and [r5] <= [r3+12] the node r3 is stored through [r15];
//! otherwise r4+4 is stored through [r15].
//!
//! The `bt.s` delay slot `mov #0,r3` always runs; the `bra` delay slot
//! `mov.l @r6,r6` is the >-key link. Both rts delay slots store through
//! r0 = [r15]: `mov.l r4,@r0` after `add #4,r4` on the taken path,
//! `mov.l r3,@r0` on the other.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r6
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    cpu.r[3] = 0; // bt.s delay slot: mov #0,r3 (always runs)
    if !cpu.t() {
        // 0x8c144bb0 loop
        loop {
            cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r1
            cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r6),r0
            cpu.set_t(cpu.r[0] > cpu.r[1]); // cmp/hi r1,r0
            if cpu.t() {
                // 0x8c144bb8: bra 0x8c144bc0 ; mov.l @r6,r6 (delay slot)
                cpu.r[6] = mem.read_u32(cpu.r[6]).unwrap_or(0);
            } else {
                // 0x8c144bbc: mov r6,r3 ; mov.l @(4,r6),r6
                cpu.r[3] = cpu.r[6];
                cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            }
            // 0x8c144bc0: tst r6,r6 ; bf 0x8c144bb0
            cpu.set_t(cpu.r[6] == 0);
            if !cpu.t() {
                continue;
            }
            break;
        }
    }
    // 0x8c144bc4: tst r3,r3 ; bt 0x8c144bd0
    cpu.set_t(cpu.r[3] == 0);
    if !cpu.t() {
        // r3 != 0: r1 = @(12,r3) ; r0 = @r5 ; cmp/hi r1,r0 ; bf 0x8c144bd8
        cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.set_t(cpu.r[0] > cpu.r[1]); // cmp/hi r1,r0
        if !cpu.t() {
            // 0x8c144bd8: r0 = @r15 ; rts ; mov.l r3,@r0
            cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[0], cpu.r[3]);
            return;
        }
    }
    // 0x8c144bd0: r0 = @r15 ; add #4,r4 ; rts ; mov.l r4,@r0
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
}
