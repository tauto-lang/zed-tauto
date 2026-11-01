use zed_extension_api::{self as zed, LanguageServerId, Result};

struct TautoExtension;

impl zed::Extension for TautoExtension {
    fn new() -> Self {
        TautoExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = worktree
            .which("tauto")
            .ok_or_else(|| "`tauto` was not found on PATH".to_string())?;
        Ok(zed::Command {
            command,
            args: vec!["lsp".into()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(TautoExtension);
