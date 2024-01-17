use std::fs::File;

struct WavFormat {
    format: u8,
    channels: u8,
    sample_rate: u16,
    bits_per_sample: u8,
    data: Vec<u8>,
}

impl WavFormat {
    fn decode(file: File) -> WavFormat {
        //read file and convert to wav format
        WavFormat {
            format: 1,
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            data: Vec::new(),
        }
    }

    fn encode(path: &str) -> File {
        //convert wav format to file
        File::create(path).unwrap()
    }
}