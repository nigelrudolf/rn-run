use serde::Serialize;

/// Wrapper for all JSON output from rn-run commands.
/// AI/automation tools should parse this structure.
#[derive(Serialize)]
pub struct Output<T: Serialize> {
    /// The command that was executed (e.g., "check-env", "kill-metro")
    pub command: String,
    /// Whether the command succeeded
    pub success: bool,
    /// Command-specific data payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error message if success is false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Suggested fix for the error (for AI consumption)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
}

impl<T: Serialize> Output<T> {
    pub fn success(command: &str, data: T) -> Self {
        Output {
            command: command.to_string(),
            success: true,
            data: Some(data),
            error: None,
            suggested_fix: None,
        }
    }

    pub fn error(command: &str, error: &str, suggested_fix: Option<&str>) -> Output<()> {
        Output {
            command: command.to_string(),
            success: false,
            data: None,
            error: Some(error.to_string()),
            suggested_fix: suggested_fix.map(|s| s.to_string()),
        }
    }

    pub fn print(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap_or_default());
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CHECK-ENV DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct EnvCheckResult {
    pub overall_status: String, // "ok", "warnings", "errors"
    pub checks: Vec<EnvCheck>,
    pub summary: String,
}

#[derive(Serialize)]
pub struct EnvCheck {
    pub name: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<String>,
    pub required_for: Vec<String>, // ["ios"], ["android"], or ["ios", "android"]
}

// ═══════════════════════════════════════════════════════════════════════════════
// RN-VERSION DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct RnVersionResult {
    pub version: String,
    pub package_manager: String, // "npm" or "yarn"
    pub notes: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SIMULATOR/EMULATOR LIST DATA STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct SimulatorListResult {
    pub simulators: Vec<Simulator>,
}

#[derive(Serialize)]
pub struct Simulator {
    pub name: String,
    pub udid: String,
    pub state: String, // "Booted" or "Shutdown"
    pub runtime: String,
}

#[derive(Serialize)]
pub struct EmulatorListResult {
    pub emulators: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SIMPLE ACTION RESULTS
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct ActionResult {
    pub action: String,
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// SCREENSHOT RESULT
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct ScreenshotResult {
    pub platform: String,
    pub path: String,
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════════════════════
// UPDATE RESULT
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Serialize)]
pub struct UpdateResultOutput {
    pub current_version: String,
    pub latest_version: String,
    pub updated: bool,
    pub message: String,
}
