#![feature(slice_patterns)]
#![feature(io)]

use std::io::prelude::*;

extern crate png;
extern crate getopts;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Lightness {
    Light,
    Normal,
    Dark,
}

impl Lightness {
    fn diff_to(self, other: &Lightness) -> usize {
        use Lightness::*;
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
enum Hue {
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
struct PietColor {
    lightness: Lightness,
    hue: Hue,
}

impl PietColor {
    fn diff_to(self, other: &PietColor) -> (usize, usize) {
        (self.hue.diff_to(&other.hue), self.lightness.diff_to(&other.lightness))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Codel {
    color: PietColor,
    x: usize,
    y: usize,
}

impl Codel {
    fn diff_to(self, other: &Codel) -> (usize, usize) {
        self.color.diff_to(&other.color)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        }
    }

    fn rotate(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
        }
    }
}

/* Uses the direction pointer and the codel chooser in order to decide which of the two Codels is
 * the farthest. */
// First, use the direction pointer. Check if a Codel is farther than the other and return it in
// this case. If it's a tie, use the codel chooser in order to decide which codel should be chosen.
//
fn compare_farthest_codels<'a>(
    c1: &'a Codel,
    c2: &'a Codel,
    dp: Direction,
    cc: Direction,
) -> &'a Codel {
    use Direction::*;
    if c1 == c2 {
        c1
    } else {
        match dp {
            Right => {
                if c1.x > c2.x {
                    &c1
                } else if c1.x < c2.x {
                    &c2
                } else {
                    match cc {
                        Left => if c1.y < c2.y { c1 } else { c2 },
                        Right => if c1.y > c2.y { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Left => {
                if c1.x < c2.x {
                    &c1
                } else if c1.x > c2.x {
                    &c2
                } else {
                    match cc {
                        Left => if c1.y > c2.y { c1 } else { c2 },
                        Right => if c1.y < c2.y { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Up => {
                if c1.y < c2.y {
                    &c1
                } else if c1.y > c2.y {
                    &c2
                } else {
                    match cc {
                        Left => if c1.x < c2.x { c1 } else { c2 },
                        Right => if c1.x > c2.x { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
            Down => {
                if c1.y > c2.y {
                    &c1
                } else if c1.y < c2.y {
                    &c2
                } else {
                    match cc {
                        Left => if c1.x > c2.x { c1 } else { c2 },
                        Right => if c1.x < c2.x { c1 } else { c2 },
                        _ => panic!("Codel chooser is neither Left or Right"),
                    }
                }
            },
        }
    }
}

/* Fetches the farthest codel belonging to the same block as the codel at position x, y
 * picture: A matrix of Codels corresponding to the piet program.
 * x: The starting position within the matrix on the x axis
 * y: The starting position within the matriy on the y ayis
 * dp: The global direction pointer
 * cc: The global codel chooser
 * returns: An (usize, &Codel) tuple. For non-white blocks, the usize corresponds to the size of
 * the block, the &Codel is the farthest codel that belongs to the same block as the starting
 * position according to the direction pointer and cc.
 * For white blocks the usize is 0 and the codel is the farthest white codel in a straight line
 * in
 * dp's direction. */
// This is basically a depth-first search implemented using a stack instead of recursion.
fn get_farthest_codel(
    picture: &Vec<Vec<Codel>>,
    x: usize,
    y: usize,
    dp: Direction,
    cc: Direction,
) -> (usize, &Codel) {
    // White blocks have a different
    if picture[y][x].color.hue == Hue::White {
        let (mut x, mut y) = (x as isize, y as isize);
        let (tmpx, tmpy) = dp.to_vector();
        while y >= 0 && y < picture.len() as isize && x >= 0 &&
            x < picture[y as usize].len() as isize &&
            picture[y as usize][x as usize].color.hue == Hue::White
        {
            y += tmpy;
            x += tmpx;
        }
        y -= tmpy;
        x -= tmpx;
        return (0, &picture[y as usize][x as usize]);
    }

    let mut codels_to_visit = vec![(x, y); 1];
    let mut visited_codels = std::collections::HashSet::new();

    let mut result = &picture[y][x];
    while codels_to_visit.len() > 0 {
        // Get the last codel added to the codel stack
        let (tmpx, tmpy) = codels_to_visit.pop().unwrap();
        // Mark it as visited
        visited_codels.insert(&picture[tmpy][tmpx]);
        // If the codel under the current codel is within our matrix bounds, its color matches the
        // starting color and it hasn't been visited yet
        if tmpy + 1 < picture.len() && &picture[tmpy + 1][tmpx].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy + 1][tmpx])
        {
            // Add it to the list of codels that need to be visited
            codels_to_visit.push((tmpx, tmpy + 1));
        }
        // If the codel above the current codel is within the bounds, the right colors and hasn't
        // been visited
        if tmpy > 0 && &picture[tmpy - 1][tmpx].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy - 1][tmpx])
        {
            // Add it to the list of codels that need to be visited
            codels_to_visit.push((tmpx, tmpy - 1));
        }
        // Same thing for the codel on the right
        if tmpx + 1 < picture[tmpy].len() &&
            &picture[tmpy][tmpx + 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx + 1])
        {
            codels_to_visit.push((tmpx + 1, tmpy));
        }
        // Same thing for the codel on the left
        if tmpx > 0 && &picture[tmpy][tmpx - 1].color == &picture[y][x].color &&
            !visited_codels.contains(&picture[tmpy][tmpx - 1])
        {
            codels_to_visit.push((tmpx - 1, tmpy));
        }
        result = compare_farthest_codels(result, &picture[tmpy][tmpx], dp, cc);
    }

    return (visited_codels.len(), result);
}

/* Checks whether there is a non-white, non-black codel aligned with the current Codel in the
 * direction of the dp within the picture.
 * returns: A (bool, bool, &Codel) tuple. The first boolean is true if there is an available
 * codel
 * in the direction of the direction pointer, false otherwise. The second bool indicates whether
 * the algorithm encountered white blocks between the current codel and the newly found codel.
 * The
 * returned Codel is the new codel when the bool is true and the starting codel when the bool is
 * false.
 * */
fn can_go_in_direction<'a>(
    picture: &'a Vec<Vec<Codel>>,
    current_codel: &'a Codel,
    dp: Direction,
) -> (bool, &'a Codel) {
    let (mut tmpx, mut tmpy) = (current_codel.x as isize, current_codel.y as isize);
    let vec = dp.to_vector();
    tmpx += vec.0;
    tmpy += vec.1;

    // If we're not within the matrix's bounds or if the codel is black, we can't go in this
    // direction
    if tmpy < 0 || tmpy as usize >= picture.len() || tmpx < 0 ||
        (tmpx as usize) >= picture[(tmpy as usize)].len() ||
        picture[tmpy as usize][tmpx as usize].color.hue == Hue::Black
    {
        return (false, current_codel);
    }
    // Return the found codel
    return (true, &picture[tmpy as usize][tmpx as usize]);
}

fn get_picture(filename: std::string::String) -> Vec<Vec<Codel>> {
    let decoder = png::Decoder::new(std::fs::File::open(filename).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buffer = vec![0; info.buffer_size()];
    reader.next_frame(&mut buffer).unwrap();

    let values_per_pixel;
    match info.color_type {
        png::ColorType::RGBA => values_per_pixel = 4,
        png::ColorType::RGB => values_per_pixel = 3,
        _ => panic!("PNG picture is neither RGB or RGBA"),
    }
    let pic_width = info.width as usize;
    let pic_height = info.height as usize;

    let mut picture: Vec<Vec<Codel>> = vec![
        vec![
            Codel {
                color: PietColor { hue: Hue::White, lightness: Lightness::Normal },
                x: 0,
                y: 0,
            };
            pic_width
        ];
        pic_height
    ];
    let mut i = 0;
    for pixel in buffer.chunks(values_per_pixel) {
        let (x, y) = (i % pic_width, i / pic_width);
        picture[y][x].x = x;
        picture[y][x].y = y;
        picture[y][x].color = match &pixel[0..3] {
            &[255, 192, 192] => PietColor { hue: Hue::Red, lightness: Lightness::Light },
            &[255, 0, 0] => PietColor { hue: Hue::Red, lightness: Lightness::Normal },
            &[192, 0, 0] => PietColor { hue: Hue::Red, lightness: Lightness::Dark },
            &[255, 255, 192] => PietColor { hue: Hue::Yellow, lightness: Lightness::Light },
            &[255, 255, 0] => PietColor { hue: Hue::Yellow, lightness: Lightness::Normal },
            &[192, 192, 0] => PietColor { hue: Hue::Yellow, lightness: Lightness::Dark },
            &[192, 255, 192] => PietColor { hue: Hue::Green, lightness: Lightness::Light },
            &[0, 255, 0] => PietColor { hue: Hue::Green, lightness: Lightness::Normal },
            &[0, 192, 0] => PietColor { hue: Hue::Green, lightness: Lightness::Dark },
            &[192, 255, 255] => PietColor { hue: Hue::Cyan, lightness: Lightness::Light },
            &[0, 255, 255] => PietColor { hue: Hue::Cyan, lightness: Lightness::Normal },
            &[0, 192, 192] => PietColor { hue: Hue::Cyan, lightness: Lightness::Dark },
            &[192, 192, 255] => PietColor { hue: Hue::Blue, lightness: Lightness::Light },
            &[0, 0, 255] => PietColor { hue: Hue::Blue, lightness: Lightness::Normal },
            &[0, 0, 192] => PietColor { hue: Hue::Blue, lightness: Lightness::Dark },
            &[255, 192, 255] => PietColor { hue: Hue::Magenta, lightness: Lightness::Light },
            &[255, 0, 255] => PietColor { hue: Hue::Magenta, lightness: Lightness::Normal },
            &[192, 0, 192] => PietColor { hue: Hue::Magenta, lightness: Lightness::Dark },
            &[0, 0, 0] => PietColor { hue: Hue::Black, lightness: Lightness::Normal },
            _ => PietColor { hue: Hue::White, lightness: Lightness::Normal },
        };
        i += 1;
    }
    return picture;
}

fn main() {
    use getopts::Options;
    use std::env;

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("f", "file", "name of the file", "FILE");
    opts.optflag("d", "debug", "use debug mode");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };

    let debug = matches.opt_present("d");

    let picture = get_picture(matches.opt_str("f").unwrap());
    let mut piet_stack: Vec<isize> = Vec::new();
    let mut dp = Direction::Right;
    let mut cc = Direction::Left;

    let mut current_codel = picture[0][0].clone();
    'main_loop: loop {
        if debug {
            println!("{:?}, {:?}, {:?}", current_codel, dp, cc);
            println!("{:?}", piet_stack);
            std::io::stdin().read_line(&mut String::new());
        }
        let next_codel;
        let mut attempts = 0;
        let mut block_size;
        'codel_choosing: loop {
            let result = get_farthest_codel(&picture, current_codel.x, current_codel.y, dp, cc);
            block_size = result.0;
            let farthest_codel = result.1;
            match (attempts % 2, can_go_in_direction(&picture, farthest_codel, dp)) {
                (_, (true, codel)) => {
                    next_codel = codel;
                    break 'codel_choosing;
                },
                (0, (false, codel)) => {
                    current_codel = codel.clone();
                    cc = cc.opposite();
                },
                (1, (false, codel)) => {
                    current_codel = codel.clone();
                    dp = dp.rotate();
                },
                _ => unreachable!(),
            }
            attempts += 1;
            if attempts >= 8 {
                break 'main_loop;
            }
        }
        match current_codel.diff_to(next_codel) {
            // Push
            (0, 1) => piet_stack.push(block_size as isize),
            // Pop
            (0, 2) => {
                piet_stack.pop();
            },
            // Add
            (1, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b + a);
                }
            },
            // Sub
            (1, 1) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b - a);
                }
            },
            // Mul
            (1, 2) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b * a);
                }
            },
            // Div
            (2, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b / a);
                }
            },
            // Mod
            // FIXME: Rust modulo semantics aren's the same as Piet's
            (2, 1) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(b % a);
                }
            },
            // Not
            (2, 2) => {
                if let Some(val) = piet_stack.pop() {
                    piet_stack.push(if val == 0 { 1 } else { 0 })
                }
            },
            // Greater
            (3, 0) => {
                if let (Some(a), Some(b)) = (piet_stack.pop(), piet_stack.pop()) {
                    piet_stack.push(if b > a { 1 } else { 0 })
                }
            },
            // Pointer
            (3, 1) => {
                if let Some(a) = piet_stack.pop() {
                    for _ in 0..a {
                        dp = dp.rotate();
                    }
                }
            },
            // Switch
            (3, 2) => {
                if let Some(a) = piet_stack.pop() {
                    for _ in 0..a {
                        cc = cc.opposite();
                    }
                }
            },
            // Duplicate
            (4, 0) => {
                if let Some(a) = piet_stack.pop() {
                    piet_stack.push(a);
                    piet_stack.push(a);
                }
            },
            // Roll
            (4, 1) => {
                if let (Some(count), Some(depth)) = (piet_stack.pop(), piet_stack.pop()) {
                    let mut before: Vec<_> = piet_stack
                        .iter()
                        .take(piet_stack.len() - (depth as usize))
                        .map(|e| *e)
                        .collect();
                    if count >= 0 {
                        let mut rolled: Vec<_> = piet_stack
                            .iter()
                            .skip(piet_stack.len() - (count as usize))
                            .map(|e| *e)
                            .collect();
                        let mut after: Vec<_> = piet_stack
                            .iter()
                            .skip(piet_stack.len() - (depth as usize))
                            .take((depth - count) as usize)
                            .map(|e| *e)
                            .collect();
                        before.append(&mut rolled);
                        before.append(&mut after);
                    } else {
                        let mut rolled: Vec<_> = piet_stack
                            .iter()
                            .skip(before.len())
                            .take(count.abs() as usize)
                            .map(|e| *e)
                            .collect();
                        let mut after: Vec<_> = piet_stack
                            .iter()
                            .skip(before.len() + rolled.len())
                            .map(|e| *e)
                            .collect();
                        before.append(&mut after);
                        while let Some(v) = rolled.pop() {
                            before.push(v);
                        }
                    }
                    piet_stack = before;
                }
            },
            // in(number)
            (4, 2) => {
                let mut input = String::new();
                match std::io::stdin().read_line(&mut input) {
                    Ok(_) => piet_stack.push(input.trim_right().parse().unwrap()),
                    Err(error) => {
                        if debug {
                            panic!(error)
                        }
                    },
                }
            },
            // in(char)
            (5, 0) => {
                let stdin = std::io::stdin();
                let mut chars = stdin.lock().chars();
                match chars.nth(0) {
                    Some(Ok(char)) => {
                        if let Some(char) = char.to_digit(10) {
                            piet_stack.push(char as isize);
                        }
                    },
                    _ => (),
                }
            },
            // out(number)
            (5, 1) => {
                if let Some(val) = piet_stack.pop() {
                    print!("{}", val)
                }
            },
            // out(char)
            (5, 2) => {
                if let Some(val) = piet_stack.pop() {
                    print!("{}", std::char::from_u32(val as u32).unwrap())
                }
            },
            (6, _) => (),
            (a, b) => panic!("Error: differences are ({},{}))", a, b),
        }
        current_codel = next_codel.clone();
    }
}
