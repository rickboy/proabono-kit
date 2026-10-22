//! `proabono`: build the component registry and install components into a
//! Proa site.
//!
//! Registry items use Proa UI's `ui-component-manifest-v1` format, so the same
//! output works with `proa ui add` if Proa ever accepts custom registries.
//! Until then this tool does the install: resolve dependencies, verify every
//! file's SHA-256, write sources into the site, register modules, and record
//! what it did in `proabono.lock.json`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const DEFAULT_REGISTRY: &str = "https://rickboy.github.io/proabono-kit/r";
const REGISTRY_ALIAS: &str = "@proabono";
const MANIFEST_SCHEMA: &str = "https://proa.so/schema/ui-component-manifest-v1.json";
const ASSET_DIR: &str = "proabono";
const LOCK_FILE: &str = "proabono.lock.json";

#[derive(Parser)]
#[command(name = "proabono", version, about = "Free, source-owned components for Proa sites")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Install components (and their dependencies) into a Proa site.
    Add {
        /// Item names, e.g. `dropdown-menu combobox`.
        #[arg(required = true)]
        items: Vec<String>,
        /// Registry base URL or local directory containing `<name>.json`.
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        /// Proa site root (searched upward for proa.config.json).
        #[arg(long, default_value = ".")]
        root: PathBuf,
        /// Overwrite files that differ from the registry version.
        #[arg(long)]
        force: bool,
        /// Resolve and verify everything without writing.
        #[arg(long)]
        dry_run: bool,
    },
    /// List the items a registry offers.
    List {
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
    },
    /// Build `registry/` sources into a static registry directory.
    Build {
        #[arg(long, default_value = "registry")]
        source: PathBuf,
        #[arg(long, default_value = "dist/r")]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Add {
            items,
            registry,
            root,
            force,
            dry_run,
        } => add(&items, &registry, &root, force, dry_run),
        Command::List { registry } => list(&registry),
        Command::Build { source, out } => build(&source, &out),
    }
}

