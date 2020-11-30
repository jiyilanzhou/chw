extern crate protoc_rust_grpc;
// 生成相应的 xx.rs 及 xx_grpc.rs 文件
fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src")
        .input("foobar.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}