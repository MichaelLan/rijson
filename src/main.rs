use std::fs::File;
use std::io::prelude::*;
use std::io::{self, SeekFrom};

fn main() -> io::Result<()> {
    let filepath = "/home/michaellan/Development/personal/git/rijon/.output/simple.json";
    let finalpath = "/home/michaellan/Development/personal/git/rijon/.output/final.json";

    let f = File::open(filepath)?;
    let save: File = File::create(finalpath)?;
    convert_to_jsonline(f, save)?;
    
    Ok(())
}

fn convert_to_jsonline(mut file: File, save_file: File) -> io::Result<()> {
    const OPEN_SQUARE_BRACKET: u8 = b'[';
    const CLOSE_SQUARE_BRACKET: u8 = b']';
    const OPEN_BRACKET: u8 = b'{';
    const CLOSE_BRACKET: u8 = b'}';

    let mut counter_square_brackets: isize = 0;
    let mut counter_brackets: isize = 0;
    
    let special_chars = [b' ', b'\t', b'\n'];
    
    let mut fisrt_iteration = true;
    let end_seek = file.seek(SeekFrom::End(0))?;
    let current_position = 0;
    file.seek(SeekFrom::Start(current_position))?;

    let mut jsonline = Vec::new();
    write_line_in_file(&[b'[', b'\n'], &save_file);
    'main: loop {
        let mut buffer = [0; 1];
        file.read_exact(&mut buffer)?;

        if fisrt_iteration {
            fisrt_iteration = false;
            if buffer[0] == OPEN_SQUARE_BRACKET {
                continue 'main;
            }
        }

        if special_chars.contains(&buffer[0]) {
            continue 'main;
        }

        // Counting brackets and square_brackets
        // Bien generico usando match
        match buffer[0] {
            OPEN_BRACKET => {counter_brackets += 1;},
            CLOSE_BRACKET => {counter_brackets -= 1;},
            OPEN_SQUARE_BRACKET => {counter_square_brackets += 1;},
            CLOSE_SQUARE_BRACKET => {counter_square_brackets -= 1;},
            _ => {},
        }

        jsonline.push(buffer[0]);
        if counter_brackets == 0 && counter_square_brackets == 0 {
            // TODO: Escribir en el archivo final
            write_line_in_file(&jsonline, &save_file);
            write_line_in_file(&[b',', b'\n'], &save_file);
            // println!(r#"{}"#, String::from_utf8(jsonline).unwrap());
            jsonline = Vec::new();
            
            'commas: loop {
                // TODO: Mejorar la lectura del final del archivo, DRY.
                let end_position = file.stream_position()?;
                if end_position == end_seek {
                    write_line_in_file(&[b']'], &save_file);
                    break 'main;
                }

                file.read_exact(&mut buffer)?;
                if OPEN_BRACKET == buffer[0] || CLOSE_SQUARE_BRACKET == buffer[0]{
                    counter_brackets += 1;
                    jsonline.push(buffer[0]);
                    break 'commas;
                } else {
                    continue 'commas;
                }
            }
        }
        
        // TODO: Mejorar la lectura del final del archivo, DRY.
        let end_position = file.stream_position()?;
        if end_position == end_seek {
            write_line_in_file(&[b']'], &save_file);
            break 'main;
        }
    }

    Ok(())
}

fn write_line_in_file(buffer: &[u8], mut file: &File) -> () {
    match file.write_all(&buffer){
        Ok(_) => {},
        Err(error) => {panic!("Algo ocurrio, {error}")},
    }
}
