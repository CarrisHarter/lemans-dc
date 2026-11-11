//! 0x8c07093c — byte copy returning the original destination.
//!
//! ```text
//! r0 = r4 ; tst r6,r6 ; r3 = r5+31
//! bt.s end ; add r5,r6        ; delay slot always runs: r6 = src + len
//! loop: r2 = *r5++ ; r3++ ; cmp/eq r5,r6 ; *r4 = r2 ; bf.s loop ; r4++
//! end:  rts ; nop
//! ```
//!
//! Hidden side effects: `r6` becomes the end pointer even on the zero-length
//! path (the `bt.s` delay slot always runs), `r3` is `src+31` plus the number
//! of bytes copied, `r2` keeps the last byte, and T is 1 on both exits (the
//! `tst` on the empty path, the final `cmp/eq` otherwise).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let len = cpu.r[6];

    cpu.r[0] = dst; // mov r4,r0
    cpu.set_t(len == 0); // tst r6,r6
    cpu.r[3] = cpu.r[5].wrapping_add(31);
    cpu.r[6] = cpu.r[5].wrapping_add(len); // delay slot of `bt.s`, always runs

    if len != 0 {
        loop {
            let byte = mem.read_u8(cpu.r[5]).unwrap_or(0);
            cpu.r[2] = byte as i8 as i32 as u32; // mov.b @r5+,r2
            cpu.r[5] = cpu.r[5].wrapping_add(1);
            cpu.r[3] = cpu.r[3].wrapping_add(1);
            let done = cpu.r[5] == cpu.r[6];
            cpu.set_t(done); // cmp/eq r5,r6
            let _ = mem.write_u8(cpu.r[4], byte); // mov.b r2,@r4
            cpu.r[4] = cpu.r[4].wrapping_add(1); // delay slot of `bf.s`
            if done {
                break;
            }
        }
    }
}
