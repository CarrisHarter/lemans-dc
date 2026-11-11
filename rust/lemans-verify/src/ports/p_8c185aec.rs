//! 0x8c185aec — float array pack into r4.
//!
//! r0 = *r15; *r4 = 0x8c1f0418 (pool 0x8c185ba0); then 16 floats are copied:
//! *(r5+0/4/8/12) to r4+4..16, *(r6+0/4/8/12) to r4+20..32, *(r7+0/4/8/12) to
//! r4+36..48, *(r0+0/4/8/12) to r4+52..64. Final regs: r5 = r4+56, r6 = r4+40,
//! r7 = r7_entry+12, r0 = r4 (rts delay slot), r1 = r4+64, r3 = r4+60,
//! fr0 = *(out+12).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let out = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r0
    let g = mem.read_u32(0x8c185ba0).unwrap(); // pool: 0x8c1f0418
    cpu.r[1] = g;
    let _ = mem.write_u32(r4, g); // mov.l r1,@r4

    let mut dst = r4 + 4;
    let mut groups = [cpu.r[5], cpu.r[6], cpu.r[7], out];
    for g in groups.iter_mut() {
        for k in 0..4u32 {
            cpu.fr[0] = f32::from_bits(mem.read_u32(g.wrapping_add(k * 4)).unwrap());
            let _ = mem.write_u32(dst, cpu.fr[0].to_bits());
            dst = dst.wrapping_add(4);
        }
        *g = g.wrapping_add(12); // add #12,rN after the 4th load
    }
    cpu.r[7] = groups[2]; // r7 = r7_entry + 12
    cpu.r[5] = r4.wrapping_add(56); // mov r4,r5; add #56,r5
    cpu.r[6] = r4.wrapping_add(40); // mov r4,r6; add #40,r6
    cpu.r[0] = r4; // rts delay slot: mov r4,r0 (after add #12,r0)
    cpu.r[1] = r4.wrapping_add(64); // mov r4,r1; add #64,r1
    cpu.r[3] = r4.wrapping_add(60); // mov r4,r3; add #60,r3
}
