//! 0x8c21bf1e - zero 30 words, then stamp a fixed byte pattern.
//!
//! 0x8c21c3f4(r4, 0, 30) clears the words. Bytes +20..+25 become
//! 40, 110, 111, 110, 101, 41. Bytes +26..+50 become 32. Byte +51 is 0.
//! Returns 0. T is the loop's final `cmp/hs` (r4 caught up with r14+51).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[6] = 30; // mov #30,r6
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[5] = 0; // bsr delay: mov #0,r5
    cpu.pr = 0x8c21bf2a;
    crate::registry::p_8c21c3f4::run(cpu, mem);
    cpu.r[2] = cpu.r[14]; // mov r14,r2
    cpu.r[4] = 110; // mov #110,r4
    cpu.r[0] = 21; // mov #21,r0
    cpu.r[2] = cpu.r[2].wrapping_add(20); // add #20,r2
    cpu.r[3] = 40; // mov #40,r3
    let _ = mem.write_u8(cpu.r[2], cpu.r[3] as u8); // mov.b r3,@r2
    cpu.r[3] = 111; // mov #111,r3
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[4] as u8); // mov.b r4,@(r0,r14)
    cpu.r[0] = 22; // mov #22,r0
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3] as u8); // mov.b r3,@(r0,r14)
    cpu.r[0] = 23; // mov #23,r0
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[4] as u8); // mov.b r4,@(r0,r14)
    cpu.r[0] = 24; // mov #24,r0
    cpu.r[3] = 101; // mov #101,r3
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3] as u8); // mov.b r3,@(r0,r14)
    cpu.r[0] = 25; // mov #25,r0
    cpu.r[2] = 41; // mov #41,r2
    cpu.r[4] = cpu.r[4].wrapping_add(26); // add #26,r4
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[2] as u8); // mov.b r2,@(r0,r14)
    cpu.r[5] = 32; // mov #32,r5
    loop {
        // bf target is 0x8c21bf54, so r3 is reloaded from r14 every trip.
        cpu.r[3] = cpu.r[14]; // mov r14,r3
        let _ = mem.write_u8(cpu.r[4], cpu.r[5] as u8); // mov.b r5,@r4
        cpu.r[3] = cpu.r[3].wrapping_add(51); // add #51,r3
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
        cpu.set_t(cpu.r[4] >= cpu.r[3]); // cmp/hs r3,r4
        if cpu.t() {
            break;
        }
    }
    cpu.r[0] = 51; // mov #51,r0
    cpu.r[3] = 0; // mov #0,r3
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[14]), cpu.r[3] as u8); // mov.b r3,@(r0,r14)
    cpu.r[0] = cpu.r[3]; // mov r3,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
