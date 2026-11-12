//! 0x8c010f44 — byte-wise copy of n bytes r5 -> r4 (forward or backward).
//!
//! `cmp/eq`/`bt` guard r4==r5 (T ends set, r0 = r4, nothing else moves).
//! `cmp/hs r5,r4` picks direction: dst < src copies forward, dst >= src
//! copies backward. The `rts` delay slot always reloads r0 = r4; T ends set
//! on every path. The `bt.s` delay slots set r7/r0 even when n == 0.
//! Every access is a **byte** access: the oracle maps size nibble 4 (`mov.b`)
//! to sign-extended `read_u8`/`write_u8`; an earlier build decoded 4/5/6 as
//! 32-bit, which is what the removed NOTE described.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    let n = cpu.r[6];
    cpu.set_t(dst == src); // cmp/eq r5,r4
    if dst == src {
        cpu.r[0] = dst; // delay slot of `rts`
        return;
    }
    if dst < src {
        // forward
        let mut r0 = src; // mov r5,r0
        cpu.r[5] = 0;     // mov #0,r5
        cpu.r[3] = 0;     // mov r5,r3
        cpu.set_t(0 >= n); // cmp/hs r6,r3
        let mut r7 = dst;  // delay slot of `bt.s`
        if n > 0 {
            let mut r5 = 0u32;
            loop {
                let c = sext8(mem, r0); // mov.b @r0+,r3 (byte, sign-extended)
                r0 = r0.wrapping_add(1);
                r5 = r5.wrapping_add(1);
                cpu.set_t(r5 >= n); // cmp/hs r6,r5
                let _ = mem.write_u8(r7, c as u8); // mov.b r3,@r7 stores low byte
                r7 = r7.wrapping_add(1); // delay slot of `bf.s`
                cpu.r[3] = c;
                cpu.r[5] = r5;
                if r5 >= n {
                    break;
                }
            }
        }
        cpu.r[7] = r7;
    } else {
        // backward
        let mut r0 = src; // mov r5,r0
        cpu.r[5] = 0;
        cpu.r[2] = 0; // mov r5,r2
        let mut r7 = dst.wrapping_add(n); // mov r4,r7 ; add r6,r7
        cpu.set_t(0 >= n); // cmp/hs r6,r2
        r0 = r0.wrapping_add(n); // delay slot of `bt.s`
        if n > 0 {
            let mut r5 = 0u32;
            loop {
                r0 = r0.wrapping_sub(1);
                r5 = r5.wrapping_add(1);
                let c = sext8(mem, r0); // mov.b @r0,r3 (true byte load)
                cpu.set_t(r5 >= n);     // cmp/hs r6,r5
                r7 = r7.wrapping_sub(1); // delay slot: mov.b r3,@-r7
                let _ = mem.write_u8(r7, c as u8); // mov.b r3,@-r7 stores one byte
                cpu.r[3] = c;
                cpu.r[5] = r5;
                if r5 >= n {
                    break;
                }
            }
        }
        cpu.r[7] = r7;
    }
    cpu.r[0] = dst; // delay slot of `rts`
}
