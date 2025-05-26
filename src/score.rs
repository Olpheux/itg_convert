use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct Stats {
    #[serde(rename = "SongScores", default)]
    pub song_scores: SongScores,
}

#[derive(Debug, Deserialize, Default)]
pub struct SongScores {
    #[serde(rename = "Song", default)]
    pub songs: Vec<Song>,
}

#[derive(Debug, Deserialize)]
pub struct Song {
    #[serde(rename = "Dir", default)]
    pub dir: String,
    #[serde(rename = "Steps", default)]
    pub steps: Vec<Steps>,
}

#[derive(Debug, Deserialize)]
pub struct Steps {
    #[serde(rename = "Difficulty", default)]
    pub difficulty: String,
    #[serde(rename = "StepsType", default)]
    pub steps_type: String,
    #[serde(rename = "HighScoreList")]
    pub high_score_list: HighScoreList,
}

#[derive(Debug, Deserialize)]
pub struct HighScoreList {
    #[serde(rename = "HighScore", default)]
    pub scores: Vec<HighScore>,
}

#[derive(Debug, Deserialize)]
pub struct HighScore {
    #[serde(rename = "Score")]
    pub score: u32,
    #[serde(rename = "TapNoteScores")]
    pub tap_note_scores: TapNoteScores,
}

#[derive(Debug, Deserialize)]
pub struct TapNoteScores {
    #[serde(rename = "Miss")]
    pub miss: u32,
    #[serde(rename = "W5")]
    pub w5: u32,
    #[serde(rename = "W4")]
    pub w4: u32,
    #[serde(rename = "W3")]
    pub w3: u32,
    #[serde(rename = "W2")]
    pub w2: u32,
    #[serde(rename = "W1")]
    pub w1: u32,
}

#[derive(Clone, Copy)]
pub struct ITGScore {
    pub bf: u32,
    pub fa: u32,
    pub ex: u32,
    pub gr: u32,
    pub de: u32,
    pub wo: u32,
    pub mi: u32
}

impl ITGScore {
    pub fn note_count(&self) -> u32 {
        self.bf + self.fa + self.ex + self.gr + self.de + self.wo + self.mi
    }
}

#[derive(Clone)]
pub struct DDRScore {
    pub ma: u32,
    pub pe: u32,
    pub gr: u32,
    pub go: u32,
    pub mi: u32
}

#[derive(Clone)]
pub struct Score {
    pub name: String,
    pub diff: String,
    pub itg_score: ITGScore,
    pub ddr_score: DDRScore,
    pub total_notes: u32,
    pub ddr_points: u32,
    pub ddr_grade: String
}