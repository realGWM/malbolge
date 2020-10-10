pub trait Colors {

    fn true_color(&self, r: u8, g: u8, b: u8) -> String;
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn blue(&self) -> String;
}

impl Colors for str {

    fn true_color(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn red(&self) -> String {
        format!("\x1b[31m{}\x1b[0m", self)
    }

    fn green(&self) -> String {
        format!("\x1b[32m{}\x1b[0m", self)
    }

    fn blue(&self) -> String {
        format!("\x1b[34m{}\x1b[0m", self)
    }
}