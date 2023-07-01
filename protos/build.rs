pub fn main() {
    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .include("src")
        .input("src/build.proto")
        .input("src/workspace.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
