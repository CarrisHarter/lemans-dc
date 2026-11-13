//! 0x8c0cc758 — wire two vtable chains, tear down two subobjects, maybe free.
//!
//! A null object returns 0 with T set. Otherwise the chain at `[object]`,
//! `[[object]]`, `[[[object]]]` and `[object]`'s second word is stamped, and
//! the relative pointers `(object+40)`, `(object+68)`, `(object+108)` and
//! `(object+0x80)` are stored back into that chain. `tst` of the sign-extended
//! count is consumed by a later `bt.s`: zero skips the second chain and both
//! teardowns. A non-zero low half stamps the chain at `[object+0x80]`, then
//! calls `0x8c0cd048(object+108, 0)` and `0x8c07c100(object+68, 0)` and stores
//! `0x8c1e9ad4` at `[object+40]`. A positive low half frees the object.
//! `r0` is the object. `r9`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.r[0] = rd(mem, 0x8c0cc86c); // 0x8c1ebcec
        cpu.r[10] = cpu.r[13];
        cpu.r[11] = cpu.r[13];
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[0]);
        cpu.r[12] = cpu.r[13];
        cpu.r[3] = rd(mem, 0x8c0cc870); // 0xec
        cpu.r[1] = cpu.r[0];
        cpu.r[4] = rd(mem, cpu.r[13]);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
        cpu.r[10] = cpu.r[10].wrapping_add(40);
        let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[1]);
        cpu.r[11] = cpu.r[11].wrapping_add(68);
        cpu.r[3] = rd(mem, cpu.r[13]);
        cpu.r[1] = cpu.r[14] as i16 as i32 as u32; // exts.w
        cpu.r[5] = rd(mem, 0x8c0cc874); // 0xd4
        cpu.set_t(cpu.r[1] == 0); // tst; T survives until the bt.s
        cpu.r[4] = rd(mem, cpu.r[3]);
        cpu.r[12] = cpu.r[12].wrapping_add(108);
        cpu.r[7] = cpu.r[13];
        cpu.r[3] = cpu.r[0];
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]);
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[3]);
        cpu.r[6] = rd(mem, cpu.r[13]);
        cpu.r[3] = cpu.r[0];
        cpu.r[4] = rd(mem, 0x8c0cc878); // 0xb4
        cpu.r[0] = cpu.r[0].wrapping_add(56);
        cpu.r[6] = rd(mem, cpu.r[6]);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        cpu.r[6] = rd(mem, cpu.r[6]);
        let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[3]);
        cpu.r[6] = rd(mem, cpu.r[13]);
        cpu.r[3] = rd(mem, 0x8c0cc87c); // 0x80
        cpu.r[9] = rd(mem, cpu.r[6]);
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
        cpu.r[1] = rd(mem, cpu.r[9]);
        cpu.r[6] = cpu.r[7];
        cpu.r[1] = rd(mem, cpu.r[1]);
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        cpu.r[0] = rd(mem, cpu.r[13]);
        cpu.r[0] = rd(mem, cpu.r[0].wrapping_add(4));
        cpu.r[10] = cpu.r[10].wrapping_sub(cpu.r[0]);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[10]);
        cpu.r[0] = rd(mem, cpu.r[13]);
        cpu.r[0] = rd(mem, cpu.r[0]);
        cpu.r[0] = rd(mem, cpu.r[0]);
        cpu.r[11] = cpu.r[11].wrapping_sub(cpu.r[0]);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(36), cpu.r[11]);
        cpu.r[0] = rd(mem, cpu.r[13]);
        cpu.r[0] = rd(mem, cpu.r[0]);
        cpu.r[12] = cpu.r[12].wrapping_sub(cpu.r[0]);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[12]);
        cpu.r[0] = rd(mem, cpu.r[13]);
        cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[0]);
        let skip = cpu.t();
        let _ = mem.write_u32(cpu.r[0].wrapping_add(12), cpu.r[6]); // bt.s delay, always
        if !skip {
            cpu.set_t(cpu.r[7] == 0);
            if !cpu.t() {
                cpu.r[1] = rd(mem, 0x8c0cc880); // 0x8c1ebe10
                cpu.r[0] = rd(mem, 0x8c0cc884); // 0x88
                cpu.r[6] = cpu.r[1];
                cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[5]);
                let _ = mem.write_u32(cpu.r[13].wrapping_add(cpu.r[0]), cpu.r[1]);
                cpu.r[5] = cpu.r[1];
                cpu.r[0] = cpu.r[3];
                cpu.r[7] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[4]);
                cpu.r[1] = cpu.r[1].wrapping_add(56);
                let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[6]);
                cpu.r[4] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[7] = cpu.r[13];
                cpu.r[6] = rd(mem, cpu.r[4]);
                cpu.r[4] = rd(mem, 0x8c0cc888); // 0x8c
                let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[5]);
                cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[4]);
                cpu.r[4] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[6] = cpu.r[13];
                cpu.r[5] = rd(mem, cpu.r[4]);
                cpu.r[0] = rd(mem, 0x8c0cc88c); // 0xa8
                cpu.r[4] = cpu.r[13];
                cpu.r[12] = rd(mem, cpu.r[5]);
                cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[0]);
                cpu.r[0] = rd(mem, 0x8c0cc890); // 0x84
                cpu.r[5] = rd(mem, 0x8c0cc894); // 0xd0
                let _ = mem.write_u32(cpu.r[12], cpu.r[1]);
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[5]);
                cpu.r[0] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[7] = cpu.r[7].wrapping_sub(cpu.r[0]);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[7]);
                cpu.r[0] = cpu.r[3];
                cpu.r[1] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[1] = rd(mem, cpu.r[1]);
                cpu.r[6] = cpu.r[6].wrapping_sub(cpu.r[1]);
                let _ = mem.write_u32(cpu.r[1].wrapping_add(36), cpu.r[6]);
                cpu.r[0] = rd(mem, cpu.r[13].wrapping_add(cpu.r[0]));
                cpu.r[4] = cpu.r[4].wrapping_sub(cpu.r[0]);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(16), cpu.r[4]);
            }
            cpu.r[0] = rd(mem, 0x8c0cc898); // 0x8c0cd048
            cpu.r[4] = cpu.r[13];
            cpu.r[4] = cpu.r[4].wrapping_add(108);
            cpu.pr = 0x8c0cc838;
            cpu.r[5] = 0;
            crate::registry::p_8c0cd048::run(cpu, mem);
            cpu.r[0] = rd(mem, 0x8c0cc89c); // 0x8c07c100
            cpu.r[4] = cpu.r[13];
            cpu.r[4] = cpu.r[4].wrapping_add(68);
            cpu.pr = 0x8c0cc842;
            cpu.r[5] = 0;
            crate::registry::p_8c07c100::run(cpu, mem);
            cpu.r[0] = cpu.r[13].wrapping_add(40);
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                cpu.r[0] = rd(mem, 0x8c0cc8a0); // 0x8c1e9ad4
                let _ = mem.write_u32(cpu.r[13].wrapping_add(40), cpu.r[0]);
            }
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c0cc8a4); // 0x8c01032c
            cpu.pr = 0x8c0cc85a;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    cpu.r[0] = cpu.r[13];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
