use std::io::Result;

pub fn main() -> Result<()> {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    prost_build::compile_protos(&["src/version.proto"], &["src/"])
}
