use mslc::MinecraftServer;

fn main() {
    // Hardcoded values for testing
    let mut server = MinecraftServer::new(
        "/home/orzmiku/mcserver/",
        "fabric-server-mc.1.21.5-loader.0.16.10-launcher.1.0.3.jar",
        "java",
        &["-Xmx1024M", "-Xms1024M"],
        true,
    );
    server.run().unwrap();
}