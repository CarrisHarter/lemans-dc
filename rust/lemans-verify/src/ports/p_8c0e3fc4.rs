//! 0x8c0e3fc4 — copy the 8-byte header at r6 to entry `r5*16` of the table at
//! 0x8c27a958, set the flag byte at 0x8c27a9d8 to 1, then walk all 8 entries:
//! for each entry whose flag byte is non-zero, multiply the running fr2 (init
//! 1.0) by the ROM constant 2.0f at 0x8c0e4024 and store at entry+8, then by
//! the entry's float at entry+4 and store at entry+12.
//!
//! ```text
//! push r14; r3=[0x8c0e401c]=0x8c27a958; r5 = r5*16 + r3
//! [r5]=[r6]; [r5+4]=[r6+4]; fldi1 fr2; r4=[0x8c0e4020]=0x8c27a9d8; r1=0; [r4]=1.b
//! r14=0; r7=r3
//! loop: r4=8; r0=r7; r0=extu.b [r0+r14]; tst r0,r0; bt skip
//!   r6=r3+4; fr1=[0x8c0e4024]=2.0f; fr0=[r6]; r5=r3+8; fr2*=fr1; [r5]=fr2
//!   r0=r3+12; fr2*=fr0; [r0]=fr2
//! skip: r1++; T = (r4 > r1); r3+=16; r14+=16 (bt.s delay, always); bt.s loop
//! rts; pop r14
//! ```
//! Hidden side effects the C does not show: r3 ends at base+128, r14 is restored
//! by the rts delay slot, r4 = 8, r7 = base, r0/r6 depend on the last entry's
//! path, T = 0 on exit, fr2 accumulates across entries.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    let base = mem.read_u32(0x8C0E401C).unwrap(); // 0x8c27a958
    cpu.r[3] = base;
    cpu.r[5] = cpu.r[5].wrapping_shl(4); // shll2 r5, twice
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap();
    cpu.r[5] = cpu.r[5].wrapping_add(base); // add r3,r5
    cpu.r[0] = mem.read_u32(cpu.r[6] + 4).unwrap();
    cpu.fr[2] = 1.0; // fldi1 fr2
    cpu.r[4] = mem.read_u32(0x8C0E4020).unwrap(); // 0x8c27a9d8
    let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[0]);
    cpu.r[1] = 0;
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[4], cpu.r[0] as u8); // mov.b r0,@r4
    cpu.r[14] = cpu.r[1]; // mov r1,r14
    cpu.r[7] = cpu.r[3]; // mov r3,r7
    loop {
        cpu.r[4] = 8; // mov #8,r4
        cpu.r[0] = cpu.r[7];
        let b = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[14])).unwrap();
        cpu.r[0] = (b as u32 & 0xFF) & 0xFF; // mov.b + extu.b x2
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            // bt not taken: multiply path
            cpu.r[6] = cpu.r[3];
            cpu.fr[1] = mem.read_f32(0x8C0E4024).unwrap(); // 2.0f
            cpu.r[6] = cpu.r[6].wrapping_add(4);
            cpu.r[5] = cpu.r[3];
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // [entry+4]
            cpu.r[0] = cpu.r[3];
            cpu.fr[2] = cpu.fr[2] * cpu.fr[1]; // fmul fr1,fr2
            cpu.r[5] = cpu.r[5].wrapping_add(8);
            let _ = mem.write_f32(cpu.r[5], cpu.fr[2]); // [entry+8]
            cpu.r[0] = cpu.r[0].wrapping_add(12);
            cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // fmul fr0,fr2
            let _ = mem.write_f32(cpu.r[0], cpu.fr[2]); // [entry+12]
        }
        cpu.r[1] = cpu.r[1].wrapping_add(1);
        cpu.set_t((cpu.r[4] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r4
        cpu.r[3] = cpu.r[3].wrapping_add(16);
        cpu.r[14] = cpu.r[14].wrapping_add(16); // bt.s delay slot: always runs
        if !cpu.t() {
            break;
        }
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); // rts delay slot: pop r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
