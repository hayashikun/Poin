use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = "../qoin/qoin/proto";
    let files: Vec<String> = fs::read_dir(proto_dir)?
        .map(|p| {
            p.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
        })
        .filter(|f| f.ends_with(".proto"))
        .map(|f| format!("{}/{}", proto_dir, f))
        .collect();
    let includes: Vec<String> = vec!["../qoin"].iter().map(|s| s.to_string()).collect();
    println!("{:?}", files);
    tonic_build::configure()
        .build_server(false)
        .compile(&files, &includes)?;
    Ok(())
}
