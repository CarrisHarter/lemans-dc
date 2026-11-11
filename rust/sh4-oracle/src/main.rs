//! Run an SH-4 image (or a slice of one) and print architectural state.
//!
//! The oracle exists to answer one question per function: does the Rust
//! rewrite compute the same registers and memory? This tiny CLI makes that
//! scriptable from the spec runner and usable by hand.

use std::path::PathBuf;
use std::process::ExitCode;

use sh4_oracle::json::{self, Json};
use sh4_oracle::{image, install_default_stack, Cpu, Memory, Stop, LOAD_BASE};

fn parse_hex(s: &str) -> Result<u32, String> {
    let t = s.trim();
    let t = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u32::from_str_radix(t, 16).map_err(|e| format!("bad hex {s:?}: {e}"))
}

fn usage() -> ExitCode {
    eprintln!(
        "usage: sh4-oracle <image> [--base 0x8C010000] [--entry 0x...] \
[--steps N] [--set rN=0xVALUE | --set fN=FLOAT] [--dump-args]"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next().filter(|a| a != "-h" && a != "--help") else {
        return usage();
    };
    let path = PathBuf::from(path);

    let mut spec_path: Option<PathBuf> = None;
    let mut base = LOAD_BASE;
    let mut entry = LOAD_BASE;
    let mut steps = 1_000_000usize;
    let mut sets: Vec<String> = Vec::new();

    while let Some(flag) = args.next() {
        let mut take = |name: &str| -> Result<String, ExitCode> {
            args.next().ok_or_else(|| {
                eprintln!("{name} needs a value");
                ExitCode::from(2)
            })
        };
        let r = match flag.as_str() {
            "--base" => take("--base").and_then(|v| parse_hex(&v).map(|v| base = v).map_err(|_| ExitCode::from(2))),
            "--entry" => take("--entry").and_then(|v| parse_hex(&v).map(|v| entry = v).map_err(|_| ExitCode::from(2))),
            "--steps" => take("--steps").and_then(|v| {
                v.parse::<usize>()
                    .map(|v| steps = v)
                    .map_err(|_| ExitCode::from(2))
            }),
            "--set" => take("--set").map(|v| sets.push(v)),
            "--spec" => take("--spec").map(|v| spec_path = Some(PathBuf::from(v))),
            other => {
                eprintln!("unknown flag {other}");
                return usage();
            }
        };
        if let Err(code) = r {
            return code;
        }
    }

    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {}: {e}", path.display());
            return ExitCode::from(1);
        }
    };

    let mut mem = image(base, &bytes);
    let mut cpu = Cpu::new();
    cpu.pc = entry;

    if let Some(path) = spec_path {
        return run_spec(&path, &mut cpu, &mut mem, base);
    }

    for spec in sets {
        let (reg, value) = match spec.split_once('=') {
            Some(p) => p,
            None => {
                eprintln!("--set expects rN=VALUE or fN=VALUE, got {spec:?}");
                return ExitCode::from(2);
            }
        };
        let (kind, idx) = reg.split_at(1);
        let Ok(idx) = idx.parse::<usize>() else {
            eprintln!("bad register {reg:?}");
            return ExitCode::from(2);
        };
        if idx > 15 {
            eprintln!("register index out of range: {reg:?}");
            return ExitCode::from(2);
        }
        match kind {
            "r" => match parse_hex(value) {
                Ok(v) => cpu.r[idx] = v,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(2);
                }
            },
            "f" => match value.parse::<f32>() {
                Ok(v) => cpu.fr[idx] = v,
                Err(e) => {
                    eprintln!("bad float {value:?}: {e}");
                    return ExitCode::from(2);
                }
            },
            _ => {
                eprintln!("register must start with r or f: {reg:?}");
                return ExitCode::from(2);
            }
        }
    }

    let stop = cpu.run(&mut mem, steps);
    println!("image      {} ({} bytes) at 0x{base:08x}", path.display(), bytes.len());
    println!("entry      0x{entry:08x}");
    match &stop {
        Ok(Stop::Halt) => println!("stop       halt (rts -> HALT_PC)"),
        Ok(Stop::Steps(n)) => println!("stop       step limit {n}"),
        Err(e) => println!("error      {e}"),
    }
    println!("steps      {}", cpu.steps);
    for row in 0..4 {
        let mut line = String::new();
        for col in 0..4 {
            let i = row * 4 + col;
            line.push_str(&format!("r{i:<2}=0x{:08x}  ", cpu.r[i]));
        }
        println!("{line}");
    }
    println!(
        "pr=0x{:08x} sr=0x{:08x} t={} pc=0x{:08x}",
        cpu.pr,
        cpu.sr,
        cpu.t(),
        cpu.pc
    );
    for row in 0..2 {
        let mut line = String::new();
        for col in 0..4 {
            let i = row * 4 + col;
            line.push_str(&format!("fr{i:<2}={:<12}  ", cpu.fr[i]));
        }
        println!("{line}");
    }
    println!("writes      {}", mem.writes.len());

    match stop {
        Ok(Stop::Halt) => ExitCode::SUCCESS,
        Ok(Stop::Steps(_)) => ExitCode::from(3),
        Err(_) => ExitCode::from(1),
    }
}

