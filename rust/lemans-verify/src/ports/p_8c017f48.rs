//! 0x8c017f48 — strcpy loop: copies bytes from r5 to r4 including the NUL.
//!
//! `mov.b @r5+,r2` sign-extends the byte into r2 and steps r5 by 1;
//! `mov.b r2,@r4` stores the low byte and steps r4 by 1. Side effects the C
//! hides: r2 = last byte (the NUL, so r2 = 0), both pointers advance past it,
//! and the loop exits with T = 1 from `tst r2,r2`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    loop {
        let v = mem.read_u8(r5).unwrap_or(0) as i8 as i32 as u32; // mov.b @r5+,r2
        r5 = r5.wrapping_add(1);
        let _ = mem.write_u8(r4, v as u8); // mov.b r2,@r4
        r4 = r4.wrapping_add(1);
        cpu.r[2] = v;
        cpu.set_t(v == 0); // tst r2,r2
        if v == 0 {
            break; // bf not taken
        }
    }
    cpu.r[4] = r4;
    cpu.r[5] = r5;
}
