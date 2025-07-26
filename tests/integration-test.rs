use libsla::GhidraSleigh;
use sleigh_processors::processor_x86;

#[test]
fn run_x86_64_libsla() -> Result<(), Box<dyn std::error::Error>> {
    let sleigh_spec = processor_x86::SLA_X86_64;
    let processor_spec = processor_x86::PSPEC_X86_64;
    GhidraSleigh::builder()
        .sleigh_spec(sleigh_spec)?
        .processor_spec(processor_spec)?
        .build()?;
    Ok(())
}
