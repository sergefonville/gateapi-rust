pub struct Candlestick {
    timestamp: u32,
    volume: f32,
    close: f32,
    highest: f32,
    lowest: f32,
    open: f32,
}

impl Candlestick {
    pub fn new(timestamp: u32,
        volume: f32,
        close: f32,
        highest: f32,
        lowest: f32,
        open: f32) -> Candlestick {
            Candlestick {timestamp, volume, close, highest, lowest, open}
        }
}