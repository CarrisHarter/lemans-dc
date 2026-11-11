//! 0x8c16eda8 — flag-gated clamped store of r5 at [r4+92]+20.
//!
//! If ([r4+8] & 1) == 0: r0=0, T=1, no stores. Otherwise, if the byte at
//! pool(0x8c27a890) is nonzero, r5 = 127 - r5 first. If r5 == [obj+20]
//! (obj=[r4+92]) the routine returns early (r0=[obj+20], T=1). Otherwise the
//! bt.s delay slot always stores [obj+20] at obj+16, r5 is clamped to
//! [0,127], stored at obj+20, and 0x00200000 (pool at 0x8c16edfc) is ORed
//! into [r4+8]. Final r0=0x00200000, r1=[r4+8]|0x00200000; T = (r5 > 127)
//! in the r5>=0 sub-path, else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r0 = mem.read_u32(r4 + 8).unwrap() & 1;
    cpu.set_t(r0 == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = r0;
        return; // rts ; nop
    }

    let mut r5 = cpu.r[5];
    let g = mem.read_u8(mem.read_u32(0x8c16edf8).unwrap()).unwrap(); // 0x8c27a890
    let g = ((g as i8) as u32) & 0xFF; // mov.b sign-extends, two extu.b undo it
    cpu.set_t(g == 0); // tst r0,r0
    if g != 0 {
        r5 = 127u32.wrapping_sub(r5); // mov #127,r0 ; sub r5,r0 ; mov r0,r5
    }

    let obj = mem.read_u32(r4 + 92).unwrap();
    let cur = mem.read_u32(obj + 20).unwrap();
    cpu.set_t(cur == r5); // cmp/eq r0,r5
    if cpu.t() {
        cpu.r[0] = cur;
        cpu.r[1] = obj;
        cpu.r[5] = r5;
        return; // bt 0x8c16edf4 ; rts ; nop
    }

    // bt.s delay slot: [obj+12+4] = [obj+20], runs whether or not taken
    mem.write_u32(obj + 16, cur).unwrap();
    cpu.set_t((r5 as i32) >= 0); // cmp/pz r5
    if cpu.t() {
        let t = (r5 as i32) > 127; // cmp/gt r0,r5 with r0=127 : T = r5 > 127
        cpu.set_t(t);
        if t {
            r5 = 127;
        }
    } else {
        r5 = 0; // bra delay slot
    }
    let obj = mem.read_u32(r4 + 92).unwrap();
    let orv = mem.read_u32(0x8c16edfc).unwrap(); // 0x00200000
    mem.write_u32(obj + 20, r5).unwrap();
    let r1 = mem.read_u32(r4 + 8).unwrap() | orv;
    mem.write_u32(r4 + 8, r1).unwrap();
    cpu.r[0] = orv;
    cpu.r[1] = r1;
    cpu.r[5] = r5;
}
