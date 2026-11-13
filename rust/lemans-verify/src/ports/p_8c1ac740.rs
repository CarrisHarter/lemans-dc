//! 0x8c1ac740 - install a mode pointer, run the blend, then retune it.
//!
//! Stores 0x8c1e5e00 at +4, the caller's r5 at +16 and r6 at +20, then calls
//! 0x8c1ac904 with those two values swapped into r4/r5. Afterwards +24 and
//! +28 are 0, +32 is 640 and +36 is 480. r6 selects a follow-up store:
//! 1 writes 1.0 at 0x8c211810 and puts 0x8c1e5e00 back at +4; 2 and 4 pick
//! a pointer from r5 and write 1.2 or 1.5 at 0x8c211810.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[4] = cpu.r[5];
    cpu.r[0] = mem.read_u32(0x8c1ac7f4).unwrap_or(0x8c1e5e00);
    cpu.r[5] = cpu.r[6];
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[4]);
    cpu.r[0] = mem.read_u32(0x8c1ac7f8).unwrap_or(0x8c1ac904);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[6]); // jsr delay
    cpu.pr = 0x8c1ac756;
    crate::registry::p_8c1ac904::run(cpu, mem);
    cpu.r[1] = cpu.r[14].wrapping_add(24);
    cpu.fr[0] = 0.0;
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(28);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[3] = cpu.r[14].wrapping_add(32);
    cpu.fr[0] = mem.read_f32(0x8c1ac7fc).unwrap_or(640.0);
    cpu.r[1] = cpu.r[14].wrapping_add(36);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.fr[0] = mem.read_f32(0x8c1ac800).unwrap_or(480.0);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(20)).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 1);
    if cpu.r[0] == 1 {
        cpu.r[0] = mem.read_u32(0x8c1ac804).unwrap_or(0x8c211810);
        cpu.fr[0] = 1.0;
        let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
        cpu.r[0] = mem.read_u32(0x8c1ac7f4).unwrap_or(0x8c1e5e00);
        let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
    } else {
        cpu.set_t(cpu.r[0] == 2);
        if cpu.r[0] == 2 {
            cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 0);
            if cpu.r[0] == 0 {
                cpu.r[0] = mem.read_u32(0x8c1ac808).unwrap_or(0x8c1e5ee8);
                let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
            } else {
                cpu.set_t(cpu.r[0] == 1);
                if cpu.r[0] == 1 {
                    cpu.r[0] = mem.read_u32(0x8c1ac80c).unwrap_or(0x8c1e5fd0);
                    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
                }
            }
            cpu.fr[0] = mem.read_f32(0x8c1ac810).unwrap_or(1.2);
            cpu.r[0] = mem.read_u32(0x8c1ac804).unwrap_or(0x8c211810);
            let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
        } else {
            cpu.set_t(cpu.r[0] == 4);
            if cpu.r[0] == 4 {
                cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(16)).unwrap_or(0);
                cpu.set_t(cpu.r[0] == 0);
                if cpu.r[0] == 0 {
                    cpu.r[0] = mem.read_u32(0x8c1ac814).unwrap_or(0x8c1e60b8);
                    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
                } else {
                    cpu.set_t(cpu.r[0] == 1);
                    if cpu.r[0] == 1 {
                        cpu.r[0] = mem.read_u32(0x8c1ac818).unwrap_or(0x8c1e61a0);
                        let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
                    } else {
                        cpu.set_t(cpu.r[0] == 2);
                        if cpu.r[0] == 2 {
                            cpu.r[0] = mem.read_u32(0x8c1ac81c).unwrap_or(0x8c1e6288);
                            let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
                        } else {
                            cpu.set_t(cpu.r[0] == 3);
                            if cpu.r[0] == 3 {
                                cpu.r[0] = mem.read_u32(0x8c1ac820).unwrap_or(0x8c1e6370);
                                let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
                            }
                        }
                    }
                }
                cpu.fr[0] = mem.read_f32(0x8c1ac824).unwrap_or(1.5);
                cpu.r[0] = mem.read_u32(0x8c1ac804).unwrap_or(0x8c211810);
                let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
            }
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
