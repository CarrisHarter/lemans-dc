//! 0x8c07b0f4 — FUN_8c07b0f4 (281 instructions, leaf): move `B` elements between
//! two ring buffers whose 3-word headers arrive on the caller's stack, and update
//! the counted ring struct in r4.
//!
//! ABI: r4 = P (ring struct: P[0]=cap, P[1]=head h, P[2]=count t, P[3]=buf).
//! Stack args at entry sp S (frame base f = S-128): A=[f+0x80]=[S+0],
//! D=[f+0x84]=[S+4], B=[f+0x88]=[S+8], E=[f+0x8c]=[S+0xc],
//! F0=[f+0x90]=[S+0x10], C=[f+0x94]=[S+0x14], F2=[f+0x98]=[S+0x18].
//! A/D/F0 point to ring headers ([0]=size, [3]=base); E/F2 are ring pointers.
//! NOTE: `mov #N,rX` immediates in the listing are DECIMAL (e.g. `mov #84,r2`
//! is offset 0x54); the pool words 0x80/0x84/0x88/0x8c/0x90/0x94 are hex data.
//!
//! CFG sketch:
//!
//! ```text
//! r6=A r3=B r5=C; T=(B==C)                                  (0x8c07b10c)
//! ├─ B==C (eq, 0x8c07b110) ──► *A=D, A[1]=B, A[2]=E (bra delay) ──► epilogue
//! └─ else (0x8c07b120):
//!     r10=h r7=t r1=cap; r7=h+t; T=(cap >u h+t)             [cmp/hi UNSIGNED here]
//!     ├─ cap>h+t: [f+0x60..0x68]={P,h,buf+(h+t)*4}, r7=(h+t)<<2, r1=f+0x60
//!     └─ else:    [f+0x60..0x68]={P,h,buf+((h+t)-cap)*4}, r7=buf+(((h+t)-cap)<<2), r1=f+0x60
//!     r0=h-C; T=((h-C) >u B); delay slot of bf.s ALWAYS: r5=C-B   (0x8c07b170)
//!     ├─ T=1 (0x8c07b172 tst B,B)
//!     │   ├─ B!=0 → A1 0x8c07b176..0x8c07b21e: setup copies the F header arg to
//!     │   │   f+0x24 {F0,C,F2}, the D header arg to f+0x30 {D,B,E}, spills P to
//!     │   │   f+0x54, 0 to f+0x58, buf+t*4 to f+0x5c; loop B times (DOWNWARD):
//!     │   │   wrap F2p/Ep to base+size*4 when they hit base, then -4,
//!     │   │   *F2p = *Ep
//!     │   └─ B==0 → A4: skip loop
//!     │   both → 0x8c07b220: P[2]=t+(C-B); T=(P[2] >=u cap); if so P[2]-=cap
//!     │            (delay-slot store of bra 0x8c07b230)
//!     └─ T=0 (0x8c07b234 tst h-C)
//!         ├─ h-C==0 → A3: straight to tail
//!         └─ else → A2 0x8c07b238..0x8c07b302: locals f+0 {D,B,E},
//!             f+0x48 {P,h,ptr}, f+0x0c {P,h,ptr}, f+0x18 {F0,C,F2};
//!             loop (h-C) times (UPWARD): *Ep = *F2p, pointers +4, wrap to base
//!             when ptr+4 == base+size*4; P[2] NOT updated on this path
//!     tail 0x8c07b304: P[1]=h-(C-B); *A=[f+0x84]=D (arg slot, never clobbered);
//!                      A[1]=B; A[2]=E=[f+0x8c]
//!     epilogue 0x8c07b318: r15 restored, r10..r14 popped (entry values), rts
//! ```
//!
//! Side effects the decompiled C does not show:
//! - r5 = C-B is computed in the `bf.s` delay slot on EVERY else path (the C
//!   shows it only inside the `if` branch).
//! - The `mov #84,r2` spill writes P to the DEAD local f+0x54 (not to the arg
//!   slot — the arg slot f+0x84 is never written, so the tail always stores D).
//! - r1/r2/r7 leftovers differ per path (r1=cap on A1/A4, h on A2, f+0x60 on
//!   A3; r2=f+0x40 on A1, f+0x50 on A2, f+0x68 on A3/A4; r7=0 on A1,
//!   *F0<<2 on A2, (h+t)<<2 or buf+(((h+t)-cap)<<2) on A3/A4); r0=E and r6=A
//!   on every path.
//! - Final T: 1 on eq/A2/A3, (P[2] >=u cap) on A1/A4. shll2 does NOT set T in
//!   this oracle, and cmp/hi/cmp/hs are unsigned.
//! - The first W computation (f+0x60..0x68) is dead except for r1/r7; A2
//!   recomputes its own ring pointer into f+0x48..f+0x50.

