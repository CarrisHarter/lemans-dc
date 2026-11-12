//! 0x8c0205e4 — find a 16-byte table entry by masked key, filling gaps on demand.
//!
//! ```text
//! save r14-r10,pr; r10 = r4 (table base); r0 = [r5+12]
//! if r0 == 0:                       // no key source
//!     r0 = [r5+4]; return r0 ? [r0] : 0
//! r14 = ([r0+12] & 0x000fffff) << 3  // 20-bit key scaled to 8
//! if r14 == 0: return 0
//! r11 = 0x118; r12 = 0x114          // entry offsets into the r4 table
//! r13 = [r10+0x118]                 // first entry pointer
//! loop:
//!     if [r13+12] == 0 && [r13] != 0:
//!         [r13+12] = [@(28, callee([r13]))]   // callee 0x8c0eaef8: r4 += 28
//!     if [r13+12] == r14: return [r13]
//!     r13 += 16
//!     continue while [r10+0x118] + ([r10+0x114] << 4) > r13 (unsigned)
//! return 0
//! ```
//! Leftovers the C does not show: r10-r14 keep their last loop values
//! (r13 = one past the tested entry on miss, the hit entry on match),
//! r0/r1 hold the final bound computation, and T ends set iff the last
//! `cmp/hi` (or the `cmp/eq` on the hit path) compared true.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[10] = cpu.r[4];
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            cpu.r[0] = 0;
        } else {
            cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // bra delay slot
        }
    } else {
        cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c020668).unwrap_or(0); // 0x000fffff
        cpu.r[14] &= cpu.r[0];
        cpu.r[14] = cpu.r[14] << 2; // shll2 (T untouched)
        let out = cpu.r[14] >> 31 != 0;
        cpu.r[14] = cpu.r[14] << 1; // shll (sets T = old bit 31)
        cpu.set_t(out);
        cpu.set_t(cpu.r[14] & cpu.r[14] == 0); // tst r14,r14
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c02066c).unwrap_or(0); // 0x118
            cpu.r[12] = mem.read_u32(0x8c020670).unwrap_or(0); // 0x114
            cpu.r[11] = cpu.r[0];
            // mov.l @(r0,r10),r13 = [r0+r10] (r0 still holds 0x118 here)
            cpu.r[13] = mem
                .read_u32(cpu.r[0].wrapping_add(cpu.r[10]))
                .unwrap_or(0);
            // bra lands on the bound check first, then bt falls into the loop
            cpu.r[0] = cpu.r[11];
            cpu.r[1] = mem
                .read_u32(cpu.r[0].wrapping_add(cpu.r[10]))
                .unwrap_or(0);
            cpu.r[0] = cpu.r[12];
            cpu.r[0] = mem
                .read_u32(cpu.r[0].wrapping_add(cpu.r[10]))
                .unwrap_or(0);
            cpu.r[0] = cpu.r[0] << 2; // shll2 x2 (T untouched)
            cpu.r[0] = cpu.r[0] << 2;
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
            cpu.set_t(cpu.r[1] > cpu.r[13]); // cmp/hi r13,r1
            if !cpu.t() {
                cpu.r[0] = 0; // bra delay slot
            } else {
                loop {
                cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
                cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
                if cpu.t() {
                    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
                    cpu.set_t(cpu.r[4] & cpu.r[4] == 0); // tst r4,r4
                    if !cpu.t() {
                        cpu.r[0] = mem.read_u32(0x8c020674).unwrap_or(0); // 0x8c0eaef8
                        crate::registry::p_8c0eaef8::run(cpu, mem);
                        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(28)).unwrap_or(0);
                        let _ = mem.write_u32(cpu.r[13].wrapping_add(12), cpu.r[0]);
                    }
                }
                cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
                cpu.set_t(cpu.r[0] == cpu.r[14]); // cmp/eq r14,r0
                if cpu.t() {
                    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0); // bra delay slot
                    break;
                }
                cpu.r[13] = cpu.r[13].wrapping_add(16);
                cpu.r[0] = cpu.r[11];
                // mov.l @(r0,r10),r1 = [r0+r10]
                cpu.r[1] = mem
                    .read_u32(cpu.r[0].wrapping_add(cpu.r[10]))
                    .unwrap_or(0);
                cpu.r[0] = cpu.r[12];
                // mov.l @(r0,r10),r0 = [r0+r10]
                cpu.r[0] = mem
                    .read_u32(cpu.r[0].wrapping_add(cpu.r[10]))
                    .unwrap_or(0);
                cpu.r[0] = cpu.r[0] << 2; // shll2 x2 (T untouched)
                cpu.r[0] = cpu.r[0] << 2;
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
                cpu.set_t(cpu.r[1] > cpu.r[13]); // cmp/hi r13,r1
                if !cpu.t() {
                    cpu.r[0] = 0; // bra delay slot
                    break;
                }
                }
            }
        } else {
            cpu.r[0] = 0; // bra delay slot
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
