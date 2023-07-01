pub fn main() {
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .include("src")
        .input("src/version.proto")
        .cargo_out_dir("protos")
        .run_from_script();
}
