# jakE desktop pet!

(jaketop if you will)

this should work on most major things (windows, mac, linux (kde, GNOME, any non-tiling wm)), but there are some exceptions and complications on some OS' and window managers on linux (i only know how to fix hyprland lmao)

## windows

windows defender will try to nag you about this. i promise you it's not a virus.

## macOS

mac and its stupid (but safe i guess) codesigning so impossibly hard to bypass for a regular user. here's how to do it:

1. open your terminal
2. type the following into your terminal (replacing the path with your actual path to jAkEtop.app):

```sh
xattr -dr com.apple.quarantine /path/to/jAkEtop.app
```

after that, you should be able to just open it up!

## hyprland

put the following window rule in one of your config files:

```lua
hl.window_rule({
	match = { class = "^(jaketop)$" },
	float = true,
	pin = true,
	no_focus = true,
	no_blur = true,
	no_dim = true,
	no_shadow = true,
	no_anim = true,
	border_size = 0,
	rounding = 0,
	opaque = true,
	size = { "monitor_w", "monitor_h" },
	move = "0 0",
})
```
