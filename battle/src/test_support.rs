use std::path::Path;

/// Loads the workspace game-data snapshot once for tests that exercise config-backed rules.
pub(crate) fn init_config() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("battle crate must live under the workspace root")
        .join("data/excel2json");
    config::init(
        data_dir
            .to_str()
            .expect("workspace game-data path must be valid UTF-8"),
    )
    .expect("test game data must load");
}
