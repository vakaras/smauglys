use log::{debug, trace};
use std::path::PathBuf;

use crate::{command::extract_file, error::IResult};

pub(crate) fn ensure_wrapper() -> IResult {
    trace!("[enter] ensure_wrapper()");
    let mut vs_code_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    vs_code_path.push("VSCodium");
    debug!("VS Code directory={:?}", vs_code_path);
    let vs_code_exe_path = vs_code_path.join("code.exe");
    debug!("vs_code_exe_path={:?}", vs_code_exe_path);
    let vs_code_original_path = vs_code_path.join("code_original.exe");
    debug!("vs_code_original_path={:?}", vs_code_original_path);
    debug!(
        "renaming: {:?} -> {:?}",
        &vs_code_exe_path, &vs_code_original_path
    );
    std::fs::rename(&vs_code_exe_path, &vs_code_original_path)?;
    debug!("creating wrapper at {:?}", &vs_code_exe_path);
    extract_file(crate::WRAPPER_BIN, &vs_code_exe_path)?;
    trace!("[exit] ensure_wrapper");
    Ok(())
}
