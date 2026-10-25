# jakE desktop pet!

## jaketop if you will

this should work on most major things (windows, mac, linux (kde, GNOME, any non-tiling wm)), but since it doesn't work on hyprland and i use it, i can give you direct instructions :3

## how to get this working on hyprland

put the following in one of your config files:

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

does it work if i push from linux?
