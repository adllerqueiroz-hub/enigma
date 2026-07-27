use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::{collections::HashSet, fs, path::Path};
use walkdir::WalkDir;

use crate::excel_confgen::{
    emit_file::emit_table_file, emit_root::emit_root_module, tables::FILTER_TABLES,
};

fn write_if_changed(path: &Path, content: &str) -> Result<()> {
    if let Ok(existing) = fs::read_to_string(path)
        && existing == content
    {
        return Ok(());
    }
    fs::write(path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn remove_stale_modules(output: &Path, active: &HashSet<&str>) -> Result<()> {
    for entry in fs::read_dir(output).with_context(|| format!("read_dir {}", output.display()))? {
        let path = entry?.path();
        let Some(stem) = path.file_stem().and_then(|value| value.to_str()) else {
            continue;
        };
        if path.extension().and_then(|value| value.to_str()) != Some("rs")
            || stem == "mod"
            || active.contains(stem)
        {
            continue;
        }
        let content =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let mut lines = content.lines();
        if lines.next() == Some("// Auto-generated from JSON data")
            && lines.next() == Some("// Do not edit manually")
        {
            fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
        }
    }
    Ok(())
}

pub fn generate_rust_modules(json_dir: &str, output_dir: &str) -> Result<()> {
    let output = Path::new(output_dir);
    fs::create_dir_all(output).with_context(|| format!("create_dir_all {}", output.display()))?;

    let filter: HashSet<&str> = FILTER_TABLES.iter().copied().collect();
    let mut tables = Vec::new();

    for entry in WalkDir::new(json_dir).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }

        let raw = entry
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid filename"))?;

        if !filter.contains(raw) {
            continue;
        }

        let snake = raw.to_string();
        tables.push(snake.clone());

        let json = fs::read_to_string(entry.path())
            .with_context(|| format!("read {}", entry.path().display()))?;
        let data: Value = serde_json::from_str(&json)?;

        let (table, records) = match data {
            Value::Array(ref a) if a.len() == 2 => (
                a[0].as_str().unwrap_or(raw).to_string(),
                a[1].as_array().cloned().unwrap_or_default(),
            ),
            Value::Array(a) => (raw.to_string(), a),
            _ => (raw.to_string(), vec![data]),
        };

        let out_file = output.join(format!("{}.rs", snake));
        let content = emit_table_file(&table, &records);
        write_if_changed(&out_file, &content)?;
    }

    tables.sort();
    remove_stale_modules(output, &tables.iter().map(String::as_str).collect())?;
    let mod_file = output.join("mod.rs");
    let mod_content = emit_root_module(&tables);
    write_if_changed(&mod_file, &mod_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn removes_only_stale_generated_modules() {
        let root = std::env::temp_dir().join(format!(
            "enigma_config_codegen_{}_{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let input = root.join("input");
        let output = root.join("output");
        fs::create_dir_all(&input).unwrap();
        fs::create_dir_all(&output).unwrap();
        fs::write(input.join("character.json"), "[\"character\",[]]").unwrap();
        fs::write(
            output.join("stale.rs"),
            "// Auto-generated from JSON data\r\n// Do not edit manually\r\n",
        )
        .unwrap();
        fs::write(output.join("manual.rs"), "// maintained by hand\n").unwrap();

        generate_rust_modules(input.to_str().unwrap(), output.to_str().unwrap()).unwrap();

        assert!(!output.join("stale.rs").exists());
        assert!(output.join("manual.rs").exists());
        assert!(output.join("character.rs").exists());
        fs::remove_dir_all(root).unwrap();
    }
}
