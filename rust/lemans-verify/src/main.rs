//! Diff a hand-written Rust port against the SH-4 oracle on the same spec.
//!
//! Both sides start from the identical seeded state. The oracle executes the
//! original bytes; the port is a function in `registry`. Registers, FPU
//! registers and every recorded store must match exactly — no tolerance, since
//! a rounding difference here compounds into gameplay.
//!
//! Usage: lemans-verify <binary> <entry-address> <spec.json> [--oracle PATH]

mod registry;

use std::path::PathBuf;
use std::process::{Command, ExitCode};

use sh4_oracle::json::{self, Json};
use sh4_oracle::{image, in_stack, install_default_stack, Cpu, Memory, LOAD_BASE};

#[derive(Clone)]
struct Seed {
    entry: u32,
    regs: Vec<(usize, u32)>,
    fr: Vec<(usize, f32)>,
    memory: Vec<(u32, Vec<u8>)>,
    /// System registers, by JSON key, seeded and compared as a unit.
    special: Vec<(&'static str, u32)>,
}

const SPECIAL: [&str; 11] = [
    "sr", "mach", "macl", "gbr", "vbr", "ssr", "spc", "sgr", "dbr", "fpul", "fpscr",
];

fn read_seed(spec: &Json, base: u32) -> Seed {
    let entry = spec.get("entry").and_then(Json::as_u32).unwrap_or(base);
    let mut regs = Vec::new();
    if let Some(Json::Object(map)) = spec.get("regs") {
        for (k, v) in map {
            if let Some(i) = k.strip_prefix('r').and_then(|s| s.parse::<usize>().ok()) {
                if let Some(value) = v.as_u32() {
                    regs.push((i, value));
                }
            }
        }
    }
    let mut fr = Vec::new();
    if let Some(Json::Object(map)) = spec.get("fr") {
        for (k, v) in map {
            if let Some(i) = k.strip_prefix('f').and_then(|s| s.parse::<usize>().ok()) {
                if let Some(value) = v.as_f32() {
                    fr.push((i, value));
                }
            }
        }
    }
    let mut memory = Vec::new();
    if let Some(items) = spec.get("memory").and_then(Json::as_array) {
        for item in items {
            if let (Some(addr), Some(hex)) = (
                item.get("addr").and_then(Json::as_u32),
                item.get("hex").and_then(Json::as_str),
            ) {
                let bytes = (0..hex.len() / 2)
                    .filter_map(|i| u8::from_str_radix(hex.get(i * 2..i * 2 + 2)?, 16).ok())
                    .collect();
                memory.push((addr, bytes));
            }
        }
    }
    let mut special = Vec::new();
    for key in SPECIAL {
        if let Some(v) = spec.get(key).and_then(Json::as_u32) {
            special.push((key, v));
        }
    }
    Seed { entry, regs, fr, memory, special }
}

fn seeded(seed: &Seed, bytes: &[u8], base: u32) -> (Cpu, Memory) {
    let mut mem = image(base, bytes);
    let mut cpu = Cpu::new();
    cpu.pc = seed.entry;
    for (i, v) in &seed.regs {
        if *i < 16 {
            cpu.r[*i] = *v;
        }
    }
    for (i, v) in &seed.fr {
        if *i < 16 {
            cpu.fr[*i] = *v;
        }
    }
    for (addr, data) in &seed.memory {
        for (i, byte) in data.iter().enumerate() {
            let _ = mem.write_u8(addr + i as u32, *byte);
        }
    }
    for (key, value) in &seed.special {
        match *key {
            "sr" => cpu.sr = *value,
            "mach" => cpu.mach = *value,
            "macl" => cpu.macl = *value,
            "gbr" => cpu.gbr = *value,
            "vbr" => cpu.vbr = *value,
            "ssr" => cpu.ssr = *value,
            "spc" => cpu.spc = *value,
            "sgr" => cpu.sgr = *value,
            "dbr" => cpu.dbr = *value,
            "fpul" => cpu.fpul = *value,
            "fpscr" => cpu.fpscr = *value,
            _ => {}
        }
    }
    mem.writes.clear();
    (cpu, mem)
}

fn parse_hex(text: &str) -> Option<u32> {
    json::parse_hex(text)
}

fn fail(message: &str) -> ExitCode {
    println!("FAIL {message}");
    ExitCode::from(1)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: lemans-verify <binary> <entry-address> <spec.json> [--oracle PATH]");
        return ExitCode::from(2);
    }
    let binary = PathBuf::from(&args[0]);
    let Some(address) = parse_hex(&args[1]) else {
        return fail(&format!("bad address {:?}", args[1]));
    };
    let spec_path = PathBuf::from(&args[2]);
    let oracle_bin = args
        .iter()
        .position(|a| a == "--oracle")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../sh4-oracle/target/release/sh4-oracle"));

