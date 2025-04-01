use std::{fmt::Debug, process::{Command, Stdio}};

pub struct MinecraftServerBuilder {
    server_path: Option<String>,
    server_jar: Option<String>,
    java_path: Option<String>,
    java_args: Option<Vec<String>>,
    gui: Option<bool>,
}

impl MinecraftServerBuilder {
    pub fn new() -> Self {
        MinecraftServerBuilder {
            server_path: None,
            server_jar: None,
            java_path: None,
            java_args: None,
            gui: None,
        }
    }

    pub fn server_path<T: Into<String>>(mut self, path: T) -> Self {
        self.server_path = Some(path.into());
        self
    }

    pub fn server_jar<T: Into<String>>(mut self, jar: T) -> Self {
        self.server_jar = Some(jar.into());
        self
    }

    pub fn java_path<T: Into<String>>(mut self, path: T) -> Self {
        self.java_path = Some(path.into());
        self
    }

    pub fn java_args<T: Into<String> + Clone>(mut self, args: &[T]) -> Self {
        self.java_args = Some(args.iter().map(|s| s.clone().into()).collect());
        self
    }

    pub fn gui(mut self, gui: bool) -> Self {
        self.gui = Some(gui);
        self
    }
    
    pub fn build(self) -> Result<MinecraftServer, MinecraftServerBuildError> {
        let server_path = self.server_path.ok_or(MinecraftServerBuildError::MissingServerPath)?;
        let server_jar = self.server_jar.ok_or(MinecraftServerBuildError::MissingServerJar)?;
        
        if !std::path::Path::new(&server_path).exists() {
            return Err(MinecraftServerBuildError::InvalidServerPath(server_path));
        }
        
        let java_path = self.java_path.unwrap_or("java".to_string());
        if Command::new(&java_path).arg("--version").output().is_err() {
            return Err(MinecraftServerBuildError::InvalidJavaPath(java_path));
        }

        Ok(MinecraftServer {
            server_path,
            server_jar,
            java_path,
            java_args: self.java_args.unwrap_or_default(),
            gui: self.gui.unwrap_or(false),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MinecraftServerBuildError {
    #[error("server path is missing")]
    MissingServerPath,
    #[error("server jar file is missing")]
    MissingServerJar,
    #[error("invalid server path: {0}")]
    InvalidServerPath(String),
    #[error("invalid Java path: {0}")]
    InvalidJavaPath(String),
    #[error("failed to execute command: {0}")]
    CommandExecutionError(#[from] std::io::Error),
}

pub struct MinecraftServer {
    pub server_path: String,
    pub server_jar: String,
    pub java_path: String,
    pub java_args: Vec<String>,
    pub gui: bool,
}

impl MinecraftServer {
    pub fn new<T: Into<String> + Clone>(server_path: T, server_jar: T, java_path: T, java_args: &[T], gui: bool) -> Self {
        MinecraftServer {
            server_path: server_path.into(),
            server_jar: server_jar.into(),
            java_path: java_path.into(),
            java_args: java_args.iter().map(|s| s.clone().into()).collect(),
            gui,
        }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let mut server = self.get_command()
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
        let _ = server.wait()?;
        Ok(())
    }

    fn get_command(&self) -> Command {
        let mut command = Command::new(&self.java_path);
        command
            .args(self.java_args.clone())
            .arg("-jar")
            .arg(&self.server_jar)
            .current_dir(&self.server_path);

            if !self.gui {
                command.arg("--nogui");
            }
        command
    }
}