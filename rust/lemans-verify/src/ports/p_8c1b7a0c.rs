//! 0x8c1b7a0c — clear a counted run of slots, then insert the tail by key.
//!
//! The count is the low 16 bits of two sign-extended halfwords at
//! `[table+0xa4]` and `[table+0xa6]`, where `table = [[0x8c23dab0]+72]`.
//! A zero count only copies `[object+4]` to `[object+8]`. Otherwise a run of
//! zero stores covers the link words: pairs at stride 40 while more than two
//! remain, then a stride-20 tail. When the count is greater than 1, each node
//! at `base+20`, `base+40`, … is inserted through `0x8c1b79d0` if `base`'s
//! key is greater and through `0x8c1b7998` otherwise. `r10`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn rdh(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[10] = cpu.r[4];
    cpu.r[0] = rd(mem, 0x8c1b7ad4); // 0x8c23dab0
    cpu.r[3] = rd(mem, cpu.r[0]);
    cpu.r[3] = cpu.r[3].wrapping_add(72);
    cpu.r[0] = rd(mem, 0x8c1b7ad8); // 0xa6
    cpu.r[4] = rd(mem, cpu.r[3]);
    cpu.r[3] = rdh(mem, cpu.r[0].wrapping_add(cpu.r[4]));
    cpu.r[0] = rd(mem, 0x8c1b7adc); // 0xa4
    cpu.r[0] = rdh(mem, cpu.r[0].wrapping_add(cpu.r[4]));
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[14] = cpu.r[3] & 0xffff;
    cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl
    let skip_fill = !cpu.t();
    cpu.r[1] = 0; // bf.s delay
    if !skip_fill {
        cpu.r[3] = rd(mem, 0x8c1b7ae0); // -2
        cpu.r[0] = 2;
        cpu.r[5] = cpu.r[14];
        cpu.set_t((cpu.r[14] as i32) > (cpu.r[0] as i32)); // cmp/gt
        let small = !cpu.t();
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]); // bf.s delay
        if !small {
            cpu.r[0] = cpu.r[1];
            cpu.r[4] = cpu.r[1];
            loop {
                cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
                cpu.r[1] = cpu.r[1].wrapping_add(2);
                cpu.set_t((cpu.r[5] as i32) > (cpu.r[1] as i32));
                let cont = cpu.t();
                cpu.r[3] = cpu.r[3].wrapping_add(4);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]);
                cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
                cpu.r[3] = cpu.r[3].wrapping_add(8);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]);
                cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
                cpu.r[3] = cpu.r[3].wrapping_add(4).wrapping_add(cpu.r[0]);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(20), cpu.r[4]);
                cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
                cpu.r[3] = cpu.r[3].wrapping_add(8).wrapping_add(cpu.r[0]);
                let _ = mem.write_u32(cpu.r[3].wrapping_add(20), cpu.r[4]);
                cpu.r[0] = cpu.r[0].wrapping_add(40); // bt.s delay
                if !cont {
                    break;
                }
            }
        }
        // 0x8c1b7aba — shared tail, also the r14 <= 2 entry.
        cpu.r[0] = cpu.r[1].wrapping_shl(2);
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
        cpu.r[0] = cpu.r[0].wrapping_shl(2);
        cpu.r[4] = 0; // bra delay
        loop {
            // 0x8c1b7a7c
            cpu.set_t((cpu.r[14] as i32) > (cpu.r[1] as i32));
            if !cpu.t() {
                break;
            }
            cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
            cpu.r[1] = cpu.r[1].wrapping_add(1);
            cpu.r[3] = cpu.r[3].wrapping_add(4);
            let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]);
            cpu.r[3] = rd(mem, cpu.r[10].wrapping_add(4));
            cpu.r[3] = cpu.r[3].wrapping_add(8);
            let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]);
            cpu.r[0] = cpu.r[0].wrapping_add(20);
        }
    }

    cpu.r[0] = rd(mem, cpu.r[10].wrapping_add(4));
    cpu.r[11] = 1;
    cpu.set_t((cpu.r[14] as i32) > (cpu.r[11] as i32));
    let _ = mem.write_u32(cpu.r[10].wrapping_add(8), cpu.r[0]);
    let skip_insert = !cpu.t();
    cpu.r[13] = rd(mem, cpu.r[10].wrapping_add(4)); // bf.s delay
    if !skip_insert {
        cpu.r[12] = 20;
        loop {
            cpu.r[6] = rd(mem, cpu.r[10].wrapping_add(4));
            cpu.fr[0] = mem.read_f32(cpu.r[13]).unwrap_or(0.0);
            cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[12]);
            cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
            cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1, fr0
            if cpu.t() {
                cpu.r[0] = rd(mem, 0x8c1b7ae4); // 0x8c1b79d0
                cpu.r[5] = cpu.r[13];
                cpu.pr = 0x8c1b7aa2;
                cpu.r[4] = cpu.r[10]; // jsr delay
                crate::registry::p_8c1b79d0::run(cpu, mem);
            } else {
                cpu.r[0] = rd(mem, 0x8c1b7ae8); // 0x8c1b7998
                cpu.r[5] = cpu.r[13];
                cpu.pr = 0x8c1b7aae;
                cpu.r[4] = cpu.r[10]; // jsr delay
                crate::registry::p_8c1b7998::run(cpu, mem);
            }
            cpu.r[11] = cpu.r[11].wrapping_add(1);
            cpu.set_t((cpu.r[14] as i32) > (cpu.r[11] as i32));
            let cont = cpu.t();
            cpu.r[12] = cpu.r[12].wrapping_add(20); // bt.s delay
            if !cont {
                break;
            }
        }
    }

    cpu.pr = rd(mem, cpu.r[15]);
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
