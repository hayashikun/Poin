use std::fs;
use std::path::Path;

const PROTO_OUT_DIR: &str = "./src/proto";

fn tree(path: &Path, postfix: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        let meta = fs::metadata(&child)?;
        if meta.is_dir() {
            files.extend(tree(child.as_path(), postfix)?);
            continue;
        }
        let name = child
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        if name.ends_with(postfix) {
            files.push(child.into_os_string().into_string().unwrap());
        }
    }
    return Ok(files);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qoin_dir = "../qoin".to_string();

    fs::remove_dir_all(PROTO_OUT_DIR)?;
    fs::create_dir(PROTO_OUT_DIR).expect("Failed to create out dir");
    fs::write(
        format!("{}/mod.rs", PROTO_OUT_DIR),
        "pub mod mediapipe;\npub mod qoin;",
    )?;

    let qoin_proto_dir = format!("{}/qoin/proto", qoin_dir);
    let mediapipe_dir = Path::new(format!("{}/bazel-qoin/external/mediapipe", qoin_dir).as_str())
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let mediapipe_proto_dir = format!("{}/mediapipe/framework/formats", mediapipe_dir);

    let mut files = Vec::new();
    files.extend(tree(&Path::new(qoin_proto_dir.as_str()), ".proto")?);
    files.extend(tree(&Path::new(mediapipe_proto_dir.as_str()), ".proto")?);

    let includes = vec![qoin_dir, mediapipe_dir];

    tonic_build::configure()
        .build_server(false)
        .out_dir(PROTO_OUT_DIR)
        .compile(&files, &includes)?;

    Ok(())
}
