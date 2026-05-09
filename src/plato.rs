//! PLATO room server client.
//!
//! PLATO is the fleet's shared knowledge base.
//! Each room is a topic-specific knowledge store.
//! Tiles are the unit of knowledge (JSON documents).

use serde::{Deserialize, Serialize};

/// HTTP client for a PLATO room server.
#[derive(Debug, Clone)]
pub struct PlatoClient {
    base_url: String,
}

impl PlatoClient {
    /// Create a new client pointing at the given PLATO server base URL.
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_owned(),
        }
    }

    /// List all rooms, optionally filtered by prefix.
    pub fn list_rooms(&self, prefix: Option<&str>) -> Result<Vec<Room>, PlatoError> {
        let url = format!("{}/rooms", self.base_url);
        let body = self.get(&url)?;
        let mut rooms: Vec<Room> = serde_json::from_str(&body)
            .map_err(|e| PlatoError::Parse(e.to_string()))?;
        if let Some(p) = prefix {
            rooms.retain(|r| r.id.starts_with(p));
        }
        Ok(rooms)
    }

    /// Get tiles from a room.
    pub fn get_tiles(&self, room_id: &str) -> Result<Vec<Tile>, PlatoError> {
        let url = format!("{}/rooms/{}/tiles", self.base_url, room_id);
        let body = self.get(&url)?;
        serde_json::from_str(&body).map_err(|e| PlatoError::Parse(e.to_string()))
    }

    /// Submit a tile to a room.
    pub fn submit_tile(&self, room_id: &str, tile: &Tile) -> Result<(), PlatoError> {
        let url = format!("{}/rooms/{}/tiles", self.base_url, room_id);
        let body = serde_json::to_string(tile)
            .map_err(|e| PlatoError::Parse(e.to_string()))?;
        self.post(&url, &body)?;
        Ok(())
    }

    /// Get latest tile from a room.
    pub fn get_latest(&self, room_id: &str) -> Result<Option<Tile>, PlatoError> {
        let tiles = self.get_tiles(room_id)?;
        Ok(tiles.into_iter().last())
    }

    // --- internal helpers (blocking HTTP, behind feature flag in real impl) ---

    fn get(&self, _url: &str) -> Result<String, PlatoError> {
        // Minimal blocking HTTP GET using std-only approach.
        // Production builds should use reqwest; this compiles without it.
        Err(PlatoError::Transport(
            "HTTP transport not yet linked; use reqwest feature".into(),
        ))
    }

    fn post(&self, _url: &str, _body: &str) -> Result<String, PlatoError> {
        Err(PlatoError::Transport(
            "HTTP transport not yet linked; use reqwest feature".into(),
        ))
    }
}

/// A PLATO room (topic-specific knowledge store).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub tile_count: u64,
    pub latest_tile: Option<String>,
}

/// A tile — the unit of knowledge in PLATO.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub room_id: String,
    pub content: serde_json::Value,
    pub timestamp: u64,
    pub hash: String,
}

/// Errors from PLATO operations.
#[derive(Debug)]
pub enum PlatoError {
    Transport(String),
    Parse(String),
}

impl std::fmt::Display for PlatoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlatoError::Transport(e) => write!(f, "transport error: {}", e),
            PlatoError::Parse(e) => write!(f, "parse error: {}", e),
        }
    }
}

impl std::error::Error for PlatoError {}
