pub struct Colors {
    colorcodes: [i32; 5],
}

impl Colors {
    pub fn get_color(&self, log_level: usize) -> i32 {
        self.colorcodes[log_level]
    }

    pub fn new() -> Colors {
        Colors {
            colorcodes: [
                37, //WHITE   -> trace
                32, //GREEN   -> info
                33, //YELLOW  -> warn
                31, //RED     -> error & fatal
                0,  //RESET
            ],
        }
    }

    pub fn set_color(&mut self, log_level: usize, color: i32) {
        self.colorcodes[log_level] = color;
    }
}
