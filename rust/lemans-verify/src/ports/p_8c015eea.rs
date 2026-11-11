//! 0x8c015eea — increment the decimal string at r4 in place when the
//! guard digit base[n] is > '4': base[n] = '0', base[n-1]++, then propagate
//! the carry down to base[1], and finally re-check base[1] (bumping
//! base[0] if needed).
//!
//! Hidden side effects the C does not show: r0 is never written; the entry
//! `bf.s` delay slot is absent so the skip path leaves r1/r7 at their seed
//! values; the loop `bf.s` delay slot sets r7 = 57; inside the loop r5 and
//! r6 stay equal and both decrement (delay slot), so the carry OR writes
//! target the current byte while the +1 lands on byte below; r3 ends as
//! the r5-1 pointer from the last carry, or base[n-1]+1; r6 ends at r4 on
//! loop exit, r5 at r4+1; exit T is the final `cmp/gt r7,r2` on base[1].

use sh4_oracle::{Cpu, Memory};

fn signext(b: u8) -> u32 {
    (b as i8 as i32) as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let n = cpu.r[5];
    let mut r5 = n;
    let mut r6 = n.wrapping_add(base); // base + n
    let mut r2 = 52u32; // mov #52,r2
    let mut r3 = signext(mem.read_u8(r6).unwrap_or(0)); // mov.b @r6,r3
    let enter = (r3 as i32) > (r2 as i32); // cmp/gt r2,r3
    cpu.set_t(enter);
    if !enter {
        cpu.r[2] = r2;
        cpu.r[3] = r3;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        return; // r0, r1, r7 untouched
    }
    r5 = n.wrapping_sub(1).wrapping_add(base); // base + n - 1
    let r1 = 48u32; // mov #48,r1
    let _ = mem.write_u8(r6, r1 as u8); // base[n] = '0'
    r3 = signext(mem.read_u8(r5).unwrap_or(0)).wrapping_add(1);
    let _ = mem.write_u8(r5, r3 as u8); // base[n-1]++
    r6 = r5; // mov r5,r6 (r5 reloaded from stack slot)
    let mut r7 = 57u32; // delay slot of bf.s: mov #57,r7 (always runs)
    if r6 > base {
        // cmp/hi r4,r6 ; bf.s loop
        loop {
            r2 = signext(mem.read_u8(r6).unwrap_or(0)); // mov.b @r6,r2
            let gt = (r2 as i32) > (r7 as i32); // cmp/gt r7,r2
            if gt {
                r2 = signext(mem.read_u8(r5).unwrap_or(0));
                r3 = r5.wrapping_sub(1);
                r2 = r2.wrapping_sub(10);
                let _ = mem.write_u8(r5, r2 as u8);
                r2 = signext(mem.read_u8(r3).unwrap_or(0)).wrapping_add(1);
                let _ = mem.write_u8(r3, r2 as u8);
            }
            r6 = r6.wrapping_sub(1);
            let cont = r6 > base; // cmp/hi r4,r6
            r5 = r5.wrapping_sub(1); // delay slot of bt.s
            if !cont {
                break;
            }
        }
    }
    // 0x8c015f30
    r5 = base.wrapping_add(1);
    r2 = signext(mem.read_u8(r5).unwrap_or(0));
    let t = (r2 as i32) > (r7 as i32); // cmp/gt r7,r2
    cpu.set_t(t);
    if t {
        r2 = signext(mem.read_u8(r5).unwrap_or(0)).wrapping_sub(10);
        let _ = mem.write_u8(r5, r2 as u8);
        r3 = signext(mem.read_u8(base).unwrap_or(0)).wrapping_add(1);
        let _ = mem.write_u8(base, r3 as u8);
    }
    cpu.r[1] = r1;
    cpu.r[2] = r2;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
