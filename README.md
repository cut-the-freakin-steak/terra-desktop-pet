# jakE desktop pet!

(jaketop if you will)

this should work on most major things (windows, mac, linux (kde, GNOME, any non-tiling wm)), but there are some exceptions and complications on some OS' and window managers on linux (i only know how to fix hyprland lmao)

## windows

windows defender will try to nag you about this. i promise you it's not a virus.
another thing, if you have an [NVIDIA GPU](https://github.com/bevyengine/bevy/issues/7544#issuecomment-2840720210), it may be broken as hell. [there's an open issue for this in bevy](https://github.com/bevyengine/bevy/issues/7544), so it's out of my hands.

here's how to fix it:

1. open NVIDIA Control Panel
2. go to "Manage 3D Settings"
3. in there, in the little window that says "global settings", scroll all the way down
4. make the value for the option "Vulkan/OpenGL present method" be "Prefer native"

after that, you should be able to just open it up and have it work!

## macOS

mac and its stupid (but safe i guess) codesigning so impossibly hard to bypass for a regular user.

here's how to fix it:

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
