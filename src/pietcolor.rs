#[cfg(feature = "default")]
extern crate termion;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lightness {
    Light,
    Normal,
    Dark,
}

impl Lightness {
    fn diff_to(self, other: &Lightness) -> usize {
        use self::Lightness::*;
        match self {
            Light => {
                match *other {
                    Light => 0,
                    Normal => 1,
                    Dark => 2,
                }
            },
            Normal => {
                match *other {
                    Light => 2,
                    Normal => 0,
                    Dark => 1,
                }
            },
            Dark => {
                match *other {
                    Light => 1,
                    Normal => 2,
                    Dark => 0,
                }
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Hue {
    Red,
    Yellow,
    Green,
    Cyan,
    Blue,
    Magenta,
    White,
    Black,
}

impl Hue {
    fn diff_to(self, other: &Hue) -> usize {
        use Hue::*;
        match self {
            Red => {
                match *other {
                    Red => 0,
                    Yellow => 1,
                    Green => 2,
                    Cyan => 3,
                    Blue => 4,
                    Magenta => 5,
                    White => 6,
                    Black => 6,
                }
            },
            Yellow => {
                match *other {
                    Red => 5,
                    Yellow => 0,
                    Green => 1,
                    Cyan => 2,
                    Blue => 3,
                    Magenta => 4,
                    White => 6,
                    Black => 6,
                }
            },
            Green => {
                match *other {
                    Red => 4,
                    Yellow => 5,
                    Green => 0,
                    Cyan => 1,
                    Blue => 2,
                    Magenta => 3,
                    White => 6,
                    Black => 6,
                }
            },
            Cyan => {
                match *other {
                    Red => 3,
                    Yellow => 4,
                    Green => 5,
                    Cyan => 0,
                    Blue => 1,
                    Magenta => 2,
                    White => 6,
                    Black => 6,
                }
            },
            Blue => {
                match *other {
                    Red => 2,
                    Yellow => 3,
                    Green => 4,
                    Cyan => 5,
                    Blue => 0,
                    Magenta => 1,
                    White => 6,
                    Black => 6,
                }
            },
            Magenta => {
                match *other {
                    Red => 1,
                    Yellow => 2,
                    Green => 3,
                    Cyan => 4,
                    Blue => 5,
                    Magenta => 0,
                    White => 6,
                    Black => 6,
                }
            },
            White => 6,
            Black => 6,
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PietColor {
    pub lightness: Lightness,
    pub hue: Hue,
}

impl PietColor {
    pub fn diff_to(self, other: &PietColor) -> (usize, usize) {
        (self.hue.diff_to(&other.hue), self.lightness.diff_to(&other.lightness))
    }
}

#[cfg(feature = "default")]
impl ::std::fmt::Display for PietColor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use self::Lightness::*;
        use termion::color::*;
        match self.hue {
            Hue::Red => {
                match self.lightness {
                    Light => write!(f, "{}{}LR", Bg(AnsiValue(217)), Fg(AnsiValue(217))),
                    Normal => write!(f, "{}{}NR", Bg(AnsiValue(196)), Fg(AnsiValue(196))),
                    Dark => write!(f, "{}{}DR", Bg(AnsiValue(124)), Fg(AnsiValue(124))),
                }
            },
            Hue::Yellow => {
                match self.lightness {
                    Light => write!(f, "{}{}LY", Bg(AnsiValue(229)), Fg(AnsiValue(229))),
                    Normal => write!(f, "{}{}NY", Bg(AnsiValue(226)), Fg(AnsiValue(226))),
                    Dark => write!(f, "{}{}DY", Bg(AnsiValue(142)), Fg(AnsiValue(142))),
                }
            },
            Hue::Green => {
                match self.lightness {
                    Light => write!(f, "{}{}LG", Bg(AnsiValue(157)), Fg(AnsiValue(157))),
                    Normal => write!(f, "{}{}NG", Bg(AnsiValue(46)), Fg(AnsiValue(46))),
                    Dark => write!(f, "{}{}DG", Bg(AnsiValue(34)), Fg(AnsiValue(34))),
                }
            },
            Hue::Cyan => {
                match self.lightness {
                    Light => write!(f, "{}{}LC", Bg(AnsiValue(159)), Fg(AnsiValue(159))),
                    Normal => write!(f, "{}{}NC", Bg(AnsiValue(51)), Fg(AnsiValue(51))),
                    Dark => write!(f, "{}{}DC", Bg(AnsiValue(37)), Fg(AnsiValue(37))),
                }
            },
            Hue::Blue => {
                match self.lightness {
                    Light => write!(f, "{}{}LB", Bg(AnsiValue(147)), Fg(AnsiValue(147))),
                    Normal => write!(f, "{}{}NB", Bg(AnsiValue(21)), Fg(AnsiValue(21))),
                    Dark => write!(f, "{}{}DB", Bg(AnsiValue(19)), Fg(AnsiValue(19))),
                }
            },
            Hue::Magenta => {
                match self.lightness {
                    Light => write!(f, "{}{}LM", Bg(AnsiValue(219)), Fg(AnsiValue(219))),
                    Normal => write!(f, "{}{}NM", Bg(AnsiValue(201)), Fg(AnsiValue(201))),
                    Dark => write!(f, "{}{}DM", Bg(AnsiValue(127)), Fg(AnsiValue(127))),
                }
            },
            Hue::White => write!(f, "{}{}WW", Bg(AnsiValue(231)), Fg(AnsiValue(231))),
            Hue::Black => write!(f, "{}{}DD", Bg(AnsiValue(0)), Fg(AnsiValue(0))),
        }
        // Hue::Red => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 192, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 0, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 0, 0))),
        //     }
        // },
        // Hue::Yellow => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 255, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 255, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 192, 0))),
        //     }
        // },
        // Hue::Green => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 255, 192))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 255, 0))),
        //         Dark => write!(f, "{} ", Bg(Rgb(0, 192, 0))),
        //     }
        // },
        // Hue::Cyan => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 255, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 255, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(255, 192, 192))),
        //     }
        // },
        // Hue::Blue => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(192, 192, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(0, 0, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(0, 0, 192))),
        //     }
        // },
        // Hue::Magenta => {
        //     match self.lightness {
        //         Light => write!(f, "{} ", Bg(Rgb(255, 192, 255))),
        //         Normal => write!(f, "{} ", Bg(Rgb(255, 0, 255))),
        //         Dark => write!(f, "{} ", Bg(Rgb(192, 0, 192))),
        //     }
        // },
        // Hue::White => write!(f, "{} ", Bg(Rgb(255, 255, 255))),
        // Hue::Black => write!(f, "{} ", Bg(Rgb(0, 0, 0))),
    }
}
