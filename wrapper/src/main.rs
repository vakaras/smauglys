use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut log = File::create("C:\\smauglys_wrapper.log")?;
    writeln!(log, "Wrapper started.").unwrap();
    let current_exe = std::env::current_exe()?;
    writeln!(log, "current_exe={:?}", current_exe).unwrap();
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    writeln!(log, "code_original_exe={:?}", code_original_exe).unwrap();
    Ok(())
}
