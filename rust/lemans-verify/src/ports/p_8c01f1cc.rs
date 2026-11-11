//! 0x8c01f1cc — copy 0x104-byte records from [r4..r5) to r6; return r6.
//!
//! Per record, when r6 != 0: 32 dwords (128 bytes) through the r14/r7
//! post-increment loop, then one `fmov.s` of the dword at +0x100 from the
//! source record to the destination. r13/r0 are loaded from the literal
//! pool (0x100 / 0x104); r13 is popped back at the end, r0 finally holds r6
//! (rts delay slot), and r14/r15 are restored by the epilogue pops.
//!
//! Leftovers the C does not show: r1 = r6_iter + 0x100, r3 = 0 and
//! r7 = dst + 0x100 after any copy, fr0 = the last copied float (bit-exact),
//! r4/r6 both advance by 0x104 per record (r6 even when its copy was
//! skipped), and T ends set (loop exits only through cmp/eq equal).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let end = cpu.r[5];
    const STRIDE: u32 = 0x104; // [0x8c01f210]
    const TAIL: u32 = 0x100; //   [0x8c01f20c]
    cpu.set_t(cpu.r[4] == end); // 0x8c01f1d0 cmp/eq r5,r4
    if cpu.r[4] != end {
        loop {
            if cpu.r[6] != 0 {
                let mut src = cpu.r[4];
                let mut dst = cpu.r[6];
                let mut count = 32u32; // 0x8c01f1e0 mov #32,r3
                while count != 0 {
                    let v = mem.read_u32(src).unwrap_or(0);
                    src = src.wrapping_add(4);
                    count = count.wrapping_sub(1); // 0x8c01f1e4 dt r3
                    let _ = mem.write_u32(dst, v);
                    let v = mem.read_u32(src).unwrap_or(0);
                    src = src.wrapping_add(4);
                    let _ = mem.write_u32(dst.wrapping_add(4), v);
                    dst = dst.wrapping_add(8); // bf.s delay slot 0x8c01f1ee
                }
                cpu.r[7] = dst;
                cpu.r[3] = 0;
                // 0x8c01f1f0..0x8c01f1fa: fmov.s @(r4+0x100),fr0 ; fmov.s fr0,@(r6+0x100)
                let f = mem.read_u32(cpu.r[4].wrapping_add(TAIL)).unwrap_or(0);
                cpu.fr[0] = f32::from_bits(f);
                cpu.r[1] = cpu.r[6].wrapping_add(TAIL);
                let _ = mem.write_u32(cpu.r[1], f);
            }
            cpu.r[4] = cpu.r[4].wrapping_add(STRIDE); // 0x8c01f1fc
            cpu.set_t(cpu.r[4] == end); // 0x8c01f1fe
            cpu.r[6] = cpu.r[6].wrapping_add(STRIDE); // bf.s delay slot 0x8c01f202
            if cpu.r[4] == end {
                break; // bf.s not taken
            }
        }
    }
    cpu.r[0] = cpu.r[6]; // 0x8c01f206 mov r6,r0
}
