//! 0x8c231272 — pack one record and one block into a stack struct, then
//! build a packet.
//!
//! The frame is 112 bytes. Words `[r4+0..16)` land at `sp+12`. Floats and
//! words from the pointer in `r5` fill the rest, with `[0x8c2c8648+16]` and
//! `[0x8c2c8648+20]` repeated three times. Slots at `sp+44`, `sp+72` and
//! `sp+100` stay untouched. `0x8c2395c0` is called with `r4 = sp+12`.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn st(mem: &mut Memory, addr: u32, value: u32) {
    let _ = mem.write_u32(addr, value);
}

fn ld_f(mem: &Memory, addr: u32) -> f32 {
    mem.read_f32(addr).unwrap_or(0.0)
}

fn st_f(mem: &mut Memory, addr: u32, value: f32) {
    let _ = mem.write_f32(addr, value);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    st(mem, cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(112);
    cpu.r[0] = cpu.r[15];
    cpu.r[1] = cpu.r[0];
    st(mem, cpu.r[1], cpu.r[5]);
    st(mem, cpu.r[0].wrapping_add(4), cpu.r[6]);
    st(mem, cpu.r[0].wrapping_add(8), cpu.r[7]);
    cpu.r[0] = 4;
    cpu.r[3] = rd(mem, cpu.r[4]);
    cpu.r[6] = rd(mem, 0x8c231340); // 0x8c2c8648
    st(mem, cpu.r[15].wrapping_add(12), cpu.r[3]);
    cpu.r[2] = rd(mem, cpu.r[4].wrapping_add(4));
    st(mem, cpu.r[15].wrapping_add(16), cpu.r[2]);
    cpu.r[3] = rd(mem, cpu.r[4].wrapping_add(8));
    st(mem, cpu.r[15].wrapping_add(20), cpu.r[3]);
    cpu.r[2] = rd(mem, cpu.r[4].wrapping_add(12));
    st(mem, cpu.r[15].wrapping_add(24), cpu.r[2]);

    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 28;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 8;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 32;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 12;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 36;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.r[2] = rd(mem, cpu.r[3].wrapping_add(52));
    st(mem, cpu.r[15].wrapping_add(40), cpu.r[2]);

    cpu.r[4] = cpu.r[6].wrapping_add(16);
    cpu.r[3] = rd(mem, cpu.r[4]);
    cpu.r[5] = cpu.r[6].wrapping_add(20);
    st(mem, cpu.r[15].wrapping_add(48), cpu.r[3]);
    cpu.r[0] = 16;
    cpu.r[3] = rd(mem, cpu.r[5]);
    st(mem, cpu.r[15].wrapping_add(52), cpu.r[3]);
    cpu.r[2] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[2]));
    cpu.r[0] = 56;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 20;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 60;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 24;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 64;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 68;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.r[2] = rd(mem, cpu.r[3].wrapping_add(56));
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[2]);
    cpu.r[0] = 76;
    cpu.r[3] = rd(mem, cpu.r[4]);
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[3]);
    cpu.r[0] = 80;
    cpu.r[2] = rd(mem, cpu.r[5]);
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[2]);

    cpu.r[0] = 28;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 84;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 32;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 88;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 36;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.fr[3] = ld_f(mem, cpu.r[0].wrapping_add(cpu.r[3]));
    cpu.r[0] = 92;
    st_f(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    cpu.r[0] = 96;
    cpu.r[3] = rd(mem, cpu.r[1]);
    cpu.r[2] = rd(mem, cpu.r[3].wrapping_add(60));
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[2]);
    cpu.r[0] = 104;
    cpu.r[3] = rd(mem, cpu.r[4]);
    cpu.r[4] = cpu.r[15];
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[3]);
    cpu.r[0] = 108;
    cpu.r[2] = rd(mem, cpu.r[5]);
    cpu.r[3] = rd(mem, 0x8c231344); // 0x8c2395c0
    st(mem, cpu.r[0].wrapping_add(cpu.r[15]), cpu.r[2]);
    cpu.pr = 0x8c231332;
    cpu.r[4] = cpu.r[4].wrapping_add(12); // jsr delay slot
    crate::registry::p_8c2395c0::run(cpu, mem);

    cpu.r[15] = cpu.r[15].wrapping_add(112);
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