/// Machine-readable oracle run: everything `lemans-verify` needs to diff.
fn run_spec(path: &PathBuf, cpu: &mut Cpu, mem: &mut Memory, base: u32) -> ExitCode {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read spec {}: {e}", path.display());
            return ExitCode::from(1);
        }
    };
    let spec = match json::parse(&text) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("bad spec JSON: {e}");
            return ExitCode::from(1);
        }
    };

    let entry = spec
        .get("entry")
        .and_then(Json::as_u32)
        .unwrap_or(base);
    let steps = spec
        .get("steps")
        .and_then(Json::as_u64)
        .unwrap_or(1_000_000) as usize;

    if let Some(regs) = spec.get("regs").and_then(|v| match v {
        Json::Object(m) => Some(m),
        _ => None,
    }) {
        for (key, value) in regs {
            if let Some(idx) = key.strip_prefix('r').and_then(|i| i.parse::<usize>().ok()) {
                if idx < 16 {
                    if let Some(v) = value.as_u32() {
                        cpu.r[idx] = v;
                    }
                }
            }
        }
    }
    if let Some(fr) = spec.get("fr").and_then(|v| match v {
        Json::Object(m) => Some(m),
        _ => None,
    }) {
        for (key, value) in fr {
            if let Some(idx) = key.strip_prefix('f').and_then(|i| i.parse::<usize>().ok()) {
                if idx < 16 {
                    if let Some(v) = value.as_f32() {
                        cpu.fr[idx] = v;
                    }
                }
            }
        }
    }
    if let Some(regions) = spec.get("memory").and_then(Json::as_array) {
        for region in regions {
            let addr = region.get("addr").and_then(Json::as_u32);
            let hex = region.get("hex").and_then(Json::as_str);
            if let (Some(addr), Some(hex)) = (addr, hex) {
                let bytes: Vec<u8> = (0..hex.len() / 2)
                    .filter_map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok())
                    .collect();
                for (i, byte) in bytes.iter().enumerate() {
                    let _ = mem.write_u8(addr + i as u32, *byte);
                }
            }
        }
    }

    install_default_stack(cpu, mem);
    mem.writes.clear(); // seeding is not an effect of the function
    cpu.pc = entry;
    let stop = if std::env::var("SH4_TRACE").is_ok() {
        let mut result = Ok(Stop::Steps(steps));
        for _ in 0..steps.min(64) {
            if cpu.pc == sh4_oracle::HALT_PC { result = Ok(Stop::Halt); break; }
            eprintln!("pc=0x{:08x} op=0x{:04x} r15=0x{:08x} pr=0x{:08x}",
                cpu.pc, mem.read_u16(cpu.pc).unwrap_or(0), cpu.r[15], cpu.pr);
            if let Err(e) = cpu.step(mem) { result = Err(e); break; }
        }
        result
    } else {
        cpu.run(mem, steps)
    };

    let mut out = std::collections::BTreeMap::new();
    out.insert(
        "stop".to_string(),
        Json::String(match &stop {
            Ok(Stop::Halt) => "halt".into(),
            Ok(Stop::Steps(_)) => "limit".into(),
            Err(_) => "error".into(),
        }),
    );
    out.insert("steps".to_string(), Json::Number(cpu.steps as f64));
    if let Err(e) = &stop {
        out.insert("error".to_string(), Json::String(e.to_string()));
    }
    out.insert(
        "regs".to_string(),
        Json::Array(
            cpu.r
                .iter()
                .map(|v| Json::String(format!("0x{v:08x}")))
                .collect(),
        ),
    );
    out.insert(
        "fr".to_string(),
        Json::Array(cpu.fr.iter().map(|v| Json::Number(*v as f64)).collect()),
    );
    out.insert("fpul".to_string(), Json::String(format!("0x{:08x}", cpu.fpul)));
    out.insert("sr".to_string(), Json::String(format!("0x{:08x}", cpu.sr)));
    out.insert(
        "writes".to_string(),
        Json::Array(
            mem.writes
                .iter()
                .map(|(a, b)| {
                    Json::Array(vec![
                        Json::String(format!("0x{a:08x}")),
                        Json::Number(*b as f64),
                    ])
                })
                .collect(),
        ),
    );

    println!("{}", Json::Object(out));
    match stop {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::from(1),
    }
}
