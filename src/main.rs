use mslc::MinecraftServerBuilder;

fn main() {
    // Hardcoded values for testing
    let mut server = MinecraftServerBuilder::new()
        .server_path("/home/orzmiku/mcserver/")
        .server_jar("server.jar")
        .java_args(&["-Xmx1024M", "-Xms1024M"])
        .build()
        .unwrap();
    server.run().unwrap();
}