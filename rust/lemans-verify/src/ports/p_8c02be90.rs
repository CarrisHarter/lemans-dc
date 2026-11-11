//! 0x8c02be90 — state-machine step: advance a per-object counter.
//!
//! Hidden side effects the C does not show:
//! - r0 ends as the zero-extended flag byte on the early-exit path
//! - r5 = 0x3b8 (counter base) and r3 = the counter value are left behind
//! - on the table path r1 = [r4+4] then is reloaded with counter+delta, and
//!   r0 ends as the table entry [ [r4+4]+0x274 ] on the no-store exit, or
//!   0x3b8 on the store exit
//! - the counter increment is the `rts` delay slot on the -1 path (+2)
//! - T keeps the last cmp result (1 on byte==1 and store paths, else the
//!   cmp/gt table>counter result)

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let flag_base = mem.read_u32(0x8C02BEDC).unwrap(); // 0x3b4
    let b = mem.read_u8(flag_base.wrapping_add(r4)).unwrap();
    cpu.r[0] = (b as u32) & 0xFF; // mov.b (sext) + extu.b + extu.b
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.t() {
        return; // bt -> rts (delay slot nop)
    }
    cpu.r[5] = mem.read_u32(0x8C02BEE0).unwrap(); // 0x3b8
    cpu.r[0] = cpu.r[5];
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
    cpu.r[0] = cpu.r[3];
    cpu.set_t(cpu.r[0] == 0xFFFF_FFFF); // cmp/eq #-1,r0
    if cpu.t() {
        // counter == -1: store -1, flag 1, counter += 2 in the rts delay slot
        let base = mem.read_u32(0x8C02BEE4).unwrap(); // 0x3bc
        cpu.r[0] = base;
        cpu.r[1] = 1; // mov #1,r1
        let _ = mem.write_u32(base.wrapping_add(r4), cpu.r[3]);
        let bbase = mem.read_u32(0x8C02BEE8).unwrap(); // 0x3d0
        cpu.r[0] = bbase;
        let _ = mem.write_u8(bbase.wrapping_add(r4), cpu.r[1] as u8);
        cpu.r[0] = cpu.r[5];
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
        cpu.r[1] = cpu.r[1].wrapping_add(2);
        let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]); // rts delay slot
        return;
    }
    cpu.r[1] = mem.read_u32(r4.wrapping_add(4)).unwrap(); // mov.l @(4,r4),r1
    let tbl = mem.read_u32(0x8C02BEEC).unwrap(); // 0x274
    cpu.r[0] = tbl;
    cpu.r[0] = mem.read_u32(tbl.wrapping_add(cpu.r[1])).unwrap(); // mov.l @(r0,r1),r0
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[3] as i32)); // cmp/gt r3,r0
    if !cpu.t() {
        return; // bf -> rts
    }
    let base = mem.read_u32(0x8C02BEE4).unwrap(); // 0x3bc
    cpu.r[0] = base;
    cpu.r[1] = 1;
    let _ = mem.write_u32(base.wrapping_add(r4), cpu.r[3]);
    let bbase = mem.read_u32(0x8C02BEE8).unwrap(); // 0x3d0
    cpu.r[0] = bbase;
    let _ = mem.write_u8(bbase.wrapping_add(r4), cpu.r[1] as u8);
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
    cpu.r[1] = cpu.r[1].wrapping_add(1);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(r4), cpu.r[1]);
}
