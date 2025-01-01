/* use crate::consts::*;

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, /* OutputStreamHandle, */ Sink};

pub struct AudioPlayer {
    sink: Sink,
    //_stream: OutputStream,
    //stream_handle: OutputStreamHandle,
} 

impl AudioPlayer {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink,
        }
    }

    pub fn play_sound(&self, sound_name: String) {
        let file = BufReader::new(File::open(format!("./sounds/{}.wav", sound_name)).unwrap());
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }
}
 */