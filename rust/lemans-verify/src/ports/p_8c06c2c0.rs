//! 0x8c06c2c0 — remove the pair entry (a,b) from the table ([r4+0x1c],
//! count [r4+0x24], floats [r4+0x20]) when a==r5 or b==r5 and the
//! link-test passes: removal happens iff (flags&r6)!=0 and the cross
//! mask ([b+0x5c]&[a+0x58])==0 (case A) / ([a+0x5c]&[b+0x58])==0 (case B).
//!
//! CFG: COND(0x8c06c3a2) -> BODY(0x8c06c2d4)
//!      -> {REMOVE_A(0x8c06c2f6) | REMOVE_B(0x8c06c35a) | NEXT(0x8c06c39e)}
//!      -> 0x8c06c3a0 (r3+=1) -> COND; COND falls through to the epilogue.
//! Removal copies the last entry (base-12+12*count) and last float
//! (base20-4+4*count) over slot i, then count-- (bra delay slot 0x8c06c338/
//! 0x8c06c39c); a skip advances r1 by 12 instead.
//!
//! Only bra/rts are delayed: delay slots that fire are `mov #-12,r7`
//! (0x8c06c2d2) and the two count-- stores.
//!
//! Side effects the C does not show: r14 := -4, r7 := -12, r3 is the scan
//! index (decremented then re-incremented on removal, so the same slot is
//! re-examined with count-1), r1 ends past the last scanned entry, r0/r10/
//! r11/r12/r13 hold intermediate pointers, fr0 := the moved float (only on
//! a removal), exit T = 0 (final cmp/gt), shll sets T to the shifted-out
//! bit (overwritten later).

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
    cpu.r[1] = ld(mem, cpu.r[4].wrapping_add(28)); // mov.l @(28,r4),r1
    cpu.r[3] = 0; // mov #0,r3
    cpu.r[14] = 0xffff_fffc; // mov.l 0x8c06c3b4,r14
    cpu.r[7] = 0xffff_fff4; // bra delay: mov #-12,r7
    loop {
        // COND 0x8c06c3a2
        cpu.r[13] = ld(mem, cpu.r[4].wrapping_add(36));
        cpu.set_t((cpu.r[13] as i32) > (cpu.r[3] as i32)); // cmp/gt r3,r13
        if !cpu.t() {
            break; // fall through to epilogue 0x8c06c3a8
        }
        // BODY 0x8c06c2d4
        cpu.r[0] = ld(mem, cpu.r[1]); // mov.l @r1,r0 (nodeA)
        cpu.set_t(cpu.r[0] == cpu.r[5]); // cmp/eq r5,r0
        let mut remove = false;
        if cpu.t() {
            // CASE_A 0x8c06c2da: nodeA == r5
            cpu.r[11] = ld(mem, cpu.r[1].wrapping_add(4)); // nodeB
            cpu.r[10] = ld(mem, cpu.r[11].wrapping_add(88)); // [nodeB+0x58]
            cpu.r[10] &= cpu.r[6];
            cpu.set_t(cpu.r[10] == 0); // tst r10,r10
            if !cpu.t() {
                cpu.r[11] = cpu.r[11].wrapping_add(92); // add #92,r11
                cpu.r[0] = cpu.r[0].wrapping_add(88); // add #88,r0
                cpu.r[10] = ld(mem, cpu.r[11]); // [nodeB+0x5c]
                cpu.r[0] = ld(mem, cpu.r[0]); // [nodeA+0x58]
                cpu.r[10] &= cpu.r[0];
                cpu.set_t(cpu.r[10] == 0); // tst r10,r10
                remove = cpu.t(); // bf 0x8c06c39e falls through on T=1 (cross==0)
            }
            if remove {
                // REMOVE_A 0x8c06c2f6
                cpu.r[10] = cpu.r[13]; // mov r13,r10
                shll(cpu, 10);
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[13]); // 3*count
                cpu.r[0] = cpu.r[3]; // mov r3,r0
                shll(cpu, 0);
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // 3*i
                cpu.r[13] = ld(mem, cpu.r[4].wrapping_add(28)); // base
                cpu.r[0] <<= 2; // shll2 r0
                cpu.r[10] <<= 2; // shll2 r10
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[13]); // dest
                cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[7]); // base-12
                cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[10]); // src
                cpu.r[12] = ld(mem, cpu.r[13]); // mov.l @r13,r12
                cpu.r[11] = cpu.r[3]; // mov r3,r11
                cpu.r[10] = ld(mem, cpu.r[13].wrapping_add(4)); // mov.l @(4,r13),r10
                cpu.r[11] <<= 2; // shll2 r11
                cpu.r[3] = cpu.r[3].wrapping_sub(1); // add #-1,r3
                st(mem, cpu.r[0], cpu.r[12]); // mov.l r12,@r0
                st(mem, cpu.r[0].wrapping_add(4), cpu.r[10]); // mov.l r10,@(4,r0)
                cpu.r[10] = ld(mem, cpu.r[13].wrapping_add(8)); // mov.l @(8,r13),r10
                st(mem, cpu.r[0].wrapping_add(8), cpu.r[10]); // mov.l r10,@(8,r0)
                cpu.r[12] = ld(mem, cpu.r[4].wrapping_add(32)); // base20
                cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(36)); // count
                cpu.r[10] = cpu.r[12]; // mov r12,r10
                cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[14]); // add r14,r12
                cpu.r[0] <<= 2; // shll2 r0
                cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[0]); // src float
                cpu.fr[0] = f32::from_bits(ld(mem, cpu.r[12])); // fmov @r12,fr0
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[11]); // add r11,r10
                st(mem, cpu.r[10], cpu.fr[0].to_bits()); // fmov fr0,@r10
                cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(36));
                cpu.r[0] = cpu.r[0].wrapping_sub(1); // add #-1,r0
                // bra 0x8c06c3a0; delay slot 0x8c06c338 stores the new count
                st(mem, cpu.r[4].wrapping_add(36), cpu.r[0]); // mov.l r0,@(36,r4)
            }
        } else {
            // CASE_B 0x8c06c33a: nodeA != r5
            cpu.r[11] = ld(mem, cpu.r[1].wrapping_add(4)); // nodeB
            cpu.set_t(cpu.r[11] == cpu.r[5]); // cmp/eq r5,r11
            if cpu.t() {
                cpu.r[10] = cpu.r[0].wrapping_add(88); // add #88,r10
                cpu.r[10] = ld(mem, cpu.r[10]); // [nodeA+0x58]
                cpu.r[10] &= cpu.r[6];
                cpu.set_t(cpu.r[10] == 0); // tst r10,r10
                if !cpu.t() {
                    cpu.r[0] = cpu.r[0].wrapping_add(92); // add #92,r0
                    cpu.r[10] = ld(mem, cpu.r[0]); // [nodeA+0x5c]
                    cpu.r[11] = cpu.r[11].wrapping_add(88); // add #88,r11
                    cpu.r[0] = ld(mem, cpu.r[11]); // [nodeB+0x58]
                    cpu.r[10] &= cpu.r[0];
                    cpu.set_t(cpu.r[10] == 0); // tst r10,r10
                    remove = cpu.t(); // bf 0x8c06c39e falls through on T=1 (cross==0)
                }
            }
            if remove {
                // REMOVE_B 0x8c06c35a
                cpu.r[0] = cpu.r[3]; // mov r3,r0
                shll(cpu, 0);
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // 3*i
                cpu.r[11] = cpu.r[13]; // mov r13,r11
                shll(cpu, 11);
                cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[13]); // 3*count
                cpu.r[10] = ld(mem, cpu.r[4].wrapping_add(28)); // base
                cpu.r[0] <<= 2; // shll2 r0
                cpu.r[11] <<= 2; // shll2 r11
                cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[10]); // dest
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[7]); // add r7,r10
                cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[11]); // src
                cpu.r[13] = ld(mem, cpu.r[10]); // mov.l @r10,r13
                cpu.r[12] = cpu.r[3]; // mov r3,r12
                cpu.r[11] = ld(mem, cpu.r[10].wrapping_add(4)); // mov.l @(4,r10),r11
                cpu.r[12] <<= 2; // shll2 r12
                cpu.r[3] = cpu.r[3].wrapping_sub(1); // add #-1,r3
                st(mem, cpu.r[0], cpu.r[13]); // mov.l r13,@r0
                st(mem, cpu.r[0].wrapping_add(4), cpu.r[11]); // mov.l r11,@(4,r0)
                cpu.r[11] = ld(mem, cpu.r[10].wrapping_add(8)); // mov.l @(8,r10),r11
                st(mem, cpu.r[0].wrapping_add(8), cpu.r[11]); // mov.l r11,@(8,r0)
                cpu.r[11] = ld(mem, cpu.r[4].wrapping_add(32)); // base20
                cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(36)); // count
                cpu.r[13] = cpu.r[11]; // mov r11,r13
                cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[14]); // add r14,r11
                cpu.r[0] <<= 2; // shll2 r0
                cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[0]); // src float
                cpu.fr[0] = f32::from_bits(ld(mem, cpu.r[11])); // fmov @r11,fr0
                cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[12]); // add r12,r13
                st(mem, cpu.r[13], cpu.fr[0].to_bits()); // fmov fr0,@r13
                cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(36));
                cpu.r[0] = cpu.r[0].wrapping_sub(1); // add #-1,r0
                // bra 0x8c06c3a0; delay slot 0x8c06c39c stores the new count
                st(mem, cpu.r[4].wrapping_add(36), cpu.r[0]); // mov.l r0,@(36,r4)
            }
        }
        if !remove {
            cpu.r[1] = cpu.r[1].wrapping_add(12); // NEXT 0x8c06c39e
        }
        cpu.r[3] = cpu.r[3].wrapping_add(1); // 0x8c06c3a0
    }
    // epilogue 0x8c06c3a8
    pop(cpu, mem, 10);
    pop(cpu, mem, 11);
    pop(cpu, mem, 12);
    pop(cpu, mem, 13);
    pop(cpu, mem, 14); // rts delay slot 0x8c06c3b2
}
