//! 0x8c015324 — little-endian bignum `a - b` over `r7` bytes, storing the
//! difference back into `a` from the top down; `r4 == 0` turns a final
//! borrow into a -1 return.
//!
//! Hidden side effects the C does not show: r12/r13/r14 are pushed and
//! popped (only r0..r3, r6, r7 survive); the entry `bf.s` delay slot bumps
//! r1 to a+n so r1 ends at exactly `a`; r6 ends at a-1 and r7 at b-1;
//! r2 and r3 both keep the last result byte; the borrow is folded with
//! 0x100 (`mov.w @0x8c01539a`); the return is -1 only when the final borrow
//! is set AND r4 == 0, else 0 when every result byte was 0, else 1.
//! The exit T bit comes from the last `tst r4,r4` / `cmp/eq #1,r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let a = cpu.r[5];
    let b = cpu.r[6];
    let n = cpu.r[7];
    let mut r1 = n.wrapping_sub(1).wrapping_add(a); // a + n - 1
    let mut r7 = n.wrapping_sub(1).wrapping_add(b); // b + n - 1
    let mut r6 = r1; // a + n - 1
    let mut borrow: u32 = 0; // r14 (saved/restored)
    let mut all_zero = true; // r12 (saved/restored)
    let mut res = cpu.r[2]; // r2/r3 keep the last result byte
    // cmp/hs r5,r6 ; bf.s (delay slot: r1 += 1, always executed)
    r1 = r1.wrapping_add(1);
    if r6 >= a {
        loop {
            let bb = mem.read_u8(r7).unwrap_or(0) as u32;
            let aa = mem.read_u8(r6).unwrap_or(0) as u32;
            let raw = aa.wrapping_sub(bb).wrapping_sub(borrow);
            let new_borrow;
            if (raw as i32) >= 0 {
                res = raw;
                new_borrow = 0;
            } else {
                res = 0x0100u32.wrapping_add(raw); // mov.w @0x8c01539a
                new_borrow = 1;
            }
            borrow = new_borrow;
            if res != 0 {
                all_zero = false; // mov r13,r12 with r13 == 0
            }
            r6 = r6.wrapping_sub(1);
            let cont = r6 >= a; // cmp/hs r5,r6
            r1 = r1.wrapping_sub(1);
            let _ = mem.write_u8(r1, res as u8); // mov.b r3,@-r1 (delay slot)
            r7 = r7.wrapping_sub(1); // delay slot of bt.s
            if !cont {
                break;
            }
        }
    }
    // r0 = borrow; cmp/eq #1,r0; bf; tst r4,r4; bf; bra (delay mov #-1,r0)
    if borrow == 1 && p1 == 0 {
        cpu.set_t(true); // tst r4,r4 with r4 == 0
        cpu.r[0] = 0xffffffff;
    } else {
        let t = all_zero; // mov r12,r0 ; cmp/eq #1,r0
        cpu.set_t(t);
        cpu.r[0] = if t { 0 } else { 1 }; // bra delay slot mov #0,r0 / mov #1,r0
    }
    cpu.r[1] = r1;
    cpu.r[2] = res;
    cpu.r[3] = res;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
