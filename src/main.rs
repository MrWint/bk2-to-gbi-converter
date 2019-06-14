#![feature(ptr_internals)]

mod bk2;
#[cfg(feature = "sdl")] mod sdl;

use clap::{App, Arg, value_t};
#[cfg(feature = "sdl")] use crate::sdl::*;
use gambatte::*;

const SAMPLE_OFFSET: u64 = 484_500; // This value represents the delay between the GBP powering on and the GB staring to execute.

fn main() {
  let app = App::new("bk2-to-gbi-converter")
              .version("0.1")
              .arg(Arg::with_name("bios")
                .long("bios")
                .value_name("BIOS_FILE")
                .help("path to the GBC bios image")
                .default_value("gbc_bios.bin")
                .takes_value(true))
              .arg(Arg::with_name("ROM_FILE")
                .help("path to the game rom image")
                .required(true)
                .index(1))
              .arg(Arg::with_name("BK2_FILE")
                .help("path to the bk2 input file")
                .required(true)
                .index(2));
  #[cfg(feature = "sdl")] let app = app.arg(Arg::with_name("scale")
                                      .short("s")
                                      .long("scale")
                                      .value_name("SCALE")
                                      .help("Scales for the SDL output")
                                      .default_value("3")
                                      .takes_value(true));
  let matches = app.get_matches();

  let bios_file_name = matches.value_of("bios").unwrap();
  let rom_file_name = matches.value_of("ROM_FILE").unwrap();
  let bk2_file_name = matches.value_of("BK2_FILE").unwrap();

  let (equal_length_frames, rtc_divisor_offset) = bk2::read_bk2_sync_settings(bk2_file_name).expect("unable to read bk2 input file!");

  #[cfg(feature = "sdl")] let sdl = Sdl::init_sdl(value_t!(matches, "scale", u32).unwrap());
  #[cfg(feature = "sdl")] let screen = SdlScreen::new(sdl.clone());
  #[cfg(not(feature = "sdl"))] let screen = NoScreen {};
  let mut gb = Gambatte::create(bios_file_name, rom_file_name, equal_length_frames, rtc_divisor_offset, screen);
  let inputs = bk2::read_bk2_inputs(bk2_file_name).expect("unable to read bk2 input file!");

  for input in inputs { gb.step(input); }
  for _ in 0..1000 { gb.step(Input::empty()); } // Run past end of inputs a bit to capture any neutral input that may be necessary.

  let mut cur_sample = 0;
  let mut cur_input = Input::empty();
  println!("{:08X} {:04X}", 0, 0); // initial input is neutral
  for (sample, input) in gb.get_input_samples() {
    if input != cur_input {
      let gbi_time = (sample + SAMPLE_OFFSET + cur_sample + SAMPLE_OFFSET) >> 10;
      println!("{:08X} {:04X}", gbi_time, input.bits());
      cur_input = input;
    }
    cur_sample = sample;
  }
}
