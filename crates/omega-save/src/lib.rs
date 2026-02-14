use anyhow::{Context, Result, anyhow, bail};
use omega_core::{GameMode, GameState};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SAVE_VERSION: u32 = 1;
const SAVE_MODE_VERSION: u32 = 1;

fn default_save_mode() -> String {
    GameMode::Classic.as_str().to_string()
}

fn default_save_mode_version() -> u32 {
    SAVE_MODE_VERSION
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveMetadata {
    pub schema: String,
    pub saved_turn: u64,
    pub saved_minutes: u64,
    #[serde(default = "default_save_mode")]
    pub mode: String,
    #[serde(default = "default_save_mode_version")]
    pub schema_mode_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl SaveMetadata {
    pub fn from_state(state: &GameState) -> Self {
        Self {
            schema: "omega-save".to_string(),
            saved_turn: state.clock.turn,
            saved_minutes: state.clock.minutes,
            mode: state.mode.as_str().to_string(),
            schema_mode_version: SAVE_MODE_VERSION,
            created_by: None,
            note: None,
        }
    }

    fn legacy_import(state: &GameState) -> Self {
        Self {
            schema: "omega-save-legacy".to_string(),
            saved_turn: state.clock.turn,
            saved_minutes: state.clock.minutes,
            mode: GameMode::Classic.as_str().to_string(),
            schema_mode_version: SAVE_MODE_VERSION,
            created_by: Some("legacy-import".to_string()),
            note: Some("Imported from legacy save envelope/schema".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SaveEnvelope {
    pub version: u32,
    pub payload: Value,
    pub metadata: SaveMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameStateV1 {
    pub state: GameState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GameStateV0Wrapped {
    game_state: GameState,
}

pub trait SaveCodec {
    fn encode(&self, state: &GameState) -> Result<String>;
    fn decode_envelope(&self, raw: &str) -> Result<SaveEnvelope>;
    fn decode_state(&self, raw: &str) -> Result<GameState>;
    fn migrate(&self, envelope: SaveEnvelope) -> Result<SaveEnvelope>;
}

#[derive(Debug, Clone, Default)]
pub struct JsonSaveCodec;

impl SaveCodec for JsonSaveCodec {
    fn encode(&self, state: &GameState) -> Result<String> {
        let envelope = SaveEnvelope {
            version: SAVE_VERSION,
            payload: serde_json::to_value(GameStateV1 { state: state.clone() })?,
            metadata: SaveMetadata::from_state(state),
        };
        Ok(serde_json::to_string_pretty(&envelope)?)
    }

    fn decode_envelope(&self, raw: &str) -> Result<SaveEnvelope> {
        let parsed = parse_raw_envelope(raw)?;
        self.migrate(parsed)
    }

    fn decode_state(&self, raw: &str) -> Result<GameState> {
        let envelope = self.decode_envelope(raw)?;
        decode_v1_state(&envelope.payload)
    }

    fn migrate(&self, envelope: SaveEnvelope) -> Result<SaveEnvelope> {
        match envelope.version {
            SAVE_VERSION => {
                let state = decode_v1_state(&envelope.payload)?;
                Ok(SaveEnvelope {
                    version: SAVE_VERSION,
                    payload: serde_json::to_value(GameStateV1 { state: state.clone() })?,
                    metadata: normalize_metadata(envelope.metadata, &state),
                })
            }
            0 => {
                let state = decode_v0_state(&envelope.payload)?;
                Ok(SaveEnvelope {
                    version: SAVE_VERSION,
                    payload: serde_json::to_value(GameStateV1 { state: state.clone() })?,
                    metadata: normalize_metadata(envelope.metadata, &state),
                })
            }
            unsupported => bail!("unsupported save schema version: {unsupported}"),
        }
    }
}

pub fn encode_json(state: &GameState) -> Result<String> {
    JsonSaveCodec.encode(state)
}

pub fn decode_json(raw: &str) -> Result<SaveEnvelope> {
    JsonSaveCodec.decode_envelope(raw)
}

pub fn decode_state_json(raw: &str) -> Result<GameState> {
    JsonSaveCodec.decode_state(raw)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadModePolicyResult {
    LoadAccepted,
    RejectedModeMismatch { expected: GameMode, found: GameMode },
}

pub fn decode_state_json_for_mode(raw: &str, expected_mode: GameMode) -> Result<GameState> {
    let state = decode_state_json(raw)?;
    if state.mode != expected_mode {
        bail!(
            "save mode mismatch: expected {}, found {}",
            expected_mode.as_str(),
            state.mode.as_str()
        );
    }
    Ok(state)
}

pub fn load_mode_policy(state: &GameState, expected_mode: GameMode) -> LoadModePolicyResult {
    if state.mode == expected_mode {
        LoadModePolicyResult::LoadAccepted
    } else {
        LoadModePolicyResult::RejectedModeMismatch { expected: expected_mode, found: state.mode }
    }
}

fn parse_raw_envelope(raw: &str) -> Result<SaveEnvelope> {
    if let Ok(envelope) = serde_json::from_str::<SaveEnvelope>(raw) {
        return Ok(envelope);
    }

    if let Ok(state) = serde_json::from_str::<GameState>(raw) {
        return Ok(SaveEnvelope {
            version: 0,
            payload: serde_json::to_value(state.clone())?,
            metadata: SaveMetadata::legacy_import(&state),
        });
    }

    let value: Value = serde_json::from_str(raw).context("invalid save JSON")?;
    let obj = value.as_object().ok_or_else(|| anyhow!("save document must be a JSON object"))?;
    if obj.get("payload").is_some() {
        let version = obj.get("version").and_then(Value::as_u64).ok_or_else(|| {
            anyhow!("save envelope has payload but invalid/missing numeric version")
        })? as u32;
        let payload =
            obj.get("payload").cloned().ok_or_else(|| anyhow!("save envelope missing payload"))?;
        let metadata =
            obj.get("metadata").cloned().map(serde_json::from_value).transpose()?.unwrap_or(
                SaveMetadata {
                    schema: "omega-save-unknown".to_string(),
                    saved_turn: 0,
                    saved_minutes: 0,
                    mode: default_save_mode(),
                    schema_mode_version: default_save_mode_version(),
                    created_by: None,
                    note: Some("Envelope had no metadata".to_string()),
                },
            );
        return Ok(SaveEnvelope { version, payload, metadata });
    }

    bail!("could not parse save data as envelope or legacy state");
}

fn normalize_metadata(metadata: SaveMetadata, state: &GameState) -> SaveMetadata {
    SaveMetadata {
        schema: if metadata.schema.is_empty() { "omega-save".to_string() } else { metadata.schema },
        saved_turn: state.clock.turn,
        saved_minutes: state.clock.minutes,
        mode: if metadata.mode.trim().is_empty() {
            state.mode.as_str().to_string()
        } else {
            metadata.mode
        },
        schema_mode_version: if metadata.schema_mode_version == 0 {
            SAVE_MODE_VERSION
        } else {
            metadata.schema_mode_version
        },
        created_by: metadata.created_by,
        note: metadata.note,
    }
}

fn decode_v1_state(payload: &Value) -> Result<GameState> {
    if let Ok(v1) = serde_json::from_value::<GameStateV1>(payload.clone()) {
        return Ok(v1.state);
    }
    if let Ok(state) = serde_json::from_value::<GameState>(payload.clone()) {
        return Ok(state);
    }
    bail!("invalid v1 payload: expected GameStateV1 or GameState");
}

fn decode_v0_state(payload: &Value) -> Result<GameState> {
    if let Ok(state) = serde_json::from_value::<GameState>(payload.clone()) {
        return Ok(state);
    }
    if let Ok(v0) = serde_json::from_value::<GameStateV0Wrapped>(payload.clone()) {
        return Ok(v0.game_state);
    }
    bail!("invalid v0 payload: expected legacy GameState or wrapper");
}

#[cfg(test)]
mod tests {
    use super::*;
    use omega_core::{GameMode, MapBounds, Position, Stats};
    use proptest::prelude::*;

    fn sample_state() -> GameState {
        let mut state = GameState::default();
        state.clock.turn = 42;
        state.clock.minutes = 252;
        state.log.push("state checkpoint".to_string());
        state
    }

    #[test]
    fn round_trip_json_v1() {
        let state = sample_state();
        let raw = encode_json(&state).expect("encode should work");
        let env = decode_json(&raw).expect("decode should work");
        let decoded_state = decode_state_json(&raw).expect("state decode should work");

        assert_eq!(env.version, SAVE_VERSION);
        assert_eq!(decoded_state, state);
        assert_eq!(env.metadata.saved_turn, 42);
        assert_eq!(env.metadata.saved_minutes, 252);
    }

    #[test]
    fn migrates_v0_envelope_to_v1() {
        let state = sample_state();
        let legacy = serde_json::json!({
            "version": 0,
            "payload": state,
            "metadata": {
                "schema": "omega-save-legacy",
                "saved_turn": 1,
                "saved_minutes": 1
            }
        });
        let raw = serde_json::to_string(&legacy).expect("serialize fixture");

        let env = decode_json(&raw).expect("decode+migrate should work");
        let migrated_state = decode_state_json(&raw).expect("decode state should work");

        assert_eq!(env.version, SAVE_VERSION);
        assert_eq!(migrated_state, sample_state());
        assert_eq!(env.metadata.saved_turn, 42);
        assert_eq!(env.metadata.saved_minutes, 252);
    }

    #[test]
    fn supports_raw_legacy_state_json() {
        let state = sample_state();
        let raw = serde_json::to_string(&state).expect("serialize raw state");

        let env = decode_json(&raw).expect("legacy state decode should work");
        let decoded = decode_state_json(&raw).expect("decode state should work");
        assert_eq!(env.version, SAVE_VERSION);
        assert_eq!(decoded, state);
        assert_eq!(env.metadata.created_by.as_deref(), Some("legacy-import"));
    }

    #[test]
    fn rejects_unsupported_versions() {
        let raw = r#"{"version":99,"payload":{"state":{}},"metadata":{"schema":"x","saved_turn":0,"saved_minutes":0}}"#;
        let err = decode_json(raw).expect_err("unsupported version should fail");
        assert!(err.to_string().contains("unsupported save schema version"));
    }

    #[test]
    fn decodes_legacy_like_payload_with_new_fields_defaulted() {
        let raw = r#"{
            "version": 1,
            "payload": {
                "state": {
                    "bounds": {"width": 10, "height": 10},
                    "clock": {"turn": 1, "minutes": 6, "minutes_per_turn": 6},
                    "player": {
                        "position": {"x": 2, "y": 2},
                        "stats": {"hp": 20, "max_hp": 20, "attack_min": 2, "attack_max": 6, "defense": 1},
                        "inventory": [],
                        "inventory_capacity": 12
                    },
                    "monsters": [],
                    "ground_items": [],
                    "log": [],
                    "status": "InProgress",
                    "monsters_defeated": 0,
                    "next_entity_id": 1,
                    "next_item_id": 1
                }
            },
            "metadata": {"schema":"omega-save","saved_turn":1,"saved_minutes":6}
        }"#;

        let state = decode_state_json(raw).expect("legacy-like payload should decode");
        assert_eq!(state.player_name, "Adventurer");
        assert_eq!(state.gold, 250);
        assert_eq!(state.world_mode, omega_core::WorldMode::DungeonCity);
        assert_eq!(state.scheduler.player_phase, 0);
    }

    #[test]
    fn preserves_wizard_and_scoring_policy_on_roundtrip() {
        let mut state = sample_state();
        state.wizard.enabled = true;
        state.wizard.scoring_allowed = false;
        state.progression.high_score_eligible = false;
        state.progression.score = 777;
        let raw = encode_json(&state).expect("encode wizard state");
        let decoded = decode_state_json(&raw).expect("decode wizard state");
        assert!(decoded.wizard.enabled);
        assert!(!decoded.wizard.scoring_allowed);
        assert!(!decoded.progression.high_score_eligible);
        assert_eq!(decoded.progression.score, 777);
    }

    #[test]
    fn save_metadata_carries_mode() {
        let mut state = sample_state();
        state.mode = GameMode::Modern;
        let raw = encode_json(&state).expect("encode should work");
        let envelope = decode_json(&raw).expect("decode should work");
        assert_eq!(envelope.metadata.mode, "modern");
        assert_eq!(envelope.metadata.schema_mode_version, SAVE_MODE_VERSION);
    }

    #[test]
    fn decode_for_mode_rejects_mismatch() {
        let mut state = sample_state();
        state.mode = GameMode::Modern;
        let raw = encode_json(&state).expect("encode should work");
        let err =
            decode_state_json_for_mode(&raw, GameMode::Classic).expect_err("mismatch must fail");
        assert!(err.to_string().contains("save mode mismatch"));
    }

    fn arbitrary_state() -> impl Strategy<Value = GameState> {
        (
            10i32..120i32,
            10i32..120i32,
            0i32..120i32,
            0i32..120i32,
            1i32..60i32,
            1i32..12i32,
            1i32..12i32,
            0u64..100_000u64,
            0u64..2_000_000u64,
            1usize..24usize,
            prop::collection::vec("log-entry-[a-z0-9]{0,16}", 0..8),
        )
            .prop_map(
                |(
                    width,
                    height,
                    px,
                    py,
                    hp,
                    attack_min,
                    attack_delta,
                    turn,
                    minutes,
                    capacity,
                    log,
                )| {
                    let mut state = GameState::new(MapBounds { width, height });
                    state.player.position = Position { x: px % width, y: py % height };
                    state.player.stats = Stats {
                        hp,
                        max_hp: hp,
                        attack_min,
                        attack_max: attack_min + attack_delta,
                        defense: 1,
                        weight: 70,
                    };
                    state.player.inventory_capacity = capacity;
                    state.clock.turn = turn;
                    state.clock.minutes = minutes;
                    state.log = log;
                    state
                },
            )
    }

    proptest! {
        #[test]
        fn prop_round_trip_preserves_randomized_state(state in arbitrary_state()) {
            let raw = encode_json(&state).expect("encode");
            let decoded = decode_state_json(&raw).expect("decode");
            prop_assert_eq!(decoded, state);
        }
    }
}
