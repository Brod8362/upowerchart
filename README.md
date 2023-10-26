# upowerchart

Small rust program to display upower history, intended for i3

!(Example screenshot)[/screenshot.png?raw=true]

# Usage

### 1. Determine the upower device

Enumerate the upower devices available on your system using `upower -d`.

Find the `model` of the one you want to use (note that you cannot use the generic display device as of right now)

Example:

```sh
$ upower -d | grep model
  model:                45N1029
```

In this case, the model needed is 45N1029.

### 2. Run upowerchart

Run upowerchart with the model determined earlier with `upowerchart -d MODEL` (in the above example, `upowerchart -d 45N1029`)

Exit `upowerchart` by left-clicking the window or pressing escape.

### 3. Move to desired location

upowerchart is not capable of relocating itself, and instead relies on the window manager to accomplish this. 

In i3, I used the following steps:

1. Spawn `upowerchart`
2. Move the window where you'd like it to spawn in the future
3. Use `xwininfo` to note the absolute upper-left X and Y coordinates
4. Put the following into your i3 config:

```sh
for_window [title="upowerchart"] floating enable, move position X_POS Y_POS
```

Replacing `X_POS` and `Y_POS` with the absolute x/y coordinates from `xwininfo`.

From now on, the window will spawn at those exact coordinates when launched.

### 4. Bind it however you'd like

For me, I bound it to left-click on the battery block of my i3bar. I use a custom fork of `i3status-rs`, and accomplished
this like so:

```toml
[[block]]
block = "battery"
format = "$icon $percentage $time $power"
full_format = "$icon $percentage $power"
warning = 40
critical = 20
interval = 10

# open battery chart on click
[[block.click]]
button = "left"
cmd = "upowerchart -d 45N1029"
```

# Installation

A `PKGBUILD` is provided. If you're not on Arch, you can probably use `cargo install`.

# Configuration

```sh
  -d,--device DEVICE    device model to use
  -w                    window width [default 300]
  -h                    window height [default 200]
  -t                    display range (hours) [default 3]
  --label-area-size LABEL_AREA_SIZE
                        label area size [default 20]
  --graph-margin GRAPH_MARGIN
                        graph margin [default 10]
  --bottom-margin-extra BOTTOM_MARGIN_EXTRA
                        bottom margin extra [default 10]
  -a,--axis-color AXIS_COLOR
                        axis color [default #FFFFFF]
  -b,--background-color BACKGROUND_COLOR
                        background color [default #000000]
  -p,--percent-color PERCENT_COLOR
                        percent color [default #00FF00]
  -c,--charging-color CHARGING_COLOR
                        charging color [default #00FFFF]
  -x,--discharging-color DISCHARGING_COLOR
                        discharging color [default #FF8800]
```
