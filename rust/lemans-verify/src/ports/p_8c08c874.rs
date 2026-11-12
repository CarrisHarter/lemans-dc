//! 0x8c08c874 - copy a 208-byte record into a ring, or refuse.
//!
//! The byte at `r4+68` forces the copy onto the buffer at `[r4+56]`.
//! When that byte is clear, the copy is refused unless `[r4+60] > [r4+64]`
//! (signed). The ring walk then stores the record at the first slot whose
//! float at +12 is not greater than the pooled 0.001, and wraps the cursor
//! at `[r4+0x80]` back to the base when it lands on `base + 0xd0 * index`.

use sh4_oracle::{Cpu, Memory};

fn copy26(cpu: &mut Cpu, mem: &mut Memory, dest: usize, count: usize) {
    cpu.r[count] = 26;
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        cpu.r[count] = cpu.r[count].wrapping_sub(1);
        cpu.set_t(cpu.r[count] == 0);
        let _ = mem.write_u32(cpu.r[dest], cpu.r[0]);
        cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[5] = cpu.r[5].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[dest].wrapping_add(4), cpu.r[0]);
        let more = !cpu.t();
        cpu.r[dest] = cpu.r[dest].wrapping_add(8);
        if !more {
            break;
        }
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[0] = cpu.r[4].wrapping_add(68);
    cpu.r[6] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[6];
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[3] = 1; // bf.s delay
    if cpu.r[6] == 0 {
        cpu.r[0] = cpu.r[4].wrapping_add(64);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(60)).unwrap_or(0);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32));
        if !cpu.t() {
            cpu.r[3] = 0;
        }
    }
    cpu.r[0] = cpu.r[3] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = cpu.r[6];
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[6] != 0 {
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap_or(0);
            copy26(cpu, mem, 3, 1);
            cpu.r[0] = 68;
            cpu.r[1] = 0;
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 0);
            cpu.r[0] = 1;
        } else {
            cpu.r[7] = mem.read_u32(cpu.r[4].wrapping_add(60)).unwrap_or(0);
            cpu.r[6] = mem.read_u32(0x8c08c96c).unwrap_or(0xd0);
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap_or(0);
            cpu.macl = cpu.r[6].wrapping_mul(cpu.r[7]);
            cpu.r[7] = cpu.macl;
            cpu.r[0] = mem.read_u32(0x8c08c970).unwrap_or(0x80);
            cpu.r[1] = cpu.r[3].wrapping_add(cpu.r[7]);
            cpu.r[13] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
            cpu.set_t(cpu.r[1] > cpu.r[13]);
            cpu.r[14] = cpu.r[13];
            if cpu.t() {
                loop {
                    cpu.r[0] = 0x8c08c974;
                    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                    cpu.r[0] = cpu.r[14].wrapping_add(12);
                    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
                    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
                    if !cpu.t() {
                        cpu.r[6] = cpu.r[14];
                        copy26(cpu, mem, 6, 3);
                        cpu.r[0] = mem.read_u32(0x8c08c96c).unwrap_or(0xd0);
                        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]);
                        cpu.r[0] = mem.read_u32(0x8c08c970).unwrap_or(0x80);
                        let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[14]);
                        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
                        cpu.set_t(cpu.r[3] == cpu.r[1]);
                        if cpu.t() {
                            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap_or(0);
                            let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]);
                        }
                        cpu.r[0] = 1;
                        break;
                    }
                    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[6]);
                    cpu.set_t(cpu.r[1] > cpu.r[14]);
                    if !cpu.t() {
                        // Fall into the base scan below.
                        cpu.r[0] = 0; // overwritten if that scan returns
                        finish_base(cpu, mem);
                        break;
                    }
                }
            } else {
                finish_base(cpu, mem);
            }
        }
    }
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn finish_base(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[13] > cpu.r[3]);
    if !cpu.t() {
        cpu.r[0] = 0;
        return;
    }
    cpu.r[0] = 0x8c08c974;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.r[0] = mem.read_u32(0x8c08c96c).unwrap_or(0xd0);
    loop {
        cpu.r[1] = cpu.r[3].wrapping_add(12);
        cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
        cpu.set_t(cpu.fr[1] > cpu.fr[0]);
        if !cpu.t() {
            cpu.r[6] = cpu.r[3];
            copy26(cpu, mem, 6, 1);
            cpu.r[1] = mem.read_u32(0x8c08c96c).unwrap_or(0xd0);
            cpu.r[0] = mem.read_u32(0x8c08c970).unwrap_or(0x80);
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3]);
            cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(60)).unwrap_or(0);
            cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(56)).unwrap_or(0);
            cpu.macl = cpu.r[1].wrapping_mul(cpu.r[5]);
            cpu.r[5] = cpu.macl;
            cpu.r[3] = cpu.r[6].wrapping_add(cpu.r[5]);
            cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
            cpu.set_t(cpu.r[1] == cpu.r[3]);
            if cpu.t() {
                let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[6]);
            }
            cpu.r[0] = 1;
            return;
        }
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
        cpu.set_t(cpu.r[13] > cpu.r[3]);
        if !cpu.t() {
            cpu.r[0] = 0;
            return;
        }
    }
}
