//! 0x8c010e84 — memmove: copy n bytes r5 -> r4, direction and width chosen
//! from `dst|src|n`.
//!
//! Prologue/epilogue push/pop r13,r12,r11 (stack window, net zero on r15).
//! r4==r5 short-circuits (T set, only r0=r4). n==0 leaves T clear and r2=0.
//! Otherwise r11 = n>>1, r12 = n>>2, r1 = 3, r2 = 0 (forward) / 1 (backward),
//! and the width comes from the low bits of dst|src|n: bit0 -> byte,
//! bits01 -> word, else dword. Forward paths leave the src pointer in r5
//! (the byte path keeps r5 = src and uses r0 as pointer), dst pointer in r7;
//! backward paths leave r13 = src+n and use r5/r7 per width. Loop counters
//! end at 0 with T set; the epilogue's `mov r4,r0` plus `rts` delay slot set
//! r0 = r4.
//!
//! Oracle decode quirks (ground truth): `mov.b/w @Rm+,Rn` load a full u32
//! but step 1/2; `mov.b/w Rm,@-Rn` step 1/2 but store a full u32. So the
//! byte/word forward paths load u32s, and the byte/word backward paths
//! write u32s. Only `mov.l` post-inc/pre-dec and plain `mov.b/w` accesses
//! behave as their names suggest.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

fn sext16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    let n = cpu.r[6];
    cpu.set_t(dst == src); // cmp/eq r5,r4
    if dst == src {
        cpu.r[0] = dst; // 0x8c010f3a
        return;         // T stays set
    }
    cpu.r[2] = 0;      // mov #0,r2
    cpu.set_t(n > 0);  // cmp/hi r2,r6
    if n == 0 {
        cpu.r[0] = dst;
        return;        // T stays clear
    }
    let r11 = n >> 1; // shlr r11 (T clobbered by the next cmp/hs)
    let r12 = n >> 2; // shlr2 r12, delay slot of `bt.s`, always runs
    let a = dst | src | n;
    // r11/r12/r13 are pushed in the prologue and popped in the epilogue,
    // so their final values are the seeds; the port leaves them alone.
    // r1 = 3 is set at 0x8c010eac / 0x8c010efc, which the byte paths skip.
    if dst < src {
        // forward (cmp/hs r5,r4 clear)
        cpu.r[2] = 0;
        if a & 1 != 0 {
            // byte forward: pointer in r0, r7 = dst, r6 = n
            let mut r0 = src;
            let mut r7 = dst;
            let mut r6 = n;
            loop {
                let c = mem.read_u32(r0).unwrap_or(0); // mov.b @r0+,r3: u32 load, step 1
                r0 = r0.wrapping_add(1);
                r6 = r6.wrapping_sub(1);
                cpu.set_t(r6 == 0); // tst r6,r6
                let _ = mem.write_u8(r7, c as u8); // mov.b r3,@r7 stores low byte
                r7 = r7.wrapping_add(1); // delay slot of `bf.s`
                cpu.r[3] = c;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[6] = r6;
            cpu.r[7] = r7;
            // r5 keeps src; r0 is overwritten by the epilogue
        } else if a & 3 != 0 {
            // word forward
            cpu.r[1] = 3; // mov #3,r1 at 0x8c010eac
            let mut r5p = src;
            let mut r7 = dst;
            let mut r6 = r11;
            loop {
                let w = mem.read_u32(r5p).unwrap_or(0); // mov.w @r5+,r3: u32 load, step 2
                r5p = r5p.wrapping_add(2);
                r6 = r6.wrapping_sub(1);
                cpu.set_t(r6 == 0);
                let _ = mem.write_u16(r7, w as u16); // mov.w r3,@r7 stores low half
                r7 = r7.wrapping_add(2); // delay slot of `bf.s`
                cpu.r[3] = w;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[5] = r5p;
            cpu.r[6] = r6;
            cpu.r[7] = r7;
        } else {
            // dword forward
            cpu.r[1] = 3; // mov #3,r1 at 0x8c010eac
            let mut r5p = src;
            let mut r7 = dst;
            let mut r6 = r12;
            loop {
                let d = mem.read_u32(r5p).unwrap_or(0); // mov.l @r5+,r3
                r5p = r5p.wrapping_add(4);
                r6 = r6.wrapping_sub(1);
                cpu.set_t(r6 == 0);
                let _ = mem.write_u32(r7, d);
                r7 = r7.wrapping_add(4); // delay slot of `bf.s`
                cpu.r[3] = d;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[5] = r5p;
            cpu.r[6] = r6;
            cpu.r[7] = r7;
        }
    } else {
        // backward (cmp/hs r5,r4 set, `bt.s` taken after its delay slot)
        cpu.r[2] = 1; // mov #1,r2
        let r13 = src.wrapping_add(n); // delay slot of `bf.s` at 0x8c010ef8
        let r0end = dst.wrapping_add(n);
        if a & 1 != 0 {
            // byte backward: r7 = src+n, r5 = dst+n, r6 = n
            let mut r7 = r13;
            let mut r5 = r0end;
            let mut r6 = n;
            loop {
                r7 = r7.wrapping_sub(1);
                let c = sext8(mem, r7); // mov.b @r7,r3 (true byte load)
                r6 = r6.wrapping_sub(1); // dt r6
                cpu.set_t(r6 == 0);
                r5 = r5.wrapping_sub(1); // delay slot: mov.b r3,@-r5
                let _ = mem.write_u32(r5, c); // ...decoded as a u32 store
                cpu.r[3] = c;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[5] = r5;
            cpu.r[6] = r6;
            cpu.r[7] = r7;
        } else if a & 3 != 0 {
            // word backward: r5 = src+n, r7 = dst+n, r6 = n>>1
            cpu.r[1] = 3; // mov #3,r1 at 0x8c010efc
            let mut r5 = r13;
            let mut r7 = r0end;
            let mut r6 = r11;
            loop {
                r5 = r5.wrapping_sub(2);
                let w = sext16(mem, r5); // mov.w @r5,r3 (true word load, sign-extended)
                r6 = r6.wrapping_sub(1); // dt r6
                cpu.set_t(r6 == 0);
                r7 = r7.wrapping_sub(2); // delay slot: mov.w r3,@-r7
                let _ = mem.write_u32(r7, w); // ...decoded as a u32 store
                cpu.r[3] = w;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[5] = r5;
            cpu.r[6] = r6;
            cpu.r[7] = r7;
        } else {
            // dword backward: r7 = src+n, r5 = dst+n, r6 = n>>2
            cpu.r[1] = 3; // mov #3,r1 at 0x8c010efc
            let mut r7 = r13;
            let mut r5 = r0end;
            let mut r6 = r12;
            loop {
                r7 = r7.wrapping_sub(4);
                let d = mem.read_u32(r7).unwrap_or(0); // mov.l @r7,r3
                r6 = r6.wrapping_sub(1); // dt r6
                cpu.set_t(r6 == 0);
                r5 = r5.wrapping_sub(4); // delay slot: mov.l r3,@-r5
                let _ = mem.write_u32(r5, d);
                cpu.r[3] = d;
                if r6 == 0 {
                    break;
                }
            }
            cpu.r[5] = r5;
            cpu.r[6] = r6;
            cpu.r[7] = r7;
        }
    }
    cpu.set_t(true); // every loop exit has counter == 0
    cpu.r[0] = dst;  // 0x8c010f3a
}
