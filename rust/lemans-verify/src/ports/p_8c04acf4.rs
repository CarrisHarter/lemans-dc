//! 0x8c04acf4 — rebuild pairwise distance vectors for `n` objects, then order
//! each object's index-pair list by ascending distance.
//!
//! Param block at r4: `+0` = row pointer array, `+4` = n. `row[i]` is a pointer
//! array; `row[i][i]` is object i, whose `+0xc` points at a position record
//! (`+0x68` = x, `+0x6c` = y) and whose `+0x10` points at a list of (a, b)
//! u32 index pairs.
//!
//! Phase 1 (i < j) reads p_j and p_i, computes dx = pj.x - pi.x,
//! dy = pj.y - pi.y in single precision, and stores to both symmetric cells
//! `row[i][j]` and `row[j][i]` in this order: dx at +4, dy at +8, then
//! dx*dx + dy*dy at +0.
//!
//! Phase 2 bubble-sorts each row's flat u32 index list. The window at list[k]
//! is `(a = list[k], b = list[k+1])` — r9 advances only 4 bytes, so consecutive
//! windows overlap — and it swaps a and b when `row[i][a] > row[i][b]` (f32 at
//! +0). r14 starts at n-2, so an n-element list gets at most n-2 passes; a pass
//! repeats while a swap happened and r14 is still positive.
//!
//! Tail: if the global pointer at 0x8c23dc10 is non-null, it memsets that
//! buffer to zero with length `*(0x8c23dc18) << 2` (call site 0x8c04adf2).
//!
//! Side effects the C hides: r0, r1, r3, r5, r6 and r7 are left as the last
//! phase put them (r7 = 1 from the phase-2 branch delay slot unless the memset
//! call overwrites it with the byte count), and T is always set on exit. The
//! prologue/epilogue only round-trip r8..r14 and PR, so nothing is modelled for
//! the stack. FPU state is single precision: the oracle stores every FR as f32
//! (FPSCR.SZ/PR do not change the decoded operation here).

use sh4_oracle::{Cpu, Memory};

#[inline]
fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

