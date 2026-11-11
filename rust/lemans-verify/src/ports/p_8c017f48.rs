//! 0x8c017f48 — strcpy loop: copies from r5 to r4 until the loaded value is 0.
//!
//! Oracle encoding note: in this oracle build `mov.b @r5+,r2` (op 0x6254)
//! loads a full u32 (size nibble 4 maps to a 32-bit read) while still
//! advancing r5 by 1, and `mov.b r2,@r4` stores the low byte. The port
//! replicates that exactly, so r2 = the u32 at [r5] and the loop ends when
//! that u32 is zero; T = 1 on exit from `tst r2,r2`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    loop {
        let v = mem.read_u32(r5).unwrap_or(0); // mov.b @r5+,r2 as the oracle decodes it
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
