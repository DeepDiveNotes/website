pub mod search;

use failure::Error;
use std::cmp::Ord;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Load all seasons in a specified directory
pub fn read_seasons_from_path(path: &str) -> Vec<Season> {
    let mut seasons = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".json") {
                    let season = get_season_from_file(entry.path());

                    match season {
                        Ok(season) => {
                            return Some(season);
                        }
                        Err(err) => {
                            println!("Could not parse {:?}", err);
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<Season>>();

    seasons.sort_by(|left, right| left.title.cmp(&right.title));

    seasons
}

/// Load a season from a json file
pub fn get_season_from_file(path: &Path) -> Result<Season, Error> {
    let mut file = File::open(path)?;

    Ok(serde_json::from_reader(&mut file)?)
}

/// A season is represents a deep dive topic
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Season {
    pub id: usize,
    pub title: String,
    pub episodes: Vec<Episode>,
}

/// Individual episode of a season
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Episode {
    pub id: usize,
    pub title: String,
    pub twitch_video_id: String,
    pub notes: Vec<Note>,
}

/// Individual note for an episode
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note {
    pub description: String,
    pub timestamp: u64,
}
