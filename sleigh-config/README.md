# Overview

This crate contains precompiled `.sla` and `.pspec` configuration files for use with
[libsla](https://crates.io/crates/libsla) or other crates that interface with the
[Ghidra](https://github.com/NationalSecurityAgency/ghidra) decompiler. The configuration files are
sourced from Ghidra version **11.4**.

## Processors

The following are the list of processor configurations available in Ghidra.

* 6502
* 68000
* 8048
* 8051
* 8085
* AARCH64
* ARM
* Atmel
* BPF
* CP1600
* CR16
* DATA
* Dalvik
* HCS08
* HCS12
* JVM
* Loongarch
* M16C
* M8C
* MC6800
* MCS96
* MIPS
* PA-RISC
* PIC
* PowerPC
* RISCV
* Sparc
* SuperH
* SuperH4
* TI_MSP430
* Toy
* V850
* Xtensa
* Z80
* eBPF
* tricore
* x86

# Example

## Processor x86

The available configuration files for the x86 processor are

```rust
sleigh_config::processor_x86::SLA_X86;
sleigh_config::processor_x86::SLA_X86_64;
sleigh_config::processor_x86::PSPEC_X86;
sleigh_config::processor_x86::PSPEC_X86_16;
sleigh_config::processor_x86::PSPEC_X86_16_REAL;
sleigh_config::processor_x86::PSPEC_X86_64;
sleigh_config::processor_x86::PSPEC_X86_64_COMPAT32;
```

## Cargo.toml

Specify the processor(s) you wish to use via feature flags.

```toml
sleigh-config = { version = "0.1", features = ["x86"] }
```

Then use a crate that uses these configuration files to interact with Ghidra Sleigh:

```rust
let sleigh = libsla::GhidraSleigh::builder()
    .processor_spec(sleigh_config::processor_x86::PSPEC_X86_64)?
    .build(sleigh_config::processor_x86::SLA_X86_64)?;

// Use Sleigh to e.g. disassemble instructions
let disassembly = sleigh.disassemble_native(&instruction_loader, address)?
```

# FAQ

## Why are the SLA configuration files not XML?

In Ghidra 11.1 the SLA configuration file format was changed to a binary format.

## Is there a way to discover configurations programmatically?

Yes. The global `SLA_DATA` and `PSPEC_DATA` arrays can be used to iterate over all configurations.
