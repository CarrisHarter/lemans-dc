//! Access-width regressions.
//!
//! The post-increment and `@(R0,Rm)` load/store families encode the width in
//! the low nibble as 4/5/6, not 0/1/2. The interpreter used to fall through to
//! a 32-bit access for those values, so `mov.b @Rm+,Rn` read four bytes and
//! every port built on that path was equivalent to the bug rather than to the
//! machine. These tests execute the real bytes from the game image.

use sh4_oracle::{image, Cpu, Memory, Region, Stop, HALT_PC, LOAD_BASE};

const DATA: u32 = 0x8C20_0000;

fn machine(code: &[u8]) -> (Cpu, Memory) {
    let mut mem = image(LOAD_BASE, code);
    mem.regions
        .push(Region::new(DATA, vec![0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22]));
    (Cpu::new(), mem)
}

#[test]
fn mov_b_post_increment_reads_one_byte() {
    // 0x8c010e62: `mov.b @r7+,r2` (0x6274) ; `rts` ; `nop`
    let (mut cpu, mut mem) = machine(&[0x74, 0x62, 0x0B, 0x00, 0x09, 0x00]);
    cpu.r[7] = DATA;
    let stop = cpu.run(&mut mem, 10).unwrap();
    assert_eq!(stop, Stop::Halt);
    assert_eq!(cpu.r[2], 0xFFFF_FFAA, "byte load sign-extends 0xAA");
    assert_eq!(cpu.r[7], DATA + 1, "post-increment is one byte");
}

#[test]
fn mov_w_post_increment_reads_one_word() {
    // 0x8c010eca: `mov.w @r5+,r3` (0x6355) ; `rts` ; `nop`
    let (mut cpu, mut mem) = machine(&[0x55, 0x63, 0x0B, 0x00, 0x09, 0x00]);
    cpu.r[5] = DATA;
    let stop = cpu.run(&mut mem, 10).unwrap();
    assert_eq!(stop, Stop::Halt);
    assert_eq!(cpu.r[3], 0xFFFF_BBAA, "word load sign-extends 0xBBAA");
    assert_eq!(cpu.r[5], DATA + 2, "post-increment is one word");
}

#[test]
fn mov_b_indexed_reads_one_byte() {
    // 0x8c010d70: `mov.b @(r0,r1),r0` (0x001C) ; `rts` ; `nop`
    let (mut cpu, mut mem) = machine(&[0x1C, 0x00, 0x0B, 0x00, 0x09, 0x00]);
    cpu.r[0] = 1;
    cpu.r[1] = DATA;
    let stop = cpu.run(&mut mem, 10).unwrap();
    assert_eq!(stop, Stop::Halt);
    assert_eq!(cpu.r[0], 0xFFFF_FFBB, "indexed byte load must not read a word");
}

#[test]
fn mov_b_predecrement_stores_one_byte() {
    // `mov.b r2,@-r3` (0x2324) ; `rts` ; `nop`
    let (mut cpu, mut mem) = machine(&[0x24, 0x23, 0x0B, 0x00, 0x09, 0x00]);
    cpu.r[2] = 0x5A;
    cpu.r[3] = DATA + 4;
    let stop = cpu.run(&mut mem, 10).unwrap();
    assert_eq!(stop, Stop::Halt);
    assert_eq!(cpu.r[3], DATA + 3);
    assert_eq!(mem.read_u8(DATA + 3).unwrap(), 0x5A);
    assert_eq!(mem.read_u8(DATA + 2).unwrap(), 0xCC, "neighbour untouched");
    assert_eq!(mem.read_u8(DATA + 4).unwrap(), 0x11, "neighbour untouched");
}

#[test]
fn pr_is_the_default_return_target() {
    let (mut cpu, mut mem) = machine(&[0x0B, 0x00, 0x09, 0x00]);
    assert_eq!(cpu.pr, HALT_PC);
    assert_eq!(cpu.run(&mut mem, 4).unwrap(), Stop::Halt);
}
