//! 0x8c14cdcc — walk a 48-byte-stride array, release each element with count -1,
//! then free the array itself.
//!
//! Null `[r4+8]` exits after the prologue (`tst` leaves T=1). Otherwise
//! `end = base + 48*(u32)[r4+4]` (shll/add/shll2/shll2, wrapping); while the
//! unsigned `cmp/hi` says base < end, call 0x8c0443c8(element, -1) — whose guard
//! declines on the negative count, so it only returns — and step by 48. Then
//! free the array base through 0x8c01032c. r11..r14 and pr are pushed, so they
//! end at entry values; T is the final free's.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let obj = cpu.r[4]; // mov r4,r11 (r11 is pushed; it ends at its entry value)
    let base = mem.read_u32(obj.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r11),r13
    cpu.set_t(base == 0); // tst r13,r13
    if base != 0 {
        let count = mem.read_u32(obj.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r11),r0
        cpu.r[0] = count;
        let mut end = count << 1; // shll r14
        end = end.wrapping_add(count); // add r0,r14
        end <<= 2; // shll2 r14
        end <<= 2; // shll2 r14
        end = end.wrapping_add(base); // add r13,r14
        let neg_one = u32::MAX; // mov #-1,r12 (pushed, so r12 ends at its entry value)
        cpu.set_t(end > base); // cmp/hi r13,r14
        if end > base {
            let mut cur = base;
            loop {
                cpu.r[0] = mem.read_u32(0x8c14ce14).unwrap_or(0x8c0443c8);
                cpu.r[5] = neg_one; // mov r12,r5
                cpu.r[4] = cur; // delay slot: mov r13,r4
                cpu.pr = 0x8c14cdfa;
                crate::registry::p_8c0443c8::run(cpu, mem);
                cur = cur.wrapping_add(48); // add #48,r13
                cpu.set_t(end > cur); // cmp/hi r13,r14
                if end > cur {
                    continue;
                }
                break;
            }
        }
        cpu.r[0] = mem.read_u32(0x8c14ce18).unwrap_or(0x8c01032c);
        cpu.r[4] = base; // delay slot: mov.l @(8,r11),r4
        cpu.pr = 0x8c14ce06;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
}
