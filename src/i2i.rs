//! I2I protocol for inter-agent communication.
//!
//! Format: `[I2I:TYPE] scope — summary`
//! Delivered via git-based bottles in for-fleet/ directory.

use serde::{Deserialize, Serialize};

/// I2I message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I2iType {
    Briefing,
    Request,
    Deliverable,
    Blocker,
    Critique,
    Sync,
}

impl I2iType {
    /// Tag as it appears in rendered text (e.g. `BRIEFING`).
    pub fn tag(&self) -> &'static str {
        match self {
            I2iType::Briefing => "BRIEFING",
            I2iType::Request => "REQUEST",
            I2iType::Deliverable => "DELIVERABLE",
            I2iType::Blocker => "BLOCKER",
            I2iType::Critique => "CRITIQUE",
            I2iType::Sync => "SYNC",
        }
    }

    fn from_tag(tag: &str) -> Option<Self> {
        match tag.to_uppercase().as_str() {
            "BRIEFING" => Some(I2iType::Briefing),
            "REQUEST" => Some(I2iType::Request),
            "DELIVERABLE" => Some(I2iType::Deliverable),
            "BLOCKER" => Some(I2iType::Blocker),
            "CRITIQUE" => Some(I2iType::Critique),
            "SYNC" => Some(I2iType::Sync),
            _ => None,
        }
    }
}

/// I2I message status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum I2iStatus {
    Complete,
    InProgress,
    Blocked,
}

impl I2iStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "complete" => Some(I2iStatus::Complete),
            "in_progress" | "in progress" | "inprogress" => Some(I2iStatus::InProgress),
            "blocked" => Some(I2iStatus::Blocked),
            _ => None,
        }
    }
}

/// An I2I message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I2iMessage {
    pub msg_type: I2iType,
    pub from: String,
    pub scope: String,
    pub summary: String,
    pub details: String,
    pub status: I2iStatus,
}

impl I2iMessage {
    /// Parse an I2I message from its text format.
    ///
    /// Expected first line: `[I2I:TYPE] scope — summary`
    /// Optional second line: `status: <status>`
    /// Remaining lines: details.
    pub fn parse(text: &str) -> Result<Self, I2iParseError> {
        let mut lines = text.lines();
        let header = lines
            .next()
            .ok_or_else(|| I2iParseError("empty input".into()))?
            .trim();

        // Parse [I2I:TYPE]
        if !header.starts_with("[I2I:") {
            return Err(I2iParseError("missing [I2I:TYPE] header".into()));
        }
        let type_end = header
            .find(']')
            .ok_or_else(|| I2iParseError("unclosed [I2I:...] bracket".into()))?;
        let type_tag = &header[5..type_end];
        let msg_type =
            I2iType::from_tag(type_tag).ok_or_else(|| I2iParseError(format!("unknown type: {}", type_tag)))?;

        let rest = &header[type_end + 1..].trim_start();

        // Split on em-dash or double-dash
        let (scope, summary) = if let Some(idx) = rest.find(" — ") {
            (&rest[..idx], &rest[idx + " — ".len()..])
        } else if let Some(idx) = rest.find("--") {
            (&rest[..idx], &rest[idx + 2..])
        } else {
            (rest as &str, "")
        };

        // Default status
        let mut status = I2iStatus::InProgress;
        let mut details_lines: Vec<&str> = Vec::new();

        for line in lines {
            let trimmed = line.trim();
            if trimmed.starts_with("status:") {
                let val = trimmed.trim_start_matches("status:").trim();
                if let Some(s) = I2iStatus::from_str(val) {
                    status = s;
                }
            } else if trimmed.starts_with("from:") {
                // handled below as part of details if needed
                details_lines.push(trimmed);
            } else {
                details_lines.push(trimmed);
            }
        }

        Ok(I2iMessage {
            msg_type,
            from: String::new(), // not in header format, defaults empty
            scope: scope.to_string(),
            summary: summary.to_string(),
            details: details_lines.join("\n"),
            status,
        })
    }

    /// Render the message to I2I text format.
    pub fn render(&self) -> String {
        let mut out = format!(
            "[I2I:{}] {} — {}\n",
            self.msg_type.tag(),
            self.scope,
            self.summary
        );
        if !self.from.is_empty() {
            out.push_str(&format!("from: {}\n", self.from));
        }
        let status_str = match self.status {
            I2iStatus::Complete => "complete",
            I2iStatus::InProgress => "in_progress",
            I2iStatus::Blocked => "blocked",
        };
        out.push_str(&format!("status: {}\n", status_str));
        if !self.details.is_empty() {
            out.push_str(&self.details);
            if !self.details.ends_with('\n') {
                out.push('\n');
            }
        }
        out
    }
}

/// Parse error for I2I messages.
#[derive(Debug)]
pub struct I2iParseError(pub String);

impl std::fmt::Display for I2iParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I2I parse error: {}", self.0)
    }
}

impl std::error::Error for I2iParseError {}