use sh4_oracle::{Cpu, Memory};

#[inline]
fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).expect("8c07b0f4: load out of range")
}

#[inline]
fn wr(mem: &mut Memory, addr: u32, v: u32) {
    mem.write_u32(addr, v).expect("8c07b0f4: store out of range")
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r = cpu.r;
    // Prologue: mov.l r14,r13,r12,r11,r10,@-r15 ; add #-108,r15
    r[15] = r[15].wrapping_sub(4);
    wr(mem, r[15], r[14]);
    r[15] = r[15].wrapping_sub(4);
    wr(mem, r[15], r[13]);
    r[15] = r[15].wrapping_sub(4);
    wr(mem, r[15], r[12]);
    r[15] = r[15].wrapping_sub(4);
    wr(mem, r[15], r[11]);
    r[15] = r[15].wrapping_sub(4);
    wr(mem, r[15], r[10]);
    r[15] = r[15].wrapping_sub(108);
    let f = r[15]; // frame base; stack args live at f+0x80..f+0x98

    // 0x8c07b102..0x8c07b10a: pool words 0x80/0x88/0x94 indexed by r15
    r[6] = rd(mem, f.wrapping_add(0x80)); // A = [S+0x00]
    r[3] = rd(mem, f.wrapping_add(0x88)); // B = [S+0x08]
    r[5] = rd(mem, f.wrapping_add(0x94)); // C = [S+0x14]
    cpu.set_t(r[3] == r[5]); // 0x8c07b10c cmp/eq r5,r3

    if r[3] == r[5] {
        // ---- eq path 0x8c07b110..0x8c07b11e ----
        r[0] = rd(mem, f.wrapping_add(0x84)); // D = [S+0x04]
        wr(mem, r[6], r[0]); // 0x8c07b114 *A = D
        wr(mem, r[6].wrapping_add(4), r[3]); // 0x8c07b116 A[1] = B
        r[0] = rd(mem, f.wrapping_add(0x8c)); // E = [S+0x0c]
        wr(mem, r[6].wrapping_add(8), r[0]); // 0x8c07b11e delay slot: A[2] = E
    } else {
        // ---- else path 0x8c07b120 ----
        r[10] = rd(mem, r[4].wrapping_add(4)); // h = P[1]
        r[7] = rd(mem, r[4].wrapping_add(8)); // t = P[2]
        r[1] = rd(mem, r[4]); // cap = P[0]
        r[7] = r[7].wrapping_add(r[10]); // h+t
        cpu.set_t(r[1] > r[7]); // 0x8c07b128 cmp/hi r7,r1 (unsigned in oracle)
        if r[1] > r[7] {
            // 0x8c07b12c: cap >u h+t, no wrap
            r[0] = rd(mem, r[4].wrapping_add(12)); // buf = P[3]
            r[2] = 0x60;
            wr(mem, f.wrapping_add(r[2]), r[4]); // [f+0x60] = P
            r[7] = r[7] << 2; // 0x8c07b134 shll2 (T untouched)
            r[2] = 0x64;
            wr(mem, f.wrapping_add(r[2]), r[10]); // [f+0x64] = h
            r[1] = f;
            r[0] = r[0].wrapping_add(r[7]); // buf + (h+t)*4
            r[2] = 0x68;
            wr(mem, f.wrapping_add(r[2]), r[0]); // [f+0x68]
            r[1] = r[1].wrapping_add(0x60); // 0x8c07b148 delay slot of bra
        } else {
            // 0x8c07b14a: wrap
            r[0] = rd(mem, r[4].wrapping_add(12)); // buf
            r[2] = 0x60;
            wr(mem, f.wrapping_add(r[2]), r[4]); // [f+0x60] = P
            r[7] = r[7].wrapping_sub(r[1]); // (h+t)-cap
            r[2] = 0x64;
            wr(mem, f.wrapping_add(r[2]), r[10]); // [f+0x64] = h
            r[7] = r[7] << 2; // 0x8c07b15a shll2
            r[7] = r[7].wrapping_add(r[0]); // 0x8c07b15e buf + ...
            r[1] = f;
            r[2] = 0x68;
            wr(mem, f.wrapping_add(r[2]), r[7]); // [f+0x68]
            r[1] = r[1].wrapping_add(0x60); // 0x8c07b166
        }
        r[2] = f.wrapping_add(0x68); // last r2 write on this path so far

        // 0x8c07b168..0x8c07b170
        r[0] = rd(mem, r[1].wrapping_add(4)); // h = [f+0x64]
        r[0] = r[0].wrapping_sub(r[5]); // h - C
        cpu.set_t(r[0] > r[3]); // 0x8c07b16c cmp/hi r3,r0
        r[5] = r[5].wrapping_sub(r[3]); // 0x8c07b170 delay slot of bf.s: C-B, ALWAYS

        if r[0] > r[3] {
            // 0x8c07b172
            cpu.set_t(r[3] & r[3] == 0); // tst r3,r3
            if r[3] != 0 {
                // ===== path A1: setup 0x8c07b176..0x8c07b1d4 + loop to 0x8c07b21e =====
                let f90 = f.wrapping_add(0x90); // &F header arg (S+0x10)
                let f84 = f.wrapping_add(0x84); // &D header arg (S+0x04)
                r[0] = f;
                r[1] = rd(mem, f90); // F0
                r[10] = f;
                r[7] = rd(mem, f90.wrapping_add(4)); // C
                r[0] = r[0].wrapping_add(0x24); // f+0x24 (add #36)
                wr(mem, r[0], r[1]); // 0x8c07b188 [f+0x24] = F0
                r[13] = f;
                wr(mem, r[0].wrapping_add(4), r[7]); // 0x8c07b18c [f+0x28] = C
                r[14] = f;
                r[7] = rd(mem, f90.wrapping_add(8)); // F2
                r[10] = r[10].wrapping_add(0x30); // f+0x30 (add #48)
                wr(mem, r[0].wrapping_add(8), r[7]); // 0x8c07b196 [f+0x2c] = F2
                r[12] = rd(mem, f84); // D
                r[1] = 0; // 0x8c07b19c mov #0,r1
                r[7] = rd(mem, f84.wrapping_add(4)); // B
                r[13] = r[13].wrapping_add(0x54); // f+0x54 (add #84)
                r[14] = r[14].wrapping_add(0x3c); // f+0x3c (add #60)
                wr(mem, r[10], r[12]); // 0x8c07b1a4 [f+0x30] = D
                wr(mem, r[10].wrapping_add(4), r[7]); // 0x8c07b1a6 [f+0x34] = B
                r[7] = rd(mem, f84.wrapping_add(8)); // E
                wr(mem, r[10].wrapping_add(8), r[7]); // 0x8c07b1aa [f+0x38] = E
                wr(mem, f.wrapping_add(0x54), r[4]); // 0x8c07b1ac [f+0x54] = P (dead local)
                wr(mem, f.wrapping_add(0x58), r[1]); // 0x8c07b1b2 [f+0x58] = 0
                r[1] = rd(mem, r[4].wrapping_add(8)); // t
                r[7] = rd(mem, r[4].wrapping_add(12)); // buf
                r[1] = r[1] << 2; // 0x8c07b1ba shll2
                r[7] = r[7].wrapping_add(r[1]); // buf + t*4
                wr(mem, f.wrapping_add(0x5c), r[7]); // 0x8c07b1c0 [f+0x5c]
                r[7] = rd(mem, r[13]); // 0x8c07b1c2 [f+0x54] (= P, just spilled)
                r[1] = rd(mem, r[13].wrapping_add(4)); // [f+0x58] = 0
                r[2] = f.wrapping_add(0x40); // 0x8c07b1c8 (mov #64 + add r15)
                wr(mem, r[14], r[7]); // 0x8c07b1ca [f+0x3c]
                wr(mem, r[14].wrapping_add(4), r[1]); // 0x8c07b1cc [f+0x40] = 0
                r[1] = rd(mem, r[13].wrapping_add(8)); // [f+0x5c]
                wr(mem, r[14].wrapping_add(8), r[1]); // 0x8c07b1d0 [f+0x44]
                r[1] = rd(mem, f.wrapping_add(0x40)); // 0x8c07b1d4 delay of bra: = 0
                // loop 0x8c07b1d6..0x8c07b21e: B iterations, downward copy
                loop {
                    r[7] = rd(mem, f.wrapping_add(0x28)); // @(40,r15)
                    r[7] = r[7].wrapping_sub(1);
                    wr(mem, f.wrapping_add(0x28), r[7]); // C counter (side effect only)
                    r[14] = rd(mem, f.wrapping_add(0x24)); // @(36,r15): F0
                    r[7] = rd(mem, f.wrapping_add(0x2c)); // @(44,r15): F2p
                    r[13] = rd(mem, r[14].wrapping_add(12)); // F0[3]
                    cpu.set_t(r[13] == r[7]); // 0x8c07b1e2
                    if r[13] == r[7] {
                        r[7] = rd(mem, r[14]); // *F0
                        r[7] = r[7] << 2; // shll2
                        r[13] = r[13].wrapping_add(r[7]);
                        wr(mem, f.wrapping_add(0x2c), r[13]); // wrap: base+size*4
                    }
                    r[7] = rd(mem, f.wrapping_add(0x34)); // @(52,r15): B counter
                    r[13] = rd(mem, f.wrapping_add(0x2c)); // F2p cur
                    r[7] = r[7].wrapping_sub(1);
                    wr(mem, f.wrapping_add(0x34), r[7]);
                    r[13] = r[13].wrapping_sub(4);
                    r[14] = rd(mem, f.wrapping_add(0x30)); // @(48,r15): D
                    wr(mem, f.wrapping_add(0x2c), r[13]); // F2p-4
                    r[13] = rd(mem, r[14].wrapping_add(12)); // D[3]
                    r[7] = rd(mem, f.wrapping_add(0x38)); // @(56,r15): Ep
                    cpu.set_t(r[13] == r[7]); // 0x8c07b200
                    if r[13] == r[7] {
                        r[7] = rd(mem, r[14]); // *D
                        r[7] = r[7] << 2;
                        r[13] = r[13].wrapping_add(r[7]);
                        wr(mem, f.wrapping_add(0x38), r[13]); // wrap
                    }
                    r[7] = rd(mem, f.wrapping_add(0x38));
                    r[7] = r[7].wrapping_sub(4);
                    wr(mem, f.wrapping_add(0x38), r[7]); // Ep-4
                    r[7] = rd(mem, f.wrapping_add(0x38));
                    r[13] = rd(mem, r[0].wrapping_add(8)); // @(8,r0): [f+0x2c] = F2p (r0 = f+0x24)
                    r[7] = rd(mem, r[7]); // *Ep
                    wr(mem, r[13], r[7]); // 0x8c07b218 *F2p = *Ep
                    r[7] = rd(mem, f.wrapping_add(0x34));
                    cpu.set_t(r[7] == r[1]); // 0x8c07b21c cmp/eq r7,r1 (r1 = 0)
                    if r[7] != r[1] {
                        continue; // 0x8c07b21e bf 0x8c07b1d6
                    }
                    break;
                }
            }
            // 0x8c07b220 (A1 after loop, or A4 straight from bt)
            r[0] = rd(mem, r[4].wrapping_add(8)); // t
            r[0] = r[0].wrapping_add(r[5]); // t + (C-B)
            wr(mem, r[4].wrapping_add(8), r[0]); // 0x8c07b224 P[2] = ...
            r[1] = rd(mem, r[4]); // cap
            r[0] = rd(mem, r[4].wrapping_add(8)); // P[2] new
            cpu.set_t(r[0] >= r[1]); // 0x8c07b22a cmp/hs r1,r0
            if r[0] >= r[1] {
                r[0] = r[0].wrapping_sub(r[1]);
                wr(mem, r[4].wrapping_add(8), r[0]); // 0x8c07b232 delay slot of bra
            }
        } else {
            // 0x8c07b234
            cpu.set_t(r[0] == 0); // tst r0,r0 (r0 = h-C)
            if r[0] != 0 {
                // ===== path A2: setup 0x8c07b238..0x8c07b2b8 + loop to 0x8c07b302 =====
                let f84 = f.wrapping_add(0x84);
                r[13] = f;
                r[1] = rd(mem, f84); // D
                r[0] = rd(mem, f84.wrapping_add(4)); // B
                wr(mem, r[13], r[1]); // 0x8c07b242 [f+0x00] = D
                wr(mem, r[13].wrapping_add(4), r[0]); // 0x8c07b244 [f+0x04] = B
                r[0] = rd(mem, f84.wrapping_add(8)); // E
                wr(mem, r[13].wrapping_add(8), r[0]); // 0x8c07b248 [f+0x08] = E
                r[13] = rd(mem, r[4].wrapping_add(4)); // h
                r[7] = rd(mem, r[4].wrapping_add(8)); // t
                r[1] = rd(mem, r[4]); // cap
                r[7] = r[7].wrapping_add(r[13]); // h+t
                cpu.set_t(r[1] > r[7]); // 0x8c07b252 cmp/hi
                if r[1] > r[7] {
                    r[0] = rd(mem, r[4].wrapping_add(12)); // buf
                    wr(mem, f.wrapping_add(0x48), r[4]); // [f+0x48] = P (mov #72)
                    r[7] = r[7] << 2;
                    wr(mem, f.wrapping_add(0x4c), r[13]); // [f+0x4c] = h (mov #76)
                    r[10] = f;
                    r[0] = r[0].wrapping_add(r[7]);
                    wr(mem, f.wrapping_add(0x50), r[0]); // [f+0x50] (mov #80)
                    r[10] = r[10].wrapping_add(0x48); // 0x8c07b272 delay of bra (add #72)
                } else {
                    r[0] = rd(mem, r[4].wrapping_add(12)); // buf
                    wr(mem, f.wrapping_add(0x48), r[4]);
                    r[7] = r[7].wrapping_sub(r[1]);
                    wr(mem, f.wrapping_add(0x4c), r[13]);
                    r[7] = r[7] << 2;
                    r[7] = r[7].wrapping_add(r[0]);
                    r[10] = f;
                    wr(mem, f.wrapping_add(0x50), r[7]);
                    r[10] = r[10].wrapping_add(0x48); // 0x8c07b290
                }
                r[2] = f.wrapping_add(0x50); // 0x8c07b28a
                let f90 = f.wrapping_add(0x90);
                r[14] = f;
                r[1] = rd(mem, r[10]); // P
                r[14] = r[14].wrapping_add(0x0c); // f+0x0c (add #12)
                r[0] = rd(mem, r[10].wrapping_add(4)); // h
                r[13] = f;
                wr(mem, r[14], r[1]); // 0x8c07b2a0 [f+0x0c] = P
                r[13] = r[13].wrapping_add(0x18); // f+0x18 (add #24)
                wr(mem, r[14].wrapping_add(4), r[0]); // 0x8c07b2a4 [f+0x10] = h
                r[0] = rd(mem, r[10].wrapping_add(8)); // ptr
                wr(mem, r[14].wrapping_add(8), r[0]); // 0x8c07b2a8 [f+0x14] = ptr
                r[1] = rd(mem, f90); // F0
                r[0] = rd(mem, f90.wrapping_add(4)); // C
                wr(mem, r[13], r[1]); // 0x8c07b2ae [f+0x18] = F0
                wr(mem, r[13].wrapping_add(4), r[0]); // 0x8c07b2b0 [f+0x1c] = C
                r[0] = rd(mem, f90.wrapping_add(8)); // F2
                wr(mem, r[13].wrapping_add(8), r[0]); // 0x8c07b2b4 [f+0x20] = F2
                r[0] = rd(mem, f.wrapping_add(0x10)); // 0x8c07b2b8 delay of bra: h = [f+0x10]
                // loop 0x8c07b2ba..0x8c07b302: (h-C) iterations, upward copy
                loop {
                    r[1] = rd(mem, f.wrapping_add(0x20)); // @(32,r15): F2p
                    r[7] = rd(mem, f.wrapping_add(8)); // @(8,r15): Ep
                    r[1] = rd(mem, r[1]); // *F2p
                    wr(mem, r[7], r[1]); // 0x8c07b2c0 *Ep = *F2p
                    r[1] = rd(mem, f.wrapping_add(4)); // @(4,r15)
                    r[1] = r[1].wrapping_add(1);
                    wr(mem, f.wrapping_add(4), r[1]); // 0x8c07b2c6
                    r[14] = rd(mem, f.wrapping_add(8));
                    r[14] = r[14].wrapping_add(4);
                    wr(mem, f.wrapping_add(8), r[14]); // 0x8c07b2cc Ep+4
                    r[1] = rd(mem, f); // @(0,r15): D
                    r[7] = rd(mem, r[1]); // *D
                    r[13] = rd(mem, r[1].wrapping_add(12)); // D[3]
                    r[7] = r[7] << 2; // 0x8c07b2d4 shll2
                    r[1] = r[13].wrapping_add(r[7]); // D[3]+*D*4
                    cpu.set_t(r[14] == r[1]); // 0x8c07b2da
                    if r[14] == r[1] {
                        wr(mem, f.wrapping_add(8), r[13]); // 0x8c07b2de wrap to D[3]
                    }
                    r[1] = rd(mem, f.wrapping_add(0x1c)); // @(28,r15): Ccnt
                    r[1] = r[1].wrapping_add(1);
                    wr(mem, f.wrapping_add(0x1c), r[1]); // 0x8c07b2e4
                    r[13] = rd(mem, f.wrapping_add(0x20));
                    r[13] = r[13].wrapping_add(4);
                    wr(mem, f.wrapping_add(0x20), r[13]); // 0x8c07b2ea F2p+4
                    r[1] = rd(mem, f.wrapping_add(0x18)); // @(24,r15): F0
                    r[7] = rd(mem, r[1]); // *F0
                    r[14] = rd(mem, r[1].wrapping_add(12)); // F0[3]
                    r[7] = r[7] << 2; // 0x8c07b2f2
                    r[1] = r[14].wrapping_add(r[7]); // F0[3]+*F0*4
                    cpu.set_t(r[13] == r[1]); // 0x8c07b2f8
                    if r[13] == r[1] {
                        wr(mem, f.wrapping_add(0x20), r[14]); // 0x8c07b2fc wrap to F0[3]
                    }
                    r[1] = rd(mem, f.wrapping_add(0x1c)); // Ccnt
                    cpu.set_t(r[1] == r[0]); // 0x8c07b300 cmp/eq r0,r1 (r0 = h)
                    if r[1] != r[0] {
                        continue; // 0x8c07b302 bf 0x8c07b2ba
                    }
                    break;
                }
            }
            // A3 (h-C==0) falls straight through to the common tail
        }

        // ---- common tail 0x8c07b304..0x8c07b316 ----
        r[0] = rd(mem, r[4].wrapping_add(4)); // h
        r[0] = r[0].wrapping_sub(r[5]); // h - (C-B)
        wr(mem, r[4].wrapping_add(4), r[0]); // 0x8c07b308 P[1] = ...
        r[0] = rd(mem, f.wrapping_add(0x84)); // D arg (slot never clobbered)
        wr(mem, r[6], r[0]); // 0x8c07b30e *A = D
        wr(mem, r[6].wrapping_add(4), r[3]); // 0x8c07b310 A[1] = B
        r[0] = rd(mem, f.wrapping_add(0x8c)); // E = [S+0x0c]
        wr(mem, r[6].wrapping_add(8), r[0]); // 0x8c07b316 A[2] = E
    }

    // Epilogue 0x8c07b318..0x8c07b324 (rts delay slot pops r14)
    r[15] = f.wrapping_add(108);
    r[10] = rd(mem, r[15]);
    r[15] = r[15].wrapping_add(4);
    r[11] = rd(mem, r[15]);
    r[15] = r[15].wrapping_add(4);
    r[12] = rd(mem, r[15]);
    r[15] = r[15].wrapping_add(4);
    r[13] = rd(mem, r[15]);
    r[15] = r[15].wrapping_add(4);
    r[14] = rd(mem, r[15]);
    r[15] = r[15].wrapping_add(4);
    cpu.r = r;
}
