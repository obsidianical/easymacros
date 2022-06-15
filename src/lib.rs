extern crate core;

pub mod x11_safe_wrapper;

pub enum XMacroInstructions {
    Delay,
    ButtonPress,
    ButtonRelease,
    MotionNotify,
    KeyCodePress,
    KeyCodeRelease,
    // Screenshot,
    KeySymPress,
    KeySymRelease,
    KeySym,
    KeyStrPress,
    KeyStrRelease,
    KeyStr,
    String,
    // ExecBlock,
    // ExecNoBlock,
}
