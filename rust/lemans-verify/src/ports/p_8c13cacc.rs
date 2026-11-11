//! 0x8c13cacc — append a 36-byte node (r5) to a list rooted in the struct at r4.
//!
//! Layout: [r4+56]=base B, [r4+60]=count N, [r4+64]=threshold, [r4+68]=flag
//! byte, [r4+0x80]=head H. Node stride is 36 bytes; the tail bound is
//! 36*N + B. Flag byte nonzero -> copy the node to *B, clear the byte,
//! return 1. Flag zero but N > threshold -> walk from H while H < 36N+B
//! (copying the first node whose float at +12 is <= f32[0x8c13cbf8]), else
//! from B while B < H; a copy advances the head by 36, wrapping to B when it
//! reaches 36N+B, and returns 1. All other paths return 0.
//!
//! Delay slots always run: `mov #1,r3` on the bf.s at 0x8c13cad8, `mov r7,r1`
//! on the bf.s at 0x8c13cb40, and the `mov #0,r0`/`mov #1,r0` in every rts
//! delay slot.

use sh4_oracle::{Cpu, Memory};

/// The 9-long copy with the r7/r6 interleaved load pattern (0x8c13cafa and
/// 0x8c13cb52 sites): leaves r7=src[6], r6=src[7], r5=src[8].
fn copy36_r7r6(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..4 {
        cpu.r[7] = mem.read_u32(s).unwrap_or(0);
        cpu.r[6] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[7]);
        let _ = mem.write_u32(d + 4, cpu.r[6]);
        d += 8;
        s += 8;
    }
    cpu.r[5] = mem.read_u32(s).unwrap_or(0); // mov.l @(32,r5),r5
    let _ = mem.write_u32(d, cpu.r[5]); // mov.l r5,@(32,r1)
}

/// The 9-long copy with the r3/r1 interleaved load pattern (0x8c13cba2 site):
/// leaves r3=src[7], r1=src[8].
fn copy36_r3r1(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32) {
    let mut d = dst;
    let mut s = src;
    for _ in 0..4 {
        cpu.r[3] = mem.read_u32(s).unwrap_or(0);
        cpu.r[1] = mem.read_u32(s + 4).unwrap_or(0);
        let _ = mem.write_u32(d, cpu.r[3]);
        let _ = mem.write_u32(d + 4, cpu.r[1]);
        d += 8;
        s += 8;
    }
    cpu.r[1] = mem.read_u32(s).unwrap_or(0); // mov.l @(32,r5),r1
    let _ = mem.write_u32(d, cpu.r[1]); // mov.l r1,@(32,r6)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];

    // 0x8c13cacc: r0 = r4+68 ; r6 = mov.b @r0 ; extu.b r6,r6 ; extu.b r6,r0 ; tst
    cpu.r[0] = r4.wrapping_add(68);
    cpu.r[6] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32; // mov.b sign-extends, extu.b clears
    cpu.r[0] = cpu.r[6];
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let mut r3: u32 = 1; // bf.s delay slot: mov #1,r3 (always runs)
    if cpu.t() {
        // bf.s not taken: r6 == 0. r0 = r4+64 ; r1 = @r0 ; r0 = @(60,r4) ; cmp/gt r1,r0
        cpu.r[0] = r4.wrapping_add(64);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r0
        if !cpu.t() {
            r3 = 0; // mov #0,r3
        }
    }
    cpu.r[3] = r3;
    // 0x8c13caea: extu.b r3,r0 ; tst r0,r0 ; bf 0x8c13caf4
    cpu.r[0] = r3 & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        // r3 == 0: rts ; mov #0,r0
        cpu.r[0] = 0;
        return;
    }
    // 0x8c13caf4: extu.b r6,r0 ; tst r0,r0 ; bt 0x8c13cb2a
    cpu.r[0] = cpu.r[6] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        return tail(cpu, mem, r4);
    }
    // r6 != 0: copy 36 bytes from r5 to [[r4+56]], clear flag byte, return 1
    cpu.r[1] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
    cpu.r[0] = 68; // mov #68,r0
    cpu.r[3] = 0; // mov #0,r3
    let dst = cpu.r[1];
    let src = cpu.r[5];
    copy36_r7r6(cpu, mem, dst, src);
    let _ = mem.write_u8(r4.wrapping_add(cpu.r[0]), cpu.r[3] as u8); // mov.b r3,@(r0,r4)
    cpu.r[0] = 1; // rts delay slot: mov #1,r0
}

