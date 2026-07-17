//! `glob` tool — filename pattern matching.
//!
//! Fast, lightweight: matches file names against glob patterns using the
//! `glob` crate. Does NOT search file contents (use `grep` for that).
//! Supports `*`, `**`, `?` patterns. Results sorted by modification time
//! (newest first).

use std::path::Path;
use std::time::SystemTime;

use crate::types::tool::{ToolKind, ToolNamespace};

const DEFAULT_HEAD_LIMIT: usize = 250;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GlobInput {
    #[schemars(description = "Glob pattern (e.g. '**/*.rs', 'src/**/mod.rs', '*.toml')")]
    pub pattern: String,

    #[schemars(description = "Directory to search in. Defaults to current working directory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[schemars(with = "crate::types::schema::GrokIntegerSchema", description = "Max results (default: 250, 0 = unlimited)")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_limit: Option<usize>,

    #[schemars(with = "crate::types::schema::GrokIntegerSchema", description = "Skip first N results for pagination")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
}

/// Entry in glob output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobEntry {
    pub path: String,
    pub size: u64,
}

/// Structured glob output.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobOutput {
    pub pattern: String,
    pub path: String,
    pub count: usize,
    pub total: usize,
    pub files: Vec<GlobEntry>,
}
impl xai_tool_runtime::ToolOutput for GlobOutput {}

#[derive(Debug, Default)]
pub struct GlobTool;

impl crate::types::tool_metadata::ToolMetadata for GlobTool {
    fn kind(&self) -> ToolKind {
        ToolKind::Search
    }

    fn tool_namespace(&self) -> ToolNamespace {
        ToolNamespace::GrokBuild
    }

    fn description_template(&self) -> &str {
        r#"Find files matching a glob pattern. Fast filename-only matching — \
does NOT search file contents (use ${{ tools.by_kind.search }} for that). \
Supports ** for recursive matching. Returns file paths sorted by \
modification time (newest first)."#
    }
}

impl xai_tool_runtime::Tool for GlobTool {
    type Args = GlobInput;
    type Output = GlobOutput;

    fn id(&self) -> xai_tool_protocol::ToolId {
        xai_tool_protocol::ToolId::new("glob").expect("valid tool id")
    }

    fn description(
        &self,
        _ctx: &::xai_tool_runtime::ListToolsContext,
    ) -> xai_tool_types::ToolDescription {
        xai_tool_types::ToolDescription::new(
            "glob",
            crate::types::tool_metadata::ToolMetadata::description_template(self),
        )
    }

    fn capabilities(&self) -> xai_tool_protocol::ToolCapabilities {
        xai_tool_protocol::ToolCapabilities {
            is_read_only: true,
            tool_scope: Some(xai_tool_protocol::ToolScope::Read),
            ..Default::default()
        }
    }

