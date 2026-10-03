use crate::{Result, read_bounded, strict_json};
use regex::Regex;
use serde_json::{Value, json};
use std::{collections::HashSet, path::Path};
use walkdir::WalkDir;

fn text(path: &Path) -> Result<String> {
    String::from_utf8(read_bounded(path, 2 * 1024 * 1024)?)
        .map(|s| s.replace("\r\n", "\n"))
        .map_err(|_| "Package text must be UTF-8".into())
}
fn require(ok: bool, reason: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(reason.into()) }
}
fn contained(root: &Path, relative: &str) -> Result<()> {
    require(
        relative.starts_with("./"),
        "Manifest paths must be relative",
    )?;
    let path = root
        .join(relative)
        .canonicalize()
        .map_err(|_| "Package target missing")?;
    require(
        path.starts_with(root) && path.is_file(),
        "Package target must stay inside root",
    )
}
pub fn validate(input: &Path) -> Result<Value> {
    let root = input
        .canonicalize()
        .map_err(|_| "Package root unavailable")?;
    let manifest = strict_json::parse(&read_bounded(&root.join("plugin.json"), 1024 * 1024)?)?;
    require(
        manifest["$schema"] == "https://agent-plugins.org/schemas/1.0.0/plugin.schema.json",
        "Unsupported plugin schema",
    )?;
    let slug =
        Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").map_err(|_| "Validator pattern unavailable")?;
    let version = Regex::new(r"^\d+\.\d+\.\d+$").map_err(|_| "Validator pattern unavailable")?;
    require(
        manifest["name"].as_str().is_some_and(|s| slug.is_match(s)),
        "Invalid plugin name",
    )?;
    require(
        manifest["version"]
            .as_str()
            .is_some_and(|s| version.is_match(s)),
        "Invalid plugin version",
    )?;
    require(
        manifest["author"]["name"] == "NAVANEETH" && manifest["license"] == "Apache-2.0",
        "Publisher or license mismatch",
    )?;
    let ui = &manifest["extensions"]["com.openai"]["interface"];
    for (field, limit) in [
        ("displayName", 30),
        ("shortDescription", 30),
        ("longDescription", 4000),
        ("developerName", 80),
    ] {
        require(
            ui[field]
                .as_str()
                .is_some_and(|s| !s.is_empty() && s.chars().count() <= limit),
            "Invalid interface text",
        )?;
    }
    for field in ["logo", "composerIcon"] {
        contained(&root, ui[field].as_str().ok_or("Missing icon path")?)?;
    }
    contained(
        &root,
        manifest["extensions"]["com.openai"]["onboardingSkill"]
            .as_str()
            .ok_or("Missing onboarding path")?,
    )?;
    for file in ["LICENSE", "NOTICE"] {
        require(root.join(file).is_file(), "License or notice missing")?;
    }
    for file in ["mcp.json", ".mcp.json", ".app.json", "hooks"] {
        require(
            !root.join(file).exists(),
            "Unexpected executable integration",
        )?;
    }
    let name = Regex::new(r"(?m)^name: (.+)$").map_err(|_| "Pattern unavailable")?;
    let description = Regex::new(r"(?m)^description: (.+)$").map_err(|_| "Pattern unavailable")?;
    let links = Regex::new(r"\[[^\]]+\]\(([^)]+)\)").map_err(|_| "Pattern unavailable")?;
    let homes =
        Regex::new(r"[A-Za-z]:[/\\]+Users[/\\]+[^/\\\s]+").map_err(|_| "Pattern unavailable")?;
    let mut skills = HashSet::new();
    let mut profiles = HashSet::new();
    let walker = WalkDir::new(&root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            ![".git", "target", "__pycache__"]
                .iter()
                .any(|n| e.file_name() == *n)
        });
    let mut count = 0;
    for entry in walker {
        let entry = entry.map_err(|_| "Package traversal failed")?;
        count += 1;
        require(count <= 30000, "Package file limit exceeded")?;
        require(
            !entry.file_type().is_symlink(),
            "Package symlinks unsupported",
        )?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let file = entry.file_name().to_string_lossy();
        require(
            !["auth.json", "config.toml", "install-manifest.json", ".env"].contains(&file.as_ref()),
            "Private configuration present",
        )?;
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !["md", "json", "toml", "yaml", "yml", "rs", "svg"].contains(&extension) {
            continue;
        }
        let content = text(path)?;
        require(!homes.is_match(&content), "Private home path present")?;
        if extension == "md" {
            for link in links.captures_iter(&content) {
                let target = link[1].split('#').next().unwrap_or("");
                if target.is_empty() || target.contains("://") || target.starts_with("mailto:") {
                    continue;
                }
                let resolved = path
                    .parent()
                    .ok_or("Missing parent")?
                    .join(target)
                    .canonicalize()
                    .map_err(|_| "Markdown link missing")?;
                require(
                    resolved.starts_with(&root) && resolved.is_file(),
                    "Markdown target escapes package",
                )?;
            }
        }
        if file == "SKILL.md" {
            require(content.starts_with("---\n"), "Missing skill frontmatter")?;
            let header = content.split("---").nth(1).ok_or("Missing skill header")?;
            let id = name.captures(header).ok_or("Skill name missing")?[1].to_string();
            require(
                slug.is_match(&id)
                    && id.len() < 64
                    && path
                        .parent()
                        .and_then(|p| p.file_name())
                        .is_some_and(|s| s == id.as_str()),
                "Invalid skill identity",
            )?;
            require(
                description
                    .captures(header)
                    .is_some_and(|d| !d[1].trim().is_empty()),
                "Skill description missing",
            )?;
            require(skills.insert(id), "Duplicate skill identity")?;
        }
        if path.parent() == Some(root.join("assets/codex-agent-profiles").as_path())
            && extension == "toml"
        {
            let profile: toml::Table =
                toml::from_str(&content).map_err(|_| "Invalid agent TOML")?;
            let id = profile
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("Profile name missing")?;
            require(
                path.file_stem().is_some_and(|s| s == id) && profiles.insert(id.to_owned()),
                "Invalid profile identity",
            )?;
            for key in ["description", "developer_instructions"] {
                require(
                    profile
                        .get(key)
                        .and_then(|v| v.as_str())
                        .is_some_and(|s| !s.trim().is_empty()),
                    "Profile instructions missing",
                )?;
            }
        }
    }
    require(
        skills.contains("global-orchestrator")
            && skills.contains("plugin-safety-review")
            && !profiles.is_empty(),
        "Core skills or profiles missing",
    )?;
    let catalog = strict_json::parse(&read_bounded(
        &root.join(".agents/plugins/marketplace.json"),
        1024 * 1024,
    )?)?;
    for item in catalog["plugins"]
        .as_array()
        .ok_or("Marketplace entries missing")?
    {
        let path = item["source"]["path"]
            .as_str()
            .ok_or("Marketplace path missing")?;
        require(path.starts_with("./"), "Marketplace path must be local")?;
        let target = root
            .join(path)
            .canonicalize()
            .map_err(|_| "Marketplace source missing")?;
        require(
            target.starts_with(&root) && target.join("plugin.json").is_file(),
            "Marketplace source outside package",
        )?;
    }
    Ok(
        json!({"manifest_and_paths":"passed", "skills":skills.len(),"native_profiles":profiles.len(),"no_mcp_or_hooks":true,"private_home_paths_absent":true,"official_schema_validation":false}),
    )
}
