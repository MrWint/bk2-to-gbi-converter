#[macro_use] extern crate bitflags;

const SAMPLES_PER_FRAME: u32 = 35112;

use std::fs::File;
use std::io::Read;
use std::os::raw::c_void;

bitflags! {
  pub struct Input: u8 {
    const DOWN   = 0b1000_0000;
    const UP     = 0b0100_0000;
    const LEFT   = 0b0010_0000;
    const RIGHT  = 0b0001_0000;
    const START  = 0b0000_1000;
    const SELECT = 0b0000_0100;
    const B      = 0b0000_0010;
    const A      = 0b0000_0001;
  }
}

extern {
  fn gambatte_create() -> *mut c_void;
  fn gambatte_loadbios(gb: *mut c_void, biosdata: *const u8, biosdatalength: usize);
  fn gambatte_load(gb: *mut c_void, romfiledata: *const u8, romfilelength: usize, flags: u32);
  fn gambatte_destroy(gb: *mut c_void);

  fn gambatte_setvideobuffer(gb: *mut c_void, videobuf: *mut u32, pitch: i32);

  fn gambatte_setrtcdivisoroffset(gb: *mut c_void, rtc_divisor_offset: i32);

  fn gambatte_setinputgetter(gb: *mut c_void, cb: extern fn(*mut c_void, u32) -> u32, target: *mut c_void);

  fn gambatte_runfor(gb: *mut c_void, samples: *mut u32) -> i32;
}

pub struct InputGetter {
  input: Input,
  sample_count: u64,
  input_samples: Vec<(u64, Input)>,
}
extern fn input_getter_fn(context: *mut c_void, sample_offset: u32) -> u32 {
  let input_getter = context as *mut InputGetter;
  unsafe {
    (*input_getter).input_samples.push(((*input_getter).sample_count + u64::from(sample_offset), (*input_getter).input));
    u32::from((*input_getter).input.bits())
  }
}

pub trait ScreenUpdateCallback {
  fn get_screen_buffer_pointer_and_pitch(&self) -> Option<(*mut u32, usize)>;
}

pub struct NoScreen;
impl ScreenUpdateCallback for NoScreen {
  fn get_screen_buffer_pointer_and_pitch(&self) -> Option<(*mut u32, usize)> { None }
}

pub struct Gambatte {
  /// Pointer to gambatte object used to identify the instance in FFI calls.
  gb: *mut c_void,
  input_getter: Box<InputGetter>, // Boxed to place it on the heap with a fixed address for Gambatte to point to.
  pub frame: u32,
  equal_length_frames: bool,
  overflow_samples: u32,
}

impl Gambatte {
  /// Create a new Gambatte instance.
  pub fn create<S: ScreenUpdateCallback + 'static>(bios_file_name: &str, rom_file_name: &str, equal_length_frames: bool, rtc_divisor_offset: i32, screen_update_callback: S) -> Gambatte {
    let bios_data = load_file(bios_file_name).expect("unable to load bios file!");
    let rom_data = load_file(rom_file_name).expect("unable to load rom file!");
    unsafe {
      let gb = gambatte_create();
      gambatte_loadbios(gb, bios_data.as_ptr(), bios_data.len());
      gambatte_load(gb, rom_data.as_ptr(), rom_data.len(), 2 /*GBA_CGB*/);

      gambatte_setrtcdivisoroffset(gb, rtc_divisor_offset);

      let input_getter = Box::new(InputGetter { input: Input::empty(), sample_count: 0, input_samples: vec![] });
      let input_getter_ptr = Box::into_raw(input_getter);
      gambatte_setinputgetter(gb, input_getter_fn, input_getter_ptr as *mut c_void);
      let input_getter = Box::from_raw(input_getter_ptr);

      if let Some((videobuf, pitch)) = screen_update_callback.get_screen_buffer_pointer_and_pitch() {
        gambatte_setvideobuffer(gb, videobuf, pitch as i32);
      }

      Gambatte {
        gb,
        input_getter,
        frame: 0,
        equal_length_frames,
        overflow_samples: 0,
      }
    }
  }

  fn step_internal(&mut self) {
    loop {
      let mut emusamples: u32 = SAMPLES_PER_FRAME - self.overflow_samples;
      
      unsafe { gambatte_runfor(self.gb, (&mut emusamples) as *mut u32); }

      self.overflow_samples += emusamples;
      self.input_getter.sample_count += u64::from(emusamples);

      if !self.equal_length_frames { // old frame timing
        self.overflow_samples = 0; // completed frame
        break;
      }

      if self.overflow_samples >= SAMPLES_PER_FRAME { // new frame timing
        self.overflow_samples -= SAMPLES_PER_FRAME;
        break;
      }
    }
  }
  /// Runs the emulation until the next frame (as defined by BizHawk's timing).
  pub fn step(&mut self, input: Input) {
    self.input_getter.input = input;
    self.step_internal();
  }

  /// List of sample count and input which were read so far.
  #[inline]
  pub fn get_input_samples(&self) -> Vec<(u64, Input)> {
    self.input_getter.input_samples.clone()
  }
}

/// Helper function to load the byte contents of a file into memory.
fn load_file(file_name: &str) -> std::io::Result<Vec<u8>> {
  let mut result: Vec<u8> = vec![];
  let mut f = File::open(file_name)?;
  f.read_to_end(&mut result)?;
  Ok(result)
}

impl Drop for Gambatte {
    fn drop(&mut self) {
        unsafe { gambatte_destroy(self.gb); }
    }
}