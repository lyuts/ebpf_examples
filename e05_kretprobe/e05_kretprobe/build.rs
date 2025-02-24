use anyhow::{anyhow, Context as _};
use aya_build::cargo_metadata;

fn main() -> anyhow::Result<()> {
    let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;
    let ebpf_package = packages
        .into_iter()
        .find(|cargo_metadata::Package { name, .. }| name == "e05_kretprobe-ebpf")
        .ok_or_else(|| anyhow!("e05_kretprobe-ebpf package not found"))?;
    aya_build::build_ebpf([ebpf_package])
}
