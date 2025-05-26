pub mod score;

use quick_xml::de::from_str;
use std::{fs, vec, env};

fn get_scores(strict_mode: bool) -> Vec<score::Score> {
    let mut scores: Vec<score::Score> = vec![];

    let xml_content = fs::read_to_string("Stats.xml").unwrap();
    let stats: score::Stats = from_str(&xml_content).unwrap();

    for song in stats.song_scores.songs {
        for step in song.steps {
            for score in step.high_score_list.scores {

                let current_itg_score = score::ITGScore {
                    bf: score.score,
                    fa: score.tap_note_scores.w1 - score.score,
                    ex: score.tap_note_scores.w2,
                    gr: score.tap_note_scores.w3,
                    de: score.tap_note_scores.w4,
                    wo: score.tap_note_scores.w5,
                    mi: score.tap_note_scores.miss,
                };

                let current_ddr_score = match strict_mode {
                    true => score::DDRScore { 
                        ma: current_itg_score.bf,
                        pe: current_itg_score.fa,
                        gr: current_itg_score.gr + current_itg_score.ex,
                        go: current_itg_score.de,
                        mi: current_itg_score.wo + current_itg_score.mi,
                    },
                    false => score::DDRScore {
                        ma: current_itg_score.bf,
                        pe: current_itg_score.fa + (current_itg_score.ex as f32 * 0.7).floor() as u32,
                        gr: (current_itg_score.ex as f32 * 0.3).floor() as u32 + (current_itg_score.gr as f32 * 0.7).floor() as u32,
                        go: (current_itg_score.gr as f32 * 0.3).floor() as u32 + current_itg_score.de,
                        mi: current_itg_score.wo + current_itg_score.mi,
                    }
                };

                let current_score = score::Score {
                    name: song.dir.clone(),
                    diff: step.difficulty.clone(),
                    itg_score: current_itg_score,
                    ddr_score: current_ddr_score,
                    total_notes: current_itg_score.note_count(),
                    ddr_points: 0,
                    ddr_grade: "".to_owned(),
                };

                scores.push(current_score);
            }
        }
    }

    scores
}

fn get_points(scores: Vec<score::Score>) -> Vec<score::Score> {
    let mut modified_scores: Vec<score::Score> = vec![];

    for mut score in scores {
        let step_score = 1000000 / score.total_notes;
        let score_per_marv = step_score;
        let score_per_perf = step_score - 10;
        let score_per_great = (step_score as f32 * 0.6).floor() as u32 - 10;
        let score_per_good = (step_score as f32 * 0.2).floor() as u32 - 10;

        // strange /10*10 at the end is because the game shows
        // scores as a multiple of 10
        score.ddr_points = (((score.ddr_score.ma * score_per_marv) +
                           (score.ddr_score.pe * score_per_perf) +
                           (score.ddr_score.gr * score_per_great) +
                           (score.ddr_score.go * score_per_good)) / 10) * 10;
        
        score.ddr_grade = match score.ddr_points {
            990000..=1000000 => "AAA".to_string(),
            950000..=989990 => "AA+".to_string(),
            900000..=949990 => "AA".to_string(),
            890000..=899990 => "AA-".to_string(),
            850000..=889990 => "A+".to_string(),
            800000..=849990 => "A".to_string(),
            790000..=799990 => "A-".to_string(),
            750000..=789990 => "B+".to_string(),
            700000..=749990 => "B".to_string(),
            690000..=699990 => "B-".to_string(),
            650000..=689990 => "C+".to_string(),
            600000..=649990 => "C".to_string(),
            590000..=599990 => "C-".to_string(),
            550000..=589990 => "D+".to_string(),
            0..=549990 => "D".to_string(),
            _ => "E".to_string()
        };

        modified_scores.push(score.clone());
    }

    modified_scores
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_strict_mode: bool;

    if &args[0].to_lowercase() == "strict" {
        is_strict_mode = true;
    } else {
        is_strict_mode = false;
    }

    let scores: Vec<score::Score> = get_scores(is_strict_mode);
    let modified_scores = get_points(scores);

    for score in modified_scores {
        println!("{} | Difficulty: {} | DDR Score: {} | {}",
                score.name,
                score.diff,
                score.ddr_points,
                score.ddr_grade);
    }
}
