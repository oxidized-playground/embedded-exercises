pub trait Calculator {
    fn whothis(&self);
    fn add(&self, x: i16, y: i16) -> i16;
    fn subtract(&self, x: i32, y: i32) -> i32;
    fn multiply(&self, x: u32, y: u32) -> u32;
}
