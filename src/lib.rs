use rodio::{source::SineWave, OutputStream, Source};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, BufReader, Write};
use std::path::Path;
use std::result;
use std::time::Duration;

pub type Result<T> = result::Result<T, Box<dyn Error>>;

const DIT: u64 = 50;
const TONE_FREQUENCY: f32 = 800.00;

macro_rules! err {
    ($($tt:tt)*) => {
        Err(Box::<dyn Error>::from(format!($($tt)*)))
    };
}

fn load_dictionary<P: AsRef<Path>>(path: P) -> Result<HashMap<char, String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let dictionary = serde_json::from_reader(reader)?;
    Ok(dictionary)
}

fn speak_letter(signal: &char) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let duration = match signal {
        '-' => 3 * DIT, // 0.5s
        '.' => DIT,     // 0.2s
        _ => return err!("test"),
    };
    let source = SineWave::new(TONE_FREQUENCY)
        // Play for 1 second
        .take_duration(Duration::from_millis(duration))
        // Convert to a source object
        .amplify(0.05); // Lower the volume to 5%
                        // Play the sound
    stream_handle.play_raw(source.convert_samples())?;
    // Keep the program alive while the sound is playing
    std::thread::sleep(Duration::from_millis(duration));
    Ok(())
}

pub fn speak(code: &str) -> Result<()> {
    write!(stdout(), "Speaking...")?;
    stdout().flush()?; // write now
                       // load dictionary
    let dictionary = load_dictionary("dictionary.json")?;
    for c in code.chars() {
        if c.is_whitespace() {
            std::thread::sleep(Duration::from_millis(3 * DIT)); // three dits between words
        } else if c == '.' {
            std::thread::sleep(Duration::from_millis(7 * DIT)); // seven dits between sentences
        } else {
            let letter = dictionary
                .get(&c)
                .ok_or(format!("dictionary does not contain letter {c}"))?;
            for signal in letter.chars() {
                speak_letter(&signal)?;
                std::thread::sleep(Duration::from_millis(DIT));
            }
        }
    }
    write!(stdout(), "Done!\n")?;
    Ok(())
}