#[inline]
fn ldf(mem: &Memory, addr: u32) -> f32 {
    mem.read_f32(addr).unwrap_or(0.0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_pr = cpu.pr;
    let saved_regs: [u32; 7] = [
        cpu.r[8], cpu.r[9], cpu.r[10], cpu.r[11], cpu.r[12], cpu.r[13], cpu.r[14],
    ];

    // 0x8c04ad04: bra 0x8c04ad7c
    cpu.r[8] = 0; // 0x8c04ad06 delay slot: mov #0,r8

    // ---- phase 1: pairwise dx/dy/distance² ----
    loop {
        // 0x8c04ad7c: mov.l @(4,r4),r0 ; cmp/hi r8,r0 ; bt 0x8c04ad08
        cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(4));
        cpu.set_t(cpu.r[0] > cpu.r[8]);
        if !cpu.t() {
            break;
        }

        cpu.r[1] = ld(mem, cpu.r[4]); // 0x8c04ad08 mov.l @r4,r1
        cpu.r[0] = cpu.r[8] << 2; // 0x8c04ad0a/0c shll2
        cpu.r[1] = ld(mem, cpu.r[1].wrapping_add(cpu.r[0])); // 0x8c04ad0e row_i
        cpu.r[5] = cpu.r[0]; // 0x8c04ad10 mov r0,r5 (i*4)
        cpu.r[14] = cpu.r[8]; // 0x8c04ad12 mov r8,r14
        cpu.r[0] = ld(mem, cpu.r[1].wrapping_add(cpu.r[0])); // 0x8c04ad14 q_i
        cpu.r[14] = cpu.r[14].wrapping_add(1); // 0x8c04ad16 add #1,r14
        cpu.r[7] = ld(mem, cpu.r[0].wrapping_add(12)); // 0x8c04ad18
        cpu.r[7] = cpu.r[7].wrapping_add(104); // 0x8c04ad1a add #104 -> p_i
        cpu.r[1] = cpu.r[7]; // 0x8c04ad1c mov r7,r1
        cpu.r[1] = cpu.r[1].wrapping_add(4); // 0x8c04ad20 delay: add #4,r1

        loop {
            // 0x8c04ad74: mov.l @(4,r4),r0 ; cmp/hi r14,r0 ; bt 0x8c04ad22
            cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(4));
            cpu.set_t(cpu.r[0] > cpu.r[14]);
            if !cpu.t() {
                break;
            }

            cpu.r[10] = cpu.r[14]; // 0x8c04ad22 mov r14,r10
            cpu.r[10] = cpu.r[10] << 2; // 0x8c04ad24 shll2 r10 (j*4)
            cpu.r[3] = ld(mem, cpu.r[4]); // 0x8c04ad26 mov.l @r4,r3
            cpu.r[0] = cpu.r[10]; // 0x8c04ad28 mov r10,r0
            cpu.r[9] = ld(mem, cpu.r[3].wrapping_add(cpu.r[0])); // 0x8c04ad2a row_j
            cpu.r[6] = ld(mem, cpu.r[9].wrapping_add(cpu.r[0])); // 0x8c04ad2c q_j
            cpu.r[0] = cpu.r[5]; // 0x8c04ad2e mov r5,r0 (i*4)
            cpu.r[3] = ld(mem, cpu.r[3].wrapping_add(cpu.r[0])); // 0x8c04ad30 row_i
            cpu.r[6] = ld(mem, cpu.r[6].wrapping_add(12)); // 0x8c04ad32
            cpu.r[0] = cpu.r[10]; // 0x8c04ad34 mov r10,r0
            cpu.r[3] = ld(mem, cpu.r[3].wrapping_add(cpu.r[0])); // 0x8c04ad36 a
            cpu.r[6] = cpu.r[6].wrapping_add(104); // 0x8c04ad38 -> p_j
            // 0x8c04ad3a pref @r3 (cache hint, no architectural effect)
            cpu.r[0] = cpu.r[5]; // 0x8c04ad3c mov r5,r0
            cpu.r[12] = ld(mem, cpu.r[9].wrapping_add(cpu.r[0])); // 0x8c04ad3e b
            // 0x8c04ad40 pref @r12 (cache hint, no architectural effect)
            cpu.fr[2] = ldf(mem, cpu.r[6]); // 0x8c04ad42 fmov @r6,fr2 (pj.x)
            cpu.r[11] = cpu.r[3]; // 0x8c04ad44 mov r3,r11
            cpu.fr[0] = ldf(mem, cpu.r[7]); // 0x8c04ad46 fmov @r7,fr0 (pi.x)
            cpu.r[6] = cpu.r[6].wrapping_add(4); // 0x8c04ad48 add #4,r6
            cpu.r[10] = cpu.r[12]; // 0x8c04ad4a mov r12,r10
            cpu.fr[2] = cpu.fr[2] - cpu.fr[0]; // 0x8c04ad4c fsub fr0,fr2 (dx)
            cpu.r[9] = cpu.r[3]; // 0x8c04ad4e mov r3,r9
            cpu.r[0] = cpu.r[12]; // 0x8c04ad50 mov r12,r0
            cpu.fr[1] = ldf(mem, cpu.r[6]); // 0x8c04ad52 fmov @r6,fr1 (pj.y)
            cpu.r[11] = cpu.r[11].wrapping_add(4); // 0x8c04ad54 -> a+4
            cpu.fr[0] = ldf(mem, cpu.r[1]); // 0x8c04ad56 fmov @r1,fr0 (pi.y)
            cpu.r[10] = cpu.r[10].wrapping_add(4); // 0x8c04ad58 -> b+4
            cpu.r[9] = cpu.r[9].wrapping_add(8); // 0x8c04ad5a -> a+8
            let _ = mem.write_f32(cpu.r[11], cpu.fr[2]); // 0x8c04ad5c a[4] = dx
            cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // 0x8c04ad5e fsub fr0,fr1 (dy)
            let _ = mem.write_f32(cpu.r[10], cpu.fr[2]); // 0x8c04ad60 b[4] = dx
            cpu.r[0] = cpu.r[0].wrapping_add(8); // 0x8c04ad62 -> b+8
            let _ = mem.write_f32(cpu.r[9], cpu.fr[1]); // 0x8c04ad64 a[8] = dy
            cpu.fr[2] = cpu.fr[2] * cpu.fr[2]; // 0x8c04ad66 fmul fr2,fr2
            let _ = mem.write_f32(cpu.r[0], cpu.fr[1]); // 0x8c04ad68 b[8] = dy
            cpu.r[14] = cpu.r[14].wrapping_add(1); // 0x8c04ad6a add #1,r14
            cpu.fr[1] = cpu.fr[1] * cpu.fr[1]; // 0x8c04ad6c fmul fr1,fr1
            cpu.fr[2] = cpu.fr[2] + cpu.fr[1]; // 0x8c04ad6e fadd fr1,fr2
            let _ = mem.write_f32(cpu.r[3], cpu.fr[2]); // 0x8c04ad70 a[0] = dist²
            let _ = mem.write_f32(cpu.r[12], cpu.fr[2]); // 0x8c04ad72 b[0] = dist²
        }
        cpu.r[8] = cpu.r[8].wrapping_add(1); // 0x8c04ad7a add #1,r8
    }

    // ---- phase 2: order each pair list by ascending distance ----
    cpu.r[8] = 0; // 0x8c04ad82 mov #0,r8
    cpu.r[6] = cpu.r[8]; // 0x8c04ad84 mov r8,r6
    cpu.r[7] = 1; // 0x8c04ad88 delay slot: mov #1,r7
    loop {
        // 0x8c04addc: mov.l @(4,r4),r0 ; cmp/hi r8,r0 ; bt 0x8c04ad8a
        cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(4));
        cpu.set_t(cpu.r[0] > cpu.r[8]);
        if !cpu.t() {
            break;
        }

        cpu.r[1] = ld(mem, cpu.r[4]); // 0x8c04ad8a mov.l @r4,r1
        cpu.r[0] = cpu.r[8] << 2; // 0x8c04ad8c/8e shll2 (i*4)
        cpu.r[12] = ld(mem, cpu.r[1].wrapping_add(cpu.r[0])); // 0x8c04ad90 row_i
        cpu.r[10] = cpu.r[7]; // 0x8c04ad92 mov r7,r10 (flag = 1)
        cpu.r[14] = ld(mem, cpu.r[4].wrapping_add(4)); // 0x8c04ad94 n
        cpu.r[1] = ld(mem, cpu.r[12].wrapping_add(cpu.r[0])); // 0x8c04ad96 q_i
        cpu.r[0] = 0xFFFF_FFFE; // 0x8c04ad98 mov.l pool -> -2
        cpu.r[11] = ld(mem, cpu.r[1].wrapping_add(16)); // 0x8c04ad9a list
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // 0x8c04ad9e r14 = n-2

        loop {
            // 0x8c04add0: cmp/pl r14 ; bf 0x8c04adda
            cpu.set_t((cpu.r[14] as i32) > 0);
            if !cpu.t() {
                break;
            }
            cpu.r[0] = cpu.r[10] & 0xFF; // 0x8c04add4 extu.b r10,r0
            cpu.set_t(cpu.r[0] == 0); // 0x8c04add6 tst r0,r0
            if cpu.t() {
                break; // 0x8c04add8 bf ada0 not taken -> 0x8c04adda
            }

            cpu.r[10] = cpu.r[6]; // 0x8c04ada0 mov r6,r10 (clear flag)
            cpu.r[9] = cpu.r[11]; // 0x8c04ada2 mov r11,r9
            cpu.r[13] = cpu.r[6]; // 0x8c04ada6 delay slot: mov r6,r13

            loop {
                // 0x8c04adca: cmp/gt r13,r14 ; bt 0x8c04ada8
                cpu.set_t((cpu.r[14] as i32) > (cpu.r[13] as i32));
                if !cpu.t() {
                    break;
                }

                cpu.r[5] = ld(mem, cpu.r[9]); // 0x8c04ada8 pair.a
                cpu.r[1] = ld(mem, cpu.r[9].wrapping_add(4)); // 0x8c04adaa pair.b
                cpu.r[0] = cpu.r[5] << 2; // 0x8c04adac/ae
                cpu.r[3] = ld(mem, cpu.r[12].wrapping_add(cpu.r[0])); // 0x8c04adb0 row_i[a]
                cpu.r[0] = cpu.r[1] << 2; // 0x8c04adb2/b4
                cpu.r[0] = ld(mem, cpu.r[12].wrapping_add(cpu.r[0])); // 0x8c04adb6 row_i[b]
                cpu.fr[1] = ldf(mem, cpu.r[3]); // 0x8c04adb8
                cpu.fr[0] = ldf(mem, cpu.r[0]); // 0x8c04adba
                cpu.set_t(cpu.fr[1] > cpu.fr[0]); // 0x8c04adbc fcmp/gt fr0,fr1
                if cpu.t() {
                    // 0x8c04adbe bf 0x8c04adc6 not taken
                    let _ = mem.write_u32(cpu.r[9], cpu.r[1]); // 0x8c04adc0
                    cpu.r[10] = cpu.r[7]; // 0x8c04adc2 flag = 1
                    let _ = mem.write_u32(cpu.r[9].wrapping_add(4), cpu.r[5]); // 0x8c04adc4
                }
                cpu.r[9] = cpu.r[9].wrapping_add(4); // 0x8c04adc6
                cpu.r[13] = cpu.r[13].wrapping_add(1); // 0x8c04adc8
            }
            cpu.r[14] = cpu.r[14].wrapping_sub(1); // 0x8c04adce add #-1,r14
        }
        cpu.r[8] = cpu.r[8].wrapping_add(1); // 0x8c04adda add #1,r8
    }

    // ---- tail: clear the optional global buffer ----
    cpu.r[0] = 0x8c23dc10; // 0x8c04ade2 mov.l pool -> DAT_8c23dc10
    cpu.r[4] = ld(mem, cpu.r[0]); // 0x8c04ade4 mov.l @r0,r4
    cpu.set_t(cpu.r[4] == 0); // 0x8c04ade6 tst r4,r4
    if !cpu.t() {
        cpu.r[0] = 0x8c23dc18; // 0x8c04adea mov.l pool -> DAT_8c23dc18
        cpu.r[5] = 0; // 0x8c04adec mov #0,r5
        cpu.r[6] = ld(mem, cpu.r[0]); // 0x8c04adee mov.l @r0,r6
        cpu.r[0] = 0x8c010f88; // 0x8c04adf0 mov.l pool -> memset
        cpu.r[6] = cpu.r[6] << 2; // 0x8c04adf4 delay slot: shll2 r6
        cpu.pr = 0x8c04adf6; // 0x8c04adf2 jsr @r0 return address
        crate::registry::p_8c010f88::run(cpu, mem);
    }

    // 0x8c04adf6 epilogue: lds.l @r15+,pr and the r8..r14 pop round-trip.
    cpu.r[8] = saved_regs[0];
    cpu.r[9] = saved_regs[1];
    cpu.r[10] = saved_regs[2];
    cpu.r[11] = saved_regs[3];
    cpu.r[12] = saved_regs[4];
    cpu.r[13] = saved_regs[5];
    cpu.r[14] = saved_regs[6];
    cpu.pr = saved_pr;
}
