//! 0x8c14b1f4 — init 116-byte struct at r4; returns r4

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c14b1f4: add #-4,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(-4) as u32;
    // 0x8c14b1f6: mov.l r4,@r15
    mem.write_u32(cpu.r[15], cpu.r[4]).unwrap();
    // 0x8c14b1f8: mov.l @r15,r7
    cpu.r[7] = mem.read_u32(cpu.r[15]).unwrap();
    // 0x8c14b1fa: mov #0,r6
    cpu.r[6] = 0u32;
    // 0x8c14b1fc: mov.l 0x8c14b264,r1
    cpu.r[1] = mem.read_u32(0x8c14b264).unwrap();
    // 0x8c14b1fe: mov r7,r0
    cpu.r[0] = cpu.r[7];
    // 0x8c14b200: add #4,r0
    cpu.r[0] = (cpu.r[0] as i32).wrapping_add(4) as u32;
    // 0x8c14b202: mov.l r1,@r7
    mem.write_u32(cpu.r[7], cpu.r[1]).unwrap();
    // 0x8c14b204: mov r7,r4
    cpu.r[4] = cpu.r[7];
    // 0x8c14b206: mov.l r6,@r0
    mem.write_u32(cpu.r[0], cpu.r[6]).unwrap();
    // 0x8c14b208: mov r7,r1
    cpu.r[1] = cpu.r[7];
    // 0x8c14b20a: mov.l r6,@(4,r0)
    mem.write_u32(cpu.r[0].wrapping_add(4), cpu.r[6]).unwrap();
    // 0x8c14b20c: mov #-1,r5
    cpu.r[5] = (-1i32) as u32;
    // 0x8c14b20e: mov.l r6,@(8,r0)
    mem.write_u32(cpu.r[0].wrapping_add(8), cpu.r[6]).unwrap();
    // 0x8c14b210: mov r7,r3
    cpu.r[3] = cpu.r[7];
    // 0x8c14b212: mov.l r5,@(16,r7)
    mem.write_u32(cpu.r[7].wrapping_add(16), cpu.r[5]).unwrap();
    // 0x8c14b214: add #24,r1
    cpu.r[1] = (cpu.r[1] as i32).wrapping_add(24) as u32;
    // 0x8c14b216: mov.l r6,@(20,r7)
    mem.write_u32(cpu.r[7].wrapping_add(20), cpu.r[6]).unwrap();
    // 0x8c14b218: add #40,r4
    cpu.r[4] = (cpu.r[4] as i32).wrapping_add(40) as u32;
    // 0x8c14b21a: mov.l r6,@r1
    mem.write_u32(cpu.r[1], cpu.r[6]).unwrap();
    // 0x8c14b21c: mov #56,r0
    cpu.r[0] = 56u32;
    // 0x8c14b21e: mov.l r6,@(4,r1)
    mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[6]).unwrap();
    // 0x8c14b220: add #68,r3
    cpu.r[3] = (cpu.r[3] as i32).wrapping_add(68) as u32;
    // 0x8c14b222: mov.l r6,@(8,r1)
    mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[6]).unwrap();
    // 0x8c14b224: mov.l r5,@(36,r7)
    mem.write_u32(cpu.r[7].wrapping_add(36), cpu.r[5]).unwrap();
    // 0x8c14b226: mov #1,r1
    cpu.r[1] = 1u32;
    // 0x8c14b228: mov.l r6,@r4
    mem.write_u32(cpu.r[4], cpu.r[6]).unwrap();
    // 0x8c14b22a: mov.l r6,@(4,r4)
    mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[6]).unwrap();
    // 0x8c14b22c: mov.l r6,@(8,r4)
    mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[6]).unwrap();
    // 0x8c14b22e: mov.l r5,@(52,r7)
    mem.write_u32(cpu.r[7].wrapping_add(52), cpu.r[5]).unwrap();
    // 0x8c14b230: mov.b r6,@(r0,r7)
    mem.write_u8(cpu.r[7].wrapping_add(cpu.r[0]), (cpu.r[6] & 0xff) as u8).unwrap();
    // 0x8c14b232: mov.l r6,@(60,r7)
    mem.write_u32(cpu.r[7].wrapping_add(60), cpu.r[6]).unwrap();
    // 0x8c14b234: mov #64,r0
    cpu.r[0] = 64u32;
    // 0x8c14b236: mov.l r6,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[6]).unwrap();
    // 0x8c14b238: mov.l r6,@r3
    mem.write_u32(cpu.r[3], cpu.r[6]).unwrap();
    // 0x8c14b23a: mov #80,r0
    cpu.r[0] = 80u32;
    // 0x8c14b23c: mov.l r6,@(4,r3)
    mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[6]).unwrap();
    // 0x8c14b23e: mov.l r6,@(8,r3)
    mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[6]).unwrap();
    // 0x8c14b240: mov.l r6,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[6]).unwrap();
    // 0x8c14b242: mov #84,r0
    cpu.r[0] = 84u32;
    // 0x8c14b244: mov.l r6,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[6]).unwrap();
    // 0x8c14b246: mov #88,r0
    cpu.r[0] = 88u32;
    // 0x8c14b248: mov.l r6,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[6]).unwrap();
    // 0x8c14b24a: mov #92,r0
    cpu.r[0] = 92u32;
    // 0x8c14b24c: mov.l r1,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[1]).unwrap();
    // 0x8c14b24e: mov #96,r0
    cpu.r[0] = 96u32;
    // 0x8c14b250: mov.b r1,@(r0,r7)
    mem.write_u8(cpu.r[7].wrapping_add(cpu.r[0]), (cpu.r[1] & 0xff) as u8).unwrap();
    // 0x8c14b252: mov #97,r0
    cpu.r[0] = 97u32;
    // 0x8c14b254: mov.b r6,@(r0,r7)
    mem.write_u8(cpu.r[7].wrapping_add(cpu.r[0]), (cpu.r[6] & 0xff) as u8).unwrap();
    // 0x8c14b256: mov #98,r0
    cpu.r[0] = 98u32;
    // 0x8c14b258: mov.b r1,@(r0,r7)
    mem.write_u8(cpu.r[7].wrapping_add(cpu.r[0]), (cpu.r[1] & 0xff) as u8).unwrap();
    // 0x8c14b25a: mov #100,r0
    cpu.r[0] = 100u32;
    // 0x8c14b25c: mov.l r6,@(r0,r7)
    mem.write_u32(cpu.r[7].wrapping_add(cpu.r[0]), cpu.r[6]).unwrap();
    // 0x8c14b25e: mov r7,r0
    cpu.r[0] = cpu.r[7];
    // 0x8c14b262: add #4,r15
    cpu.r[15] = (cpu.r[15] as i32).wrapping_add(4) as u32;
}
