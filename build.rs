fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "../qoin/qoin/proto/hello.proto",
            "../qoin/qoin/proto/landmark.proto",
            "../qoin/qoin/proto/face_mesh.proto",
            "../qoin/qoin/proto/hand_tracking.proto",
        ],
        &["../qoin"],
    )?;
    Ok(())
}
