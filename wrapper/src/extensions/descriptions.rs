use std::path::Path;

use super::Extension;

pub(crate) const PYTHON_EXTENSION: Extension = Extension {
    identifier: "ms-python.python",
    post_install: Some(python),
};

fn python(_vscode_exe: &Path) -> Result<(), String> {
    // TODO: A stub.
    Ok(())
}

pub(crate) const VSCODE_LANGUAGE_PACK_LT_EXTENSION: Extension = Extension {
    identifier: "vakaras.vscode-language-pack-lt",
    post_install: Some(vscode_language_pack_lt),
};

fn vscode_language_pack_lt(_vscode_exe: &Path) -> Result<(), String> {
    crate::code::set_locale("lt");
    Ok(())
}

pub(crate) const DEBUG_VISUALIZER_EXTENSION: Extension = Extension {
    identifier: "hediet.debug-visualizer",
    post_install: None,
};


