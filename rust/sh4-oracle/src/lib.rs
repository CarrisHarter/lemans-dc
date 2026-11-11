//! Minimal SH-4 (SH7750) interpreter used as a behavioural oracle.
//!
//! Scope is deliberate: this exists to answer "does my Rust rewrite compute the
//! same thing as the original routine?", not to run a Dreamcast. It implements
//! the integer core, delayed branches, the general-purpose register set, and the
//! single-precision FPU subset that decompiled game code actually uses.
//!
//! Anything not implemented returns [`Error::Unimplemented`] with the opcode and
//! PC, so a gap is visible instead of silently producing wrong results.
//!
//! Deliberate omissions: no MMU, no cache, no interrupts, no DMA, no store
//! queues (writes are recorded so a test can compare memory effects), and no
//! multi-threading. Those belong to the console, not to a function-level oracle.

use std::fmt;

/// Address the Katana bootstrap maps `1ST_READ.BIN` at.
pub const LOAD_BASE: u32 = 0x8C01_0000;

/// Sentinel return address: `rts` to this stops the machine.
pub const HALT_PC: u32 = 0xFFFF_FFF0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Instruction outside any mapped region.
    FetchOutOfRange(u32),
    /// Load/store outside any mapped region.
    DataOutOfRange(u32, u32),
    /// Decoder gap. Carries pc, opcode and a short tag.
    Unimplemented(u32, u16, &'static str),
    /// Execution limit hit; not an error in itself, but a test failure.
    StepLimit(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FetchOutOfRange(pc) => write!(f, "fetch outside mapped memory at 0x{pc:08x}"),
            Error::DataOutOfRange(addr, pc) => {
                write!(f, "data access 0x{addr:08x} outside mapped memory (pc 0x{pc:08x})")
            }
            Error::Unimplemented(pc, op, tag) => {
                write!(f, "unimplemented {tag} 0x{op:04x} at 0x{pc:08x}")
            }
            Error::StepLimit(n) => write!(f, "step limit {n} reached"),
        }
    }
}

impl std::error::Error for Error {}

/// Why execution stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// `rts` returned to [`HALT_PC`].
    Halt,
    Steps(usize),
}

/// One flat window of memory. The oracle uses one; the real machine has many.
#[derive(Debug, Clone)]
pub struct Region {
    pub base: u32,
    pub bytes: Vec<u8>,
}

impl Region {
    pub fn new(base: u32, bytes: Vec<u8>) -> Self {
        Self { base, bytes }
    }

    pub fn contains(&self, addr: u32, len: u32) -> bool {
        let end = addr as u64 + len as u64;
        addr >= self.base && end <= self.base as u64 + self.bytes.len() as u64
    }
}

/// Byte-addressable memory plus a log of writes, so tests can diff side effects.
#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub regions: Vec<Region>,
    pub writes: Vec<(u32, u8)>,
}

impl Memory {
    pub fn with_region(region: Region) -> Self {
        Self { regions: vec![region], writes: Vec::new() }
    }

    fn locate(&self, addr: u32, len: u32) -> Option<(&Region, usize)> {
        self.regions
            .iter()
            .find(|r| r.contains(addr, len))
            .map(|r| (r, (addr - r.base) as usize))
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, Error> {
        let (r, off) = self.locate(addr, 2).ok_or(Error::DataOutOfRange(addr, addr))?;
        Ok(u16::from_le_bytes([r.bytes[off], r.bytes[off + 1]]))
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8, Error> {
        let (r, off) = self.locate(addr, 1).ok_or(Error::DataOutOfRange(addr, addr))?;
        Ok(r.bytes[off])
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), Error> {
        let (r, off) = self.locate(addr, 1).ok_or(Error::DataOutOfRange(addr, addr))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off] = value;
        self.writes.push((addr, value));
        Ok(())
    }

    pub fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), Error> {
        let (r, off) = self.locate(addr, 2).ok_or(Error::DataOutOfRange(addr, addr))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off..off + 2].copy_from_slice(&value.to_le_bytes());
        self.writes.extend((0..2).map(|i| (addr + i, value.to_le_bytes()[i as usize])));
        Ok(())
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, Error> {
        let (r, off) = self.locate(addr, 4).ok_or(Error::DataOutOfRange(addr, addr))?;
        let mut b = [0u8; 4];
        b.copy_from_slice(&r.bytes[off..off + 4]);
        Ok(u32::from_le_bytes(b))
    }

