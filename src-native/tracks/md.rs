use super::tag::SetInfoError;
use super::{generate_filename, Tag};
use crate::library_types::Track;
use crate::{str_to_option, UniResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TrackMD {
  name: String,
  artist: String,
  albumName: String,
  albumArtist: String,
  composer: String,
  grouping: String,
  genre: String,
  year: String,
  trackNum: String,
  trackCount: String,
  discNum: String,
  discCount: String,
  bpm: String,
  comments: String,
}

pub fn update_track_info(
  tracks_dir: &PathBuf,
  track: &mut Track,
  tag: &mut Tag,
  new_info: TrackMD,
) -> UniResult<()> {
  let old_path = tracks_dir.join(&track.file);
  if !old_path.exists() {
    panic!("File does not exist: {}", track.file);
  }
  let ext = old_path.extension().unwrap_or_default().to_string_lossy();

  // name
  match new_info.name.as_ref() {
    "" => tag.remove_title(),
    value => tag.set_title(value),
  };
  let new_name = new_info.name.clone();

  // artists
  match new_info.artist.as_ref() {
    "" => tag.remove_artists(),
    value => tag.set_artist(value),
  };
  let new_artist = new_info.artist.clone();

  // album_name
  match new_info.albumName.as_ref() {
    "" => tag.remove_album(),
    value => tag.set_album(value),
  };
  let new_album_name = str_to_option(new_info.albumName);

  // album_artist
  match new_info.albumArtist.as_ref() {
    "" => tag.remove_album_artists(),
    value => tag.set_album_artist(value),
  };
  let new_album_artist = str_to_option(new_info.albumArtist);

  // composer
  match new_info.composer.as_ref() {
    "" => tag.remove_composers(),
    value => tag.set_composer(value),
  };
  let new_composer = str_to_option(new_info.composer);

  // grouping
  match new_info.grouping.as_ref() {
    "" => tag.remove_groupings(),
    value => tag.set_grouping(value),
  };
  let new_grouping = str_to_option(new_info.grouping);

  // genre
  match new_info.genre.as_ref() {
    "" => tag.remove_genres(),
    value => tag.set_genre(value),
  };
  let new_genre = str_to_option(new_info.genre);

  // year
  let new_year_i32 = match new_info.year.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid year"),
    },
  };
  let new_year_i64 = new_year_i32.map(|n| i64::from(n));
  match new_year_i32 {
    None => tag.remove_year(),
    Some(value) => tag.set_year(value),
  };

  // track_number, track_count
  let new_track_number: Option<u32> = match new_info.trackNum.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid track number"),
    },
  };
  let new_track_count: Option<u32> = match new_info.trackCount.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid track count"),
    },
  };
  match tag.set_track_info(new_track_number, new_track_count) {
    Ok(()) => {}
    // don't set tag at all if number is required
    Err(SetInfoError::NumberRequired) => tag.set_track_info(None, None)?,
    Err(e) => Err(e)?,
  }

  // disc_number, disc_count
  let new_disc_number: Option<u32> = match new_info.discNum.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid disc number"),
    },
  };
  let new_disc_count: Option<u32> = match new_info.discCount.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid disc count"),
    },
  };
  match tag.set_disc_info(new_disc_number, new_disc_count) {
    Ok(()) => {}
    // don't set tag at all if number is required
    Err(SetInfoError::NumberRequired) => tag.set_disc_info(None, None)?,
    Err(e) => Err(e)?,
  };

  let new_bpm: Option<u16> = match new_info.bpm.as_ref() {
    "" => None,
    value => match value.parse() {
      Ok(n) => Some(n),
      Err(_) => throw!("Invalid bpm"),
    },
  };
  match new_bpm {
    None => tag.remove_bpm(),
    Some(value) => tag.set_bpm(value),
  };

  // comment
  match new_info.comments.as_ref() {
    "" => tag.remove_comments(),
    value => tag.set_comment(value),
  };
  let new_comments = str_to_option(new_info.comments);

  // save tag
  tag.write_to_path(&old_path);

  // move file
  if new_name != track.name || new_artist != track.artist {
    let new_filename = generate_filename(&tracks_dir, &new_artist, &new_name, &ext);
    let new_path = tracks_dir.join(&new_filename);
    match fs::rename(old_path, new_path) {
      Ok(_) => {
        track.file = new_filename;
      }
      Err(_) => {}
    }
  }

  track.name = new_name;
  track.artist = new_artist;
  track.albumName = new_album_name;
  track.albumArtist = new_album_artist;
  track.composer = new_composer;
  track.grouping = new_grouping;
  track.genre = new_genre;
  track.year = new_year_i64;
  track.trackNum = new_track_number;
  track.trackCount = new_track_count;
  track.discNum = new_disc_number;
  track.discCount = new_disc_count;
  track.bpm = new_bpm.map(|n| n.into());
  track.comments = new_comments;

  Ok(())
}
