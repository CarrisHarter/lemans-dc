//! 0x8c14b2ec — advance r1 = mem[r4+8] by 4 until r1 >= mem[r4+8] + 4*mem[r4+4],
//! then store 0 to mem[r4+4] in the rts delay slot and return 0.
//!
//! Loop shape: `cmp/hi r1,r3 ; bf 0x8c14b300` guards entry, then
//! `add #4,r1 ; cmp/hi r1,r3 ; bt 0x8c14b2fa` repeats (bt/bf immediate here).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap(); // mov.l @(8,r4),r1
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r0
    cpu.r[3] = cpu.r[1]; // mov r1,r3
    cpu.r[0] = cpu.r[0] << 2; // shll2 r0
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // add r0,r3
    cpu.set_t(cpu.r[3] > cpu.r[1]); // cmp/hi r1,r3: T = r3 > r1
    if cpu.t() {
        // bf not taken: enter the add #4,r1 / bt loop
        loop {
            cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
            cpu.set_t(cpu.r[3] > cpu.r[1]); // cmp/hi r1,r3
            if !cpu.t() {
                break; // bt not taken -> 0x8c14b300
            }
        }
    }
    cpu.r[0] = 0; // mov #0,r0
    // rts delay slot: mov.l r0,@(4,r4)
    mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]).unwrap();
}
