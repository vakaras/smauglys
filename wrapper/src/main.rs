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
    let home_dir = dirs::home_dir().unwrap();
    writeln!(log, "home_dir={:?}", home_dir).unwrap();
    let mut vs_code_extensions_dir = home_dir;
    vs_code_extensions_dir.push(".vscode");
    vs_code_extensions_dir.push(".extensions");
    writeln!(log, "vs_code_extensions_dir={:?}", vs_code_extensions_dir).unwrap();
    let extensions_pattern = vs_code_extensions_dir.join("*");
    writeln!(log, "extensions_pattern={:?}", extensions_pattern).unwrap();

    for entry in
        glob::glob(extensions_pattern.to_str().unwrap()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => writeln!(log, "{:?}", path.display())?,
            Err(e) => writeln!(log, "{:?}", e)?,
        }
    }
    let package_json_pattern = extensions_pattern.join("package.json");
    writeln!(log, "extensions_pattern={:?}", package_json_pattern).unwrap();
    for entry in
        glob::glob(package_json_pattern.to_str().unwrap()).expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => writeln!(log, "{:?}", path.display())?,
            Err(e) => writeln!(log, "{:?}", e)?,
        }
    }
    Ok(())
}