// ---------------------------------------------------------------------------
// Manifest types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    #[serde(rename = "$schema")]
    schema: String,
    schema_version: u32,
    name: String,
    version: String,
    registry: String,
    kind: String,
    tier: String,
    license: String,
    dependencies: Vec<String>,
    #[serde(default)]
    rust_modules: Vec<RustModules>,
    #[serde(default)]
    cargo_dependencies: Vec<serde_json::Value>,
    files: Vec<ManifestFile>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RustModules {
    target: String,
    modules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ManifestFile {
    source: String,
    target: String,
    sha256: String,
}

/// `registry/<name>/item.json`, the authoring format.
#[derive(Debug, Deserialize)]
struct ItemSource {
    name: String,
    module: String,
    description: String,
    #[serde(default)]
    dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Index {
    registry: String,
    items: Vec<IndexItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexItem {
    name: String,
    description: String,
    version: String,
    dependencies: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Lock {
    registry: String,
    items: BTreeMap<String, LockItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockItem {
    version: String,
    files: BTreeMap<String, String>,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

// ---------------------------------------------------------------------------
// build
// ---------------------------------------------------------------------------

fn build(source: &Path, out: &Path) -> Result<()> {
    let mut dirs: Vec<PathBuf> = fs::read_dir(source)
        .with_context(|| format!("reading {}", source.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.join("item.json").is_file())
        .collect();
    dirs.sort();

    if out.exists() {
        fs::remove_dir_all(out).with_context(|| format!("clearing {}", out.display()))?;
    }
    fs::create_dir_all(out)?;

    let mut index = Index {
        registry: REGISTRY_ALIAS.into(),
        items: Vec::new(),
    };

    for dir in dirs {
        let item: ItemSource = serde_json::from_slice(&fs::read(dir.join("item.json"))?)
            .with_context(|| format!("parsing {}/item.json", dir.display()))?;

        let mut files = Vec::new();
        for (sub, alias) in [
            ("src", format!("@components/{}", item.module)),
            ("assets", format!("@assets/{ASSET_DIR}")),
        ] {
            let base = dir.join(sub);
            if !base.is_dir() {
                continue;
            }
            for path in walk(&base)? {
                let rel = path.strip_prefix(&base)?.to_string_lossy().replace('\\', "/");
                let bytes = fs::read(&path)?;
                let hash = sha256_hex(&bytes);
                let source_rel = format!("artifacts/sha256/{hash}/{}/{sub}/{rel}", item.name);
                let dest = out.join(&source_rel);
                fs::create_dir_all(dest.parent().unwrap())?;
                fs::write(&dest, &bytes)?;
                files.push(ManifestFile {
                    source: source_rel,
                    target: format!("{alias}/{rel}"),
                    sha256: format!("sha256:{hash}"),
                });
            }
        }
        if files.is_empty() {
            bail!("{} has no files under src/ or assets/", item.name);
        }

        let mut fingerprint = String::new();
        for f in &files {
            fingerprint.push_str(&f.target);
            fingerprint.push(' ');
            fingerprint.push_str(&f.sha256);
            fingerprint.push('\n');
        }
        for dep in &item.dependencies {
            fingerprint.push_str(dep);
            fingerprint.push('\n');
        }
        let version = format!("sha256:{}", sha256_hex(fingerprint.as_bytes()));

        let manifest = Manifest {
            schema: MANIFEST_SCHEMA.into(),
            schema_version: 1,
            name: item.name.clone(),
            version: version.clone(),
            registry: REGISTRY_ALIAS.into(),
            kind: "component".into(),
            tier: "free".into(),
            license: "MIT OR Apache-2.0".into(),
            dependencies: item.dependencies.clone(),
            rust_modules: vec![RustModules {
                target: "@components/mod.rs".into(),
                modules: vec![item.module.clone()],
            }],
            cargo_dependencies: Vec::new(),
            files,
        };
        let json = serde_json::to_string_pretty(&manifest)? + "\n";
        fs::write(out.join(format!("{}.json", item.name)), json)?;
        println!("built {:<16} {}", item.name, &version[..19]);

        index.items.push(IndexItem {
            name: item.name,
            description: item.description,
            version,
            dependencies: item.dependencies,
        });
    }

    fs::write(out.join("index.json"), serde_json::to_string_pretty(&index)? + "\n")?;
    println!("wrote {} items to {}", index.items.len(), out.display());
    Ok(())
}

fn walk(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d)? {
            let path = entry?.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

// ---------------------------------------------------------------------------
// Registry access (HTTP or local directory)
// ---------------------------------------------------------------------------

fn fetch(registry: &str, rel: &str) -> Result<Vec<u8>> {
    let base = registry.trim_end_matches('/');
    if base.starts_with("http://") || base.starts_with("https://") {
        let url = format!("{base}/{rel}");
        let response = ureq::get(&url).call().with_context(|| format!("GET {url}"))?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut response.into_reader(), &mut bytes)?;
        Ok(bytes)
    } else {
        let path = Path::new(base).join(rel);
        fs::read(&path).with_context(|| format!("reading {}", path.display()))
    }
}

fn fetch_manifest(registry: &str, name: &str) -> Result<Manifest> {
    let bytes = fetch(registry, &format!("{name}.json"))
        .with_context(|| format!("item `{name}` not found in {registry}"))?;
    let manifest: Manifest =
        serde_json::from_slice(&bytes).with_context(|| format!("parsing manifest for `{name}`"))?;
    if manifest.schema_version != 1 {
        bail!("`{name}` uses manifest schema {}; this tool reads v1", manifest.schema_version);
    }
    Ok(manifest)
}

fn list(registry: &str) -> Result<()> {
    let index: Index = serde_json::from_slice(&fetch(registry, "index.json")?)?;
    for item in index.items {
        println!("{:<16} {}", item.name, item.description);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// add
// ---------------------------------------------------------------------------

struct Site {
    root: PathBuf,
    components: PathBuf,
    assets: PathBuf,
}

fn find_site(start: &Path) -> Result<Site> {
    let start = fs::canonicalize(start).with_context(|| format!("{} not found", start.display()))?;
    let root = start
        .ancestors()
        .find(|dir| dir.join("proa.config.json").is_file())
        .ok_or_else(|| anyhow!("no proa.config.json in {} or its parents", start.display()))?
        .to_path_buf();
    let config: serde_json::Value = serde_json::from_slice(&fs::read(root.join("proa.config.json"))?)?;
    let path_of = |key: &str, default: &str| {
        config
            .pointer(&format!("/paths/{key}"))
            .and_then(|v| v.as_str())
            .unwrap_or(default)
            .to_string()
    };
    Ok(Site {
        components: root.join(path_of("components", "src/components")),
        assets: root.join(path_of("assets", "public/proa")),
        root,
    })
}

fn resolve_target(site: &Site, alias: &str) -> Result<PathBuf> {
    let (base, rest) = if let Some(rest) = alias.strip_prefix("@components/") {
        (&site.components, rest)
    } else if let Some(rest) = alias.strip_prefix("@assets/") {
        (&site.assets, rest)
    } else {
        bail!("unsupported target alias `{alias}`");
    };
    let rel = Path::new(rest);
    if rel
        .components()
        .any(|c| !matches!(c, Component::Normal(_)))
    {
        bail!("refusing unsafe target path `{alias}`");
    }
    Ok(base.join(rel))
}

/// Dependencies first, each item once.
fn resolve_items(registry: &str, names: &[String]) -> Result<Vec<Manifest>> {
    fn visit(
        registry: &str,
        name: &str,
        seen: &mut BTreeSet<String>,
        stack: &mut Vec<String>,
        out: &mut Vec<Manifest>,
    ) -> Result<()> {
        let name = name.strip_prefix("@proabono/").unwrap_or(name);
        if seen.contains(name) {
            return Ok(());
        }
        if stack.iter().any(|s| s == name) {
            bail!("dependency cycle: {} -> {name}", stack.join(" -> "));
        }
        let manifest = fetch_manifest(registry, name)?;
        stack.push(name.to_string());
        for dep in &manifest.dependencies {
            if dep.starts_with('@') && !dep.starts_with("@proabono/") {
                bail!("`{name}` depends on `{dep}` from another registry; install it with `proa ui add`");
            }
            visit(registry, dep, seen, stack, out)?;
        }
        stack.pop();
        seen.insert(name.to_string());
        out.push(manifest);
        Ok(())
    }

    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for name in names {
        visit(registry, name, &mut seen, &mut Vec::new(), &mut out)?;
    }
    Ok(out)
}

enum Action {
    Create,
    Update,
    Unchanged,
}

struct PlannedFile {
    path: PathBuf,
    bytes: Vec<u8>,
    sha256: String,
    action: Action,
}

fn add(names: &[String], registry: &str, root: &Path, force: bool, dry_run: bool) -> Result<()> {
    let site = find_site(root)?;
    let manifests = resolve_items(registry, names)?;

    // Plan and verify everything before touching the site.
    let mut planned = Vec::new();
    let mut conflicts = Vec::new();
    let mut modules: BTreeMap<PathBuf, Vec<String>> = BTreeMap::new();
    for manifest in &manifests {
        for file in &manifest.files {
            let bytes = fetch(registry, &file.source)?;
            let actual = format!("sha256:{}", sha256_hex(&bytes));
            if actual != file.sha256 {
                bail!(
                    "checksum mismatch for {} in `{}`: expected {}, got {actual}",
                    file.target,
                    manifest.name,
                    file.sha256
                );
            }
            let path = resolve_target(&site, &file.target)?;
            let action = match fs::read(&path) {
                Ok(existing) if existing == bytes => Action::Unchanged,
                Ok(_) if force => Action::Update,
                Ok(_) => {
                    conflicts.push(path.clone());
                    Action::Update
                }
                Err(_) => Action::Create,
            };
            planned.push(PlannedFile {
                path,
                bytes,
                sha256: actual,
                action,
            });
        }
        for group in &manifest.rust_modules {
            let target = resolve_target(&site, &group.target)?;
            modules.entry(target).or_default().extend(group.modules.iter().cloned());
        }
    }

    if !conflicts.is_empty() {
        let list: Vec<String> = conflicts
            .iter()
            .map(|p| format!("  {}", display(&site.root, p)))
            .collect();
        bail!(
            "these files differ from the registry version (local edits?):\n{}\nre-run with --force to overwrite",
            list.join("\n")
        );
    }

    for file in &planned {
        let tag = match file.action {
            Action::Create => "create",
            Action::Update => "update",
            Action::Unchanged => "same  ",
        };
        println!("{tag} {}", display(&site.root, &file.path));
    }

    if dry_run {
        println!("dry run: verified {} files, nothing written", planned.len());
        return Ok(());
    }

    for file in &planned {
        if matches!(file.action, Action::Unchanged) {
            continue;
        }
        fs::create_dir_all(file.path.parent().unwrap())?;
        fs::write(&file.path, &file.bytes)
            .with_context(|| format!("writing {}", file.path.display()))?;
    }

    for (mod_rs, names) in &modules {
        register_modules(mod_rs, names)?;
    }

    // Lock file
    let lock_path = site.root.join(LOCK_FILE);
    let mut lock: Lock = fs::read(&lock_path)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default();
    lock.registry = registry.to_string();
    let mut cursor = 0;
    for manifest in &manifests {
        let count = manifest.files.len();
        let files = planned[cursor..cursor + count]
            .iter()
            .map(|f| (display(&site.root, &f.path), f.sha256.clone()))
            .collect();
        cursor += count;
        lock.items.insert(
            manifest.name.clone(),
            LockItem {
                version: manifest.version.clone(),
                files,
            },
        );
    }
    fs::write(&lock_path, serde_json::to_string_pretty(&lock)? + "\n")?;

    let installed: Vec<&str> = manifests.iter().map(|m| m.name.as_str()).collect();
    println!("installed {} (recorded in {LOCK_FILE})", installed.join(", "));
    print_next_steps(&site)?;
    Ok(())
}

/// Append `pub mod name;` for each module missing from `mod.rs`.
fn register_modules(mod_rs: &Path, names: &[String]) -> Result<()> {
    let mut source = fs::read_to_string(mod_rs).unwrap_or_default();
    let mut changed = false;
    for name in names {
        let declared = source.lines().any(|line| {
            let line = line.trim();
            line == format!("pub mod {name};") || line == format!("mod {name};")
        });
        if !declared {
            if !source.is_empty() && !source.ends_with('\n') {
                source.push('\n');
            }
            source.push_str(&format!("pub mod {name};\n"));
            changed = true;
        }
    }
    if changed {
        fs::create_dir_all(mod_rs.parent().unwrap())?;
        fs::write(mod_rs, source)?;
    }
    Ok(())
}

fn print_next_steps(site: &Site) -> Result<()> {
    let layout = site.root.join("src/layouts/root.rs");
    let linked = fs::read_to_string(&layout)
        .map(|s| s.contains("proabono.css"))
        .unwrap_or(false);
    if !linked {
        let assets_rel = display(&site.root, &site.assets);
        println!();
        println!("next: link the stylesheet and runtime once in your root layout");
        println!("(assets live in {assets_rel}/{ASSET_DIR}; the default static mount serves them at /static/proa/{ASSET_DIR}):");
        println!(r#"  <link rel="stylesheet" href="/static/proa/proabono/proabono.css" />"#);
        println!(r#"  <script src="/static/proa/proabono/proabono.js" defer></script>"#);
    }
    Ok(())
}

fn display(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
