//! 0x8c1cc3b4 — classify global flag bytes into a small integer code.
//!
//! Original: a chain of `mov.l pool,r0 ; mov.b @r0,r0 ; extu.b ; tst ; bt`
//! tests. Pool at 0x8c1cc440..0x8c1cc458 names the flag bytes:
//! 0x8c2b8758, 0x8c215d88, 0x8c215d80, 0x8c2b8748, 0x8c2b8750, 0x8c2b8738,
//! 0x8c2b8730. Returns 0/1/2/3 in `r0` (set in the rts delay slot). T ends as
//! the last `tst` result (1 when the deciding byte was zero).
//!
//! ```text
//! b(0x8c2b8758)==0 -> 0 ; b(0x8c215d88)==0 -> 0
//! b(0x8c215d80)!=0 -> { b(0x8c2b8748)==0 -> 2
//!                       b(0x8c2b8750)==0 -> 3 ; else 1 }
//! b(0x8c215d80)==0 -> { b(0x8c2b8738)!=0 -> { b(0x8c2b8748)==0 -> 0
//!                                             b(0x8c2b8750)==0 -> 3 ; else 1 }
//!                       b(0x8c2b8738)==0 -> { b(0x8c2b8730)==0 -> 0 ; else 2 } }
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Each step mirrors one mov.l/mov.b/extu.b/tst from the original.
    let flag = |mem: &Memory, pool: u32| -> u32 {
        let p = mem.read_u32(pool).unwrap_or(0);
        mem.read_u8(p).unwrap_or(0) as u32
    };
    let b = flag(mem, 0x8c1cc440);
    cpu.set_t(b == 0);
    if b == 0 {
        cpu.r[0] = 0; // rts delay: mov #0,r0
        return;
    }
    let b = flag(mem, 0x8c1cc444);
    cpu.set_t(b == 0);
    if b == 0 {
        cpu.r[0] = 0;
        return;
    }
    let b = flag(mem, 0x8c1cc448);
    cpu.set_t(b == 0);
    if b != 0 {
        // 0x8c1cc3d8 branch (0x8c215d80 flag set)
        let b = flag(mem, 0x8c1cc44c); // 0x8c2b8748
        cpu.set_t(b == 0);
        if b == 0 {
            cpu.r[0] = 2; // 0x8c1cc3fa: rts delay mov #2,r0
            return;
        }
        let b = flag(mem, 0x8c1cc450); // 0x8c2b8750
        cpu.set_t(b == 0);
        if b == 0 {
            cpu.r[0] = 3; // 0x8c1cc3f4
            return;
        }
        cpu.r[0] = 1; // 0x8c1cc3f0: rts delay mov #1,r0
        return;
    }
    // 0x8c1cc3fe branch (0x8c215d80 flag clear)
    let b = flag(mem, 0x8c1cc454); // 0x8c2b8738
    cpu.set_t(b == 0);
    if b == 0 {
        let b = flag(mem, 0x8c1cc458); // 0x8c2b8730
        cpu.set_t(b == 0);
        if b == 0 {
            cpu.r[0] = 0; // 0x8c1cc43c
            return;
        }
        cpu.r[0] = 2; // 0x8c1cc438
        return;
    }
    let b = flag(mem, 0x8c1cc44c); // 0x8c2b8748
    cpu.set_t(b == 0);
    if b == 0 {
        cpu.r[0] = 0;
        return;
    }
    let b = flag(mem, 0x8c1cc450); // 0x8c2b8750
    cpu.set_t(b == 0);
    if b == 0 {
        cpu.r[0] = 3; // 0x8c1cc426
        return;
    }
    cpu.r[0] = 1; // 0x8c1cc422
}
