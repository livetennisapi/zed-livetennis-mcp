//! Zed extension registering the Live Tennis MCP server as a context server.
//!
//! The server is the `livetennisapi-mcp` npm package, spawned over stdio. Zed's extension API
//! hands back a process to spawn, so a remote HTTP MCP endpoint is not expressible here; the
//! stdio entrypoint is the correct target.
//!
//! The API key is read *only* from the user's Zed settings via [`ContextServerSettings`] and
//! handed to the child process as an environment variable. This extension never reads the
//! ambient process environment.

use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

/// The npm package providing the MCP server. Zed downloads and updates it at runtime; it is
/// deliberately not vendored into the extension.
const PACKAGE_NAME: &str = "livetennisapi-mcp";

/// Path to the stdio entrypoint within the package's install tree, relative to the working
/// directory Zed designates for this extension.
///
/// Matches the package's `bin."livetennisapi-mcp"` field (`./dist/index.js`).
const SERVER_ENTRYPOINT: &str = "node_modules/livetennisapi-mcp/dist/index.js";

/// The environment variable the server reads its API key from.
const API_KEY_ENV_VAR: &str = "LIVETENNISAPI_KEY";

/// Where users obtain a key. Surfaced in every error path so a misconfiguration is self-solving.
const SIGNUP_URL: &str = "https://livetennisapi.com/subscribe/free";

/// Settings for the Live Tennis MCP server, supplied under
/// `context_servers.livetennis-mcp.settings` in the user's Zed settings.
#[derive(Debug, Deserialize, JsonSchema)]
struct LiveTennisMcpSettings {
    /// Your Live Tennis API key (looks like `twjp_…`).
    ///
    /// Get a free key at https://livetennisapi.com/subscribe/free
    api_key: String,
}

struct LiveTennisMcpExtension;

impl LiveTennisMcpExtension {
    /// Guidance shown when the key is absent or blank.
    ///
    /// Interpolates the real context server id so the snippet is correct even if the user
    /// registered the server under a different key.
    fn missing_api_key_error(context_server_id: &ContextServerId, reason: &str) -> String {
        format!(
            "{reason}\n\n\
             Add your Live Tennis API key to your Zed settings:\n\n\
             {{\n  \
                 \"context_servers\": {{\n    \
                     \"{context_server_id}\": {{\n      \
                         \"settings\": {{\n        \
                             \"api_key\": \"twjp_…\"\n      \
                         }}\n    \
                     }}\n  \
                 }}\n\
             }}\n\n\
             Get a free key at {SIGNUP_URL}"
        )
    }
}

impl zed::Extension for LiveTennisMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        // Read the key from the user's Zed settings. Note this looks up the id Zed actually
        // invoked us with rather than a hardcoded string, so a renamed entry still resolves.
        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;

        let Some(settings) = settings.settings else {
            return Err(Self::missing_api_key_error(
                context_server_id,
                "The Live Tennis MCP server is not configured yet.",
            ));
        };

        let settings: LiveTennisMcpSettings = serde_json::from_value(settings).map_err(|err| {
            Self::missing_api_key_error(
                context_server_id,
                &format!("Invalid settings for the Live Tennis MCP server: {err}."),
            )
        })?;

        let api_key = settings.api_key.trim().to_string();
        if api_key.is_empty() {
            return Err(Self::missing_api_key_error(
                context_server_id,
                "The Live Tennis MCP server's `api_key` setting is empty.",
            ));
        }

        // Track the latest published version rather than pinning a hardcoded one, so users pick
        // up server fixes without waiting on an extension release.
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME)?;

        if installed_version.as_deref() != Some(latest_version.as_str()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        // Use the Node binary Zed provides; never assume "node" is on PATH.
        let node = zed::node_binary_path()?;

        let entrypoint = std::env::current_dir()
            .map_err(|err| format!("Failed to resolve the extension working directory: {err}"))?
            .join(SERVER_ENTRYPOINT);

        Ok(Command {
            command: node,
            args: vec![entrypoint.to_string_lossy().to_string()],
            // The key reaches the server only through this explicitly constructed environment.
            env: vec![(API_KEY_ENV_VAR.to_string(), api_key)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema = serde_json::to_string(&schemars::schema_for!(LiveTennisMcpSettings))
            .map_err(|err| format!("Failed to serialize the settings schema: {err}"))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(LiveTennisMcpExtension);