    #[tracing::instrument(name = "tool.glob", skip_all, fields(pattern = %input.pattern))]
    async fn run(
        &self,
        ctx: xai_tool_runtime::ToolCallContext,
        input: GlobInput,
    ) -> Result<GlobOutput, xai_tool_runtime::ToolError> {
        use crate::types::tool_metadata::{resolve_cwd, shared_resources};
        let resources = shared_resources(&ctx)?;
        let cwd = resolve_cwd(&ctx, &resources).await?;

        let head_limit = input.head_limit.unwrap_or(DEFAULT_HEAD_LIMIT);
        let offset = input.offset.unwrap_or(0);

        let root_path = if let Some(ref p) = input.path {
            if Path::new(p).is_absolute() {
                Path::new(p).to_path_buf()
            } else {
                cwd.join(p)
            }
        } else {
            cwd.clone()
        };

        if !root_path.exists() {
            return Err(xai_tool_runtime::ToolError::execution(
                xai_tool_protocol::ToolId::new("glob").expect("valid"),
                format!("path not found: {}", root_path.display()),
            ));
        }

        // Build a full-path glob pattern for the `glob` crate
        let search_pattern = if root_path == cwd {
            input.pattern.clone()
        } else {
            format!("{}/{}", root_path.display(), input.pattern)
        };

        let mut entries: Vec<((String, u64), SystemTime)> = match glob::glob(&search_pattern) {
            Ok(paths) => {
                let paths: Vec<std::path::PathBuf> = paths.filter_map(|r| r.ok()).collect();
                let mut found = std::collections::HashSet::new();
                paths.into_iter()
                    .filter(|p| p.is_file())
                    .filter_map(|p| {
                        let rel = p.strip_prefix(&root_path).ok()?;
                        let rel_str = rel.to_string_lossy().to_string();
                        if rel_str.is_empty() {
                            return None;
                        }
                        if !found.insert(rel_str.clone()) {
                            return None;
                        }
                        let size = std::fs::metadata(&p).ok()?.len();
                        // Use creation or modification time for ordering
                        let mtime = std::fs::metadata(&p)
                            .ok()?
                            .modified()
                            .ok()?;
                        Some(((rel_str, size), mtime))
                    })
                    .collect::<Vec<_>>()
            }
            Err(e) => {
                return Err(xai_tool_runtime::ToolError::execution(
                    xai_tool_protocol::ToolId::new("glob").expect("valid"),
                    format!("invalid glob pattern: {e}"),
                ));
            }
        };

        // Sort by modification time, newest first
        entries.sort_by(|a, b| b.1.cmp(&a.1));

        let total = entries.len();
        let files: Vec<GlobEntry> = entries
            .into_iter()
            .skip(offset)
            .take(if head_limit == 0 { total } else { head_limit })
            .map(|((path, size), _)| GlobEntry { path, size })
            .collect();

        Ok(GlobOutput {
            pattern: input.pattern,
            path: root_path.display().to_string(),
            count: files.len(),
            total,
            files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::resources::{Cwd, Resources};
    use crate::types::tool_metadata::test_ctx;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn tool_name_and_description() {
        let tool = GlobTool;
        assert_eq!(
            xai_tool_runtime::Tool::id(&tool).as_str(),
            "glob"
        );
        assert!(
            crate::types::tool_metadata::ToolMetadata::description_template(&tool)
                .contains("glob pattern")
        );
    }

    #[tokio::test]
    async fn glob_finds_rs_files() {
        let tmp = TempDir::new().unwrap();
        File::create(tmp.path().join("main.rs")).unwrap();
        File::create(tmp.path().join("lib.rs")).unwrap();
        File::create(tmp.path().join("README.md")).unwrap();

        let mut resources = Resources::new();
        resources.insert(Cwd(tmp.path().to_path_buf()));

        let tool = GlobTool;
        let result = xai_tool_runtime::Tool::run(
            &tool,
            test_ctx(resources.into_shared()),
            GlobInput {
                pattern: "*.rs".to_string(),
                path: None,
                head_limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.total, 2);
        assert_eq!(result.count, 2);
        let names: Vec<&str> = result.files.iter().map(|e| e.path.as_str()).collect();
        assert!(names.contains(&"main.rs"));
        assert!(names.contains(&"lib.rs"));
    }

    #[tokio::test]
    async fn glob_recursive_double_star() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        fs::create_dir(&sub).unwrap();
        File::create(sub.join("deep.rs")).unwrap();
        File::create(tmp.path().join("root.rs")).unwrap();

        let mut resources = Resources::new();
        resources.insert(Cwd(tmp.path().to_path_buf()));

        let tool = GlobTool;
        let result = xai_tool_runtime::Tool::run(
            &tool,
            test_ctx(resources.into_shared()),
            GlobInput {
                pattern: "**/*.rs".to_string(),
                path: None,
                head_limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();

        let names: Vec<&str> = result.files.iter().map(|e| e.path.as_str()).collect();
        assert!(names.contains(&"root.rs"), "got: {:?}", names);
        assert!(
            names.iter().any(|n| n.contains("deep.rs")),
            "got: {:?}",
            names
        );
    }

    #[tokio::test]
    async fn glob_respects_head_limit() {
        let tmp = TempDir::new().unwrap();
        for i in 0..10 {
            File::create(tmp.path().join(format!("file_{}.rs", i))).unwrap();
        }

        let mut resources = Resources::new();
        resources.insert(Cwd(tmp.path().to_path_buf()));

        let result = xai_tool_runtime::Tool::run(
            &GlobTool,
            test_ctx(resources.into_shared()),
            GlobInput {
                pattern: "*.rs".to_string(),
                path: None,
                head_limit: Some(3),
                offset: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(result.count, 3);
        assert_eq!(result.total, 10);
    }

    #[tokio::test]
    async fn glob_bad_pattern_errors() {
        let tmp = TempDir::new().unwrap();
        let mut resources = Resources::new();
        resources.insert(Cwd(tmp.path().to_path_buf()));

        let result = xai_tool_runtime::Tool::run(
            &GlobTool,
            test_ctx(resources.into_shared()),
            GlobInput {
                pattern: "[invalid".to_string(),
                path: None,
                head_limit: None,
                offset: None,
            },
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn glob_nonexistent_path_errors() {
        let tmp = TempDir::new().unwrap();
        let mut resources = Resources::new();
        resources.insert(Cwd(tmp.path().to_path_buf()));

        let result = xai_tool_runtime::Tool::run(
            &GlobTool,
            test_ctx(resources.into_shared()),
            GlobInput {
                pattern: "*.rs".to_string(),
                path: Some("/nonexistent/path".to_string()),
                head_limit: None,
                offset: None,
            },
        )
        .await;

        assert!(result.is_err());
    }
}
