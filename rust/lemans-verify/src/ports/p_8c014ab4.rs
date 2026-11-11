//! 0x8c014ab4 — double-precision round/truncate of the fr5/fr4 pair.
//!
//! Spills fr5/fr4 at [r15]/[r15+4], classifies via 0x8c012260. Nonzero class:
//! writes the mode flag, fills both words with -1, reloads fr1/fr0. Zero
//! class: extracts the exponent nibble; negative short-circuits double-zero
//! stores to [r14]/[r14+4]; else if exponent >= 52 stores the pair directly,
//! else computes a shift count with round-to-word logic (shad by -5, clamps,
//! neg-path bit fiddling), masks [r15+4] with the shifted -1, then the common
//! tail reloads fr3/fr2 from [r14], sets double FPSCR mode, subtracts, and
//! sets bit7 of [r15+7] when the [r14] double exceeds the zero constant.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[2] = cpu.r[15];
    cpu.r[2] = cpu.r[2].wrapping_add(4);
    cpu.r[3] = mem.read_u32(0x8c014b68).unwrap_or(0); // 0x8c012260
    cpu.r[13] = (-1i32) as u32;
    cpu.r[14] = cpu.r[4];
    let _ = mem.write_f32(cpu.r[15], cpu.fr[5]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]);
    cpu.fr[5] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // jsr delay slot
    cpu.pr = 0x8c014ace;
    crate::registry::p_8c012260::run(cpu, mem);
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0
    cpu.r[4] = cpu.r[0]; // bt.s delay slot: always runs
    if cpu.t() {
        exp_path(cpu, mem);
        return;
    }
    cpu.r[0] = cpu.r[4];
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    cpu.r[0] = cpu.r[4]; // bf.s delay slot: always runs
    if cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c014b6c).unwrap_or(0); // 0x8c23d8b8
        cpu.r[2] = mem.read_u16(0x8c014b58).unwrap_or(0) as i16 as i32 as u32; // 0x4b8
        let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    }
    cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0 (plain bf: no delay slot)
    if cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c014b6c).unwrap_or(0);
        cpu.r[3] = mem.read_u16(0x8c014b5a).unwrap_or(0) as i16 as i32 as u32; // 0x44c
        let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    }
    cpu.r[1] = cpu.r[15];
    cpu.r[3] = cpu.r[15];
    let _ = mem.write_u32(cpu.r[1], cpu.r[13]);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[13]); // bra delay slot
    tail_load(cpu, mem);
}

fn tail_load(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014bbc
    cpu.r[0] = 4;
    cpu.fr[1] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn exp_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014af6
    cpu.r[4] = cpu.r[15];
    cpu.r[0] = mem.read_u16(cpu.r[4].wrapping_add(6)).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[2] = mem.read_u16(0x8c014b5c).unwrap_or(0) as i16 as i32 as u32; // 0x7ff
    cpu.r[4] = cpu.r[0] & 0xffff; // extu.w r0,r4
    cpu.r[3] = mem.read_u16(0x8c014b5e).unwrap_or(0) as i16 as i32 as u32; // 0xfc01
    cpu.r[4] >>= 2; // shlr2
    cpu.r[4] >>= 2; // shlr2
    cpu.r[4] &= cpu.r[2];
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[3]);
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    if cpu.t() {
        wide_path(cpu, mem);
        return;
    }
    cpu.r[0] = 0x8c014b70; // mova
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[14];
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[3] = cpu.fr[2];
    let _ = mem.write_f32(cpu.r[0], cpu.fr[3]);
    cpu.r[0] = cpu.r[0].wrapping_sub(4); // bra delay slot: fmov fr3,@-r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[3]);
    common_tail(cpu, mem);
}

fn wide_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014b1c
    cpu.r[0] = 4;
    cpu.fr[3] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[2] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = cpu.r[14];
    cpu.r[6] = 52;
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[6] as i32)); // cmp/ge r6,r4
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]);
    cpu.r[0] = cpu.r[0].wrapping_sub(4); // bt.s delay slot: fmov fr3,@-r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[3]);
    if cpu.t() {
        common_tail(cpu, mem);
        return;
    }
    shift_path(cpu, mem);
}

fn shift_path(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014b30
    cpu.r[5] = cpu.r[6];
    cpu.r[5] = cpu.r[5].wrapping_sub(cpu.r[4]);
    cpu.r[0] = cpu.r[5];
    cpu.set_t((cpu.r[0] as i32) >= 0); // cmp/pz r0
    if !cpu.t() {
        cpu.r[0] = cpu.r[0].wrapping_add(31);
    }
    cpu.r[1] = (-5i32) as u32;
    cpu.r[0] = ((cpu.r[0] as i32) >> 5) as u32; // shad r1,r0
    cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
    if cpu.t() {
        cpu.r[0] = 0;
        cpu.r[1] = cpu.r[15];
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    }
    cpu.r[0] = cpu.r[5];
    cpu.set_t((cpu.r[0] as i32) >= 0); // cmp/pz r0
    cpu.r[4] = cpu.r[13]; // bf.s delay slot: always runs (r13 = -1)
    if !cpu.t() {
        cpu.r[0] = !cpu.r[0]; // not r0,r0
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.r[0] &= 31;
        cpu.r[0] = !cpu.r[0];
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        mask_store(cpu, mem);
        return;
    }
    cpu.r[0] &= 31; // bra delay slot
    mask_store(cpu, mem);
}

fn mask_store(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014b7e
    cpu.r[3] = cpu.r[15];
    cpu.r[2] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
    // shld r0,r4 (r4 = -1)
    let amount = cpu.r[0] as i32;
    if amount >= 0 {
        cpu.r[4] <<= (amount as u32) & 31;
    } else {
        cpu.r[4] >>= ((-amount) as u32) & 31;
    }
    cpu.r[2] &= cpu.r[4];
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[2]);
    common_tail(cpu, mem);
}

fn common_tail(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014b88
    cpu.r[0] = cpu.r[14];
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[3] = cpu.fpscr; // sts fpscr,r3
    cpu.r[2] = mem.read_u32(0x8c014bcc).unwrap_or(0); // 0x80000
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = 4;
    cpu.r[3] |= cpu.r[2];
    cpu.fr[1] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
    cpu.r[0] = 4;
    cpu.fpscr = cpu.r[3]; // lds r3,fpscr
    cpu.fr[0] = cpu.fr[0] - cpu.fr[2]; // fsub fr2,fr0
    let _ = mem.write_f32(cpu.r[15], cpu.fr[1]);
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[0]);
    cpu.r[0] = 0x8c014bd0; // mova
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[14];
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.fr[3] = cpu.fr[2];
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[2] > cpu.fr[0]); // fcmp/gt fr0,fr2
    if !cpu.t() {
        tail_load(cpu, mem);
        return;
    }
    cpu.r[3] = cpu.r[15];
    cpu.r[0] = mem.read_u8(cpu.r[3].wrapping_add(7)).unwrap_or(0) as u32;
    cpu.r[0] |= 128;
    let _ = mem.write_u8(cpu.r[3].wrapping_add(7), cpu.r[0] as u8);
    tail_load(cpu, mem);
}
