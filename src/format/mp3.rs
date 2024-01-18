use std::fs::File;

struct MP3Format {
    format: u8,
    channels: u8,
    sample_rate: u16,
    bits_per_sample: u8,
    data: Vec<u8>,
}

impl MP3Format {
    fn decode(file: File) -> MP3Format {
        //read file and convert to mp3 format
        MP3Format {
            format: 2,
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            data: Vec::new(),
        }
    }

    fn encode(path: &str) -> File {
        //convert mp3 format to file
        File::create(path).unwrap()
    }
}
