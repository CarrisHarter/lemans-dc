//! 0x8c0d3258 — depth-first tree surgery keyed on a name substring.
//!
//! `r4` = value handed to the recursive calls (unused otherwise), `r5` = node,
//! `r6` = the new-parent argument for 0x8c05cb48.
//!
//! ```text
//! r13 = node ; r12 = r6 ; r11 = r4
//! r4 = 0x8c05b2ac(node)                     ; node->field_0x24 ?: fallback
//! r5 = 0x8c1ff690 + [0x8c0d32e8] (= 0x8c1ff78b, "instancetree")
//! r0 = 0x8c0113f4(r4, r5)                   ; substring search, 0 = no match
//! if r0 != 0: 0x8c05cb48(node, r6, 0) ; return
//! r14 = node->child                         ; [node+0x0c]
//! while r14 != 0:
//!     r0 = 0x8c0113f4(name(r14), needle)
//!     r13 = r14->next                       ; [r14+0x08], delay slot
//!     if r0 != 0: 0x8c05cb48(r14, r6, 0)
//!     else:
//!         r5 = r14->child
//!         while r5 != 0:
//!             r14 = r5->next
//!             f(r11, r5, r12)               ; self-call
//!             r5 = r14
//!     r14 = r13
//! ```
//!
//! The needle is the byte string at 0x8c1ff78b (`"instancetree"`), not the
//! `TOGGLE_BILINEAR` label Ghidra attached to 0x8c1ff690: the literal `0xfb` is
//! added to that base. A nonzero search result (match) destroys the node;
//! a zero result (no match) descends into its children.
//!
//! T is 1 on every exit: the final check before the epilogue is always a `tst`
//! of a register known to be zero (`tst r14,r14` before the early `bt`,
//! `tst r13,r13` before the loop-back `bf.s`, or the callee's own T after the
//! two destroy paths — 0x8c05cb48 also returns with T = 1).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d3258  mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0d325a  mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0d325c  mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0d325e  mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c0d3260  mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c0d3262  mov r6,r12
    cpu.r[12] = cpu.r[6];
    // 0x8c0d3264  sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0d3266  mov r4,r11
    cpu.r[11] = cpu.r[4];

    // 0x8c0d3268  mov.l 0x8c0d32e0,r0  (0x8c05b2ac)
    cpu.r[0] = 0x8c05b2ac;
    // 0x8c0d326a  jsr @r0 ; 0x8c0d326c mov r13,r4 (delay slot)
    cpu.pr = 0x8c0d326e;
    cpu.r[4] = cpu.r[13];
    crate::registry::p_8c05b2ac::run(cpu, mem);
    // 0x8c0d326e  mov r0,r4
    cpu.r[4] = cpu.r[0];
    // 0x8c0d3270  mov.l 0x8c0d32e4,r5  (0x8c1ff690)
    cpu.r[5] = 0x8c1ff690;
    // 0x8c0d3272  mov.l 0x8c0d32e8,r1  (0xfb)
    cpu.r[1] = 0xfb;
    // 0x8c0d3274  mov.l 0x8c0d32ec,r0  (0x8c0113f4)
    cpu.r[0] = 0x8c0113f4;
    // 0x8c0d3276  jsr @r0 ; 0x8c0d3278 add r1,r5 (delay slot)
    cpu.pr = 0x8c0d327a;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
    crate::registry::p_8c0113f4::run(cpu, mem);
    // 0x8c0d327a  tst r0,r0 ; 0x8c0d327c bt 0x8c0d328c
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // 0x8c0d327e  mov.l 0x8c0d32f0,r0  (0x8c05cb48)
        cpu.r[0] = 0x8c05cb48;
        // 0x8c0d3280  mov r12,r5
        cpu.r[5] = cpu.r[12];
        // 0x8c0d3282  mov r13,r4
        cpu.r[4] = cpu.r[13];
        // 0x8c0d3284  jsr @r0 ; 0x8c0d3286 mov #0,r6 (delay slot)
        cpu.pr = 0x8c0d3288;
        cpu.r[6] = 0;
        crate::registry::p_8c05cb48::run(cpu, mem);
        // 0x8c0d3288  bra 0x8c0d32d4 ; 0x8c0d328a nop (delay slot)
        epilogue(cpu, mem);
        return;
    }

    // 0x8c0d328c  mov.l @(12,r13),r14
    cpu.r[14] = rd(mem, cpu.r[13].wrapping_add(12));
    // 0x8c0d328e  tst r14,r14 ; 0x8c0d3290 bt 0x8c0d32d4
    cpu.set_t(cpu.r[14] == 0);
    if cpu.t() {
        epilogue(cpu, mem);
        return;
    }

    loop {
        // 0x8c0d3292  mov.l 0x8c0d32e0,r0  (0x8c05b2ac)
        cpu.r[0] = 0x8c05b2ac;
        // 0x8c0d3294  mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c0d3296  jsr @r0 ; 0x8c0d3298 mov.l @(8,r14),r13 (delay slot)
        cpu.pr = 0x8c0d329a;
        cpu.r[13] = rd(mem, cpu.r[14].wrapping_add(8));
        crate::registry::p_8c05b2ac::run(cpu, mem);
        // 0x8c0d329a  mov r0,r4
        cpu.r[4] = cpu.r[0];
        // 0x8c0d329c  mov.l 0x8c0d32e4,r5
        cpu.r[5] = 0x8c1ff690;
        // 0x8c0d329e  mov.l 0x8c0d32e8,r1
        cpu.r[1] = 0xfb;
        // 0x8c0d32a0  mov.l 0x8c0d32ec,r0
        cpu.r[0] = 0x8c0113f4;
        // 0x8c0d32a2  jsr @r0 ; 0x8c0d32a4 add r1,r5 (delay slot)
        cpu.pr = 0x8c0d32a6;
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]);
        crate::registry::p_8c0113f4::run(cpu, mem);
        // 0x8c0d32a6  tst r0,r0 ; 0x8c0d32a8 bt 0x8c0d32b8
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            // 0x8c0d32aa  mov.l 0x8c0d32f0,r0  (0x8c05cb48)
            cpu.r[0] = 0x8c05cb48;
            // 0x8c0d32ac  mov r12,r5
            cpu.r[5] = cpu.r[12];
            // 0x8c0d32ae  mov r14,r4
            cpu.r[4] = cpu.r[14];
            // 0x8c0d32b0  jsr @r0 ; 0x8c0d32b2 mov #0,r6 (delay slot)
            cpu.pr = 0x8c0d32b4;
            cpu.r[6] = 0;
            crate::registry::p_8c05cb48::run(cpu, mem);
            // 0x8c0d32b4  bra 0x8c0d32ce ; 0x8c0d32b6 nop (delay slot)
        } else {
            // 0x8c0d32b8  mov.l @(12,r14),r5
            cpu.r[5] = rd(mem, cpu.r[14].wrapping_add(12));
            // 0x8c0d32ba  tst r5,r5 ; 0x8c0d32bc bt 0x8c0d32ce
            cpu.set_t(cpu.r[5] == 0);
            if !cpu.t() {
                loop {
                    // 0x8c0d32be  mov.l @(8,r5),r14
                    cpu.r[14] = rd(mem, cpu.r[5].wrapping_add(8));
                    // 0x8c0d32c0  mov r12,r6
                    cpu.r[6] = cpu.r[12];
                    // 0x8c0d32c2  mov.l 0x8c0d32f4,r0  (0x8c0d3258, self)
                    cpu.r[0] = 0x8c0d3258;
                    // 0x8c0d32c4  jsr @r0 ; 0x8c0d32c6 mov r11,r4 (delay slot)
                    cpu.pr = 0x8c0d32c8;
                    cpu.r[4] = cpu.r[11];
                    crate::registry::p_8c0d3258::run(cpu, mem);
                    // 0x8c0d32c8  tst r14,r14 ; 0x8c0d32ca bf.s 0x8c0d32be
                    cpu.set_t(cpu.r[14] == 0);
                    let again = !cpu.t();
                    // 0x8c0d32cc  mov r14,r5 (delay slot)
                    cpu.r[5] = cpu.r[14];
                    if !again {
                        break;
                    }
                }
            }
        }
        // 0x8c0d32ce  tst r13,r13 ; 0x8c0d32d0 bf.s 0x8c0d3292
        cpu.set_t(cpu.r[13] == 0);
        if cpu.t() {
            break;
        }
        // 0x8c0d32d2  mov r13,r14 (delay slot)
        cpu.r[14] = cpu.r[13];
    }

    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d32d4  lds.l @r15+,pr
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d32d6  mov.l @r15+,r11
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d32d8  mov.l @r15+,r12
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d32da  mov.l @r15+,r13
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d32dc  rts ; 0x8c0d32de mov.l @r15+,r14 (delay slot)
    cpu.r[14] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
