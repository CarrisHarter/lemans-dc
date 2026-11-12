//! 0x8c026b64 — red-black tree insertion fix-up (`_Rb_tree_rebalance` shape).
//!
//! `r4` is the freshly inserted node, `r5` the root/sentinel node it must not
//! reach. Node layout: +0 left, +4 right, +8 parent pointer whose low bit is
//! the colour flag (1 = red, 0 = black). The port keeps the procedure's root
//! copy in the `add #-4,r15` slot: rotations receive `r5 = r15` and may rewrite
//! that slot, exactly like the original.
//!
//! Literal-pool loads are read from the image (0x8c026c84 = 0xfffffffe,
//! 0x8c026c88 = 0x8c026ce0, 0x8c026c8c = 0x8c026c90). The two callees are
//! `rotate_left` (0x8c026ce0) and `rotate_right` (0x8c026c90); they are called
//! at each `jsr` with the original delay-slot argument already in place.
//!
//! Side effects the C hides: `subc rX,rX`/`add #1` leave r10/r13/r14 as the
//! tested colour bit; the exits always clear the low bit of the current root's
//! +8 field (`shlr`/`shll` leave T = 0); r1 holds the root copy and r0 its
//! masked +8 field on return. The stack teardown restores r10-r14 and pr, so
//! r15 and the callee-saved registers end where they started.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: push r14, r13, r12, r11, r10, pr; reserve the root slot.
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
    cpu.r[11] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
    cpu.r[12] = mem.read_u32(0x8c026c84).unwrap_or(0); // 0xfffffffe
    cpu.r[0] |= 1;
    let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]); // delay of `bra`

    'rebalance: loop {
        // 0x8c026c54: while (node != root && parent->f8 & 1 != 0)
        cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.set_t(cpu.r[1] == cpu.r[11]);
        if cpu.r[1] == cpu.r[11] {
            break;
        }
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.r[4] &= cpu.r[12]; // parent
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
        cpu.set_t((cpu.r[0] & 1) == 0); // 0x8c026c60 tst #1,r0
        // 0x8c026c62 subc r10,r10 ; 0x8c026c64 add #1,r10 ; 0x8c026c66 mov r10,r0
        cpu.r[10] = 0u32.wrapping_sub(cpu.t() as u32).wrapping_add(1);
        cpu.r[0] = cpu.r[10];
        cpu.set_t(cpu.r[10] == 1); // 0x8c026c68 cmp/eq #1,r0
        if cpu.r[10] != 1 {
            break;
        }

        // 0x8c026b80: grandparent and the left/right split.
        let gp = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0) & cpu.r[12];
        cpu.r[0] = gp;
        cpu.r[1] = mem.read_u32(gp).unwrap_or(0);
        cpu.set_t(cpu.r[1] == cpu.r[4]);
        if cpu.r[1] != cpu.r[4] {
            // 0x8c026bf2: parent is the right child; r1 is the left uncle.
            cpu.set_t(cpu.r[1] == 0);
            if cpu.r[1] != 0 {
                cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
                cpu.set_t((cpu.r[0] & 1) == 0);
                cpu.r[14] = 0u32.wrapping_sub(cpu.t() as u32).wrapping_add(1);
                cpu.r[0] = cpu.r[14];
                cpu.set_t(cpu.r[14] == 1);
                if cpu.r[14] == 1 {
                    // 0x8c026c04: red uncle, recolour and climb to the grandparent.
                    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
                    cpu.r[0] &= cpu.r[12];
                    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
                    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
                    cpu.r[0] &= cpu.r[12];
                    let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]);
                    cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
                    cpu.r[0] &= cpu.r[12];
                    cpu.r[11] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
                    cpu.r[11] &= cpu.r[12];
                    cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
                    cpu.r[0] |= 1;
                    let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]);
                    continue 'rebalance;
                }
            }
            // 0x8c026c20: black/null uncle, rotate right then left.
            cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.set_t(cpu.r[0] == cpu.r[11]);
            if cpu.r[0] == cpu.r[11] {
                cpu.r[0] = mem.read_u32(0x8c026c8c).unwrap_or(0); // 0x8c026c90
                cpu.r[11] = cpu.r[4];
                cpu.r[5] = cpu.r[15]; // delay slot of `jsr`
                crate::registry::p_8c026c90::run(cpu, mem);
            }
            cpu.r[3] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
            cpu.r[5] = cpu.r[15];
            cpu.r[1] = mem.read_u32(0x8c026c88).unwrap_or(0); // 0x8c026ce0
            cpu.r[3] &= cpu.r[12];
            cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] &= cpu.r[12];
            let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
            cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] &= cpu.r[12];
            cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
            cpu.r[3] &= cpu.r[12];
            cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] |= 1;
            let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
            cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
            cpu.r[0] &= cpu.r[12];
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
            cpu.r[4] &= cpu.r[12]; // delay slot: and r12,r4
            crate::registry::p_8c026ce0::run(cpu, mem);
            continue 'rebalance;
        }

        // 0x8c026b8a: parent is the left child; r1 is the right uncle.
        cpu.r[1] = mem.read_u32(gp.wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[1] == 0);
        if cpu.r[1] != 0 {
            cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
            cpu.set_t((cpu.r[0] & 1) == 0);
            cpu.r[13] = 0u32.wrapping_sub(cpu.t() as u32).wrapping_add(1);
            cpu.r[0] = cpu.r[13];
            cpu.set_t(cpu.r[13] == 1);
            if cpu.r[13] == 1 {
                // 0x8c026b9e: red uncle, recolour and climb to the grandparent.
                cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] &= cpu.r[12];
                let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]);
                cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] &= cpu.r[12];
                let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]);
                cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] &= cpu.r[12];
                cpu.r[11] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
                cpu.r[11] &= cpu.r[12];
                cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] |= 1;
                let _ = mem.write_u32(cpu.r[11].wrapping_add(8), cpu.r[0]);
                continue 'rebalance;
            }
        }
        // 0x8c026bba: black/null uncle, rotate left then right.
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[11]);
        if cpu.r[0] == cpu.r[11] {
            cpu.r[0] = mem.read_u32(0x8c026c88).unwrap_or(0); // 0x8c026ce0
            cpu.r[11] = cpu.r[4];
            cpu.r[5] = cpu.r[15]; // delay slot of `jsr`
            crate::registry::p_8c026ce0::run(cpu, mem);
        }
        cpu.r[3] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.r[5] = cpu.r[15];
        cpu.r[1] = mem.read_u32(0x8c026c8c).unwrap_or(0); // 0x8c026c90
        cpu.r[3] &= cpu.r[12];
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] &= cpu.r[12];
        let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] &= cpu.r[12];
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
        cpu.r[3] &= cpu.r[12];
        cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] |= 1;
        let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] &= cpu.r[12];
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
        cpu.r[4] &= cpu.r[12]; // delay slot: and r12,r4
        crate::registry::p_8c026c90::run(cpu, mem);
        continue 'rebalance;
    }

    // 0x8c026c6c: root colour = black, then tear down the frame.
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
    cpu.set_t((cpu.r[0] & 1) != 0); // shlr r0
    cpu.r[0] >>= 1;
    cpu.set_t((cpu.r[0] >> 31) != 0); // shll r0
    cpu.r[0] <<= 1;
    let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
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
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
