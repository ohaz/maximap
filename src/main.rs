extern crate captrs;
extern crate minifb;
extern crate tini;
extern crate text_io;
extern crate resize;

use minifb::{Key, Window, WindowOptions};
use captrs::*;
use tini::Ini;
use std::collections::HashMap;
use text_io::read;

#[derive(Default)]
struct Configuration {
    screen_to_record: usize,
    left: usize,
    right: usize,
    top: usize,
    bottom: usize,
    scale: usize,
}

fn get_config() -> Configuration {
    let mut loaded_config: Configuration = Default::default();
    let config = Ini::from_file("config.ini").unwrap();
    loaded_config.screen_to_record = config.get("General", "ScreenToRecord").unwrap();
    let mut sections: HashMap<u16, String> = HashMap::new();
    let mut index = 0;
    println!("Which configuration to load?");
    for (section, _) in config.iter() {
        if section != "General"
        {
            println!("{} - {}", index, section);
            sections.insert(index, section.to_string());
            index += 1;
        }
    }
    let mut read_index: i32 = -1;
    while !(read_index >= 0 && read_index < sections.len() as i32)
    {
        read_index = read!();
    }
    let section_to_load = &sections[&(read_index as u16)];
    println!("Loading section {}", section_to_load);
    for (key, value) in config.iter_section(section_to_load).unwrap() {
        match key.as_str() {
            "left" => loaded_config.left = value.parse::<usize>().unwrap(),
            "right" => loaded_config.right = value.parse::<usize>().unwrap(),
            "top" => loaded_config.top = value.parse::<usize>().unwrap(),
            "bottom" => loaded_config.bottom = value.parse::<usize>().unwrap(),
            "scale" => loaded_config.scale = value.parse::<usize>().unwrap(),
            _ => println!("Invalid configuration line {}={}", key, value),
        }
    }
    loaded_config
}

fn main() {
    let config: Configuration = get_config();

    let mut capturer = Capturer::new(config.screen_to_record).unwrap();
    let _ = capturer.capture_store_frame().unwrap();

    //let (w, h) = capturer.geometry();
    let w = config.right - config.left;
    let h = config.bottom - config.top;
    println!("Creating window with size: {} x {}", w, h);
    let mut buffer: Vec<u32> = vec![0; w * config.scale * h * config.scale];


    let mut window = Window::new(
        "Maximap - ESC to exit",
        w * config.scale,
        h * config.scale,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let mut error_count = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_screen_result = capturer.capture_frame();
        let current_screen: Vec<Bgr8> = match current_screen_result {
            Ok(screen) => screen,
            Err(_) => {if error_count > 3 {panic!("Failed capturing too often")} else {error_count +=1; continue}},
        };
        error_count -= 1;
        let (screen_width, _) = capturer.geometry();
        for i in 0..h {
            for j in 0..w {
                let location = (i + config.top) * screen_width as usize + j + config.left;
                let color: Bgr8 = current_screen[location];
                let value: u32 = ((color.r as u32) << 16) + ((color.g as u32) << 8) + color.b as u32;
                let new_location: usize = i * config.scale * config.scale * w + j * config.scale;
                for si in 0..config.scale {
                    for sj in 0..config.scale {
                        buffer[new_location + sj + (si * w * config.scale)] = value;
                    }
                }
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, w * config.scale, h * config.scale)
            .unwrap();
    }
}
