use gadget_common::prelude::*;
use shell_sdk::prelude::*;

pub mod protocol;

fn main() {
    let output = crate::protocol::executor::run_shell_command!("docker run hello-world");
    println!("{output:?}");
    return;
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_process_command() {
        let output = crate::protocol::executor::run_shell_command!("echo Hello World!");
        println!("{output:?}");
        let output = crate::protocol::executor::run_shell_command!("ls", "-al");
        println!("{output:?}");
        let output = crate::protocol::executor::run_shell_command!("ls");
        println!("{output:?}");
    }
}