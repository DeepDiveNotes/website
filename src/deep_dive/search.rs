use super::{Episode, Note, Season};
use rayon::prelude::*;

use regex::Regex;

/// Search methods
pub trait Search {
	type SearchResult;

	/// Search a given struct for a result
    fn search(&self, query: &Regex) -> Option<Self::SearchResult>;
}

impl Search for Vec<super::Season> {
	type SearchResult = Vec<SeasonSearchResult>;

	/// Search seasons in parallel, return aggregate of results
    fn search(&self, query: &Regex) -> Option<Vec<SeasonSearchResult>> {
        let results = self.par_iter()
            .filter_map(|season| season.search(query))
            .fold(
                || Vec::new(),
                |mut collector, results| {
                    collector.push(results);
                    collector
                },
            )
            .reduce(
                || Vec::new(),
                |mut a, mut b| {
                    a.append(&mut b);
                    a
                },
            );

		Some(results)
    }
}

impl Search for Season {
	type SearchResult = SeasonSearchResult;

	/// Search episodes in season in parallel, return aggregate of results
	fn search(&self, query: &Regex) -> Option<SeasonSearchResult> {
        let results = self.episodes.par_iter()
			.filter_map(|episode| episode.search(query))
			.fold(
				|| Vec::new(),
				|mut collector, results| {
					collector.push(results);
					collector
				},
			)
			.reduce(
				|| Vec::new(),
				|mut a, mut b| {
					a.append(&mut b);
					a
				},
			);

		if results.is_empty() {
			return None;
		}

		Some(SeasonSearchResult {
			title: self.title.clone(),
			id: self.id,
			episode_results: results,
		})
    }
}

impl Search for Episode {
	type SearchResult = EpisodeSearchResult;

	/// Search for notes in an episode
	fn search(&self, query: &Regex) -> Option<EpisodeSearchResult> {
		let results = self.notes.iter().filter_map(|note| note.search(query)).collect::<Vec<Note>>();

		if results.is_empty() {
			return None;
		}

		Some(EpisodeSearchResult {
			title: self.title.clone(),
			id: self.id,
			note_results: results,
		})
	}
}

impl Search for Note {
	type SearchResult = Note;

	fn search(&self, query: &Regex) -> Option<Note> {
		if query.is_match(&self.description) {
			Some(self.clone())
		} else {
			None
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonSearchResult {
	title: String,
	id: usize,
	episode_results: Vec<EpisodeSearchResult>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodeSearchResult {
	title: String,
	id: usize,
	note_results: Vec<Note>
}