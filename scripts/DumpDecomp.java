// Disassemble a raw-loaded SH-4 image, find functions, dump decompilation.
// Args (all optional): entry addresses as 0x...; if none given, only discovered
// functions from the sweep are reported.
// @category Analysis

import java.util.ArrayList;
import java.util.List;

import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.plugin.core.analysis.AutoAnalysisManager;
import ghidra.app.script.GhidraScript;
import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.program.model.address.Address;
import ghidra.program.model.address.AddressSet;
import ghidra.program.model.listing.Function;
import ghidra.program.model.listing.FunctionManager;
import ghidra.program.model.listing.Instruction;
import ghidra.program.model.listing.Listing;
import ghidra.program.model.mem.MemoryBlock;

public class DumpDecomp extends GhidraScript {

    @Override
    public void run() throws Exception {
        println("### language: " + currentProgram.getLanguageID());
        println("### imageBase: 0x" + currentProgram.getImageBase());

        for (MemoryBlock b : currentProgram.getMemory().getBlocks()) {
            println(String.format("### block %s 0x%s-0x%s init=%s",
                    b.getName(), b.getStart(), b.getEnd(), b.isInitialized()));
        }

        MemoryBlock block = currentProgram.getMemory().getBlocks()[0];
        Address start = block.getStart();
        Address end = block.getEnd();

        AddressSet whole = new AddressSet(start, end);
        long before = currentProgram.getListing().getNumInstructions();
        DisassembleCommand dc = new DisassembleCommand(whole, null, true);
        dc.applyTo(currentProgram, monitor);
        long after = currentProgram.getListing().getNumInstructions();
        println("### linear sweep: " + before + " -> " + after + " instructions");

        AutoAnalysisManager mgr = AutoAnalysisManager.getAnalysisManager(currentProgram);
        mgr.startAnalysis(monitor);

        for (String arg : getScriptArgs()) {
            Address a = toAddr(arg);
            if (currentProgram.getFunctionManager().getFunctionAt(a) == null) {
                createFunction(a, "entry_" + a);
            }
        }

        Listing listing = currentProgram.getListing();
        println("### first bytes disassembly");
        Instruction ins = listing.getInstructionAt(start);
        int shown = 0;
        while (ins != null && shown < 40) {
            println(String.format("  %s  %-30s", ins.getAddress(), ins.toString()));
            ins = ins.getNext();
            shown++;
        }

        DecompInterface ifc = new DecompInterface();
        ifc.toggleCCode(true);
        if (!ifc.openProgram(currentProgram)) {
            println("### decompiler open failed: " + ifc.getLastMessage());
            return;
        }
        FunctionManager fm = currentProgram.getFunctionManager();
        println("### function count: " + fm.getFunctionCount());

        List<Function> funcs = new ArrayList<>();
        for (Function f : fm.getFunctions(true)) {
            funcs.add(f);
        }
        int n = 0;
        for (Function f : funcs) {
            if (n++ >= 12) {
                break;
            }
            println("");
            println("=== " + f.getName() + " @ " + f.getEntryPoint() + " ===");
            DecompileResults res = ifc.decompileFunction(f, 120, monitor);
            if (res.decompileCompleted()) {
                println(res.getDecompiledFunction().getC());
            } else {
                println("DECOMPILE FAILED: " + res.getErrorMessage());
            }
        }
        ifc.dispose();
        println("### done");
    }
}
