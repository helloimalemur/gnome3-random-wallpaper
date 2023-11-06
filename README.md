# gnome3-random-wallpaper
gnome3 random wallpaper

Globs pictures from your /home/user/Pictures folder.

**Build**
```shell
cargo build --release
```
<br><br>

**Run as service**

config/Settings.toml
```toml
run_as_service="true"
interval="3"
```
<br><br>

**Run via Cron**

set run_as_service to false
```shell
*/15 * * * * /path/to/repo/gnome3-wallpaper-changer/target/release/gnome3-wallpaper-changer
```
