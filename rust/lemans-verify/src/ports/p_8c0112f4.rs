//! 0x8c0112f4 — strlen: count bytes up to the NUL at r4.
//!
//! `bra` delay slot zeroes r5; the loop leaves r4 one past the terminating
//! byte, r3 = the sign-extended terminator (0), and the `rts` delay slot moves
//! the count r5 into r0. T ends set.
//!
//! `mov.b @r4+,r3` is a **byte** load: the oracle maps size nibble 4 to
//! `read_u8` sign-extended (lib.rs `load`, which documents that mapping 4/5/6
//! to a 32-bit access "made `mov.b @Rm+,Rn` read four bytes"). An earlier
//! version of this port used `read_u32` and diverged whenever the bytes after
//! the NUL inside the same word were non-zero; `notes/specs/0x8c0112f4.alias.json`
//! is the witness.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut p = cpu.r[4];
    let mut len = 0u32;
    loop {
        let c = mem.read_u8(p).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4+,r3
        p = p.wrapping_add(1);
        cpu.r[3] = c;
        if c == 0 {
            cpu.set_t(true);
            break; // bf not taken
        }
        len = len.wrapping_add(1); // 0x8c0112f8
    }
    cpu.r[5] = len;
    cpu.r[4] = p;
    cpu.r[0] = len; // delay slot of `rts`
}
