//! 0x8c06c16c — walk the node list headed at [r4+0xc] (next at +0x44) and,
//! for the first node with ([node+0x58]&r6)!=0 and ([node+0x68]!=[r5+0x68]
//! or [r5+0x68]==0), append the pair (r5, node) to the pair table
//! ([r4+0x1c], count [r4+0x24], floats [r4+0x20]) unless the pair is
//! already present. Node==r5 and matching non-zero ids skip the node.
//!
//! CFG: prologue -> LOOP(0x8c06c18c)
//!      -> {skip via 0x8c06c192 | ADV 0x8c06c226 | skip via 0x8c06c1b4
//!          | SEARCH 0x8c06c1b8 -> {FOUND r0=1 -> 0x8c06c1f6 | no-match ->
//!            APPEND 0x8c06c1fc} -> ADV 0x8c06c226} -> CHECK(0x8c06c22a)
//!      -> LOOP | epilogue(0x8c06c22e).
//!
//! Only bra/rts are delayed: delay slots that fire are `mov.l @r7,r7`
//! (0x8c06c194/0x8c06c1b6/0x8c06c1fa), `mov r1,r0` (0x8c06c1be/0x8c06c1ee),
//! `mov r10,r0` (0x8c06c1d4/0x8c06c1e4) and the rts pop (0x8c06c23c).
//! Plain bt/bf do NOT run their delay slots in this ISA.
//!
//! Side effects the C does not show: r8 := -1, r3 := r5+0x68, r1 := 0,
//! r10 := 1, fr0 := 0.0 (fldi0) and is stored to the float table on append;
//! r0 ends as 1 (pair found) or 0 (appended); r9/r11/r12/r13/r14 hold
//! search-loop leftovers; shll r9 at 0x8c06c218 sets T to the shifted-out
//! bit (overwritten by the final tst); exit T = 1 (r7==0 at CHECK).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}
#[inline]
fn st(mem: &mut Memory, addr: u32, v: u32) {
    let _ = mem.write_u32(addr, v);
}
#[inline]
fn push(cpu: &mut Cpu, mem: &mut Memory, n: usize) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    st(mem, cpu.r[15], cpu.r[n]);
}
#[inline]
fn pop(cpu: &mut Cpu, mem: &Memory, n: usize) {
    cpu.r[n] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
#[inline]
fn shll(cpu: &mut Cpu, n: usize) {
    let out = cpu.r[n] >> 31 != 0;
    cpu.r[n] <<= 1;
    cpu.set_t(out);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, 14);
    push(cpu, mem, 13);
    push(cpu, mem, 12);
    push(cpu, mem, 11);
    push(cpu, mem, 10);
    push(cpu, mem, 9);
    push(cpu, mem, 8);
    cpu.r[7] = ld(mem, cpu.r[4].wrapping_add(12)); // mov.l @(12,r4),r7
    cpu.set_t(cpu.r[7] == 0); // tst r7,r7
    if !cpu.t() {
        // bt 0x8c06c22e not taken: head != 0
        cpu.r[8] = 0xffff_ffff; // mov #-1,r8
        cpu.r[3] = cpu.r[5].wrapping_add(104); // mov r5,r3; add #104,r3
        cpu.fr[0] = 0.0; // fldi0 fr0
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[10] = 1; // mov #1,r10
        loop {
            // LOOP 0x8c06c18c
            cpu.set_t(cpu.r[7] == cpu.r[5]); // cmp/eq r5,r7
            if cpu.t() {
                // 0x8c06c190: add #68,r7; bra 0x8c06c22a (delay: mov.l @r7,r7)
                cpu.r[7] = cpu.r[7].wrapping_add(68);
                cpu.r[7] = ld(mem, cpu.r[7]);
            } else {
                // 0x8c06c196
                cpu.r[0] = ld(mem, cpu.r[7].wrapping_add(88)); // [r7+0x58]
                cpu.r[0] &= cpu.r[6];
                cpu.set_t(cpu.r[0] == 0); // tst r0,r0
                if cpu.t() {
                    // bt 0x8c06c226 -> ADV
                    cpu.r[7] = cpu.r[7].wrapping_add(68);
                    cpu.r[7] = ld(mem, cpu.r[7]);
                } else {
                    // 0x8c06c1a2
                    cpu.r[0] = cpu.r[7].wrapping_add(104); // add #104,r0
                    cpu.r[9] = ld(mem, cpu.r[3]); // mov.l @r3,r9 ([r5+0x68])
                    cpu.r[0] = ld(mem, cpu.r[0]); // mov.l @r0,r0 ([r7+0x68])
                    cpu.set_t(cpu.r[0] == cpu.r[9]); // cmp/eq r0,r9
                    let eq = cpu.t();
                    cpu.set_t(cpu.r[9] & cpu.r[9] == 0); // tst r9,r9 (0x8c06c1ae)
                    let id_zero = cpu.t();
                    if eq && !id_zero {
                        // 0x8c06c1b2: add #68,r7; bra 0x8c06c22a (delay: @r7,r7)
                        cpu.r[7] = cpu.r[7].wrapping_add(68);
                        cpu.r[7] = ld(mem, cpu.r[7]);
                    } else {
                        // SEARCH 0x8c06c1b8
                        cpu.r[13] = ld(mem, cpu.r[4].wrapping_add(36)); // count
                        cpu.r[14] = cpu.r[1]; // mov r1,r14
                        cpu.r[0] = cpu.r[1]; // bra delay 0x8c06c1be
                        loop {
                            // 0x8c06c1ea
                            cpu.set_t((cpu.r[13] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r13
                            if !cpu.t() {
                                cpu.r[0] = cpu.r[1]; // 0x8c06c1ee
                                break;
                            }
                            // 0x8c06c1c0
                            cpu.r[12] = ld(mem, cpu.r[4].wrapping_add(28)); // mov.l @(28,r4),r12
                            cpu.r[11] = ld(mem, cpu.r[0].wrapping_add(cpu.r[12])); // mov.l @(r0,r12),r11
                            cpu.set_t(cpu.r[11] == cpu.r[5]); // cmp/eq r5,r11
                            let mut found = false;
                            if cpu.t() {
                                // 0x8c06c1c8
                                cpu.r[9] = cpu.r[12].wrapping_add(4); // add #4,r9
                                cpu.r[9] = ld(mem, cpu.r[0].wrapping_add(cpu.r[9])); // mov.l @(r0,r9),r9
                                cpu.set_t(cpu.r[9] == cpu.r[7]); // cmp/eq r7,r9
                                if cpu.t() {
                                    cpu.r[0] = cpu.r[10]; // bra delay 0x8c06c1d4
                                    found = true;
                                }
                            }
                            if !found {
                                // 0x8c06c1d6
                                cpu.set_t(cpu.r[11] == cpu.r[7]); // cmp/eq r7,r11
                                if cpu.t() {
                                    cpu.r[12] = cpu.r[12].wrapping_add(4); // add #4,r12 (0x8c06c1da)
                                    cpu.r[9] = ld(mem, cpu.r[0].wrapping_add(cpu.r[12])); // mov.l @(r0,r12),r9
                                    cpu.set_t(cpu.r[9] == cpu.r[5]); // cmp/eq r5,r9
                                    if cpu.t() {
                                        cpu.r[0] = cpu.r[10]; // bra delay 0x8c06c1e4
                                        found = true;
                                    }
                                }
                            }
                            if found {
                                break; // -> 0x8c06c1f0 with r0 = 1
                            }
                            cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
                            cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
                        }
                        cpu.r[0] &= 0xff; // extu.b r0,r0
                        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
                        if cpu.t() {
                            // bt 0x8c06c1fc -> APPEND
                            cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(36)); // mov.l @(36,r4),r0
                            cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
                            st(mem, cpu.r[4].wrapping_add(36), cpu.r[0]); // mov.l r0,@(36,r4)
                            cpu.r[9] = ld(mem, cpu.r[4].wrapping_add(36)); // mov.l @(36,r4),r9
                            cpu.r[11] = ld(mem, cpu.r[4].wrapping_add(32)); // mov.l @(32,r4),r11
                            cpu.r[9] <<= 2; // shll2 r9
                            cpu.r[0] = 0xffff_fffc; // mov.l 0x8c06c240,r0
                            cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[0]); // add r0,r11
                            cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[9]); // add r9,r11
                            st(mem, cpu.r[11], cpu.fr[0].to_bits()); // fmov fr0,@r11
                            cpu.r[11] = ld(mem, cpu.r[4].wrapping_add(36)); // mov.l @(36,r4),r11
                            cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(28)); // mov.l @(28,r4),r0
                            cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[8]); // add r8,r11
                            cpu.r[9] = cpu.r[11]; // mov r11,r9
                            shll(cpu, 9); // shll r9
                            cpu.r[9] = cpu.r[9].wrapping_add(cpu.r[11]); // add r11,r9
                            cpu.r[9] <<= 2; // shll2 r9
                            cpu.r[9] = cpu.r[9].wrapping_add(cpu.r[0]); // add r0,r9
                            st(mem, cpu.r[9], cpu.r[5]); // mov.l r5,@r9
                            st(mem, cpu.r[9].wrapping_add(4), cpu.r[7]); // mov.l r7,@(4,r9)
                            st(mem, cpu.r[9].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r9)
                        } else {
                            // found (r0==1): 0x8c06c1f6 r7+=68 + bra delay r7=[r7]
                            // is the same single advance the ADV block below performs
                        }
                        // ADV 0x8c06c226 (append falls through here)
                        cpu.r[7] = cpu.r[7].wrapping_add(68);
                        cpu.r[7] = ld(mem, cpu.r[7]);
                    }
                }
            }
            // CHECK 0x8c06c22a
            cpu.set_t(cpu.r[7] == 0); // tst r7,r7
            if cpu.t() {
                break; // bf not taken -> epilogue
            }
        }
    }
    // epilogue 0x8c06c22e
    pop(cpu, mem, 8);
    pop(cpu, mem, 9);
    pop(cpu, mem, 10);
    pop(cpu, mem, 11);
    pop(cpu, mem, 12);
    pop(cpu, mem, 13);
    pop(cpu, mem, 14); // rts delay slot 0x8c06c23c
}
