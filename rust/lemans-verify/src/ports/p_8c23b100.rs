//! 0x8c23b100 — selectable rewrite of the 0x3000 / 0x30 / 0x300 flag fields
//! of `*r4`, chosen by `r5` = 0..4.
//!
//! ```text
//! r7 = 0x0000eeee; r0 = r5; T = (r0 == 0); push r14; r6 = *r4
//! bt.s 0x8c23b19a        (delay: r14 = 0)
//! cmp/eq #1 -> 0x8c23b122; #2 -> 0x8c23b19a; #3 -> 0x8c23b134;
//! #4 -> 0x8c23b184; else 0x8c23b19a
//! ```
//!
//! * mode 0, 2 and anything outside 1..4: `*r4` is rewritten with its own
//!   value, `r0 = r5 = 0x0000eeee`; T is 1 for 0/2 (the matching `cmp/eq`) and
//!   0 for the fall-through (`cmp/eq #4` missed).
//! * mode 1: if `r6 & 0x3000 == 0`, unchanged and T=1; otherwise
//!   `r6 = (r6 & 0xffffff4f) | 0x40`, T=0, `r0 = 0`.
//! * mode 3: if `r6 & 0x30 == 0`, unchanged and T=1; otherwise
//!   `r6 = (r6 & 0xffff4fff) | 0x4000`, T=0, `r0 = 0`.
//! * mode 4: if `r6 & 0x300 == 0`, unchanged and T=1; otherwise
//!   `r6 = (r6 & 0xfffcffff) | 0x40000`, T=0, `r0 = 0`.
//!
//! Every path stores r6 back to `*r4` and restores r14 from the stack; the
//! immediate `and`/`or` pairs do not touch T, so T is whatever the last
//! compare/tst set.

use sh4_oracle::{Cpu, Memory};

/// 0x8c23b19c: `mov r5,r0; mov.l r6,@r4; rts` (delay `mov.l @r15+,r14`).
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(0x8c23b168).unwrap_or(0x0000_eeee); // 0x0000eeee
    cpu.r[0] = cpu.r[5]; // mov r5,r0
    cpu.set_t(cpu.r[0] == 0); // cmp/eq #0,r0

    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[6] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r6
    cpu.r[14] = 0; // 0x8c23b10c: bt.s delay slot, always executes

    if cpu.t() {
        // 0x8c23b19a: bra target, `mov r7,r5`.
        cpu.r[5] = cpu.r[7];
        epilogue(cpu, mem);
        return;
    }

    cpu.set_t(cpu.r[0] == 1); // 0x8c23b10e: cmp/eq #1,r0
    if cpu.t() {
        cpu.r[3] = mem.read_u16(0x8c23b160).unwrap_or(0x3000) as i16 as i32 as u32; // 0x3000
        cpu.set_t(cpu.r[6] & cpu.r[3] == 0); // tst r6,r3
        if cpu.t() {
            // 0x8c23b130: bra 0x8c23b19c, delay `mov r7,r5`.
            cpu.r[5] = cpu.r[7];
        } else {
            cpu.r[1] = mem.read_u16(0x8c23b162).unwrap_or(0xff4f) as i16 as i32 as u32; // 0xffffff4f
            cpu.r[6] &= cpu.r[1]; // and r1,r6
            cpu.r[3] = 64; // 0x8c23b12e: bra 0x8c23b190 delay slot, mov #64,r3
            cpu.r[6] |= cpu.r[3]; // 0x8c23b190: or r3,r6
            cpu.r[5] = cpu.r[14]; // 0x8c23b194: bra 0x8c23b19c delay slot
        }
        epilogue(cpu, mem);
        return;
    }

    cpu.set_t(cpu.r[0] == 2); // 0x8c23b112: cmp/eq #2,r0
    if cpu.t() {
        cpu.r[5] = cpu.r[7]; // 0x8c23b19a
        epilogue(cpu, mem);
        return;
    }

    cpu.set_t(cpu.r[0] == 3); // 0x8c23b116: cmp/eq #3,r0
    if cpu.t() {
        cpu.r[3] = 48; // mov #48,r3
        cpu.set_t(cpu.r[6] & cpu.r[3] == 0); // tst r6,r3
        if cpu.t() {
            // 0x8c23b180: bra 0x8c23b19c, delay `mov r7,r5`.
            cpu.r[5] = cpu.r[7];
        } else {
            cpu.r[3] = mem.read_u16(0x8c23b164).unwrap_or(0x4000) as i16 as i32 as u32; // 0x4000
            cpu.r[1] = mem.read_u32(0x8c23b16c).unwrap_or(0xffff_4fff); // 0xffff4fff
            cpu.r[6] &= cpu.r[1]; // 0x8c23b140: bra 0x8c23b190 delay slot
            cpu.r[6] |= cpu.r[3]; // 0x8c23b190
            cpu.r[5] = cpu.r[14]; // 0x8c23b194: bra 0x8c23b19c delay slot
        }
        epilogue(cpu, mem);
        return;
    }

    cpu.set_t(cpu.r[0] == 4); // 0x8c23b11a: cmp/eq #4,r0
    if cpu.t() {
        cpu.r[2] = mem.read_u16(0x8c23b1c0).unwrap_or(0x300) as i16 as i32 as u32; // 0x0300
        cpu.set_t(cpu.r[6] & cpu.r[2] == 0); // tst r6,r2
        if cpu.t() {
            // 0x8c23b196: bra 0x8c23b19c, delay `mov r7,r5`.
            cpu.r[5] = cpu.r[7];
        } else {
            cpu.r[1] = mem.read_u32(0x8c23b1c4).unwrap_or(0xfffc_ffff); // 0xfffcffff
            cpu.r[3] = mem.read_u32(0x8c23b1c8).unwrap_or(0x0004_0000); // 0x40000
            cpu.r[6] &= cpu.r[1];
            cpu.r[6] |= cpu.r[3]; // 0x8c23b190
            cpu.r[5] = cpu.r[14]; // 0x8c23b194: bra 0x8c23b19c delay slot
        }
        epilogue(cpu, mem);
        return;
    }

    // 0x8c23b11e: bra 0x8c23b19a; delay nop. T=0 from the missed cmp/eq #4.
    cpu.r[5] = cpu.r[7];
    epilogue(cpu, mem);
}
