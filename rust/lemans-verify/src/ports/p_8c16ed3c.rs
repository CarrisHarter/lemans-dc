//! 0x8c16ed3c — flag-gated clamped stores on a sub-object.
//!
//! If ([r4+8] & 1) == 0: r0=0, T=1, no stores. Otherwise obj=[r4+92]:
//!  - if [obj+108] != r5, clamp r5 to [0, 0x1fff] (pool word at 0x8c16eda0),
//!    store it at obj+108 and OR 0x40000000 (pool at 0x8c16eda4) into [r4+8];
//!  - if [obj+112] != r6, clamp r6 to [0, 31], store it at obj+112 and OR
//!    0x40000000 into [r4+8] again.
//! Final T: 1 if the second cmp/eq matched or bit was clear; else
//! (r6 > 31) in the r6>=0 sub-path, 0 in the r6<0 sub-path. r0 is 0x40000000
//! when the second store ran, else [obj+112]; r1 is obj when the second
//! block was skipped, else [r4+8]|0x40000000.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r0 = mem.read_u32(r4 + 8).unwrap() & 1;
    cpu.set_t(r0 == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = r0;
        return; // rts ; nop
    }

    let obj = mem.read_u32(r4 + 92).unwrap();
    let mut r1 = obj;
    let mut r5 = cpu.r[5];

    // first block at 0x8c16ed44
    let cur = mem.read_u32(obj + 108).unwrap();
    cpu.set_t(cur == r5); // cmp/eq r0,r5
    if !cpu.t() {
        cpu.set_t((r5 as i32) >= 0); // cmp/pz r5
        if cpu.t() {
            let lim = mem.read_u32(0x8c16eda0).unwrap(); // 0x1fff
            let t = (r5 as i32) > lim as i32; // cmp/gt r0,r5 : T = r5 > r0
            cpu.set_t(t);
            if t {
                r5 = lim;
            }
        } else {
            r5 = 0; // bra delay slot
        }
        mem.write_u32(obj + 108, r5).unwrap();
        let orv = mem.read_u32(0x8c16eda4).unwrap(); // 0x40000000
        r1 = mem.read_u32(r4 + 8).unwrap() | orv;
        mem.write_u32(r4 + 8, r1).unwrap();
        cpu.r[5] = r5;
    }

    // second block at 0x8c16ed70
    r1 = mem.read_u32(r4 + 92).unwrap();
    let mut r0 = mem.read_u32(r1 + 112).unwrap();
    let mut r6 = cpu.r[6];
    cpu.set_t(r0 == r6); // cmp/eq r0,r6
    if cpu.t() {
        cpu.r[0] = r0;
        cpu.r[1] = r1;
        return; // bt 0x8c16ed9c ; rts ; nop
    }
    cpu.set_t((r6 as i32) >= 0); // cmp/pz r6
    if cpu.t() {
        let t = (r6 as i32) > 31; // cmp/gt r0,r6 with r0=31 : T = r6 > 31
        cpu.set_t(t);
        if t {
            r6 = 31;
        }
    } else {
        r6 = 0; // bra delay slot
    }
    mem.write_u32(obj + 112, r6).unwrap();
    r0 = mem.read_u32(0x8c16eda4).unwrap(); // 0x40000000
    r1 = mem.read_u32(r4 + 8).unwrap() | r0;
    mem.write_u32(r4 + 8, r1).unwrap();
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[6] = r6;
}
