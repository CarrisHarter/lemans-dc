# Step 5 (of the pipeline): open the target through rea

Prerequisites: patched rea registered as MCP server `rea`, and
`GHIDRA_INSTALL_DIR` / `JAVA_HOME` set in its server env (already done in
`~/.pi/agent/mcp.json`).

## Through MCP (agent path)

```
open_binary {
  path: "/home/holden/Projects/lemans-dc/analysis/1st_read.bin",
  format: "raw-binary",
  architecture: "sh4",
  entry_point: "0x8C010100",
  provider_id: "ghidra"
}
```

Then `list_procedures`, `search_strings`, `procedure_pseudo_code`,
`procedure_references`, `decompile_all {output_dir}`.

Cold Ghidra start can take minutes on a multi-MB image; the MCP entry allows
900 s per request.

## Through the CLI (same fork)

```bash
cd ~/Projects/rea-sh4
export GHIDRA_INSTALL_DIR=~/opt/ghidra_12.1.4_PUBLIC
export JAVA_HOME=~/opt/jdk-21.0.13+9
node scripts/rea.mjs inspect /home/holden/Projects/lemans-dc/analysis/1st_read.bin \
  --target-format raw-binary --architecture sh4 --entry-point 0x8C010100 \
  --provider ghidra --json
```

## Known limit

`entry_point` is where file offset 0 is mapped, and it also becomes the seeded
`entry` function. Because a Katana binary keeps its 0x100-byte header at offset
0, prefer a header-stripped image mapped at `0x8C010100`; otherwise seed the real
code entry separately (`REA_GHIDRA_SEED_FILE`, set in the server env and applied
at import time).
