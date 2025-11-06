fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(
            &["proto/audit_event.proto", "proto/audit_control.proto", "proto/audit_query.proto", "proto/audit_crypto.proto", "proto/vector_api.proto"],
            &["proto"]?,
        )?;

    Ok(())
}
