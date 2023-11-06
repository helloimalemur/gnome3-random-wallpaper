# gnome3-random-wallpaper
gnome3 random wallpaper

Globs pictures from your /home/user/Pictures folder.

**Build**
```shell
bash -e install.sh
```
<br><br>

**Run as service**

~/.Gnome3-wallpaper-changer
```toml
run_as_service="true"
interval="3"
```
<br><br>

**Run via Cron**

set run_as_service to false
```shell
*/15 * * * * ~/.Gnome3-wallpaper-changer
```
