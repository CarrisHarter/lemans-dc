//! 0x8c13d02c — search the table at [r4+8] + i*80 for i in [0, [r4+4]) for
//! the key r5. On the first match, look up [[r4+104] + i*8] and return 0 if
//! it is -1, else 2. No match returns 0. r4 is left at r4+104 on a match.
//!
//! Final T: 1 when the matched entry was -1 (cmp/eq #-1,r0), 0 otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[6] = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r6
    cpu.r[0] = 0; // bra delay slot: mov #0,r0
    loop {
        // 0x8c13d05e: cmp/hi r0,r6 ; bt 0x8c13d032
        cpu.set_t(cpu.r[6] > cpu.r[0]); // cmp/hi r0,r6
        if !cpu.t() {
            // 0x8c13d062: rts ; mov #0,r0
            cpu.r[0] = 0;
            return;
        }
        // 0x8c13d032: r3 = r0*80 + @(8,r4) ; cmp/eq r5,r3 ; bf 0x8c13d05c
        let i = cpu.r[0];
        let mut r3 = i << 2; // shll2 r3 -> 4i
        r3 = r3.wrapping_add(i); // add r0,r3 -> 5i
        r3 <<= 2; // shll2 -> 20i
        let base = mem.read_u32(r4.wrapping_add(8)).unwrap_or(0);
        cpu.r[1] = base; // mov.l @(8,r4),r1
        r3 <<= 2; // shll2 -> 80i
        r3 = r3.wrapping_add(base);
        cpu.r[3] = r3;
        cpu.set_t(r3 == cpu.r[5]); // cmp/eq r5,r3
        if cpu.t() {
            // 0x8c13d044: r4 += 104 ; r1 = @r4 ; r0 = r0*8 ; r0 = @(r0,r1)
            cpu.r[4] = r4.wrapping_add(104);
            cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            let mut r0 = i;
            r0 <<= 2; // shll2 r0 -> 4i
            cpu.set_t(r0 >> 31 != 0); // shll r0: T = old bit 31
            r0 <<= 1; // 8i
            cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(r0)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 0xffff_ffff); // cmp/eq #-1,r0
            if cpu.t() {
                // 0x8c13d052: rts ; mov #0,r0
                cpu.r[0] = 0;
            } else {
                // 0x8c13d056: mov #2,r0 ; rts ; nop
                cpu.r[0] = 2;
            }
            return;
        }
        // 0x8c13d05c: add #1,r0
        cpu.r[0] = cpu.r[0].wrapping_add(1);
    }
}
