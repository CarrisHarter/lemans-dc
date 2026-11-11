//! 0x8c0bd8c8 — set_mode_or_clear.
//!
//! `tst r5,r5 ; bf L ; [r4+60]=0 ; rts` else `r0=r5 ; cmp/eq #1,r0 ; bf end ;
//! [r4+64]=0` — a null mode clears `+60`, mode 1 clears `+64`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let zero = cpu.r[5] == 0;
    cpu.set_t(zero); // tst r5,r5
    if zero {
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[4] + 60, 0); // delay slot of `rts`
        return;
    }
    cpu.r[0] = cpu.r[5];
    let one = cpu.r[0] == 1;
    cpu.set_t(one); // cmp/eq #1,r0
    if one {
        cpu.r[0] = 64;
        cpu.r[1] = 0;
        let _ = mem.write_u32(cpu.r[4] + 64, 0);
    }
}
