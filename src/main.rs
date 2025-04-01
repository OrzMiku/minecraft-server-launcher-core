use mslc::{MinecraftServerBuilder, MinecraftServerBuildError};

fn main() -> Result<(), MinecraftServerBuildError> {
    let mut server = MinecraftServerBuilder::new()
        .java_path("invalid_java_path")
        .server_path("/home/orzmiku/mcserver/")
        .server_jar("server.jar")
        // .java_args(&["-Xmx1024M", "-Xms1024M"])
        .build()?;
    
    server.run()?;
    Ok(())
}