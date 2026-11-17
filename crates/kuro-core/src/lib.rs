//! Platform-independent study data and validation. No browser APIs or credentials.
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
pub const SCHEMA: u32 = 1;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub subject: String,
    pub date: String,
    pub minutes: u32,
    pub done: bool,
    pub topic: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub body: String,
    pub topic: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    Independent,
    Hinted,
    Incorrect,
    Skipped,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attempt {
    pub id: String,
    pub topic: String,
    pub outcome: Outcome,
    pub seconds: u32,
    pub date: String,
    pub mistake: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    pub id: String,
    pub front: String,
    pub back: String,
    pub due: String,
    pub stage: u8,
    pub reviews: u32,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum TopicStatus {
    #[default]
    NotStarted,
    Learning,
    Understood,
    Foundational,
    Main,
    Advanced,
    RevisionNeeded,
    Ready,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub schema: u32,
    pub revision: u32,
    pub theme: String,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>,
    pub attempts: Vec<Attempt>,
    pub cards: Vec<Card>,
    pub topics: BTreeMap<String, TopicStatus>,
}
impl Default for State {
    fn default() -> Self {
        Self {
            schema: SCHEMA,
            revision: 0,
            theme: "NOCTURNE".into(),
            tasks: vec![],
            notes: vec![],
            attempts: vec![],
            cards: vec![],
            topics: BTreeMap::new(),
        }
    }
}
pub fn valid_date(s: &str) -> bool {
    s.len() == 10 && NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
}
pub fn accuracy(attempts: &[Attempt]) -> Option<f64> {
    let attempted = attempts
        .iter()
        .filter(|x| x.outcome != Outcome::Skipped)
        .count();
    (attempted > 0).then(|| {
        100.0
            * attempts
                .iter()
                .filter(|x| x.outcome == Outcome::Independent)
                .count() as f64
            / attempted as f64
    })
}
pub fn review(card: &mut Card, recalled: bool, date: &str) -> Result<(), String> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| "Invalid review date")?;
    let intervals = [1, 3, 7, 14, 30];
    if !recalled {
        card.stage = 0;
    }
    let days = intervals[usize::from(card.stage).min(4)];
    card.due = date
        .checked_add_signed(chrono::Duration::days(days))
        .ok_or("Review date out of range")?
        .to_string();
    if recalled {
        card.stage = (card.stage + 1).min(4);
    }
    card.reviews += 1;
    Ok(())
}
pub fn validate(s: &State) -> Result<(), String> {
    if s.schema != SCHEMA {
        return Err("Unsupported schema. Original data was not changed.".into());
    }
    if !["NOCTURNE", "MONO", "PAPER", "PLUM", "SKY"].contains(&s.theme.as_str()) {
        return Err("Invalid theme".into());
    }
    let mut ids = std::collections::HashSet::new();
    for id in s
        .tasks
        .iter()
        .map(|x| &x.id)
        .chain(s.notes.iter().map(|x| &x.id))
        .chain(s.attempts.iter().map(|x| &x.id))
        .chain(s.cards.iter().map(|x| &x.id))
    {
        if id.is_empty() || !ids.insert(id) {
            return Err("Empty or duplicate record ID".into());
        }
    }
    if s.tasks.iter().any(|x| {
        x.title.trim().is_empty()
            || x.title.len() > 500
            || !valid_date(&x.date)
            || x.minutes == 0
            || x.minutes > 720
    }) {
        return Err("Invalid task title, date or duration (1–720 minutes)".into());
    }
    if s.notes
        .iter()
        .any(|x| x.title.trim().is_empty() || x.title.len() > 500 || x.body.len() > 1_000_000)
    {
        return Err("Invalid note".into());
    }
    if s.attempts.iter().any(|x| {
        x.topic.trim().is_empty() || !valid_date(&x.date) || x.seconds == 0 || x.seconds > 86400
    }) {
        return Err("Invalid attempt".into());
    }
    if s.cards
        .iter()
        .any(|x| x.front.trim().is_empty() || !valid_date(&x.due) || x.stage > 4)
    {
        return Err("Invalid revision card".into());
    }
    Ok(())
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Backup {
    format: String,
    version: u32,
    sha256: String,
    data: String,
}
pub fn export(s: &State) -> Result<String, String> {
    validate(s)?;
    let data = serde_json::to_string(s).map_err(|e| e.to_string())?;
    let backup = serde_json::to_string_pretty(&Backup {
        format: "KURO-STUDY-BACKUP".into(),
        version: 1,
        sha256: format!("{:x}", Sha256::digest(data.as_bytes())),
        data,
    })
    .map_err(|e| e.to_string())?;
    if backup.len() > 20_000_000 {
        return Err("Backup exceeds the supported 20 MB limit; no unusable backup was downloaded. Your stored data is unchanged.".into());
    }
    Ok(backup)
}
pub fn import(json: &str) -> Result<State, String> {
    if json.len() > 20_000_000 {
        return Err("Backup exceeds 20 MB limit".into());
    }
    let b: Backup = serde_json::from_str(json).map_err(|_| "Malformed backup")?;
    if b.format != "KURO-STUDY-BACKUP" || b.version != 1 {
        return Err("Unsupported backup format".into());
    }
    if b.sha256 != format!("{:x}", Sha256::digest(b.data.as_bytes())) {
        return Err("Backup checksum mismatch. Nothing was imported.".into());
    }
    let s: State = serde_json::from_str(&b.data).map_err(|_| "Invalid study data")?;
    validate(&s)?;
    Ok(s)
}
/// Native targets must implement this contract before claiming native persistence.
pub trait Repository {
    fn load(&self) -> Result<State, String>;
    fn save(&self, expected_revision: u32, next: &State) -> Result<(), String>;
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MusicConnection {
    Unconfigured,
    Disconnected,
    Authorizing,
    Connected,
    Expired,
    Unavailable,
}
pub trait MusicProvider {
    fn name(&self) -> &str;
    fn connection(&self) -> MusicConnection;
    fn playback_available(&self) -> bool;
}
#[cfg(test)]
mod tests {
    use super::*;
    fn task() -> Task {
        Task {
            id: "t1".into(),
            title: "Kinematics".into(),
            subject: "Physics".into(),
            date: "2026-11-17".into(),
            minutes: 120,
            done: false,
            topic: None,
        }
    }
    #[test]
    fn backup_roundtrip() {
        let mut s = State::default();
        s.tasks.push(task());
        s.notes.push(Note {
            id: "n1".into(),
            title: "Study".into(),
            body: "ψ <script>".into(),
            topic: Some("kinematics".into()),
        });
        assert_eq!(s, import(&export(&s).unwrap()).unwrap());
    }
    #[test]
    fn oversized_export_cannot_produce_an_unrestorable_backup() {
        let mut s = State::default();
        for i in 0..21 {
            s.notes.push(Note {
                id: format!("n{i}"),
                title: "Long note".into(),
                body: "a".repeat(1_000_000),
                topic: None,
            });
        }
        assert!(export(&s).unwrap_err().contains("20 MB"));
    }
    #[test]
    fn corrupt_backup_rejected() {
        let b = export(&State::default())
            .unwrap()
            .replace("NOCTURNE", "PAPER");
        assert!(import(&b).is_err());
        assert!(import("{}").is_err());
    }
    #[test]
    fn future_schema_rejected() {
        let s = State {
            schema: 2,
            ..State::default()
        };
        assert!(validate(&s).is_err());
    }
    #[test]
    fn duplicates_rejected() {
        let s = State {
            tasks: vec![task(), task()],
            ..State::default()
        };
        assert!(validate(&s).is_err());
    }
    #[test]
    fn hinted_not_independent() {
        let a = [
            Outcome::Independent,
            Outcome::Hinted,
            Outcome::Incorrect,
            Outcome::Skipped,
        ]
        .map(|outcome| Attempt {
            id: "a".into(),
            topic: "p".into(),
            outcome,
            seconds: 60,
            date: "2026-11-17".into(),
            mistake: String::new(),
        });
        assert!((accuracy(&a).unwrap() - 100.0 / 3.0).abs() < 0.001);
        assert_eq!(accuracy(&[]), None);
    }
    #[test]
    fn review_intervals_and_failure() {
        let mut c = Card {
            id: "c1".into(),
            front: "F=ma".into(),
            back: "".into(),
            due: "2026-11-17".into(),
            stage: 0,
            reviews: 0,
        };
        review(&mut c, true, "2026-11-17").unwrap();
        assert_eq!(c.due, "2026-11-18");
        review(&mut c, true, "2026-11-18").unwrap();
        assert_eq!(c.due, "2026-11-21");
        review(&mut c, false, "2026-11-21").unwrap();
        assert_eq!(c.due, "2026-11-22");
        assert_eq!(c.stage, 0);
    }
    #[test]
    fn dates_and_limits() {
        assert!(!valid_date("2027-02-29"));
        assert!(valid_date("2028-02-29"));
        let mut s = State::default();
        s.tasks.push(task());
        s.tasks[0].minutes = 0;
        assert!(validate(&s).is_err());
    }
    #[test]
    fn completion_does_not_grant_mastery() {
        let mut s = State::default();
        s.tasks.push(task());
        s.tasks[0].done = true;
        assert!(s.topics.is_empty());
    }
}
