//! 0x8c215ec8 — four block operations driven by global pointer/end pairs:
//!   1. fill u32 zeros at *0x8c1ccf40 .. *0x8c1ccf4c (step 4)
//!   2. copy bytes *0x8c1ccf28 -> *0x8c1ccf30 .. *0x8c1ccf3c
//!   3. fill bytes 0 at *0x8c23bcf8 .. *0x8c23bd04
//!   4. copy bytes *0x8c23bce0 -> *0x8c23bce8 .. *0x8c23bcf4
//!
//! Every loop is a bottom-tested `bra` into `cmp/hs end,ptr ; bf body`, so the
//! end pointers are reloaded through their globals on each pass. `r4` is the
//! fill value (0) until loop 4 loads the destination pointer into it. The
//! rts delay slot is a nop; T ends 1 (last `cmp/hs` saw ptr >= end).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);

    // Loop 1 (0x8c215ed6): u32 zero fill.
    let r3pool = p(mem, 0x8c215f48); // 0x8c1ccf40
    let r6pool = p(mem, 0x8c215f44); // 0x8c1ccf4c
    cpu.r[4] = 0;
    cpu.r[3] = r3pool;
    cpu.r[6] = r6pool;
    let mut r5 = p(mem, r3pool);
    loop {
        let end = p(mem, r6pool);
        cpu.r[3] = end;
        cpu.set_t(r5 >= end); // cmp/hs r3,r5
        if r5 < end {
            mem.write_u32(r5, cpu.r[4]).ok();
            r5 = r5.wrapping_add(4);
        } else {
            break;
        }
    }
    cpu.r[5] = r5;

    // Loop 2 (0x8c215eee): byte copy src -> dst.
    let r3pool = p(mem, 0x8c215f50); // 0x8c1ccf30
    let r2pool = p(mem, 0x8c215f54); // 0x8c1ccf28
    let r7pool = p(mem, 0x8c215f4c); // 0x8c1ccf3c
    cpu.r[3] = r3pool;
    cpu.r[2] = r2pool;
    cpu.r[7] = r7pool;
    let mut r5 = p(mem, r3pool);
    let mut r6 = p(mem, r2pool); // bra delay slot
    cpu.r[6] = r6;
    loop {
        let end = p(mem, r7pool);
        cpu.r[2] = end;
        cpu.set_t(r5 >= end); // cmp/hs r2,r5
        if r5 < end {
            let b = mem.read_u8(r6).unwrap_or(0) as i8 as i32 as u32; // mov.b @r6+,r3
            r6 = r6.wrapping_add(1);
            cpu.r[6] = r6;
            cpu.r[3] = b;
            mem.write_u8(r5, b as u8).ok(); // mov.b r3,@r5
            r5 = r5.wrapping_add(1);
        } else {
            break;
        }
    }
    cpu.r[5] = r5;

    // Loop 3 (0x8c215f00): byte zero fill.
    let r2pool = p(mem, 0x8c215f5c); // 0x8c23bcf8
    let r6pool = p(mem, 0x8c215f58); // 0x8c23bd04
    cpu.r[2] = r2pool;
    cpu.r[6] = r6pool;
    let mut r5 = p(mem, r2pool); // bra delay slot
    loop {
        let end = p(mem, r6pool);
        cpu.r[2] = end;
        cpu.set_t(r5 >= end); // cmp/hs r2,r5
        if r5 < end {
            mem.write_u8(r5, cpu.r[4] as u8).ok(); // mov.b r4,@r5
            r5 = r5.wrapping_add(1);
        } else {
            break;
        }
    }
    cpu.r[5] = r5;

    // Loop 4 (0x8c215f18): byte copy src -> dst (dst pointer in r4).
    let r2pool = p(mem, 0x8c215f64); // 0x8c23bce8
    let r3pool = p(mem, 0x8c215f68); // 0x8c23bce0
    let r6pool = p(mem, 0x8c215f60); // 0x8c23bcf4
    cpu.r[2] = r2pool;
    cpu.r[3] = r3pool;
    cpu.r[4] = p(mem, r2pool);
    cpu.r[6] = r6pool;
    let mut r4 = cpu.r[4];
    let mut r5 = p(mem, r3pool); // bra delay slot
    loop {
        let end = p(mem, r6pool);
        cpu.r[3] = end;
        cpu.set_t(r4 >= end); // cmp/hs r3,r4
        if r4 < end {
            let b = mem.read_u8(r5).unwrap_or(0) as i8 as i32 as u32; // mov.b @r5+,r2
            r5 = r5.wrapping_add(1);
            cpu.r[2] = b;
            mem.write_u8(r4, b as u8).ok(); // mov.b r2,@r4
            r4 = r4.wrapping_add(1);
        } else {
            break;
        }
    }
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    // rts ; nop (delay)
}