    let bytes = match std::fs::read(&binary) {
        Ok(b) => b,
        Err(e) => return fail(&format!("cannot read {}: {e}", binary.display())),
    };
    let text = match std::fs::read_to_string(&spec_path) {
        Ok(t) => t,
        Err(e) => return fail(&format!("cannot read {}: {e}", spec_path.display())),
    };
    let spec = match json::parse(&text) {
        Ok(v) => v,
        Err(e) => return fail(&format!("bad spec JSON: {e}")),
    };
    let seed = read_seed(&spec, LOAD_BASE);

    let Some(port) = registry::lookup(address) else {
        return fail(&format!("no port registered for 0x{address:08x}"));
    };

    // Oracle side: run the original bytes in a separate process.
    let output = match Command::new(&oracle_bin)
        .arg(&binary).arg("--spec").arg(&spec_path)
        .output()
    {
        Ok(o) => o,
        Err(e) => return fail(&format!("cannot run oracle {}: {e}", oracle_bin.display())),
    };
    if !output.status.success() {
        return fail(&format!(
            "oracle exited {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let oracle_json = String::from_utf8_lossy(&output.stdout).to_string();
    let oracle = match json::parse(&oracle_json) {
        Ok(v) => v,
        Err(e) => return fail(&format!("bad oracle JSON: {e}")),
    };
    match oracle.get("stop").and_then(Json::as_str) {
        Some("halt") => {}
        other => {
            let detail = oracle.get("error").and_then(Json::as_str).unwrap_or("");
            return fail(&format!("oracle did not halt cleanly: {other:?} {detail}"));
        }
    }
    let Some(Json::Array(oracle_regs)) = oracle.get("regs") else {
        return fail("oracle output has no regs array");
    };

    // Port side: same seed, hand-written implementation.
    let (mut cpu, mut mem) = seeded(&seed, &bytes, LOAD_BASE);
    install_default_stack(&mut cpu, &mut mem);
    mem.writes.clear();
    port(&mut cpu, &mut mem);
    mem.writes.retain(|(a, _)| !in_stack(*a));

    let mut diffs = Vec::new();
    for (i, expected) in oracle_regs.iter().enumerate() {
        let want = expected.as_u32().unwrap_or(0);
        if cpu.r[i] != want {
            diffs.push(format!("r{i}: port 0x{:08x} != original 0x{want:08x}", cpu.r[i]));
        }
    }
    for key in SPECIAL {
        let Some(want) = oracle.get(key).and_then(Json::as_u32) else {
            continue;
        };
        let got = match key {
            "sr" => cpu.sr,
            "mach" => cpu.mach,
            "macl" => cpu.macl,
            "gbr" => cpu.gbr,
            "vbr" => cpu.vbr,
            "ssr" => cpu.ssr,
            "spc" => cpu.spc,
            "sgr" => cpu.sgr,
            "dbr" => cpu.dbr,
            "fpul" => cpu.fpul,
            "fpscr" => cpu.fpscr,
            _ => 0,
        };
        if got != want {
            if key == "sr" {
                diffs.push(format!(
                    "sr: port 0x{got:08x} != original 0x{want:08x} (T {} vs {})",
                    got & 1,
                    want & 1
                ));
            } else {
                diffs.push(format!("{key}: port 0x{got:08x} != original 0x{want:08x}"));
            }
        }
    }
    if let Some(Json::Array(oracle_fr)) = oracle.get("fr") {
        for (i, expected) in oracle_fr.iter().enumerate().take(16) {
            // The oracle emits raw bit patterns so infinities and NaNs survive
            // JSON, which has no representation for them.
            let want = expected.as_u32().unwrap_or(0);
            if cpu.fr[i].to_bits() != want {
                diffs.push(format!(
                    "fr{i}: port 0x{:08x} ({}) != original 0x{want:08x}",
                    cpu.fr[i].to_bits(),
                    cpu.fr[i]
                ));
            }
        }
    }
    if let Some(Json::Array(oracle_writes)) = oracle.get("writes") {
        let mut want: Vec<(u32, u8)> = Vec::new();
        for row in oracle_writes {
            if let Json::Array(pair) = row {
                if pair.len() == 2 {
                    if let (Some(a), Some(b)) = (pair[0].as_u32(), pair[1].as_u64()) {
                        want.push((a, b as u8));
                    }
                }
            }
        }
        want.retain(|(a, _)| !in_stack(*a));
        if want != mem.writes {
            diffs.push(format!(
                "writes: port {} entries, original {} entries",
                mem.writes.len(),
                want.len()
            ));
            for (i, (w, p)) in want.iter().zip(mem.writes.iter()).enumerate() {
                if w != p {
                    diffs.push(format!("  first write diff #{i}: original {w:?} port {p:?}"));
                    break;
                }
            }
        }
    }

    if diffs.is_empty() {
        let steps = oracle.get("steps").and_then(Json::as_u64).unwrap_or(0);
        println!("ok   0x{address:08x} matches original ({steps} oracle steps)");
        ExitCode::SUCCESS
    } else {
        println!("FAIL 0x{address:08x}: {} difference(s)", diffs.len());
        for line in diffs.iter().take(8) {
            println!("  {line}");
        }
        ExitCode::from(1)
    }
}
