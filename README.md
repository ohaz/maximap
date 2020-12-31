# maximap
A tool to display the minimap of a game larger on a second screen. Should work on all operating systems supported by `captrs` and `rust_minifb`, so Windows and Linux should work. I have not tested it on Linux yet though.

Attention: This application behaves similar to streaming tools (like OBS). This means it requires quite a bit of CPU resources. On my system (a 6 core @ 3.7Gh) it uses between 4% and 13% of my CPU.

## Screenshots

The console with the game chooser that is shown when maximap starts:
![Console](https://user-images.githubusercontent.com/1176206/103399783-ee210c00-4b42-11eb-8611-7c3fa581c333.png)

The tool in action (it's not looking that much larger because my second screen has a lower resolution):
![Starcraft 2 Example](https://user-images.githubusercontent.com/1176206/103399804-0729bd00-4b43-11eb-9ad0-00db17522c4b.png)

## Compiling
Just run `cargo build`

## Running
To run the application you need a config file in the same folder as the executable.

Take this example config, edit it to your wishes and save it as `config.ini`:

```INI
[General]
ScreenToRecord=0

[Game 1]
top=1000
bottom=1400
left=1000
right=1400
scale=2

[Game 2]
top=1000
bottom=1400
left=1000
right=1400
scale=2
```

ScreenToRecord is the number of the screen to record from. Usually, your primary monitor has the value `0`.
The next sections can define games. You can call them any way you like. Each section needs to contain the following values:

* `top`: The amount of pixels above the minimap
* `bottom`: The amount of pixels beneath the minimap
* `left`: The amount of pixels to the left of the minimap
* `right`: The amount of pixels to the right of the minimap
* `scale`: The amount to scale the minimap by. Can only be full integers at the moment.

An easy way to figure out the `top`, `bottom`, `left` and `right` values is to start the game, create a screenshot and then look at the screenshot in a tool like Paint, Paint.net or any other image editing tool.
Example: You open your screenshot, hover with your mouse to the topmost pixel of the minimap and enter it as `top`. Then you hover to the leftmost pixel and enter it as `left`. The same continues for `right` and `bottom`.

After you have created this ini file, you can simply start the application. A console window should appear, listing all games defined in the `config.ini` file, requesting you to choose one. Just enter the number of the game you want to record.
Afterwards, a new window appears, with the (scaled up) version of the recorded area.
