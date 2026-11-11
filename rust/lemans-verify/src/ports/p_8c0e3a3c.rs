//! 0x8c0e3a3c — copy a 4x4 transform: the last four words of the struct at r6
//! (r6+28..r6+40) go to the global at 0x8c27acf8, and the floats at r6+4, r6+8,
//! r6+12 plus a hard 0.0 (fldi0 fr0) go to the global at 0x8c27ad28.
//! Single path, no branches.
//!
//! ```text
//! add #-16,r15
//! r5 = r6+28; r4=[r5]; r3=[r5+4]; r1=r6+4; r0=r6+8; r7=[0x8c0e3a98]=0x8c27acf8
//! [r7]=r4; [r7+4]=r3; r4=[r5+8]; r6=r6+12; fldi0 fr0; r5=[r5+12]
//! [r7+8]=r4; r4=[0x8c0e3a9c]=0x8c27ad28; [r7+12]=r5
//! spill [r6+4],[r6+8],[r6+12],0.0 to r15+0,4,8,12; reload; store to r4..r4+12
//! rts; add #16,r15
//! ```
//! Hidden side effects the C does not show: r6 ends at param+12, r5 = the loaded
//! word [r6+40], r4 = 0x8c27ad28, r7 = 0x8c27acf8, r2 = r15+12, r3 = r15-16,
//! r0/r1 = reloaded stack words, fr0 = 0.0, fr1 = float at r6+12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15
    let s = cpu.r[6];
    cpu.r[5] = s.wrapping_add(28);
    cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap(); // [r6+28]
    cpu.r[1] = s.wrapping_add(4);
    cpu.r[3] = mem.read_u32(cpu.r[5] + 4).unwrap(); // [r6+32]
    cpu.r[0] = s.wrapping_add(8);
    cpu.r[7] = mem.read_u32(0x8C0E3A98).unwrap(); // 0x8c27acf8
    cpu.r[2] = 0;
    let _ = mem.write_u32(cpu.r[7], cpu.r[4]);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2
    let _ = mem.write_u32(cpu.r[7] + 4, cpu.r[3]);
    cpu.r[4] = mem.read_u32(cpu.r[5] + 8).unwrap(); // [r6+36]
    cpu.r[6] = s.wrapping_add(12); // add #12,r6
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[5] = mem.read_u32(cpu.r[5] + 12).unwrap(); // [r6+40]
    cpu.r[3] = cpu.r[15]; // mov r15,r3
    let _ = mem.write_u32(cpu.r[7] + 8, cpu.r[4]);
    cpu.r[4] = mem.read_u32(0x8C0E3A9C).unwrap(); // 0x8c27ad28
    let _ = mem.write_u32(cpu.r[7] + 12, cpu.r[5]);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr1 = [r6+4]
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // fmov fr1,@r2 = [r15+0]
    cpu.r[2] = 4;
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap(); // [r6+8]
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // [r15+4]
    cpu.r[2] = 8;
    cpu.fr[1] = mem.read_f32(cpu.r[6]).unwrap(); // [r6+12]
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[1]); // [r15+8]
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // [r15+12] = 0.0
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[3] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[3] + 8).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[3] + 12).unwrap();
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(16); // rts delay slot
}