    pub fn read_f32(&self, addr: u32) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_u32(addr)?))
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), Error> {
        let pc_hint = addr;
        let (r, off) = self
            .locate(addr, 4)
            .ok_or(Error::DataOutOfRange(addr, pc_hint))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off..off + 4].copy_from_slice(&value.to_le_bytes());
        self.writes.extend((0..4).map(|i| (addr + i, value.to_le_bytes()[i as usize])));
        Ok(())
    }

    pub fn write_f32(&mut self, addr: u32, value: f32) -> Result<(), Error> {
        self.write_u32(addr, value.to_bits())
    }

    /// Replace the whole region body without recording writes (test setup).
    pub fn load(&mut self, req_base: u32, bytes: &[u8]) -> Result<(), Error> {
        self.regions
            .push(Region::new(req_base, bytes.to_vec()));
        Ok(())
    }
}

/// Architectural state.
#[derive(Debug, Clone)]
pub struct Cpu {
    pub r: [u32; 16],
    pub pc: u32,
    pub pr: u32,
    pub sr: u32,
    /// FPU registers, always single precision in this oracle.
    pub fr: [f32; 16],
    pub fpscr: u32,
    /// FPU communication register, used by float/ftrc/flds/fsts.
    pub fpul: u32,
    /// Pending delayed-branch target, consumed after the delay slot runs.
    pub delay: Option<u32>,
    pub steps: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            pc: LOAD_BASE,
            pr: HALT_PC,
            sr: 0,
            fr: [0.0; 16],
            fpscr: 0,
            fpul: 0,
            delay: None,
            steps: 0,
        }
    }

    #[inline]
    pub fn t(&self) -> bool {
        self.sr & 1 != 0
    }

    #[inline]
    fn set_t(&mut self, v: bool) {
        if v {
            self.sr |= 1;
        } else {
            self.sr &= !1;
        }
    }

    #[inline]
    pub fn reg(&self, n: usize) -> u32 {
        self.r[n]
    }

    /// Execute one instruction. A delayed branch consumes two steps.
    pub fn step(&mut self, mem: &mut Memory) -> Result<(), Error> {
        if let Some(target) = self.delay.take() {
            let pc = self.pc;
            let op = fetch(mem, pc)?;
            self.pc = pc.wrapping_add(2);
            self.execute(op, mem, pc)?;
            if self.delay.is_some() {
                return Err(Error::Unimplemented(pc, op, "branch in delay slot"));
            }
            self.pc = target;
            self.steps += 1;
            return Ok(());
        }

        let pc = self.pc;
        let op = fetch(mem, pc)?;
        self.pc = pc.wrapping_add(2);
        self.execute(op, mem, pc)?;
        self.steps += 1;
        Ok(())
    }

    /// Run from `self.pc` until `rts` reaches [`HALT_PC`] or `limit` steps.
    pub fn run(&mut self, mem: &mut Memory, limit: usize) -> Result<Stop, Error> {
        for _ in 0..limit {
            if self.pc == HALT_PC {
                return Ok(Stop::Halt);
            }
            self.step(mem)?;
        }
        Ok(Stop::Steps(limit))
    }

    /// Call a function by ABI: r4..r7 hold the first four integer arguments.
    /// Returns `r0` after the routine returns to [`HALT_PC`].
    pub fn call(
        &mut self,
        mem: &mut Memory,
        entry: u32,
        args: &[u32],
        limit: usize,
    ) -> Result<u32, Error> {
        self.pc = entry;
        self.pr = HALT_PC;
        self.delay = None;
        self.r[4..7].copy_from_slice(&args[..args.len().min(4)]);
        match self.run(mem, limit)? {
            Stop::Halt => Ok(self.r[0]),
            Stop::Steps(n) => Err(Error::StepLimit(n)),
        }
    }

    fn execute(&mut self, op: u16, mem: &mut Memory, pc: u32) -> Result<(), Error> {
        let n = ((op >> 8) & 0xF) as usize;
        let m = ((op >> 4) & 0xF) as usize;
        let disp8 = (op & 0xFF) as i8 as i32;

        match op >> 12 {
            0x0 => match op {
                0x0009 => Ok(()),                       // nop
                0x000B => { self.delay = Some(self.pr); Ok(()) }  // rts (delayed)
                0x0002 => {                              // stc SR,Rn
                    self.r[n] = self.sr;
                    Ok(())
                }
                0x0008 => { self.set_t(false); Ok(()) }  // clrt
                0x0018 => { self.set_t(true); Ok(()) }   // sett
                _ => match op & 0xF {
                    0x4 | 0x5 | 0x6 => {
                        // mov.b/w/l Rm,@(R0,Rn)
                        let addr = self.r[0].wrapping_add(self.r[n]);
                        self.store(mem, addr, self.r[m], op & 0xF)?;
                        Ok(())
                    }
                    0xC | 0xD | 0xE => {
                        // mov.b/w/l @(R0,Rm),Rn
                        let addr = self.r[0].wrapping_add(self.r[m]);
                        self.r[n] = self.load(mem, addr, op & 0xF)?;
                        Ok(())
                    }
                    _ => Err(Error::Unimplemented(pc, op, "0x0 family")),
                },
            },
            0x1 => {
                // mov.l Rm,@(disp,Rn)
                let addr = self.r[n].wrapping_add((op & 0xF) as u32 * 4);
                self.store(mem, addr, self.r[m], 0x2)?;
                Ok(())
            }
            0x2 => match op & 0xF {
                0x0 | 0x1 | 0x2 => {
                    // mov.b/w/l Rm,@Rn
                    self.store(mem, self.r[n], self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x4 | 0x5 | 0x6 => {
                    // mov.b/w/l Rm,@-Rn   (decrement first)
                    let step = match op & 0xF { 0x4 => 1, 0x5 => 2, _ => 4 };
                    self.r[n] = self.r[n].wrapping_sub(step);
                    self.store(mem, self.r[n], self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x8 => {
                    // tst Rm,Rn
                    self.set_t(self.r[n] & self.r[m] == 0);
                    Ok(())
                }
                0x9 => {
                    // and Rm,Rn
                    self.r[n] &= self.r[m];
                    Ok(())
                }
                0xA => {
                    // xor Rm,Rn
                    self.r[n] ^= self.r[m];
                    Ok(())
                }
                0xB => {
                    // or Rm,Rn
                    self.r[n] |= self.r[m];
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x2 family")),
            },
            0x3 => match op & 0xF {
                0x0 => {                                 // cmp/eq Rm,Rn
                    self.set_t(self.r[n] == self.r[m]);
                    Ok(())
                }
                0x2 => {                                 // cmp/hs Rm,Rn (unsigned >=)
                    self.set_t(self.r[n] >= self.r[m]);
                    Ok(())
                }
                0x3 => {                                 // cmp/ge Rm,Rn (signed >=)
                    self.set_t((self.r[n] as i32) >= (self.r[m] as i32));
                    Ok(())
                }
                0x6 => {                                 // cmp/hi Rm,Rn (unsigned >)
                    self.set_t(self.r[n] > self.r[m]);
                    Ok(())
                }
                0x7 => {                                 // cmp/gt Rm,Rn (signed >)
                    self.set_t((self.r[n] as i32) > (self.r[m] as i32));
                    Ok(())
                }
                0x8 => {                                 // sub Rm,Rn
                    self.r[n] = self.r[n].wrapping_sub(self.r[m]);
                    Ok(())
                }
                0xC => {                                 // add Rm,Rn
                    self.r[n] = self.r[n].wrapping_add(self.r[m]);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x3 family")),
            },
            0x4 => match op & 0xF0FF {
                0x4000 => {                              // shll Rn
                    let out = self.r[n] >> 31 != 0;
                    self.r[n] = self.r[n] << 1;
                    self.set_t(out);
                    Ok(())
                }
                0x4001 => {                              // shlr Rn
                    let out = self.r[n] & 1 != 0;
                    self.r[n] = self.r[n] >> 1;
                    self.set_t(out);
                    Ok(())
                }
                0x4008 => {                              // shll2 Rn
                    self.r[n] = self.r[n] << 2;
                    Ok(())
                }
                0x4009 => {                              // shlr2 Rn
                    self.r[n] = self.r[n] >> 2;
                    Ok(())
                }
                0x4010 => {                              // dt Rn
                    self.r[n] = self.r[n].wrapping_sub(1);
                    self.set_t(self.r[n] == 0);
                    Ok(())
                }
                0x4073 => {                              // not Rn
                    self.r[n] = !self.r[n];
                    Ok(())
                }
                0x407B => {                              // neg Rn
                    self.r[n] = (self.r[n] as i32).wrapping_neg() as u32;
                    Ok(())
                }
                0x4018 => {                              // shll8 Rn
                    self.r[n] = self.r[n] << 8;
                    Ok(())
                }
                0x4019 => {                              // shlr8 Rn
                    self.r[n] = self.r[n] >> 8;
                    Ok(())
                }
                0x4028 => {                              // shll16 Rn
                    self.r[n] = self.r[n] << 16;
                    Ok(())
                }
                0x4029 => {                              // shlr16 Rn
                    self.r[n] = self.r[n] >> 16;
                    Ok(())
                }
                0x4011 => {                              // cmp/pz Rn
                    self.set_t((self.r[n] as i32) >= 0);
                    Ok(())
                }
                0x4015 => {                              // cmp/pl Rn
                    self.set_t((self.r[n] as i32) > 0);
                    Ok(())
                }
                0x4002 => {                              // sts PR,Rn
                    self.r[n] = self.pr;
                    Ok(())
                }
                0x400E => {                              // ldc Rm,SR
                    self.sr = self.r[m];
                    Ok(())
                }
                0x402A => {                              // lds Rm,PR
                    self.pr = self.r[m];
                    Ok(())
                }
                0x4022 => {                              // sts.l PR,@-Rn
                    self.r[n] = self.r[n].wrapping_sub(4);
                    mem.write_u32(self.r[n], self.pr)?;
                    Ok(())
                }
                0x4026 => {                              // lds.l @Rm+,PR
                    self.pr = mem.read_u32(self.r[m])?;
                    self.r[m] = self.r[m].wrapping_add(4);
                    Ok(())
                }
                0x400B => {                              // jsr @Rn (delayed)
                    self.pr = pc.wrapping_add(4);
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                0x402B => {                              // jmp @Rn (delayed)
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x4 family")),
            },
            0x6 => match op & 0xF {
                0x0 | 0x1 | 0x2 => {                     // mov.b/w/l @Rm,Rn
                    self.r[n] = self.load(mem, self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x3 => {                                 // mov Rm,Rn
                    self.r[n] = self.r[m];
                    Ok(())
                }
                0x4 | 0x5 | 0x6 => {                     // mov.b/w/l @Rm+,Rn
                    self.r[n] = self.load(mem, self.r[m], op & 0xF)?;
                    let step = match op & 0xF { 0x4 => 1, 0x5 => 2, _ => 4 };
                    self.r[m] = self.r[m].wrapping_add(step);
                    Ok(())
                }
                0xC => {                                 // extu.b Rm,Rn
                    self.r[n] = self.r[m] & 0xFF;
                    Ok(())
                }
                0xD => {                                 // extu.w Rm,Rn
                    self.r[n] = self.r[m] & 0xFFFF;
                    Ok(())
                }
                0xE => {                                 // exts.b Rm,Rn
                    self.r[n] = self.r[m] as u8 as i8 as i32 as u32;
                    Ok(())
                }
                0xF => {                                 // exts.w Rm,Rn
                    self.r[n] = self.r[m] as u16 as i16 as i32 as u32;
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x6 family")),
            },
            0x7 => {
                // add #imm,Rn
                self.r[n] = (self.r[n] as i32).wrapping_add(disp8) as u32;
                Ok(())
            }
            0x8 => {
                let disp = disp8;
                let target = (pc as i32 + 4 + disp * 2) as u32;
                match op >> 8 {
                    0x80 => {                            // mov.b R0,@(disp,Rn)
                        let addr = self.r[n].wrapping_add(disp as u32);
                        self.store(mem, addr, self.r[0], 0x0)?;
                        Ok(())
                    }
                    0x81 => {                            // mov.w R0,@(disp,Rn)
                        let addr = self.r[n].wrapping_add(disp as u32 * 2);
                        self.store(mem, addr, self.r[0], 0x1)?;
                        Ok(())
                    }
                    0x84 => {                            // mov.b @(disp,Rm),R0
                        let addr = self.r[m].wrapping_add(disp as u32);
                        self.r[0] = self.load(mem, addr, 0x0)?;
                        Ok(())
                    }
                    0x85 => {                            // mov.w @(disp,Rm),R0
                        let addr = self.r[m].wrapping_add(disp as u32 * 2);
                        self.r[0] = self.load(mem, addr, 0x1)?;
                        Ok(())
                    }
                    0x88 => {                            // cmp/eq #imm,R0
                        self.set_t(self.r[0] == (op & 0xFF) as i8 as i32 as u32);
                        Ok(())
                    }
                    0x89 => { if self.t() { self.pc = target; } Ok(()) }   // bt
                    0x8B => { if !self.t() { self.pc = target; } Ok(()) }  // bf
                    0x8D => { if self.t() { self.delay = Some(target); } Ok(()) }  // bt.s
                    0x8F => { if !self.t() { self.delay = Some(target); } Ok(()) } // bf.s
                    _ => Err(Error::Unimplemented(pc, op, "0x8 family")),
                }
            }
            0x9 => {
                // mov.w @(disp,PC),Rn  (word literal)
                let addr = (pc & !1).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 2);
                self.r[n] = mem.read_u16(addr)? as i16 as i32 as u32;
                Ok(())
            }
            0xA => {
                // bra disp12 (delayed)
                let disp = (op & 0x0FFF) as i16 as i32;
                self.delay = Some((pc as i32 + 4 + disp * 2) as u32);
                Ok(())
            }
            0xB => {
                // bsr disp12 (delayed)
                let disp = (op & 0x0FFF) as i16 as i32;
                self.pr = pc.wrapping_add(4);
                self.delay = Some((pc as i32 + 4 + disp * 2) as u32);
                Ok(())
            }
            0xC => match op >> 8 {
                0xC7 => {                                // mova @(disp,PC),R0
                    self.r[0] = (pc & !3).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 4);
                    Ok(())
                }
                0xC8 => {                                // tst #imm,R0
                    self.set_t(self.r[0] & (op & 0xFF) as u32 == 0);
                    Ok(())
                }
                0xC9 => {                                // and #imm,R0
                    self.r[0] &= (op & 0xFF) as u32;
                    Ok(())
                }
                0xCA => {                                // xor #imm,R0
                    self.r[0] ^= (op & 0xFF) as u32;
                    Ok(())
                }
                0xCB => {                                // or #imm,R0
                    self.r[0] |= (op & 0xFF) as u32;
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0xC family")),
            },
            0xD => {
                // mov.l @(disp,pc),Rn
                let addr = (pc & !3).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 4);
                self.r[n] = mem.read_u32(addr)?;
                Ok(())
            }
            0xE => {
                // mov #imm,Rn
                self.r[n] = (op & 0xFF) as i8 as i32 as u32;
                Ok(())
            }
            0xF => self.execute_fpu(op, mem, pc, n, m),
            _ => Err(Error::Unimplemented(pc, op, "top nibble")),
        }
    }

    /// Load/store helper: size 0 = byte, 1 = word, 2 = long, sign-extended.
    fn load(&self, mem: &Memory, addr: u32, size: u16) -> Result<u32, Error> {
        match size {
            0 => Ok(mem.read_u8(addr)? as i8 as i32 as u32),
            1 => Ok(mem.read_u16(addr)? as i16 as i32 as u32),
            _ => mem.read_u32(addr),
        }
    }

    fn store(&self, mem: &mut Memory, addr: u32, value: u32, size: u16) -> Result<(), Error> {
        match size {
            0 => mem.write_u8(addr, value as u8),
            1 => mem.write_u16(addr, value as u16),
            _ => mem.write_u32(addr, value),
        }
    }

    fn execute_fpu(
        &mut self,
        op: u16,
        mem: &mut Memory,
        pc: u32,
        n: usize,
        m: usize,
    ) -> Result<(), Error> {
        match op & 0xF {
            0x0 => {
                // fadd FRm,FRn
                self.fr[n] = self.fr[n] + self.fr[m];
                Ok(())
            }
            0x1 => {
                // fsub FRm,FRn
                self.fr[n] = self.fr[n] - self.fr[m];
                Ok(())
            }
            0x2 => {
                // fmul FRm,FRn
                self.fr[n] = self.fr[n] * self.fr[m];
                Ok(())
            }
            0x3 => {
                // fdiv FRm,FRn
                self.fr[n] = self.fr[n] / self.fr[m];
                Ok(())
            }
            0xE => {
                // fmac FR0,FRm,FRn: FRn = FRn + FR0 * FRm, one rounding step.
                // mul_add(a, b) computes a*b + b, so the product pair comes first.
                self.fr[n] = self.fr[0].mul_add(self.fr[m], self.fr[n]);
                Ok(())
            }
            0x4 => {
                // fcmp/eq FRm,FRn
                self.set_t(self.fr[n] == self.fr[m]);
                Ok(())
            }
            0x5 => {
                // fcmp/gt FRm,FRn
                self.set_t(self.fr[n] > self.fr[m]);
                Ok(())
            }
            0x2D if op & 0x00FF == 0x2D => {
                // float FPUL,FRn
                self.fr[n] = self.fpul as i32 as f32;
                Ok(())
            }
            0x3D if op & 0x00FF == 0x3D => {
                // ftrc FRm,FPUL
                self.fpul = self.fr[m] as i32 as u32;
                Ok(())
            }
            0x0D if op & 0x00FF == 0x1D => {
                // fsts FPUL,FRn
                self.fr[n] = f32::from_bits(self.fpul);
                Ok(())
            }
            0x0D if op & 0x00FF == 0x0D => {
                // flds FRm,FPUL
                self.fpul = self.fr[m].to_bits();
                Ok(())
            }
            0xC => {
                // fmov FRm,FRn
                self.fr[n] = self.fr[m];
                Ok(())
            }
            0x8 => {
                // fmov.s @Rm,FRn
                self.fr[n] = mem.read_f32(self.r[m])?;
                Ok(())
            }
            0xA => {
                // fmov.s FRm,@Rn
                mem.write_f32(self.r[n], self.fr[m])?;
                Ok(())
            }
            0x5 if op & 0x00FF == 0x0D => {
                // flds FRm,FPUL  (FPUL is not modelled; keep the value in fpscr-unused)
                Err(Error::Unimplemented(pc, op, "flds (FPUL not modelled)"))
            }
            0xD if op & 0x00FF == 0x8D => {
                // fldi0 FRn
                self.fr[n] = 0.0;
                Ok(())
            }
            0xD if op & 0x00FF == 0x9D => {
                // fldi1 FRn
                self.fr[n] = 1.0;
                Ok(())
            }
            _ => Err(Error::Unimplemented(pc, op, "0xF family")),
        }
    }
}

fn fetch(mem: &Memory, pc: u32) -> Result<u16, Error> {
    mem.read_u16(pc).map_err(|_| Error::FetchOutOfRange(pc))
}

/// Build a memory image with `bytes` mapped at `base`.
pub fn image(base: u32, bytes: &[u8]) -> Memory {
    Memory::with_region(Region::new(base, bytes.to_vec()))
}
