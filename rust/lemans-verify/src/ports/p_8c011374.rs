//! 0x8c011374 — scan haystack r4 for the first byte that matches a needle
//! dword in r5.
//!
//! Despite the strstr-shaped C, the inner loop compares the haystack byte
//! at r14 = r4 + i against `mov.b @(r0,r6),r1` of the needle — which the
//! oracle decodes as a u32 load, like all group-0 indexed byte loads. The
//! outer head load `mov.b @(r0,r7),r3` is also a u32 load. r12/r14 are
//! saved/restored through the stack (net zero on r15); the initial `bra`
//! delay slot sets r7 = r12 = 0. Found: the `bra` delay slot at 0x8c0113a0
//! sets r0 = r14 and T ends clear; NULL: r0 = 0, T set.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let hay = cpu.r[4];
    let needle = cpu.r[5];
    let mut r7 = 0u32; // delay slot of the initial `bra`: mov r12,r7
    let mut result: u32;
    let mut t = true;
    'outer: loop {
        // outer head 0x8c0113a4: mov.b @(r0,r7),r3 decodes as a u32 load
        let h = mem.read_u32(hay.wrapping_add(r7)).unwrap_or(0);
        cpu.r[3] = h;
        t = h == 0;
        if h == 0 {
            result = 0; // mov #0,r0 at 0x8c0113ac
            break;
        }
        // outer body 0x8c01137e: r14 = r4 + r7, r6 = r12 = 0
        let r14 = hay.wrapping_add(r7);
        let mut r6 = 0u32;
        loop {
            // inner head 0x8c011388: mov.b @(r0,r6),r1 decodes as a u32 load
            let c = mem.read_u32(needle.wrapping_add(r6)).unwrap_or(0);
            cpu.r[1] = c;
            if c == 0 {
                // bt 0x8c011396 -> needle dword is zero -> outer next
                cpu.r[3] = c; // reload at 0x8c011398 reads the same zero dword
                t = true;
                cpu.r[6] = r6;
                r7 = r7.wrapping_add(1); // 0x8c0113a2
                continue 'outer;
            }
            let h2 = sext8(mem, r14); // mov.b @r14,r3 is a true byte load
            if h2 == c {
                // match: 0x8c011396 reloads needle[r6] (non-zero) -> bra found
                cpu.r[3] = c;
                t = false;
                cpu.r[6] = r6;
                result = r14; // delay slot of the `bra` at 0x8c01139e
                break 'outer;
            }
            r6 = r6.wrapping_add(1); // 0x8c011386
        }
    }
    cpu.set_t(t);
    cpu.r[7] = r7;
    cpu.r[0] = result;
}
