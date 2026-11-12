//! 0x8c172fcc - range-check a selector, decode its three bytes, compare them.
//!
//! The low bytes of `r5`, `r6` and `r7` are stashed at `r15+20`, `r15+12` and
//! `r15+16`. A null `r4` reports through 0x8c1725fc with code 101 and returns 0;
//! a selector outside `[0x8c2b5264, 0x8c2b5264 + 0x8c2b5268)` reports with code
//! 0x1fb and returns 0. Inside the range 0x8c173084 decodes `r4` into three bytes
//! at `r15+0/+4/+8`, and the function returns 1 only when all three equal the
//! stashed bytes.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(24); // add #-24,r15
    cpu.r[0] = 20; // mov #20,r0
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[5] as u8); // mov.b r5,@(r0,r15)
    cpu.r[0] = cpu.r[6]; // mov r6,r0
    let _ = mem.write_u8(cpu.r[15].wrapping_add(12), cpu.r[0] as u8); // mov.b r0,@(12,r15)
    cpu.set_t((cpu.r[4] & cpu.r[4]) == 0); // tst r4,r4
    cpu.r[0] = 16; // mov #16,r0
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[7] as u8); // bf.s delay: mov.b r7,@(r0,r15)
    if cpu.t() {
        cpu.r[5] = mem.read_u32(0x8c173038).unwrap_or(0x8c1df4c8); // mov.l 0x8c173038,r5
        cpu.r[4] = 101; // bra delay: mov #101,r4
        report(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c173014
    } else {
        cpu.r[3] = mem.read_u32(0x8c173028).unwrap_or(0x8c2b5264); // mov.l 0x8c173028,r3
        cpu.r[5] = cpu.r[4]; // mov r4,r5
        cpu.r[6] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r6
        cpu.set_t(cpu.r[5] >= cpu.r[6]); // cmp/hs r6,r5
        let mut in_range = false;
        if cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c17303c).unwrap_or(0x8c2b5268); // mov.l 0x8c17303c,r2
            cpu.r[7] = mem.read_u32(cpu.r[2]).unwrap_or(0); // mov.l @r2,r7
            cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[6]); // add r6,r7
            cpu.set_t(cpu.r[5] >= cpu.r[7]); // cmp/hs r7,r5
            if !cpu.t() {
                in_range = true; // bra delay: mov #1,r7
            }
        }
        cpu.r[7] = u32::from(in_range); // mov #0,r7 on both fall-throughs
        cpu.set_t((cpu.r[7] & cpu.r[7]) == 0); // tst r7,r7
        if cpu.t() {
            cpu.r[6] = cpu.r[15]; // mov r15,r6
            cpu.r[7] = cpu.r[15]; // mov r15,r7
            cpu.r[6] = cpu.r[6].wrapping_add(4); // add #4,r6
            cpu.r[7] = cpu.r[7].wrapping_add(8); // add #8,r7
            cpu.r[5] = cpu.r[15]; // bsr delay: mov r15,r5
            cpu.pr = 0x8c173050;
            crate::registry::p_8c173084::run(cpu, mem);
            cpu.r[1] = cpu.r[15].wrapping_add(20); // mov r15,r1 / add #20,r1
            cpu.r[3] = mem.read_u8(cpu.r[1]).unwrap_or(0) as u32; // mov.b @r1,r3
            cpu.r[0] = mem.read_u8(cpu.r[15]).unwrap_or(0) as u32; // mov.b @r15,r0
            cpu.set_t(cpu.r[0] == cpu.r[3]); // cmp/eq r3,r0
            let mut equal = cpu.t();
            if equal {
                cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(4)).unwrap_or(0) as u32; // mov.b @(4,r15),r0
                cpu.r[3] = cpu.r[0]; // mov r0,r3
                cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(12)).unwrap_or(0) as u32; // mov.b @(12,r15),r0
                cpu.set_t(cpu.r[3] == cpu.r[0]); // cmp/eq r0,r3
                equal = cpu.t();
                if equal {
                    cpu.r[1] = cpu.r[15].wrapping_add(16); // mov r15,r1 / add #16,r1
                    cpu.r[3] = mem.read_u8(cpu.r[1]).unwrap_or(0) as u32; // mov.b @r1,r3
                    cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(8)).unwrap_or(0) as u32; // mov.b @(8,r15),r0
                    cpu.set_t(cpu.r[0] == cpu.r[3]); // cmp/eq r3,r0
                    equal = cpu.t();
                }
            }
            cpu.r[15] = cpu.r[15].wrapping_add(24); // add #24,r15
            cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.r[15] = cpu.r[15].wrapping_add(4);
            cpu.r[0] = u32::from(equal); // rts delay: mov #1,r0 / mov #0,r0
            return;
        }
        cpu.r[4] = mem.read_u16(0x8c173018).unwrap_or(0x01fb) as i16 as i32 as u32; // mov.w
        cpu.r[5] = mem.read_u32(0x8c173040).unwrap_or(0x8c1df4f8); // mov.l 0x8c173040,r5
        report(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c173014
    }
    cpu.r[15] = cpu.r[15].wrapping_add(24); // add #24,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn report(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c173024).unwrap_or(0x8c1725fc); // mov.l 0x8c173024,r2
    cpu.pr = 0x8c17300e; // jsr delay is nop
    crate::registry::p_8c1725fc::run(cpu, mem);
}
