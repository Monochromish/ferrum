use crate::library::{load_library, Paths};
use crate::library_types::{Library, TrackID, TrackListID};
use crate::page;
use crate::sort::sort;
use atomicwrites::{AllowOverwrite, AtomicFile};
use serde::Serialize;
use std::io::{Error, ErrorKind, Write};
use std::time::Instant;

pub struct Data {
  pub paths: Paths,
  pub is_dev: bool,
  pub library: Library,
  pub open_playlist_track_ids: Vec<TrackID>,
  pub page_track_ids: Option<Vec<TrackID>>,
  pub open_playlist_id: TrackListID,
  pub filter: String,
  pub sort_key: String,
  pub sort_desc: bool,
}

impl Data {
  pub fn save(&mut self) -> Result<(), Error> {
    let mut now = Instant::now();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"	"); // tab

    let mut json = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut json, formatter);
    self.library.serialize(&mut ser)?;
    println!("Stringify: {}ms", now.elapsed().as_millis());

    now = Instant::now();
    let file_path = &self.paths.library_json;
    let af = AtomicFile::new(file_path, AllowOverwrite);
    let result = af.write(|f| f.write_all(&json));
    match result {
      Ok(_) => {}
      Err(err) => {
        return Err(Error::new(
          ErrorKind::Other,
          format!("Error saving: {}", err),
        ))
      }
    }
    println!("Write: {}ms", now.elapsed().as_millis());
    Ok(())
  }
}

pub fn load_data(is_dev: &bool) -> Result<Data, &'static str> {
  let app_name = if *is_dev { "Ferrum Dev" } else { "Ferrum" };

  let music_dir = dirs::audio_dir().ok_or("Music folder not found")?;
  let library_dir = music_dir.join(app_name);
  let library_json_path = library_dir.clone().join("Library.json");

  let paths = Paths {
    library_dir: library_dir.clone(),
    tracks_dir: library_dir.join("Tracks"),
    artworks_dir: library_dir.join("Artworks"),
    library_json: library_json_path,
  };

  let loaded_library = load_library(&paths);

  let mut data = Data {
    is_dev: *is_dev,
    paths: paths,
    library: loaded_library,
    open_playlist_id: "root".to_string(),
    open_playlist_track_ids: vec![],
    page_track_ids: None,
    filter: "".to_string(),
    sort_key: "index".to_string(),
    sort_desc: true,
  };
  data.open_playlist_track_ids = page::get_track_ids(&data)?;
  sort(&mut data, "dateAdded", true)?;
  return Ok(data);
}
