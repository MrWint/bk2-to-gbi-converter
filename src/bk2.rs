use gambatte::Input;
use std::io::{BufRead, BufReader, Read};

pub fn read_bk2_inputs(file_name: &str) -> zip::result::ZipResult<Vec<Input>> {
  let path = std::path::Path::new(file_name);
  let file = std::fs::File::open(&path).unwrap();
  let mut archive = zip::ZipArchive::new(file)?;

  let mut result: Vec<Input> = vec![];
  let file = archive.by_name("Input Log.txt")?;
  let file = BufReader::new(file);
  for line in file.lines() {
    let l = line.unwrap();
    if !l.starts_with('|') { continue; }
    let mut input = Input::empty();
    if l.contains('D') { input |= Input::DOWN; }
    if l.contains('U') { input |= Input::UP; }
    if l.contains('L') { input |= Input::LEFT; }
    if l.contains('R') { input |= Input::RIGHT; }
    if l.contains('S') { input |= Input::START; }
    if l.contains('s') { input |= Input::SELECT; }
    if l.contains('B') { input |= Input::B; }
    if l.contains('A') { input |= Input::A; }
    result.push(input);
  }
  Ok(result)
}

pub fn read_bk2_sync_settings(file_name: &str) -> zip::result::ZipResult<(bool, i32)> {
  let path = std::path::Path::new(file_name);
  let file = std::fs::File::open(&path).unwrap();
  let mut archive = zip::ZipArchive::new(file)?;

  let mut file = archive.by_name("SyncSettings.json")?;
  let mut sync_settings_bytes: Vec<u8> = vec![];
  file.read_to_end(&mut sync_settings_bytes)?;

  let sync_settings = json::parse(std::str::from_utf8(&sync_settings_bytes).expect("SyncSettings are not a valid UTF-8 string")).expect("SyncSettings are not valid JSON");

  let input_type = &sync_settings["o"]["$type"].as_str().expect("SyncSettings: $type is not a valid string");
  assert!(input_type.contains("GambatteSyncSettings"), "SyncSettings: Input file does not appear to be a Gambatte movie");
  assert!(sync_settings["o"]["GBACGB"].as_bool() == Some(true), "SyncSettings: Moive does not use CGB in GBA mode");

  let rtc_offset: i32 = sync_settings["o"]["RTCDivisorOffset"].as_i32().unwrap_or_else(|| { eprintln!("WARNING: no valid RTC offset found in SyncSettings, assuming 0."); 0 });
  let efl: bool = sync_settings["o"]["EqualLengthFrames"].as_bool().unwrap_or_else(|| { eprintln!("WARNING: no valid EqualLengthFrames found in SyncSettings, assuming false."); false });

  Ok((efl, rtc_offset))
}
