use mslc::MinecraftServerBuilder;

fn main() {
    // Hardcoded values for testing
    let mut server = MinecraftServerBuilder::new()
        .server_path("/home/orzmiku/mcserver/")
        .server_jar("fabric-server-mc.1.21.5-loader.0.16.10-launcher.1.0.3.jar")
        .java_args(&["-Xmx1024M", "-Xms1024M"])
        .build()
        .unwrap();
    server.run().unwrap();
}