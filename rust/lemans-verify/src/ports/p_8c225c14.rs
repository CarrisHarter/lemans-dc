//! 0x8c225c14 — validate a descriptor at `[r4+28]`: range/shape checks on
//! the count and bounds words, then map count 0xff/0x1ff/0x3ff/0x7ff to
//! `[r4+4]` = 0/1/2/3. Returns 0 on success, -1 (rts delay slot) on any
//! failed check.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    macro_rules! fail {
        () => {{
            cpu.r[0] = (-1i32) as u32; // rts delay slot
            return;
        }};
    }
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0);
    cpu.r[6] = 1;
    cpu.r[7] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= (cpu.r[6] as i32)); // cmp/ge r6,r7
    if !cpu.t() {
        fail!();
    }
    cpu.r[2] = mem.read_u16(0x8c225cee).unwrap_or(0) as i16 as i32 as u32; // 0x7ff
    cpu.set_t((cpu.r[7] as i32) > (cpu.r[2] as i32)); // cmp/gt r2,r7
    if cpu.t() {
        fail!();
    }
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[3] == cpu.r[2]); // cmp/eq r2,r3
    if !cpu.t() {
        fail!();
    }
    cpu.r[7] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= 0); // cmp/pz r7
    if !cpu.t() {
        fail!();
    }
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= (cpu.r[2] as i32)); // cmp/ge r2,r7
    if cpu.t() {
        fail!();
    }
    cpu.r[7] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= 0); // cmp/pz r7
    if !cpu.t() {
        fail!();
    }
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r7
    if cpu.t() {
        fail!();
    }
    cpu.r[7] = mem.read_u32(cpu.r[5].wrapping_add(52)).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= 0); // cmp/pz r7
    if !cpu.t() {
        fail!();
    }
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) > (cpu.r[2] as i32)); // cmp/gt r2,r7
    if cpu.t() {
        fail!();
    }
    cpu.r[7] = mem.read_u32(cpu.r[5].wrapping_add(56)).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) >= 0); // cmp/pz r7
    if !cpu.t() {
        fail!();
    }
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t((cpu.r[7] as i32) > (cpu.r[3] as i32)); // cmp/gt r3,r7
    if cpu.t() {
        fail!();
    }
    cpu.r[2] = mem.read_u32(0x8c225cfc).unwrap_or(0); // 0x8c2c02b4
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[1] & cpu.r[1] == 0); // tst r1,r1
    if cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap_or(0);
        cpu.r[0] = mem.read_u16(cpu.r[4].wrapping_add(16)).unwrap_or(0) as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r0
        if !cpu.t() {
            fail!();
        }
        cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap_or(0);
        cpu.r[0] = mem.read_u16(cpu.r[4].wrapping_add(18)).unwrap_or(0) as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r0
        if !cpu.t() {
            fail!();
        }
    }
    cpu.r[1] = mem.read_u16(0x8c225cf0).unwrap_or(0) as i16 as i32 as u32; // 0xff
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0
    if cpu.t() {
        cpu.r[3] = 0;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[3]); // bra delay slot
        cpu.r[0] = 0;
        return;
    }
    cpu.r[1] = mem.read_u16(0x8c225cf2).unwrap_or(0) as i16 as i32 as u32; // 0x1ff
    cpu.set_t(cpu.r[0] == cpu.r[1]);
    if cpu.t() {
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[6]); // bra delay slot (r6 = 1)
        cpu.r[0] = 0;
        return;
    }
    cpu.r[1] = mem.read_u16(0x8c225cf4).unwrap_or(0) as i16 as i32 as u32; // 0x3ff
    cpu.set_t(cpu.r[0] == cpu.r[1]);
    if cpu.t() {
        cpu.r[2] = 2;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[2]); // bra delay slot
        cpu.r[0] = 0;
        return;
    }
    cpu.r[1] = mem.read_u16(0x8c225cee).unwrap_or(0) as i16 as i32 as u32; // 0x7ff
    cpu.set_t(cpu.r[0] == cpu.r[1]);
    if cpu.t() {
        cpu.r[1] = 3;
        let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[1]); // bra delay slot
        cpu.r[0] = 0;
        return;
    }
    fail!();
}
