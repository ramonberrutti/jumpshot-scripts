

fn main() {
    tonic_build::configure()
        .compile(&["proto/script/v1/script_service.proto"], &["proto"])
        .unwrap();
}