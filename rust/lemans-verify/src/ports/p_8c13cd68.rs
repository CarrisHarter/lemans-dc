//! 0x8c13cd68 — 16-byte-node sibling of 0x8c13cc50 with an r14 save and a
//! zero-threshold compare (fldi0).
//!
//! Layout: [r4+56]=base B, [r4+60]=count N, [r4+64]=threshold, [r4+68]=flag
//! byte, [r4+0x80]=head H. Tail bound is 16*N + B. Flag byte nonzero -> copy
//! 16 bytes from r5 to *B, clear the byte, return 1. Flag zero but N >
//! threshold -> walk from H while 16N+B > H copying the first node whose
//! float at +8 is <= 0.0, else walk from B while H > B; a copy advances the
//! head by 16 (wrapping to B at 16N+B) and returns 1. Otherwise return 0.
//! r14 is pushed at entry and popped in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

/// 4-long copy, r6/r3 interleaved pattern (0x8c13cd98 site, dest in r7):
/// leaves r6=src[2], r3=src[3].
fn copy16_r6r3(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..2 {
        cpu.r[6] = mem.read_u32(s).unwrap_or(0);
        cpu.r[3] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[6]);
        let _ = mem.write_u32(d + 4, cpu.r[3]);
        d += 8;
        s += 8;
    }
}

