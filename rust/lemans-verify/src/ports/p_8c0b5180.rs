//! 0x8c0b5180 — write a value and a float into one element, or into all of them.
//!
//! `r4` points at `{array@0, count@4}`; each element is 44 bytes, with the value
//! word at +36 and the float at +40. `r5 == -2` writes every element, any other
//! r5 writes `44*r5` (signed `mul.l`). r6 is the word, fr4 the float.
//! r9..r14 and fr15 are pushed, so they end at entry values; the observable
//! residue is r0 (last +40 address), r5 (`44*index` on the direct path), and
//! MACH/MACL on the direct path. T ends 0 on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let desc = cpu.r[4]; // mov r4,r9
    let selector = cpu.r[5];
    let value = cpu.r[6]; // mov r6,r10
    let f = cpu.fr[4]; // fmov fr4,fr15
    let array = mem.read_u32(desc).unwrap_or(0); // mov.l @r9,r0
    let count = mem.read_u32(desc.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r9),r0
    let minus_two = 0xffff_fffeu32; // mov.l 0x8c0b51fc,r11

    cpu.set_t(minus_two == selector); // cmp/eq r11,r5
    if selector == minus_two {
        // bra 0x8c0b51d0, delay r12 = r14 = 0
        let mut r12 = 0u32;
        let mut r14 = 0u32;
        loop {
            // 0x8c0b51d0 reloads r0 with the count every pass, so the exit
            // value of r0 is the count, not the last store address.
            cpu.r[0] = count; // mov.l @(4,r9),r0
            cpu.set_t((count as i32) > (r14 as i32)); // cmp/gt r14,r0
            if (count as i32) <= (r14 as i32) {
                break;
            }
            // 0x8c0b51a2: r14 == -2 is unreachable for an ascending counter.
            let addr = array.wrapping_add(r12); // mov.l @r9,r0 ; add r12,r0
            let _ = mem.write_u32(addr.wrapping_add(36), value); // mov.l r10,@(36,r0)
            cpu.r[0] = addr.wrapping_add(40); // add #40,r0
            let _ = mem.write_f32(addr.wrapping_add(40), f); // fmov fr15,@r0
            r12 = r12.wrapping_add(44); // add #44,r12
            r14 = r14.wrapping_add(1); // add #1,r14
        }
        // The count<=0 exit also leaves the count in r0 (set before the test).
    } else {
        let product = 44i64 * (selector as i32 as i64); // mul.l r0,r5 with r0 = 44
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[5] = cpu.macl; // sts macl,r5
        let addr = array.wrapping_add(cpu.r[5]); // mov.l @r9,r0 ; add r5,r0
        let _ = mem.write_u32(addr.wrapping_add(36), value); // mov.l r10,@(36,r0)
        cpu.r[0] = addr.wrapping_add(40); // add #40,r0
        let _ = mem.write_f32(addr.wrapping_add(40), f); // fmov fr15,@r0
    }
}
