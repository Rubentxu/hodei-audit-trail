fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .emit_rerun_if_changed(true)
        .compile_protos(
            &[
                "proto/common.proto",
                "proto/audit_event.proto",
                "proto/audit_control.proto",
                "proto/audit_query.proto",
                "proto/audit_crypto.proto",
                "proto/vector_api.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
