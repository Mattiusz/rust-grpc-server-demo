fn main() {
    let proto_file = "./protos/my_grpc_service.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
