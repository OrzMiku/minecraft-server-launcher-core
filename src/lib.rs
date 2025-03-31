use std::{io::{BufRead, BufReader, BufWriter, Write}, process::{Command, Stdio}};

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
            java_path: Some("java".to_string()),
            java_args: None,
            gui: Some(false),
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
        if self.server_path.is_none() {
            return Err(MinecraftServerBuildError::MissingServerPath);
        } else if self.server_jar.is_none() {
            return Err(MinecraftServerBuildError::MissingServerJar);
        }
        Ok(MinecraftServer {
            server_path: self.server_path.unwrap(),
            server_jar: self.server_jar.unwrap(),
            java_path: self.java_path.unwrap(),
            java_args: self.java_args.unwrap_or_default(),
            gui: self.gui.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub enum MinecraftServerBuildError {
    MissingServerPath,
    MissingServerJar,
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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdin = server.stdin.take().unwrap();
        let stdout = server.stdout.take().unwrap();
        let stderr = server.stderr.take().unwrap();

        let stdout_thread = std::thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => eprintln!("Error reading stdout: {}", e),
                }
            }
        });

        let stderr_thread = std::thread::spawn(move || {
            let reader = std::io::BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(line) => eprintln!("{}", line),
                    Err(e) => eprintln!("Error reading stderr: {}", e),
                }
            }
        });

        let stdin_thread = std::thread::spawn(move || {
            let mut writer = BufWriter::new(stdin);
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                writer.write_all(input.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        });

        stdout_thread.join().unwrap();
        stderr_thread.join().unwrap();
        stdin_thread.join().unwrap();
        let _ = server.wait().unwrap();
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