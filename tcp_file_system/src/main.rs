use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-l")
        .output();
    // let a = output.stdout;
    let a = match output {
        Ok(out) => String::from_utf8(out.stdout).unwrap(),
        Err(_) => String::from("\"ls\" not executed due to some wrong."),
    };
    print!("{}", &a);
}
