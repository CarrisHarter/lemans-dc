//! 0x8c216dc0 — streq.
//!
//! Returns 1 when the two NUL-terminated strings at `r4` and `r5` are equal,
//! else 0. Both pointers advance past the terminator on the equal path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    let mut result: u32 = 0;
    loop {
        let r2 = (mem.read_u8(r5).unwrap_or(0) as i8 as i32) as u32;
        cpu.r[2] = r2;
        r5 = r5.wrapping_add(1);
        let r3 = (mem.read_u8(r4).unwrap_or(0) as i8 as i32) as u32;
        cpu.r[3] = r3;
        r4 = r4.wrapping_add(1);
        let eq = r2 == r3;
        cpu.set_t(eq);
        if !eq {
            result = 0;
            break;
        }
        let r3b = (mem.read_u8(r4).unwrap_or(0) as i8 as i32) as u32;
        cpu.r[3] = r3b;
        let t = r3b == 0;
        cpu.set_t(t);
        if t {
            let r1 = (mem.read_u8(r5).unwrap_or(0) as i8 as i32) as u32;
            cpu.r[1] = r1;
            let t2 = r1 == 0;
            cpu.set_t(t2);
            result = if t2 { 1 } else { 0 };
            break;
        }
    }
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.r[0] = result; // delay slot of `rts`
}
