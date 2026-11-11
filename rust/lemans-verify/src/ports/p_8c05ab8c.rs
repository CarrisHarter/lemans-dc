//! 0x8c05ab8c — transpose-swap of 4x4 float cells: for i in 1..4, for j in 0..i,
//! swap the dword at `base + 16*i + 4*j` with the one at `base + 16*j + 4*i`.
//!
//! Original prologue pushes r14/r13/r12/r11 (all popped back, r15 restored),
//! then runs: r6 = r5 = r11 = 0, r12 = r4, r0 = 4; outer loop `cmp/pl r6`,
//! inner loop of `fmov @r3,fr1 / fmov @r1,fr0 / fmov fr0,@r3 / fmov fr1,@r1`
//! with r3 = r12+r14, r1 = r13+r11, r14 += 4, r13 += 16, r7 counting.
//!
//! Leftovers the C does not show: r0 = 4, r5 = 0, r6 = 4, r7 = 3 at exit;
//! r1 = base+0x2c and r3 = base+0x38 (last inner iteration's pointers);
//! fr0/fr1 hold the pre-swap dwords of the last pair (bit-exact); T ends
//! clear because the final `cmp/gt r6,r0` compares 4 > 4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let param = cpu.r[4];
    let r0 = 4u32; // 0x8c05ab9c mov #4,r0
    let r5 = 0u32; // 0x8c05ab9a mov r6,r5 (r6 == 0 here)
    let mut r6 = 0u32; // 0x8c05ab94 mov #0,r6
    let mut r12 = param; // 0x8c05ab96 mov r4,r12
    let mut r11 = 0u32; // 0x8c05ab98 mov r6,r11
    let mut r7 = 0u32;
    cpu.r[0] = r0;
    cpu.r[5] = r5;
    loop {
        // 0x8c05ab9e cmp/pl r6 ; 0x8c05aba0 bf.s + delay 0x8c05aba2 mov r5,r7
        r7 = r5;
        if r6 > 0 {
            let mut r14 = r5; // 0x8c05aba4 mov r5,r14
            let mut r13 = param; // 0x8c05aba6 mov r4,r13
            loop {
                let r1 = r13.wrapping_add(r11); // 0x8c05abae add r11,r1
                let r3 = r12.wrapping_add(r14); // 0x8c05abac add r14,r3
                let fr1 = f32::from_bits(mem.read_u32(r3).unwrap_or(0)); // 0x8c05abb0 fmov @r3,fr1
                r7 = r7.wrapping_add(1); // 0x8c05abb2 add #1,r7
                let fr0 = f32::from_bits(mem.read_u32(r1).unwrap_or(0)); // 0x8c05abb4 fmov @r1,fr0
                let t = r6 > r7; // 0x8c05abb6 cmp/gt r7,r6
                r14 = r14.wrapping_add(4); // 0x8c05abb8 add #4,r14
                let _ = mem.write_u32(r3, fr0.to_bits()); // 0x8c05abba fmov fr0,@r3
                r13 = r13.wrapping_add(16); // 0x8c05abbc add #16,r13
                let _ = mem.write_u32(r1, fr1.to_bits()); // 0x8c05abc0 bt.s delay: fmov fr1,@r1
                cpu.fr[0] = fr0;
                cpu.fr[1] = fr1;
                cpu.r[1] = r1;
                cpu.r[3] = r3;
                if !t {
                    break;
                }
            }
        }
        // 0x8c05abc2 add #1,r6 ; 0x8c05abc4 cmp/gt r6,r0
        r6 = r6.wrapping_add(1);
        let t = r0 > r6;
        r12 = r12.wrapping_add(16); // 0x8c05abc6 add #16,r12
        r11 = r11.wrapping_add(4); // 0x8c05abca bt.s delay: add #4,r11
        cpu.r[6] = r6;
        cpu.r[7] = r7;
        if !t {
            break;
        }
    }
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.set_t(r0 > r6); // last T write: 0x8c05abc4 cmp/gt r6,r0
}
