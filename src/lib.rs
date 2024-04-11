use hashmap::StationsMap;

use std::fs;

pub mod hashmap;
pub mod measurement;

pub fn parse_file(path: &str) -> StationsMap {
    let mut map = StationsMap::new();

    let buffer = fs::read(path).unwrap();

    let mut i = 0;
    let mut j = 0;
    for byte in buffer.iter() {
        // new line
        if *byte == b'\n' {
            process_line(&buffer[i..j], &mut map);
            i = j + 1;
        }
        j += 1;
    }

    // let mut reader = BufReader::new(file);
    // while let Ok(n) = reader.read_until(b'\n', &mut buf) {
    //     if n == 0 {
    //         // EOF
    //         break;
    //     }

    //     let line = &buf[..n - 1];

    //     process_line(line, &mut map);

    //     buf.clear();
    // }

    map
}

fn process_line(line: &[u8], map: &mut StationsMap) {
    let line = unsafe { String::from_utf8_unchecked(line.to_vec()) };
    if let Some((station, t)) = line.split_once(";") {
        let t = t.parse::<f64>().unwrap();
        map.upsert(station, t);
    }
}
