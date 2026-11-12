//! 0x8c1352a8 - construct a 48-byte record.
//!
//! Writes the vtable at `+0` three times (0x8c1eb6a0, 0x8c1eeef0, then
//! 0x8c1eeed8, which is the one that survives), runs 0x8c1261d4 on `+28`, zeroes
//! `+16`, `+20` and the four halfwords at `+40..+46`, copies the three words from
//! 0x8c207fb0 into `+4/+8/+12`, stores the record pointer at `+24`, and finally
//! copies the local `{0, 0, 0, 24}` halfword block over `+40..+46`. Returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(24); // add #-24,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[0] = mem.read_u32(0x8c135340).unwrap_or(0x8c1eb6a0); // mov.l 0x8c135340,r0
    cpu.r[1] = mem.read_u32(0x8c135344).unwrap_or(0x8c1eeef0); // mov.l 0x8c135344,r1
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]); // mov.l r0,@r3
    cpu.r[0] = mem.read_u32(0x8c135348).unwrap_or(0x8c1261d4); // mov.l 0x8c135348,r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[4] = cpu.r[4].wrapping_add(28); // jsr delay: add #28,r4
    cpu.pr = 0x8c1352c2;
    crate::registry::p_8c1261d4::run(cpu, mem);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[7] = cpu.r[15]; // mov r15,r7
    cpu.r[1] = mem.read_u32(0x8c13534c).unwrap_or(0x8c1eeed8); // mov.l 0x8c13534c,r1
    cpu.r[0] = 40; // mov #40,r0
    cpu.r[7] = cpu.r[7].wrapping_add(12); // add #12,r7
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[1] = 0; // mov #0,r1
    cpu.r[6] = mem.read_u32(0x8c135350).unwrap_or(0x8c207fb0); // mov.l 0x8c135350,r6
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u16); // mov.w r1,@(r0,r3)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = 42; // mov #42,r0
    cpu.r[3] = cpu.r[15]; // mov r15,r3
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u16); // mov.w r1,@(r0,r4)
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = 44; // mov #44,r0
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u16); // mov.w r1,@(r0,r4)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = 46; // mov #46,r0
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u16); // mov.w r1,@(r0,r4)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r14
    cpu.r[0] = 20; // mov #20,r0
    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap_or(0); // mov.l @r6,r5
    cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r6),r4
    let _ = mem.write_u32(cpu.r[7], cpu.r[5]); // mov.l r5,@r7
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[4]); // mov.l r4,@(4,r7)
    cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r6),r4
    let _ = mem.write_u32(cpu.r[7].wrapping_add(8), cpu.r[4]); // mov.l r4,@(8,r7)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]); // mov.l r1,@(16,r14)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[1] as u8); // mov.b r1,@(r0,r4)
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r5
    cpu.r[0] = cpu.r[1]; // mov r1,r0
    cpu.r[4] = mem.read_u32(cpu.r[7]).unwrap_or(0); // mov.l @r7,r4
    cpu.r[5] = cpu.r[5].wrapping_add(4); // add #4,r5
    cpu.r[1] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r7),r1
    let _ = mem.write_u32(cpu.r[5], cpu.r[4]); // mov.l r4,@r5
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[1]); // mov.l r1,@(4,r5)
    cpu.r[1] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r7),r1
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r5)
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    let _ = mem.write_u32(cpu.r[1].wrapping_add(24), cpu.r[1]); // mov.l r1,@(24,r1)
    let _ = mem.write_u16(cpu.r[15].wrapping_add(4), cpu.r[0] as u16); // mov.w r0,@(4,r15)
    let _ = mem.write_u16(cpu.r[15].wrapping_add(6), cpu.r[0] as u16); // mov.w r0,@(6,r15)
    let _ = mem.write_u16(cpu.r[15].wrapping_add(8), cpu.r[0] as u16); // mov.w r0,@(8,r15)
    cpu.r[0] = 24; // mov #24,r0
    let _ = mem.write_u16(cpu.r[15].wrapping_add(10), cpu.r[0] as u16); // mov.w r0,@(10,r15)
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.r[0] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r3,r0
    cpu.r[1] = cpu.r[1].wrapping_add(40); // add #40,r1
    let _ = mem.write_u16(cpu.r[1], cpu.r[0] as u16); // mov.w r0,@r1
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(2)).unwrap_or(0) as i16 as i32 as u32; // mov.w @(2,r3),r0
    let _ = mem.write_u16(cpu.r[1].wrapping_add(2), cpu.r[0] as u16); // mov.w r0,@(2,r1)
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32; // mov.w @(4,r3),r0
    let _ = mem.write_u16(cpu.r[1].wrapping_add(4), cpu.r[0] as u16); // mov.w r0,@(4,r1)
    cpu.r[0] = mem.read_u16(cpu.r[3].wrapping_add(6)).unwrap_or(0) as i16 as i32 as u32; // mov.w @(6,r3),r0
    let _ = mem.write_u16(cpu.r[1].wrapping_add(6), cpu.r[0] as u16); // mov.w r0,@(6,r1)
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(24); // add #24,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
