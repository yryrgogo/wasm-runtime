struct VM {
    frames: Vec<Frame>,

    stack: Vec<u8>,
    sp: usize,
}

struct Frame {
    locals: Vec<u8>,
}
