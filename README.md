# jakE desktop pet!

(jAkEtop if you will)

this should work on most major operating systems (windows, mac, linux with kde; GNOME; any non-tiling wm that uses wayland or x11), but there are some exceptions and complications on certain OS' and window managers on linux. look below for troubleshooting help. :3

# troubleshooting

## windows

### windows defender

windows defender will try to nag you about this. i _promise_ you it's not a virus. just click open anyways.

### window transparency

there is a currently open [bevy issue](https://github.com/bevyengine/bevy/issues/7544) relating to windows and window transparency, so most bugs that happen that are about this are out of my control.

the windows operating system has trouble running transparent windows. y'know, transparent. like most windows are.
_funny._

if you have an [NVIDIA](https://github.com/bevyengine/bevy/issues/7544#issuecomment-2840720210) GPU:
it may be broken as hell. but there is a solution!

1. open NVIDIA Control Panel
2. go to "Manage 3D Settings"
3. in there, in the little window that says "global settings", scroll all the way down
4. make the value for the option "Vulkan/OpenGL present method" be "Prefer native"

enjoy!

if you have an AMD GPU:
im sorry.

unfortunately, due to some some inexplicable shadow magic, AMD simply does not work sometimes. either the window is not transparent, or it does work but there are very weird artifacts that happen with the sprite, and i cannot for the life of me find the root cause. i do not have a fix for any of these issues for the time being, so unfortunately all the advice i can give you if you're on windows and you have an AMD GPU is to update your graphics drivers, pick a god, and pray.

## macOS

mac and its stupid (but safe i guess) codesigning so impossibly hard to bypass for a regular user.

1. open your terminal
2. type the following into your terminal (replacing the path with your actual path to jAkEtop.app):

```sh
xattr -dr com.apple.quarantine /path/to/jAkEtop.app
```

after that, you should be able to just open it up!

## linux

ahhh, linux. linux is bliss. it worked first try with my transparent window setup. there's no weird codesigning or anything of the sort. i love FOSS. <3
there are some edge cases where the overlay window doesnt work though, and that's with tiling WMs.

### hyprland

put the following window rule into one of your config files:

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

if you have any other window managers with which this app doesn't work, feel free to open an issue (or a pull request!!! :3) describing your problem and potential fixes for it. i would really appreciate it. thank you!!!

# building

if you wanna build the project yourself, you'll have to install the [dioxus-cli](https://docs.rs/crate/dioxus-cli/latest), and the [just command runner](https://github.com/casey/just) (if you don't wanna install just, you can simply look in the Justfile for the commands just is replacing).
all of the commands you need are in the Justfile! some important ones are:

- `just build`, for building
- `just run`, for running
- `just hot`, for hot reloading runs of the app, useful for iteration
