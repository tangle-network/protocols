#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! no_args_discrete {
    ($cmd:expr) => {{
        std::process::Command::new("sh")
            .arg("-c")
            .arg($cmd)
            .output()
            .expect(&format!("Failed to execute: {:?} - Input should be a valid command", $cmd))
    }};
}
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! no_args_discrete {
    ($cmd:expr) => {{
        std::process::Command::new("cmd")
            .arg("/C")
            .arg($cmd)
            .output()
            .expect(&format!("Failed to execute: {:?} - Input should be a valid command", $cmd))
    }};
}
#[cfg(target_family = "unix")]
#[macro_export]
macro_rules! no_args_continuous {
    ($cmd:expr) => {{
        async_process::Command::new("sh")
            .args("-c", $cmd)
            // .arg("-c")
            // .arg($cmd)
            .stdout(async_process::Stdio::piped())
            .spawn()
            .expect(&format!("Failed to execute: {:?} - Input should be a valid command", $cmd))
    }};
}
#[cfg(target_family = "windows")]
#[macro_export]
macro_rules! no_args_continuous {
    ($cmd:expr) => {{
        async_process::Command::new("cmd")
            .args(["/C", $cmd])
            // .arg("/C")
            // .arg($cmd)
            .stdout(async_process::Stdio::piped())
            .spawn()
            .expect(&format!("Failed to execute: {:?} - Input should be a valid command", $cmd))
    }};
}
#[macro_export]
macro_rules! run_discrete_command {
    ($cmd:expr) => {{
        let output = crate::no_args_discrete!($cmd);
        if !output.status.success() {
            eprintln!("The following command failed with {}: {}", output.status, $cmd);
        }
        std::str::from_utf8(&output.stdout).unwrap_or("Invalid shell output").to_owned()
    }};
    ($cmd:expr, $($args:expr),*) => {{
        let mut command = std::process::Command::new($cmd);
        $(
            let arguments = String::try_from($args).unwrap_or(format!("Argument {:?} is causing an error", $args)).split_whitespace().map(str::to_string).collect::<Vec<String>>();
            command.args(arguments);
        )*
        let output = command
            .output()
            .expect(&format!("Failed to execute: {} {:?}", $cmd, &[$($args),*]));
        if !output.status.success() {
            eprintln!("The following command failed with {}: {}", output.status, $cmd);
        }
        std::str::from_utf8(&output.stdout).unwrap_or("Invalid shell output").to_owned()
    }};
}
#[macro_export]
macro_rules! run_continuous_command {
    ($cmd:expr) => {{
        let mut child: async_process::Child = crate::no_args_continuous!($cmd);
        if child.stderr.is_some() {
            eprintln!("The following command failed with {:?}: {}", child.stderr.unwrap(), $cmd);
        }
        futures_lite::io::BufReader::new(child.stdout.take().unwrap()).lines()
    }};
    ($cmd:expr, $($args:expr),*) => {{
        let mut command = async_process::Command::new($cmd);
        $(
            command.args(String::try_from($args).unwrap_or(format!("Argument {:?} is causing an error", $args)).split_whitespace().map(str::to_string).collect::<Vec<String>>());
        )*
        let mut child = command
            .stdout(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to execute: {} {:?}", $cmd, &[$($args),*]));
        if child.stderr.is_some() {
            eprintln!("The following command failed with {:?}: {}", child.stderr.unwrap(), $cmd);
        }
        futures_lite::io::BufReader::new(child.stdout.take().unwrap()).lines()
    }};
}