/// 4-long copy, r3/r1 interleaved pattern (0x8c13cdd6 and 0x8c13ce10 sites):
/// leaves r3=src[2], r1=src[3].
fn copy16_r3r1(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..2 {
        cpu.r[3] = mem.read_u32(s).unwrap_or(0);
        cpu.r[1] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[3]);
        let _ = mem.write_u32(d + 4, cpu.r[1]);
        d += 8;
        s += 8;
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];

    // prologue: mov.l r14,@-r15 (stack write, excluded from the diff)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);

    // 0x8c13cd6a..0x8c13cd76: r0 = r4+68 ; r6 = mov.b @r0 ; extu.b x2 ; tst ; bf.s
    cpu.r[0] = r4.wrapping_add(68);
    cpu.r[6] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[6];
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let mut r3: u32 = 1; // bf.s delay slot: mov #1,r3 (always runs)
    if cpu.t() {
        // r6 == 0: r0 = r4+64 ; r1 = @r0 ; r0 = @(60,r4) ; cmp/gt r1,r0 ; bt 0x8c13cd88
        cpu.r[0] = r4.wrapping_add(64);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r0
        if !cpu.t() {
            r3 = 0; // mov #0,r3
        }
    }
    cpu.r[3] = r3;
    // 0x8c13cd88: extu.b r3,r0 ; tst ; bf 0x8c13cd92
    cpu.r[0] = r3 & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = 0; // bra delay slot: mov #0,r0
        return epilogue(cpu, mem);
    }
    // 0x8c13cd92: extu.b r6,r0 ; tst ; bt 0x8c13cdb4
    cpu.r[0] = cpu.r[6] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        return tail(cpu, mem, r4);
    }
    // r6 != 0: copy 16 bytes from r5 to [[r4+56]], clear flag byte, return 1
    cpu.r[7] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // dest
    cpu.r[0] = 68; // mov #68,r0
    cpu.r[1] = 0; // mov #0,r1
    let dst = cpu.r[7];
    let src = cpu.r[5];
    copy16_r6r3(cpu, mem, dst, src);
    let _ = mem.write_u8(r4.wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[0] = 1; // bra delay slot: mov #1,r0
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c13ce46: rts ; mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn tail(cpu: &mut Cpu, mem: &mut Memory, r4: u32) {
    // 0x8c13cdb4..0x8c13cdc6: r1 = 16*N ; r14 = B ; r3 = H ; r7 = 16N+B ; cmp/hi r3,r7
    cpu.r[1] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
    cpu.r[0] = mem.read_u32(0x8c13ce4c).unwrap_or(0); // 0x80
    cpu.r[1] <<= 2; // shll2 r1 -> 4N (T untouched)
    cpu.r[14] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
    cpu.r[1] <<= 2; // shll2 -> 16N
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // H
    cpu.r[7] = cpu.r[14];
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]); // end = 16N + B
    cpu.set_t(cpu.r[7] > cpu.r[3]); // cmp/hi r3,r7
    cpu.r[6] = cpu.r[3]; // bf.s delay slot: mov r3,r6 (always runs)
    let end = cpu.r[7];

    if cpu.t() {
        // first loop 0x8c13cdcc: r0 = r6+8 ; fr1 = @r0 ; fcmp/gt fr0,fr1 (fr0 = 0.0)
        cpu.fr[0] = 0.0; // fldi0 fr0
        loop {
            cpu.r[0] = cpu.r[6].wrapping_add(8);
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[1] > cpu.fr[0]);
            if cpu.t() {
                // 0x8c13cdfa: add #16,r6 ; cmp/hi r6,r7 ; bt 0x8c13cdcc
                cpu.r[6] = cpu.r[6].wrapping_add(16);
                cpu.set_t(end > cpu.r[6]);
                if cpu.t() {
                    continue;
                }
                break;
            }
            let dst = cpu.r[6];
            let src = cpu.r[5];
            copy16_r3r1(cpu, mem, dst, src);
            cpu.r[0] = mem.read_u32(0x8c13ce4c).unwrap_or(0); // 0x80
            cpu.r[6] = cpu.r[6].wrapping_add(16);
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[6]); // mov.l r6,@(r0,r4)
            cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // mov.l @(r0,r4),r1
            cpu.set_t(cpu.r[1] == end); // cmp/eq r7,r1
            if cpu.t() {
                cpu.r[1] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]);
            }
            cpu.r[0] = 1; // bra delay slot: mov #1,r0
            return epilogue(cpu, mem);
        }
    }
    // 0x8c13ce00: cmp/hi r14,r3 ; bf 0x8c13ce44
    cpu.set_t(cpu.r[3] > cpu.r[14]);
    if !cpu.t() {
        cpu.r[0] = 0; // mov #0,r0 at 0x8c13ce44
        return epilogue(cpu, mem);
    }
    // second loop 0x8c13ce06: r0 = r14+8 ; fr1 = @r0 ; fcmp/gt fr0,fr1 (fr0 = 0.0)
    cpu.fr[0] = 0.0; // fldi0 fr0
    let h = cpu.r[3];
    loop {
        cpu.r[0] = cpu.r[14].wrapping_add(8);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if cpu.t() {
            // 0x8c13ce3e: add #16,r14 ; cmp/hi r14,r3 ; bt 0x8c13ce06
            cpu.r[14] = cpu.r[14].wrapping_add(16);
            cpu.set_t(h > cpu.r[14]);
            if cpu.t() {
                continue;
            }
            cpu.r[0] = 0; // 0x8c13ce44: mov #0,r0
            return epilogue(cpu, mem);
        }
        let dst = cpu.r[14];
        let src = cpu.r[5];
        copy16_r3r1(cpu, mem, dst, src);
        cpu.r[0] = mem.read_u32(0x8c13ce4c).unwrap_or(0); // 0x80
        cpu.r[14] = cpu.r[14].wrapping_add(16);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[14]); // mov.l r14,@(r0,r4)
        // recompute end = 16*N + B into r3 (r5 ends holding 16*N)
        cpu.r[5] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
        cpu.r[6] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
        cpu.r[5] <<= 2; // shll2 r5 -> 4N
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // [r4+0x80]
        cpu.r[3] = cpu.r[6];
        cpu.r[5] <<= 2; // shll2 -> 16N
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // 16N + B
        cpu.set_t(cpu.r[1] == cpu.r[3]); // cmp/eq r1,r3
        if cpu.t() {
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[6]); // mov.l r6,@(r0,r4)
        }
        cpu.r[0] = 1; // bra delay slot: mov #1,r0
        return epilogue(cpu, mem);
    }
}
