//! 0x8c010e84 — memmove: copy n bytes r5 -> r4, overlap-safe.
//!
//! Direction from `cmp/hs r5,r4`: dst below src goes forward, else backward.
//! Width from the low bits of `dst|src|n` (r7): bit0 -> byte, bits01 -> word,
//! else dword. `r11 = n>>1`, `r12 = n>>2` (the two shifts run in the dispatch;
//! `shlr2 r12` is the `bt.s` delay slot and runs on both paths), `r1 = 3`
//! only on the three wide paths — the byte paths never set it. `r5` keeps the
//! entry src pointer on the byte paths (they use `r0`/`r7` as cursors) and
//! ends advanced elsewhere; `r3` keeps the last copied unit, sign-extended
//! for byte/word backwards copies (`mov.b/w @Rm,Rn` sign-extend).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    let n = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.set_t(dst == src); // cmp/eq r5,r4
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    if dst == src {
        cpu.r[0] = dst;
        cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return; // T stays set
    }
    cpu.r[2] = 0; // mov #0,r2
    cpu.set_t(n > 0); // cmp/hi r2,r6
    if n == 0 {
        cpu.r[0] = dst;
        cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return; // T stays clear
    }
    cpu.r[7] = dst; // mov r4,r7
    cpu.r[11] = n; // mov r6,r11
    cpu.r[7] = cpu.r[7] | src; // or r5,r7
    cpu.r[12] = n; // mov r6,r12
    cpu.set_t((cpu.r[11] & 1) != 0);
    cpu.r[11] = cpu.r[11] >> 1; // shlr r11
    let back = dst >= src; // cmp/hs r5,r4
    cpu.set_t(back);
    cpu.r[7] = cpu.r[7] | n; // or r6,r7 ; a = dst|src|n
    cpu.r[12] = cpu.r[12] >> 2; // delay slot of `bt.s`: shlr2 r12
    if back {
        backward(cpu, mem, dst, src, n);
    } else {
        forward(cpu, mem, dst, src, n);
    }
    cpu.r[0] = dst; // 0x8c010f3a
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn forward(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32, n: u32) {
    let a = cpu.r[7];
    cpu.r[3] = 1; // mov #1,r3
    cpu.set_t((a & 1) == 0); // tst r7,r3
    if a & 1 == 0 {
        cpu.r[1] = 3; // mov #3,r1
        cpu.set_t((a & 3) == 0); // tst r1,r7
        if a & 3 == 0 {
            // dword forward: r6 = r12, r7 = dst; mov.l @r5+,r3 ... mov.l r3,@r7
            cpu.r[6] = cpu.r[12];
            cpu.r[7] = dst;
            loop {
                let d = mem.read_u32(cpu.r[5]).unwrap_or(0);
                cpu.r[5] = cpu.r[5].wrapping_add(4);
                cpu.r[6] = cpu.r[6].wrapping_sub(1);
                cpu.set_t(cpu.r[6] == 0); // tst r6,r6
                let _ = mem.write_u32(cpu.r[7], d);
                cpu.r[7] = cpu.r[7].wrapping_add(4); // delay slot of `bf.s`
                cpu.r[3] = d;
                if cpu.r[6] == 0 {
                    break;
                }
            }
            return;
        }
        // word forward: r6 = r11, r7 = dst; mov.w @r5+,r3 ... mov.w r3,@r7
        cpu.r[6] = cpu.r[11];
        cpu.r[7] = dst;
        loop {
            let w = mem.read_u16(cpu.r[5]).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[5] = cpu.r[5].wrapping_add(2);
            cpu.r[6] = cpu.r[6].wrapping_sub(1);
            cpu.set_t(cpu.r[6] == 0);
            let _ = mem.write_u16(cpu.r[7], w as u16);
            cpu.r[7] = cpu.r[7].wrapping_add(2); // delay slot of `bf.s`
            cpu.r[3] = w;
            if cpu.r[6] == 0 {
                break;
            }
        }
        return;
    }
    // byte forward: r0 = src, r7 = dst, r6 = n
    cpu.r[0] = src;
    cpu.r[7] = dst;
    cpu.r[6] = n;
    loop {
        let c = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.r[6] = cpu.r[6].wrapping_sub(1);
        cpu.set_t(cpu.r[6] == 0);
        let _ = mem.write_u8(cpu.r[7], c as u8);
        cpu.r[7] = cpu.r[7].wrapping_add(1); // delay slot of `bf.s`
        cpu.r[3] = c;
        if cpu.r[6] == 0 {
            break;
        }
    }
}

fn backward(cpu: &mut Cpu, mem: &mut Memory, dst: u32, src: u32, n: u32) {
    let a = cpu.r[7];
    cpu.r[2] = 1; // mov #1,r2
    cpu.r[0] = dst;
    cpu.set_t((a & 1) == 0); // tst r7,r2
    cpu.r[13] = src; // mov r5,r13
    cpu.r[0] = cpu.r[0].wrapping_add(n); // add r6,r0
    cpu.r[13] = cpu.r[13].wrapping_add(n); // delay slot of `bf.s`
    if a & 1 != 0 {
        // byte backward: r7 = src+n, r5 = dst+n, r6 = n
        cpu.r[7] = cpu.r[13];
        cpu.r[5] = cpu.r[0];
        cpu.r[6] = n;
        loop {
            cpu.r[7] = cpu.r[7].wrapping_sub(1);
            let c = mem.read_u8(cpu.r[7]).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[6] = cpu.r[6].wrapping_sub(1); // dt r6
            cpu.set_t(cpu.r[6] == 0);
            cpu.r[5] = cpu.r[5].wrapping_sub(1); // delay slot: mov.b r3,@-r5
            let _ = mem.write_u8(cpu.r[5], c as u8);
            cpu.r[3] = c;
            if cpu.r[6] == 0 {
                break;
            }
        }
        return;
    }
    cpu.r[1] = 3; // mov #3,r1 at 0x8c010efc
    cpu.set_t((a & 3) == 0); // tst r1,r7
    if a & 3 == 0 {
        // dword backward: r6 = r12, r7 = src+n, r5 = dst+n
        cpu.r[6] = cpu.r[12];
        cpu.r[7] = cpu.r[13];
        cpu.r[5] = cpu.r[0];
        loop {
            cpu.r[7] = cpu.r[7].wrapping_sub(4);
            let d = mem.read_u32(cpu.r[7]).unwrap_or(0);
            cpu.r[6] = cpu.r[6].wrapping_sub(1); // dt r6
            cpu.set_t(cpu.r[6] == 0);
            cpu.r[5] = cpu.r[5].wrapping_sub(4); // delay slot: mov.l r3,@-r5
            let _ = mem.write_u32(cpu.r[5], d);
            cpu.r[3] = d;
            if cpu.r[6] == 0 {
                break;
            }
        }
        return;
    }
    // word backward: r6 = r11, r5 = src+n, r7 = dst+n
    cpu.r[6] = cpu.r[11];
    cpu.r[5] = cpu.r[13];
    cpu.r[7] = cpu.r[0];
    loop {
        cpu.r[5] = cpu.r[5].wrapping_sub(2);
        let w = mem.read_u16(cpu.r[5]).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // dt r6
        cpu.set_t(cpu.r[6] == 0);
        cpu.r[7] = cpu.r[7].wrapping_sub(2); // delay slot: mov.w r3,@-r7
        let _ = mem.write_u16(cpu.r[7], w as u16);
        cpu.r[3] = w;
        if cpu.r[6] == 0 {
            break;
        }
    }
}
