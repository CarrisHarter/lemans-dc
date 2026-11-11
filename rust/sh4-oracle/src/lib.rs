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

        match op >> 12 {
            0x0 => match op {
                0x0009 => Ok(()), // nop
                0x000B => {
                    // rts: return through PR, delayed
                    self.delay = Some(self.pr);
                    Ok(())
                }
                0x0008 => {
                    // clrt
                    self.set_t(false);
                    Ok(())
                }
                0x0018 => {
                    // sett
                    self.set_t(true);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x0 family")),
            },
            0x3 => {
                // add Rm,Rn
                if op & 0xF == 0xC {
                    self.r[n] = self.r[n].wrapping_add(self.r[m]);
                    Ok(())
                } else {
                    Err(Error::Unimplemented(pc, op, "0x3 family"))
                }
            }
            0x4 => match op & 0xF0FF {
                0x4010 => {
                    // dt Rn: decrement, T = (result == 0)
                    self.r[n] = self.r[n].wrapping_sub(1);
                    self.set_t(self.r[n] == 0);
                    Ok(())
                }
                0x4015 => {
                    // cmp/pl Rn: T = (Rn > 0)
                    let v = self.r[n] as i32;
                    self.set_t(v > 0);
                    Ok(())
                }
                0x400B => {
                    // jsr @Rn: PR = pc+4, delayed
                    self.pr = pc.wrapping_add(4);
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                0x402B => {
                    // jmp @Rn: delayed, PR unchanged
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x4 family")),
            },
            0x6 => {
                // mov Rm,Rn
                if op & 0xF == 0x3 {
                    self.r[n] = self.r[m];
                    Ok(())
                } else {
                    Err(Error::Unimplemented(pc, op, "0x6 family"))
                }
            }
            0x7 => {
                // add #imm,Rn
                let imm = (op & 0xFF) as i8 as i32;
                self.r[n] = (self.r[n] as i32).wrapping_add(imm) as u32;
                Ok(())
            }
            0x8 => {
                let disp = (op & 0xFF) as i8 as i32;
                let target = (pc as i32 + 4 + disp * 2) as u32;
                match op >> 8 {
                    0x89 => {
                        // bt
                        if self.t() {
                            self.pc = target;
                        }
                        Ok(())
                    }
                    0x8B => {
                        // bf
                        if !self.t() {
                            self.pc = target;
                        }
                        Ok(())
                    }
                    0x8D => {
                        // bt.s
                        if self.t() {
                            self.delay = Some(target);
                        }
                        Ok(())
                    }
                    0x8F => {
                        // bf.s
                        if !self.t() {
                            self.delay = Some(target);
                        }
                        Ok(())
                    }
                    _ => Err(Error::Unimplemented(pc, op, "0x8 family")),
                }
            }
            0xD => {
                // mov.l @(disp,pc),Rn
                let disp = (op & 0xFF) as u32;
                let addr = (pc & !3).wrapping_add(4).wrapping_add(disp * 4);
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
