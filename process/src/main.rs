use std::process::Command;

fn childprocess()
{
    println!("---child process---");

    let output = Command::new("rustc").arg("--version")
                .output().unwrap_or_else(|e|{ 
                    panic!("failed to execute process {}", e)
                } 
    );

    if output.status.success(){
        let s = String::from_utf8_lossy(&output.stdout);
        println!("succeeded stdout={}", s)
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("failed stderr={}", s)
    }
}

fn main() {
    println!("---Start---");

    childprocess();

    println!("---End---");
}
