//! 0x8c239882 — claim the first free slot of a 44-byte slot table.
//!
//! Arguments: r4 = table base (0 disables it), r5 = output word that receives
//! the claimed slot's address. The table keeps an 8-bit occupancy mask at
//! +0x248, a count at +0x24c and 44-byte slots from +0x254.
//!
//! ```text
//! if (r4 == 0) return 0x0000eeee;
//! for (i = 0; i < 8; i++) {
//!     bit = 1 << i;
//!     if ((mask & bit) != 0) continue;
//!     mask |= bit;
//!     count += 1;
//!     *(u32*)(base + 0x254 + 44*i) = i;
//!     *r5 = base + 0x254 + 44*i;
//!     return 0;
//! }
//! return 0;
//! ```
//!
//! The original pushes r14/r13 on entry and pops them on the way out, so both
//! end at their entry values and the frame pointer is unchanged. `r6` keeps
//! the table base on every path through the table; `r7` starts at 1, and on
//! the claim path becomes the claimed slot address. `r4` is the loop index
//! (8 when the table is full). `r2` is `1 << i` during the scan and the new
//! count on the claim path; `r3` is the last mask read on the exhausted path
//! and `base + 0x254` on the claim path. `r0` is 0 on both table returns and
//! 0x0000eeee for a null table.
//!
//! `mul.l r13,r4` writes the whole 64-bit product, so a claim sets
//! `mach = 0` and `macl = 44*i`; a full scan never executes it and leaves both
//! untouched. Flag detail: `tst r3,r2` sets T = 1 on the claim path and never
//! changes again, while the exhausted path ends on `cmp/ge r14,r4`
//! (r4 = 8 >= 8) with T = 1; the null path's `tst r4,r4` also yields T = 1.
//! Every exit therefore leaves T set. Store order is mask, slot, count, `*r5`.

use sh4_oracle::{Cpu, Memory};

const MASK_OFF: u32 = 0x0248;
const COUNT_OFF: u32 = 0x024c;
const SLOT_OFF: u32 = 0x0254;
const SLOT_STRIDE: i64 = 44;
const SLOTS: u32 = 8;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.set_t(base == 0); // mov.l r14,@-r15 ; tst r4,r4 ; mov.l r13,@-r15
    if base == 0 {
        // bt 0x8c2398d2 ; mov.l 0x8c2398ec,r0 (0x0000eeee)
        cpu.r[0] = mem.read_u32(0x8c2398ec).unwrap_or(0x0000_eeee);
        return;
    }

    cpu.r[6] = base; // mov r4,r6
    cpu.r[7] = 1; // mov #1,r7

    let mut index = 0u32; // mov #0,r4
    loop {
        cpu.r[0] = MASK_OFF; // mov.w 0x8c2398e4,r0
        cpu.r[2] = cpu.r[7] << index; // mov r7,r2 ; shad r4,r2 (index < 8)
        let mask = mem.read_u32(base.wrapping_add(MASK_OFF)).unwrap_or(0); // mov.l @(r0,r6),r3
        cpu.r[3] = mask;
        let free = mask & cpu.r[2] == 0;
        cpu.set_t(free); // tst r3,r2
        if !free {
            // bf 0x8c2398c8 ; add #1,r4 ; cmp/ge r14,r4 ; bf 0x8c239894
            index = index.wrapping_add(1);
            cpu.r[4] = index;
            cpu.set_t(index >= SLOTS); // cmp/ge r14,r4 (signed, both non-negative)
            if index < SLOTS {
                continue;
            }
            // bra 0x8c2398d4 ; mov #0,r0
            cpu.r[0] = 0;
            return;
        }

        // mul.l r13,r4 ; sts macl,r7
        let product = SLOT_STRIDE * index as i64;
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[7] = cpu.macl;
        // mov.l @(r0,r6),r2 ; mov #1,r3 ; shad r4,r3 ; or r3,r2
        cpu.r[2] = mask | (1u32 << index);
        // mov.w 0x8c2398e2,r3 (0x0254) ; add r6,r3
        cpu.r[3] = base.wrapping_add(SLOT_OFF);
        // mov.l r2,@(r0,r6) — mask is stored first
        let _ = mem.write_u32(base.wrapping_add(MASK_OFF), cpu.r[2]);
        // add #4,r0 ; add r7,r3 ; mov.l r4,@r3
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
        let _ = mem.write_u32(cpu.r[7], index);
        // mov.l @(r0,r6),r2 ; add #1,r2 ; mov.l r2,@(r0,r6)
        cpu.r[2] = mem
            .read_u32(base.wrapping_add(COUNT_OFF))
            .unwrap_or(0)
            .wrapping_add(1);
        let _ = mem.write_u32(base.wrapping_add(COUNT_OFF), cpu.r[2]);
        // bra delay slot: mov.l r7,@r5
        let _ = mem.write_u32(cpu.r[5], cpu.r[7]);
        cpu.r[4] = index;
        // bra 0x8c2398ce ; mov #0,r0
        cpu.r[0] = 0;
        return;
    }
}
