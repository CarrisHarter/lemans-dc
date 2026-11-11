//! 0x8c02ca34 — normalised angle: `1.0 - (c2 / (half*v)) * w` with early-outs.
//!
//! ```text
//! mov.l  0x8c02ca78,r0        ; r0 = 0x44c
//! mov.l  @(r0,r4),r0          ; r0 = [r4 + 0x44c]
//! tst    r0,r0
//! bf.s   0x8c02ca72           ; zero -> return 1.0
//! fldi1  fr0                  ; delay slot: fr0 = 1.0 on EVERY path
//! mov.l  0x8c02ca7c,r1        ; r1 = 0x450
//! mov    r4,r0
//! add    r1,r0
//! fmov   @r0,fr1              ; fr1 = [r4 + 0x450]
//! fcmp/gt fr0,fr1             ; T = fr1 > fr0 (1.0)
//! bt     0x8c02ca72           ; > 1.0 -> return 1.0
//! mov.l  @(4,r4),r3
//! mova   0x8c02ca80,r0
//! mov.l  0x8c02ca84,r1        ; r1 = 0x278
//! fmov   @r0,fr1              ; fr1 = 0.5 (const at 0x8c02ca80)
//! add    r1,r3
//! mov.l  0x8c02ca88,r1        ; r1 = 0x39c
//! fmov   @r3,fr3
//! add    r1,r4                ; r4 += 0x39c
//! fmov   @r4,fr2
//! fmul   fr1,fr3              ; fr3 = 0.5 * v
//! fcmp/gt fr3,fr2             ; T = fr2 > fr3
//! bf     0x8c02ca68
//! mova   0x8c02ca8c,r0
//! rts
//! fmov   @r0,fr0              ; delay slot: fr0 = const 0x3da3d70a (fr2 > fr3)
//! mova   0x8c02ca90,r0
//! fmov   @r0,fr1              ; fr1 = 0x3f6b851f
//! fdiv   fr3,fr1              ; fr1 /= fr3
//! fmul   fr2,fr1              ; fr1 *= fr2
//! fsub   fr1,fr0              ; fr0 = 1.0 - fr1
//! rts
//! nop
//! ```
//! Hidden side effects: `fldi1 fr0` in the `bf.s` delay slot runs on all
//! paths (it is the 1.0 return value); r0 ends as the 0x44c slot value, then
//! r4+0x450, then a mova constant; r1 ends as 0x39c; r3/r4 stay advanced;
//! T is 0 on the zero path, 1 on both early-outs, and `fr2 > fr3` on the
//! main paths (the branch is `fcmp/gt fr3,fr2`, i.e. T = fr2 > fr3).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8C02CA78).unwrap(); // 0x44c
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(base)).unwrap();
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.fr[0] = 1.0; // delay slot of `bf.s`, runs on every path
    if cpu.r[0] == 0 {
        return; // bf.s target: rts + nop
    }
    cpu.r[1] = mem.read_u32(0x8C02CA7C).unwrap(); // 0x450
    cpu.r[0] = base.wrapping_add(cpu.r[1]);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    let over = cpu.fr[1] > cpu.fr[0]; // fcmp/gt fr0,fr1 -> T = fr1 > fr0
    cpu.set_t(over);
    if over {
        return; // bt target: rts + nop
    }
    cpu.r[3] = mem.read_u32(base.wrapping_add(4)).unwrap();
    cpu.r[0] = 0x8C02CA80; // mova
    cpu.r[1] = mem.read_u32(0x8C02CA84).unwrap(); // 0x278
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap(); // 0.5
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    cpu.r[1] = mem.read_u32(0x8C02CA88).unwrap(); // 0x39c
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[4] = base.wrapping_add(cpu.r[1]);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[3] = cpu.fr[3] * cpu.fr[1]; // fmul fr1,fr3
    let over = cpu.fr[2] > cpu.fr[3]; // fcmp/gt fr3,fr2 -> T = fr2 > fr3
    cpu.set_t(over);
    if over {
        cpu.r[0] = 0x8C02CA8C; // mova
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // delay slot of `rts`
        return;
    }
    cpu.r[0] = 0x8C02CA90; // mova
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[1] = cpu.fr[1] / cpu.fr[3]; // fdiv fr3,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
    cpu.fr[0] = cpu.fr[0] - cpu.fr[1]; // fsub fr1,fr0
}
