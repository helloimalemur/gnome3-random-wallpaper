#!/bin/bash

if [[ -d "$HOME"/.config/Gnome3-wallpaper-changer/ ]]; then echo 'exists'; else mkdir "$HOME"/.config/Gnome3-wallpaper-changer/; fi
cp -r config/* "$HOME"/.config/Gnome3-wallpaper-changer/
cargo build --release
cp target/release/Gnome3-wallpaper-changer "$HOME"/.Gnome3-wallpaper-changer
