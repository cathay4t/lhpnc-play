// SPDX-License-Identifier: Apache-2.0

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // compiling protos using path on build time
    tonic_prost_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
