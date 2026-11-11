//! 0x8c215fe4 — byte copy `r4`→`r5`, do-while on `r6`.
//!
//! ```text
//! loop: r3 = [r4+] ; [r5] = r3 ; r5 += 1
//!       cmp/pl r6 ; bt.s loop ; add #-1,r6
//! rts ; nop
//! ```
//!
//! The body runs before the first test, so `r6 == 0` still copies one byte.
//! `mov.b` sign-extends into `r3`, and `r6` ends at 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let byte = mem.read_u8(cpu.r[4]).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.r[3] = byte as i8 as i32 as u32; // mov.b sign-extends
        let _ = mem.write_u8(cpu.r[5], byte);
        cpu.r[5] = cpu.r[5].wrapping_add(1);

        let go = (cpu.r[6] as i32) > 0;
        cpu.set_t(go); // cmp/pl r6
        cpu.r[6] = cpu.r[6].wrapping_sub(1); // delay slot of `bt.s`
        if !go {
            break;
        }
    }
}
