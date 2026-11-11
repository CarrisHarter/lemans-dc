//! 0x8c146d78 — count bytes from the table at mem[r4+12] into 6 bucket counters
//! on the stack (mem[r15+cat*4] += 1 each), accumulating the byte values in
//! r12; returns the last counter value (after +1) in r0.
//!
//! Loop: r11 = mem[r4+8] iterations; each iteration reads table[i] (i = r13),
//! takes r10 = byte & 0xff, maps it through the cmp/hi chain
//! (<2 -> 0, <51 -> 1, <101 -> 2, <151 -> 3, <201 -> 4, else 5) and does
//! mem[r15+cat*4] += 1. Non-`.s` bf are immediate in this oracle (the `bra`
//! sitting in their delay slots only runs when the bf is *not* taken), and the
//! bt.s back-edge delay slot `add r10,r12` runs on every iteration including
//! the final non-taken one. Stack stores are excluded from the diff, but the
//! bucket counters are real memory the loop reads back, so they are modelled
//! with actual stack accesses.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14/r13/r12/r11/r10/r9/r8,@-r15 ; add #-24,r15
    // (stack window is excluded from the write diff; only r15 arithmetic and
    // the bucket counters below are observable)
    let (s8, s9, s10, s11, s12, s13, s14) =
        (cpu.r[8], cpu.r[9], cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(28);
    cpu.r[15] = cpu.r[15].wrapping_sub(24);
    cpu.r[12] = 0; // mov #0,r12
    for off in [0u32, 4, 8, 12, 16, 20] {
        mem.write_u32(cpu.r[15].wrapping_add(off), cpu.r[12]).unwrap();
    }
    cpu.r[11] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap(); // mov.l @(8,r4),r11
    cpu.set_t((cpu.r[11] as i32) > 0); // cmp/pl r11
    cpu.r[13] = cpu.r[12]; // bf.s delay slot: mov r12,r13 (runs either way)
    if cpu.t() {
        cpu.r[8] = mem.read_u32(0x8c146e08).unwrap(); // 0x97 = 151
        cpu.r[7] = cpu.r[12]; // mov r12,r7
        cpu.r[1] = cpu.r[15]; // mov r15,r1
        cpu.r[3] = 101; // mov #101,r3
        cpu.r[6] = 51; // mov #51,r6
        cpu.r[5] = 1; // mov #1,r5
        cpu.r[14] = 2; // mov #2,r14
        loop {
            // 0x8c146dac
            cpu.r[9] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap(); // mov.l @(12,r4),r9
            cpu.r[0] = cpu.r[13]; // mov r13,r0
            cpu.r[0] = cpu.r[0] << 2; // shll2 r0
            cpu.r[0] = mem.read_u32(cpu.r[9].wrapping_add(cpu.r[0])).unwrap(); // mov.l @(r0,r9),r0
            cpu.r[10] = cpu.r[0] & 0xff; // extu.b r0,r10
            // cmp/hi chain; a non-taken bf runs the bra in its delay slot,
            // whose own delay slot sets the category register r9
            cpu.set_t(cpu.r[14] > cpu.r[10]); // cmp/hi r10,r14 (2 > r10)
            if cpu.t() {
                cpu.r[9] = cpu.r[7]; // delay slot mov r7,r9 (category 0)
            } else {
                cpu.set_t(cpu.r[6] > cpu.r[10]); // cmp/hi r10,r6 (51 > r10)
                if cpu.t() {
                    cpu.r[9] = cpu.r[5]; // mov r5,r9 (category 1)
                } else {
                    cpu.set_t(cpu.r[3] > cpu.r[10]); // cmp/hi r10,r3 (101 > r10)
                    if cpu.t() {
                        cpu.r[9] = cpu.r[14]; // mov r14,r9 (category 2)
                    } else {
                        cpu.set_t(cpu.r[8] > cpu.r[10]); // cmp/hi r10,r8 (151 > r10)
                        if cpu.t() {
                            cpu.r[9] = 3; // mov #3,r9
                        } else {
                            cpu.r[0] = mem.read_u32(0x8c146e0c).unwrap(); // 0xc9 = 201
                            cpu.set_t(cpu.r[0] > cpu.r[10]); // cmp/hi r10,r0
                            if cpu.t() {
                                cpu.r[9] = 4; // mov #4,r9
                            } else {
                                cpu.r[9] = 5; // mov #5,r9 (fallthrough 0x8c146de0)
                            }
                        }
                    }
                }
            }
            // 0x8c146de2
            cpu.r[9] = cpu.r[9] << 2; // shll2 r9
            cpu.r[9] = cpu.r[9].wrapping_add(cpu.r[1]); // add r1,r9
            cpu.r[0] = mem.read_u32(cpu.r[9]).unwrap(); // mov.l @r9,r0
            cpu.r[13] = cpu.r[13].wrapping_add(1); // add #1,r13
            cpu.set_t((cpu.r[11] as i32) > (cpu.r[13] as i32)); // cmp/gt r13,r11
            cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
            mem.write_u32(cpu.r[9], cpu.r[0]).unwrap(); // mov.l r0,@r9
            cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[10]); // bt.s delay slot: add r10,r12
            if !cpu.t() {
                break; // bt.s not taken -> 0x8c146df4
            }
        }
    }
    // 0x8c146df4: epilogue — add #24,r15, pop r8..r13, rts with pop r14
    cpu.r[15] = cpu.r[15].wrapping_add(24);
    cpu.r[8] = s8;
    cpu.r[9] = s9;
    cpu.r[10] = s10;
    cpu.r[11] = s11;
    cpu.r[12] = s12;
    cpu.r[13] = s13;
    cpu.r[14] = s14;
    cpu.r[15] = cpu.r[15].wrapping_add(28);
}
