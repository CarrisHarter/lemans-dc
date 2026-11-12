//! 0x8c07e47c — move a cursor `n` elements forward/backward through an
//! indexed buffer.
//!
//! `r4` points at a three-word cursor `{ descriptor, index, element }`; the
//! descriptor is `{ capacity, ?, limit, base }`. Only descriptor words 0, 2
//! and 3 are read. `r5` is the signed delta. The routine moves `element` by
//! `|n|` words and adjusts `index` by `-n`, wrapping `element` back to
//! `base + (overflow << 2)` when the step leaves the window.
//!
//! Register shape of the original (all compared, so it is not cosmetic):
//! * positive `n` loads the descriptor into `r3`, leaves `r6`/`r7` at their
//!   entry values;
//! * negative `n` loads it into `r6`, puts `-n` in `r7` (and `(-n) << 2` in
//!   `r7` only on the `cmp/hi` fall-through branch), and `descriptor[0]` in
//!   `r3`.
//!
//! T ends as the only flag-setting instruction before the exit: `cmp/hs`
//! (`limit + index >= n`, positive `n`) or `cmp/hi` (`capacity > reached`,
//! negative `n`). In the oracle `shll2`, `sub`, `add` and `mov` leave T alone.
//!
//! `n == i32::MIN` makes `neg` stay negative, and the original calls itself
//! through the literal at 0x8c07e4f4 with the same arguments, forever: it has
//! no clean-halt state to compare against, so this port returns unchanged
//! after the prologue on that single input.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    let cursor = cpu.r[4];
    let n = cpu.r[5];
    let idx_addr = cursor.wrapping_add(4);
    let elem_addr = cursor.wrapping_add(8);
    let index = mem.read_u32(idx_addr).unwrap_or(0);

    if (n as i32) >= 0 {
        // cmp/pz r5 ; bt 0x8c07e4c2
        let descriptor = mem.read_u32(cursor).unwrap_or(0);
        cpu.r[3] = descriptor;
        let limit = mem.read_u32(descriptor.wrapping_add(8)).unwrap_or(0);
        // cmp/hs r5,r1 — unsigned, and it decides the branch and final T.
        let fits = limit.wrapping_add(index) >= n;
        let element = mem.read_u32(elem_addr).unwrap_or(0);
        let new_element = if fits {
            // mov.l @(8,r4),r0 ; shll2 r0 ; sub r0,r1
            element.wrapping_sub(n << 2)
        } else {
            // mov.l @r3,r1 ; mov.l @(12,r3),r0 ; sub r5,r1 ; shll2 r1 ; add r0,r1
            let capacity = mem.read_u32(descriptor).unwrap_or(0);
            let base = mem.read_u32(descriptor.wrapping_add(12)).unwrap_or(0);
            base.wrapping_add(capacity.wrapping_sub(n) << 2)
        };
        // Delay slot of `bra 0x8c07e4e6`: the element store lands first.
        let _ = mem.write_u32(elem_addr, new_element);
        let index = mem.read_u32(idx_addr).unwrap_or(0); // mov.l @(4,r4),r1
        let new_index = index.wrapping_sub(n); // sub r5,r1
        cpu.r[1] = new_index;
        let _ = mem.write_u32(idx_addr, new_index);
        cpu.r[0] = cursor; // mov r4,r0
        cpu.set_t(fits);
    } else {
        let magnitude = n.wrapping_neg();
        if magnitude == 0x8000_0000 {
            return; // see module comment: the original never returns here
        }
        // neg r5,r7 ; cmp/pz r7 ; bt 0x8c07e492
        let descriptor = mem.read_u32(cursor).unwrap_or(0);
        cpu.r[6] = descriptor;
        let limit = mem.read_u32(descriptor.wrapping_add(8)).unwrap_or(0);
        let capacity = mem.read_u32(descriptor).unwrap_or(0);
        cpu.r[3] = capacity;
        // add r0,r1 ; add r7,r1
        let reached = limit.wrapping_add(index).wrapping_add(magnitude);
        // cmp/hi r1,r3 — unsigned; this is both the branch and the final T.
        let in_window = capacity > reached;
        let element = mem.read_u32(elem_addr).unwrap_or(0);
        let new_element = if in_window {
            // shll2 r7 ; add r7,r0
            element.wrapping_add(magnitude << 2)
        } else {
            // mov.l @(12,r6),r0 ; sub r3,r1 ; shll2 r1 ; add r0,r1
            let base = mem.read_u32(descriptor.wrapping_add(12)).unwrap_or(0);
            base.wrapping_add(reached.wrapping_sub(capacity) << 2)
        };
        // Delay slot of `bra 0x8c07e4b6`: the element store lands first.
        let _ = mem.write_u32(elem_addr, new_element);
        let index = mem.read_u32(idx_addr).unwrap_or(0); // mov.l @(4,r4),r1
        let new_index = index.wrapping_sub(n); // sub r5,r1
        cpu.r[1] = new_index;
        let _ = mem.write_u32(idx_addr, new_index);
        cpu.r[7] = if in_window { magnitude << 2 } else { magnitude };
        cpu.r[0] = cursor; // mov r4,r0
        cpu.set_t(in_window);
    }

    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
