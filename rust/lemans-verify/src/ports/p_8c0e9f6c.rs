//! 0x8c0e9f6c — same 4-float reduction as 0x8c0e9ef0, different literals.
//!
//! ```text
//! CFG (identical shape to 0x8c0e9ef0):
//!   push r14; r14=[r15+4]; fr2=[0x8c0e9fe0]; T=(r5>0); r6=0;
//!   fr0=[0x8c0e9fe4]; fr3=fr2; bf.s EPI (delay: fr1=fr0)
//!   PRE (0x8c0e9f82, runs ONCE): r7=r6; r3=r4+16; r4+=20
//!   LOOP (0x8c0e9f8a, the bt.s back-edge target): r1=r3+r7; r0=r4+r7
//!     fr4=[r1]; T=(fr4>fr2); delay fr5=[r0]; if !T: fr2=fr4
//!     T=(fr5>fr3); if !T: fr3=fr5
//!     T=(fr4>fr0); if !T: fr4=fr0
//!     T=(fr5>fr1); delay fr0=fr4 (BOTH paths); if !T: fr5=fr1
//!     r6+=1; fr1=fr5; T=(r5>r6); delay r7+=24; if T goto LOOP
//!   EPI (0x8c0e9fca): [r14]=fr2; [r14+4]=fr3; [r14+8]=fr0; [r14+12]=fr1;
//!     rts; delay pop r14
//! ```
//! Only the mova literals differ from 0x8c0e9ef0 (0x8c0e9fe0 / 0x8c0e9fe4).
//! Hidden side effects are the same: the pre-header fixes r3=base+16 and
//! r4=base+20 (the back edge skips it), r7 += 24 per iteration, r0/r1 =
//! out+8/out+4 at exit, fr4/fr5 leftovers, T=0 on exit, and the r5<=0 skip
//! path leaves r3/r4/r7 at their entry values.

use sh4_oracle::{Cpu, Memory};

const DAT_A: u32 = 0x8C0E9FE0; // fr2/fr3 initial value
const DAT_B: u32 = 0x8C0E9FE4; // fr0/fr1 initial value

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[14] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap(); // mov.l @(4,r15),r14

    cpu.fr[2] = mem.read_f32(DAT_A).unwrap(); // mova 0x8c0e9fe0,r0 ; fmov @r0,fr2
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    cpu.r[6] = 0; // mov #0,r6
    cpu.fr[0] = mem.read_f32(DAT_B).unwrap(); // mova 0x8c0e9fe4,r0 ; fmov @r0,fr0
    cpu.fr[3] = cpu.fr[2]; // fmov fr2,fr3
    // bf.s 0x8c0e9fca; delay slot runs on both paths:
    cpu.fr[1] = cpu.fr[0]; // fmov fr0,fr1
    if (cpu.r[5] as i32) > 0 {
        // PRE, 0x8c0e9f82 (executed once; the bt.s back edge targets 0x8c0e9f8a)
        cpu.r[7] = cpu.r[6]; // mov r6,r7
        cpu.r[3] = cpu.r[4].wrapping_add(16); // mov r4,r3 ; add #16,r3
        cpu.r[4] = cpu.r[4].wrapping_add(20); // add #20,r4
        loop {
            // LOOP, 0x8c0e9f8a
            cpu.r[1] = cpu.r[3].wrapping_add(cpu.r[7]); // mov r3,r1 ; add r7,r1
            cpu.r[0] = cpu.r[4].wrapping_add(cpu.r[7]); // mov r4,r0 ; add r7,r0
            cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr4
            cpu.set_t(cpu.fr[4] > cpu.fr[2]); // fcmp/gt fr2,fr4
            cpu.fr[5] = mem.read_f32(cpu.r[0]).unwrap(); // delay slot of bf.s: fmov @r0,fr5
            if !cpu.t() {
                cpu.fr[2] = cpu.fr[4]; // fmov fr4,fr2 (0x8c0e9f9e)
            }
            // 0x8c0e9fa0
            cpu.set_t(cpu.fr[5] > cpu.fr[3]); // fcmp/gt fr3,fr5
            if !cpu.t() {
                cpu.fr[3] = cpu.fr[5]; // fmov fr5,fr3 (0x8c0e9fa8)
            }
            // 0x8c0e9faa
            cpu.set_t(cpu.fr[4] > cpu.fr[0]); // fcmp/gt fr0,fr4
            if !cpu.t() {
                cpu.fr[4] = cpu.fr[0]; // fmov fr0,fr4 (0x8c0e9fb2)
            }
            // 0x8c0e9fb4
            cpu.set_t(cpu.fr[5] > cpu.fr[1]); // fcmp/gt fr1,fr5
            cpu.fr[0] = cpu.fr[4]; // delay slot of bf.s: fmov fr4,fr0 (BOTH paths)
            if !cpu.t() {
                cpu.fr[5] = cpu.fr[1]; // fmov fr1,fr5 (0x8c0e9fbe)
            }
            // 0x8c0e9fc0
            cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
            cpu.fr[1] = cpu.fr[5]; // fmov fr5,fr1
            cpu.set_t((cpu.r[5] as i32) > (cpu.r[6] as i32)); // cmp/gt r6,r5
            cpu.r[7] = cpu.r[7].wrapping_add(24); // delay slot of bt.s: add #24,r7
            if !cpu.t() {
                break;
            }
        }
    }
    // EPI, 0x8c0e9fca
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    let _ = mem.write_f32(cpu.r[14], cpu.fr[2]); // fmov fr2,@r14
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[3]); // fmov fr3,@r1
    cpu.r[0] = cpu.r[0].wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0
    cpu.r[14] = cpu.r[14].wrapping_add(12); // add #12,r14
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]); // fmov fr1,@r14
    // rts; delay slot:
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
