//! 0x8c051140 — copy a 0x40-byte record from r5 to r4, overwriting dst[0]
//! with a fixed header pointer, then increment the counter at
//! `table + dst[1]*4`. Returns r4.
//!
//! ABI: r4 = dst, r5 = src.
//!
//! Side effects the decompiled C does not show: the C's FPSCR.SZ double
//! branch does not exist in the machine code — the last two dwords are moved
//! with plain single-precision `fmov`, leaving fr0 = src[15]; r5 ends at
//! src+60, r6 = src+56, r7 = dst+56, r0 = dst, r1 = counter+1 and
//! r3 = &counter. Store order is the unrolled order of the original
//! (0,4,32,8,12,16,44,20,24,28,36,40,48,52,56,60,counter), which the verifier
//! compares byte-for-byte. T is untouched (no flag instruction runs).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];

    let header = mem.read_u32(0x8c0511b8).unwrap_or(0); // 0x8c1e8da8
    let table = mem.read_u32(0x8c0511bc).unwrap_or(0); // 0x8c1f6ba8

    let r6 = r5.wrapping_add(56);
    let r7 = r4.wrapping_add(56);
    let r3 = r4.wrapping_add(60);
    let r11 = r4.wrapping_add(20);
    let r12 = r5.wrapping_add(20);

    // Stores in exact original order.
    mem.write_u32(r4, header).unwrap(); // [0]
    let d1 = mem.read_u32(r5.wrapping_add(4)).unwrap();
    mem.write_u32(r4.wrapping_add(4), d1).unwrap(); // [1]
    let d8 = mem.read_u32(r5.wrapping_add(32)).unwrap();
    mem.write_u32(r4.wrapping_add(32), d8).unwrap(); // [8]
    let d2 = mem.read_u32(r5.wrapping_add(8)).unwrap();
    mem.write_u32(r4.wrapping_add(8), d2).unwrap(); // [2]
    let d3 = mem.read_u32(r5.wrapping_add(12)).unwrap();
    mem.write_u32(r4.wrapping_add(12), d3).unwrap(); // [3]
    let d4 = mem.read_u32(r5.wrapping_add(16)).unwrap();
    mem.write_u32(r4.wrapping_add(16), d4).unwrap(); // [4]
    let d11 = mem.read_u32(r5.wrapping_add(44)).unwrap();
    mem.write_u32(r4.wrapping_add(44), d11).unwrap(); // [11]
    let d5 = mem.read_u32(r12).unwrap();
    let d6 = mem.read_u32(r12.wrapping_add(4)).unwrap();
    mem.write_u32(r11, d5).unwrap(); // [5]
    mem.write_u32(r11.wrapping_add(4), d6).unwrap(); // [6]
    let d7 = mem.read_u32(r12.wrapping_add(8)).unwrap();
    mem.write_u32(r11.wrapping_add(8), d7).unwrap(); // [7]
    let d9 = mem.read_u32(r5.wrapping_add(36)).unwrap();
    mem.write_u32(r4.wrapping_add(36), d9).unwrap(); // [9]
    let d10 = mem.read_u32(r5.wrapping_add(40)).unwrap();
    mem.write_u32(r4.wrapping_add(40), d10).unwrap(); // [10]
    let d12 = mem.read_u32(r5.wrapping_add(48)).unwrap();
    mem.write_u32(r4.wrapping_add(48), d12).unwrap(); // [12]
    let d13 = mem.read_u32(r5.wrapping_add(52)).unwrap();
    mem.write_u32(r4.wrapping_add(52), d13).unwrap(); // [13]
    let f14 = mem.read_f32(r6).unwrap();
    mem.write_f32(r7, f14).unwrap(); // [14]
    let f15 = mem.read_f32(r5.wrapping_add(60)).unwrap(); // r5 already +60
    mem.write_f32(r3, f15).unwrap(); // [15]

    // counter: *(table + dst[1]*4) += 1
    let caddr = table.wrapping_add(d1.wrapping_mul(4));
    let c = mem.read_u32(caddr).unwrap();
    let c1 = c.wrapping_add(1);
    mem.write_u32(caddr, c1).unwrap();

    cpu.r[0] = r4;
    cpu.r[1] = c1;
    cpu.r[3] = caddr;
    cpu.r[5] = r5.wrapping_add(60);
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.fr[0] = f15;
    // r11..r14 popped from the stack: net unchanged; T never set.
}
