//! 0x8c0c0614 — recursive release of a three-deep pointer tree.
//!
//! `r5` points at a struct holding two node pointers. A node is two more
//! pointers; each invocation walks three levels inline, calls itself on the
//! third-level pointer (the literal-pool word at 0x8c0c06dc is this routine's
//! own entry), frees each visited node through the free thunk at 0x8c01032c
//! (literal pool word 0x8c0c06e0), and finally frees the argument struct.
//!
//! The prologue pushes r14/r13/r12/r11/pr; the epilogue pops all of them, so
//! those registers (including r12/r13) end at their ENTRY values, not at the
//! body's `r12 = param_1` / `r13 = param_2`. r15 likewise returns to entry.
//! Every `bt` here has no delay slot; the `jsr` delay slots are modelled
//! before the call. The final T and r0-r6 are whatever the last `free(r13)`
//! leaves, since the epilogue sets no flags and touches none of r0-r10.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c0614  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c0616  mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0c0618  mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0c061a  mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c0c061c  mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c0c061e  mov r4,r12
    cpu.r[12] = cpu.r[4];
    // 0x8c0c0620  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // first pointer of the argument struct
    // 0x8c0c0622  mov.l @r13,r14
    cpu.r[14] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    // 0x8c0c0624  tst r14,r14
    let t = cpu.r[14] == 0;
    cpu.set_t(t);
    if !t {
        // 0x8c0c0628  mov.l @r14,r11
        cpu.r[11] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        // 0x8c0c062a  tst r11,r11
        let t = cpu.r[11] == 0;
        cpu.set_t(t);
        if !t {
            // 0x8c0c062e  mov.l @r11,r5
            cpu.r[5] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            // 0x8c0c0630  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0634  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c0636  jsr @r0 ; delay slot 0x8c0c0638 nop
                cpu.pr = 0x8c0c063a;
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c063a  mov.l @(4,r11),r5
            cpu.r[5] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
            // 0x8c0c063c  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0640  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c0642  jsr @r0 ; delay slot 0x8c0c0644 mov r12,r4
                cpu.pr = 0x8c0c0646;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c0646  mov.l 0x8c0c06e0,r0  (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
            // 0x8c0c0648  jsr @r0 ; delay slot 0x8c0c064a mov r11,r4
            cpu.pr = 0x8c0c064c;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // second child of the first node
        // 0x8c0c064c  mov.l @(4,r14),r11
        cpu.r[11] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        // 0x8c0c064e  tst r11,r11
        let t = cpu.r[11] == 0;
        cpu.set_t(t);
        if !t {
            // 0x8c0c0652  mov.l @r11,r5
            cpu.r[5] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            // 0x8c0c0654  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0658  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c065a  jsr @r0 ; delay slot 0x8c0c065c mov r12,r4
                cpu.pr = 0x8c0c065e;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c065e  mov.l @(4,r11),r5
            cpu.r[5] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
            // 0x8c0c0660  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0664  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c0666  jsr @r0 ; delay slot 0x8c0c0668 mov r12,r4
                cpu.pr = 0x8c0c066a;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c066a  mov.l 0x8c0c06e0,r0  (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
            // 0x8c0c066c  jsr @r0 ; delay slot 0x8c0c066e mov r11,r4
            cpu.pr = 0x8c0c0670;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c0c0670  mov.l 0x8c0c06e0,r0  (0x8c01032c)
        cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
        // 0x8c0c0672  jsr @r0 ; delay slot 0x8c0c0674 mov r14,r4
        cpu.pr = 0x8c0c0676;
        cpu.r[4] = cpu.r[14];
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // second pointer of the argument struct
    // 0x8c0c0676  mov.l @(4,r13),r14
    cpu.r[14] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    // 0x8c0c0678  tst r14,r14
    let t = cpu.r[14] == 0;
    cpu.set_t(t);
    if !t {
        // 0x8c0c067c  mov.l @r14,r11
        cpu.r[11] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        // 0x8c0c067e  tst r11,r11
        let t = cpu.r[11] == 0;
        cpu.set_t(t);
        if !t {
            // 0x8c0c0682  mov.l @r11,r5
            cpu.r[5] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            // 0x8c0c0684  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0688  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c068a  jsr @r0 ; delay slot 0x8c0c068c mov r12,r4
                cpu.pr = 0x8c0c068e;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c068e  mov.l @(4,r11),r5
            cpu.r[5] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
            // 0x8c0c0690  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c0694  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c0696  jsr @r0 ; delay slot 0x8c0c0698 mov r12,r4
                cpu.pr = 0x8c0c069a;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c069a  mov.l 0x8c0c06e0,r0  (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
            // 0x8c0c069c  jsr @r0 ; delay slot 0x8c0c069e mov r11,r4
            cpu.pr = 0x8c0c06a0;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // second child of the second node
        // 0x8c0c06a0  mov.l @(4,r14),r11
        cpu.r[11] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        // 0x8c0c06a2  tst r11,r11
        let t = cpu.r[11] == 0;
        cpu.set_t(t);
        if !t {
            // 0x8c0c06a6  mov.l @r11,r5
            cpu.r[5] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            // 0x8c0c06a8  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c06ac  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c06ae  jsr @r0 ; delay slot 0x8c0c06b0 mov r12,r4
                cpu.pr = 0x8c0c06b2;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c06b2  mov.l @(4,r11),r5
            cpu.r[5] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
            // 0x8c0c06b4  tst r5,r5
            let t = cpu.r[5] == 0;
            cpu.set_t(t);
            if !t {
                // 0x8c0c06b8  mov.l 0x8c0c06dc,r0  (0x8c0c0614)
                cpu.r[0] = mem.read_u32(0x8c0c06dc).unwrap_or(0);
                // 0x8c0c06ba  jsr @r0 ; delay slot 0x8c0c06bc mov r12,r4
                cpu.pr = 0x8c0c06be;
                cpu.r[4] = cpu.r[12];
                crate::registry::p_8c0c0614::run(cpu, mem);
            }
            // 0x8c0c06be  mov.l 0x8c0c06e0,r0  (0x8c01032c)
            cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
            // 0x8c0c06c0  jsr @r0 ; delay slot 0x8c0c06c2 mov r11,r4
            cpu.pr = 0x8c0c06c4;
            cpu.r[4] = cpu.r[11];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c0c06c4  mov.l 0x8c0c06e0,r0  (0x8c01032c)
        cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
        // 0x8c0c06c6  jsr @r0 ; delay slot 0x8c0c06c8 mov r14,r4
        cpu.pr = 0x8c0c06ca;
        cpu.r[4] = cpu.r[14];
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // 0x8c0c06ca  mov.l 0x8c0c06e0,r0  (0x8c01032c)
    cpu.r[0] = mem.read_u32(0x8c0c06e0).unwrap_or(0);
    // 0x8c0c06cc  jsr @r0 ; delay slot 0x8c0c06ce mov r13,r4
    cpu.pr = 0x8c0c06d0;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c01032c::run(cpu, mem);

    // epilogue: restore everything the prologue pushed
    // 0x8c0c06d0  lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c06d2  mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c06d4  mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c06d6  mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c06d8  rts ; delay slot 0x8c0c06da mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
