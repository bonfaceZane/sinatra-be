
pub struct Track {
	title: String,
	artist: String,
	album: String,
	track_number: u32,
	year: u32,
}

pub struct TrackList {
	tracks: Vec<Track>,
}

