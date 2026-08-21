use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    tauri_build::build();

    let migrations_dir = PathBuf::from("src/db/migrations");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=src/db/migrations");

    let mut entries: Vec<(u32, PathBuf)> = fs::read_dir(&migrations_dir)
        .expect("migrations directory not found")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            let stem = path.file_stem()?.to_str()?.to_string();
            let version_str = stem.split('_').next()?;
            let version = version_str.parse::<u32>().ok()?;
            Some((version, path))
        })
        .collect();

    entries.sort_by_key(|(v, _)| *v);

    let mut code = String::from("pub const MIGRATIONS: &[(u32, &str)] = &[\n");
    for (version, path) in &entries {
        let abs = fs::canonicalize(path).unwrap();
        code.push_str(&format!(
            "    ({}, include_str!(\"{}\")),\n",
            version,
            abs.display()
        ));
        println!("cargo:rerun-if-changed={}", abs.display());
    }
    code.push_str("];\n");

    fs::write(out_dir.join("migrations.rs"), code).unwrap();
}
