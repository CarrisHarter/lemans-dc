//! 0x8c0bd52c — param_3[i] := max(param_1[i], param_2[i]) (signed) for i in 0..12.
//!
//! CFG: prologue (push r14/r13, r15 -= 48), six if/else maxes for the fields
//! at offsets 12,4,8,0,16,20, then a 6-iteration loop (bt.s, delay slot
//! `add #4,r14`) over offsets 24..=44, then 12 stores through r3, epilogue.
//! Every branch rejoins immediately; the loop runs exactly 6 times.
//!
//! param_3 is the stack argument: `mov.l @(56,r15),r3` after r15 -= 56
//! reads [r15_entry], so in the port it is mem[r15].
//!
//! Hidden side effects: r0 := max(p1[11],p2[11]), r1 := r15_entry-32
//! (r15'+24 with r15' = r15_entry-56), r3 := p3+24, r4 := max(p1[10],p2[10]),
//! r5 := p2+24, r6 := 6, r7 := 6, T := 0 (final `cmp/gt r7,r6`: 6 > 6 is
//! false). r13/r14/r15 net unchanged (pushed/popped). The 12 stores go to
//! p3 in ascending order.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let p2 = cpu.r[5];
    let sp = cpu.r[15];
    let p3 = mem.read_u32(sp).unwrap(); // @(56,r15) with r15 already -56
    let mut last: [u32; 2] = [0; 2]; // [max at +40, max at +44]
    for i in 0..12u32 {
        let off = 4 * i;
        let a = mem.read_u32(p1 + off).unwrap() as i32;
        let b = mem.read_u32(p2 + off).unwrap() as i32;
        let m = a.max(b) as u32;
        if off == 40 {
            last[0] = m;
        }
        if off == 44 {
            last[1] = m;
        }
        let _ = mem.write_u32(p3 + off, m);
    }
    cpu.r[0] = last[1]; // mov.l @(20,r1),r0 with r1 = r15'+24
    cpu.r[1] = sp.wrapping_sub(32);
    cpu.r[3] = p3 + 24; // add #24,r3
    cpu.r[4] = last[0]; // mov.l @(16,r1),r4
    cpu.r[5] = p2 + 24; // add #24,r5
    cpu.r[6] = 6;
    cpu.r[7] = 6; // six `add #1,r7`
    cpu.set_t(false); // cmp/gt r7,r6 -> 6 > 6
}
