use std::collections::HashMap;
use std::fmt::format;
use std::{process, thread};
use std::time::Duration;
use walkdir::{DirEntry, WalkDir};
use rand::prelude::*;

fn main() {
    let user_bind = String::from_utf8(process::Command::new("whoami").output().unwrap().stdout).unwrap();
    let user = user_bind.trim();
    let config_path = format!("/home/{}/.config/Gnome3-wallpaper-changer/Settings.toml", user);

    println!("{}", config_path);
    // load config
    let settings = config::Config::builder()
        .add_source(config::File::with_name(config_path.as_str()))
        .build()
        .unwrap();
    let settings_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    let interval = settings_map.get("interval").unwrap();
    let run_as_service = settings_map.get("run_as_service").unwrap();
    let user = settings_map.get("user").unwrap();
    let wallpapers_folder = settings_map.get("wallpapers_folder").unwrap();





    // let user = String::from_utf8(process::Command::new("whoami").output().unwrap().stdout).unwrap();
    let path = format!("/home/{}{}", user.trim(), wallpapers_folder.trim());
    let walker = WalkDir::new(path).into_iter();
    let mut vec_of_wallpaper: Vec<String> = vec![];
    for mut entry in walker.filter_entry(|e| !is_hidden(e)) {
        if entry.as_mut().unwrap().path().is_file().clone() {
            // println!("{}", entry.as_mut().unwrap().path().display().to_string().clone());
            let new_string = entry.as_mut().unwrap().path().display().to_string().clone();
            vec_of_wallpaper.push(entry.unwrap().clone().path().display().to_string())
        }
    }

    if run_as_service.contains("true") {
        loop {
            set_random_wallpaper(vec_of_wallpaper.clone());
            thread::sleep(Duration::new(interval.parse::<u64>().unwrap(), 0))
        }
    } else {
        set_random_wallpaper(vec_of_wallpaper.clone());
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn set_random_wallpaper(vec_of_wallpaper: Vec<String>) {


    let size = vec_of_wallpaper.len();
    let mut x = thread_rng();
    let rand = x.gen_range(0..size);


    println!("{}", vec_of_wallpaper.get(rand).unwrap());

    let file_uri = format!("file:///{}", vec_of_wallpaper.get(rand).unwrap());

    println!("{}", file_uri);
    let _ = process::Command::new("/usr/bin/gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(file_uri.clone())
        .output()
        .unwrap()
        .stdout;
}