/// Tail path at 0x8c13cb2a (flag byte r6 == 0, r3 != 0).
fn tail(cpu: &mut Cpu, mem: &mut Memory, r4: u32) {
    // 0x8c13cb2a..0x8c13cb40: end = 36*N + B in r3 ; r7 = H ; cmp/hi r7,r3
    cpu.r[1] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
    cpu.r[0] = mem.read_u32(0x8c13cbf4).unwrap_or(0); // 0x80
    let mut r3 = cpu.r[1];
    r3 <<= 2; // shll2 r3 (T untouched)
    cpu.set_t(r3 >> 31 != 0); // shll r3: T = old bit 31
    r3 <<= 1;
    r3 = r3.wrapping_add(cpu.r[1]); // 9N
    cpu.r[7] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // H = [r4+0x80]
    r3 <<= 2; // shll2 -> 36N
    cpu.r[6] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
    r3 = r3.wrapping_add(cpu.r[6]);
    cpu.r[3] = r3; // end
    cpu.set_t(r3 > cpu.r[7]); // cmp/hi r7,r3
    cpu.r[1] = cpu.r[7]; // bf.s delay slot: mov r7,r1 (always runs)
    let end = r3;

    if cpu.t() {
        // first loop 0x8c13cb48: r0 = r1+12 ; fr1 = @r0 ; fcmp/gt fr0,fr1
        cpu.r[0] = 0x8c13cbf8; // mova 0x8c13cbf8,r0
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        loop {
            cpu.r[0] = cpu.r[1].wrapping_add(12);
            cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
            if cpu.t() {
                // 0x8c13cb8a: add #36,r1 ; cmp/hi r1,r3 ; bt 0x8c13cb48
                cpu.r[1] = cpu.r[1].wrapping_add(36);
                cpu.set_t(end > cpu.r[1]);
                if cpu.t() {
                    continue;
                }
                break;
            }
            // node qualifies: copy 36 bytes from r5 to r1
            let dst = cpu.r[1];
            let src = cpu.r[5];
            copy36_r7r6(cpu, mem, dst, src);
            cpu.r[0] = mem.read_u32(0x8c13cbf4).unwrap_or(0); // 0x80
            cpu.r[1] = cpu.r[1].wrapping_add(36);
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]); // mov.l r1,@(r0,r4)
            cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // mov.l @(r0,r4),r1
            cpu.set_t(cpu.r[1] == end); // cmp/eq r3,r1
            if cpu.t() {
                cpu.r[1] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]);
            }
            cpu.r[0] = 1; // rts delay slot: mov #1,r0
            return;
        }
    }
    // 0x8c13cb90: cmp/hi r6,r7 ; bf 0x8c13cbee
    cpu.set_t(cpu.r[7] > cpu.r[6]);
    if !cpu.t() {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        return;
    }
    // second loop 0x8c13cb98: r0 = r6+12 ; fr1 = @r0 ; fcmp/gt fr0,fr1
    cpu.r[0] = 0x8c13cbf8; // mova
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    let h = cpu.r[7];
    loop {
        cpu.r[0] = cpu.r[6].wrapping_add(12);
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]); // fcmp/gt fr0,fr1
        if cpu.t() {
            // 0x8c13cbe8: add #36,r6 ; cmp/hi r6,r7 ; bt 0x8c13cb98
            cpu.r[6] = cpu.r[6].wrapping_add(36);
            cpu.set_t(h > cpu.r[6]);
            if cpu.t() {
                continue;
            }
            cpu.r[0] = 0; // rts delay slot at 0x8c13cbee: mov #0,r0
            return;
        }
        // node qualifies: copy 36 bytes from r5 to r6 (r3/r1 pattern)
        let dst = cpu.r[6];
        let src = cpu.r[5];
        copy36_r3r1(cpu, mem, dst, src);
        cpu.r[0] = mem.read_u32(0x8c13cbf4).unwrap_or(0); // 0x80
        cpu.r[6] = cpu.r[6].wrapping_add(36);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[6]); // mov.l r6,@(r0,r4)
        // recompute end = 36*N + B into r3
        cpu.r[1] = mem.read_u32(r4.wrapping_add(60)).unwrap_or(0); // N
        cpu.r[5] = mem.read_u32(r4.wrapping_add(56)).unwrap_or(0); // B
        let mut e = cpu.r[1];
        e <<= 2; // shll2
        cpu.set_t(e >> 31 != 0); // shll
        e <<= 1;
        e = e.wrapping_add(cpu.r[1]); // 9N
        e <<= 2; // 36N
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap_or(0); // [r4+0x80]
        e = e.wrapping_add(cpu.r[5]);
        cpu.r[3] = e;
        cpu.set_t(cpu.r[1] == e); // cmp/eq r1,r3
        if cpu.t() {
            let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[5]); // mov.l r5,@(r0,r4)
        }
        cpu.r[0] = 1; // rts delay slot: mov #1,r0
        return;
    }
}
