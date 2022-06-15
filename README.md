# easymacros

This program is inspired by xmacro, however it isn't xmacro.

Note: This program is for personal use. I will modify it as I deem necessary, but feel free to open issues for suggestions or bug reports. Contributions are welcome! 

## TODOs

- [ ] Playing macros (xmacro like)
  - [x] Delay support
  - [x] KeySym/KeyCode/KeyStr action support
  - [x] MotionNotify and button support
  - [ ] String typing support (Not too high priority, but I'll add it some time probably)
  - [ ] ExecBlock/ExecNoBlock support (not high priority)
- [ ] Recording macros (xmacro like)
  - [ ] Delay
  - [ ] Keyboard actions
  - [ ] Mouse actions

## Ideas

I may or may not do these, but they sound fun to implement to me!

- [ ] additional macro features
  - relative cursor movements
  - screenshotting whole/parts ot the screen (using external programs)
- [ ] Macro daemon kind of thing to listen in the background for keyboard shortcuts?
  - [ ] the daemon itself
  - [ ] config file
  - [ ] rofi integration
  - [ ] dmenu integration
  - [ ] custom macro manager
- [ ] macro language?
  - [ ] Sending keys
  - [ ] mouse movements/events
  - [ ] control flow stuff/math/just normal scripting language stuff
  - [ ] reading/writing to clipboard
  - [ ] calling/recording other macros
  - [ ] running commands
- [ ] GUI Macro editor which is actually user friendly

## Platform support

- [x] Linux x11 (only tested on i3wm)
- [ ] Linux Wayland (makes heavy use of X apis, I will only do this if I myself switch to Wayland. I'm open to suggestions how to do it though!)
- [ ] MacOS (Might work because of XQuartz?)
- [ ] Windows (Yeah, I'm not doing that myself. Unless I have to use Windows for anything.)

## Installation

Currently only manually possible via `cargo build --release` and then moving the result into your $PATH.