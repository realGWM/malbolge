//If singular is trit, then plural is tryte.
pub type Tryte = u32;

//Each tryte is 10 trits long.
pub const TRITS: Tryte = 10;

pub const MAX: Tryte = 59049;
pub const EOF: Tryte = 59048;

pub fn add(first: Tryte, second: Tryte) -> Tryte {
    (first + second) % MAX
}

pub fn move_right(mut tryte: Tryte) -> Tryte {
    let last = tryte % 3;
    tryte /= 3;
    tryte += Tryte::pow(3, TRITS - 1) * last;
    tryte
}

pub fn crazy(mut first: Tryte, mut second: Tryte) -> Tryte {
    let mut res = 0;
    for pow in 0..TRITS {
        let rem1 = first % 3;
        let rem2 = second % 3;
        first /= 3;
        second /= 3;
        let crz = match (rem1, rem2) {
            (0, 0) => 1,
            (0, 1) => 0,
            (0, 2) => 0,
            (1, 0) => 1,
            (1, 1) => 0,
            (1, 2) => 2,
            (2, 0) => 2,
            (2, 1) => 2,
            (2, 2) => 1,
            _ => panic!("Impossible remainders")
        };
        res += crz * Tryte::pow(3, pow as u32);
    }
    res
}

pub fn encrypt(tryte: Tryte) -> Tryte {
    let result = match tryte % 94 {
        0 => 57,
        1 => 109,
        2 => 60,
        3 => 46,
        4 => 84,
        5 => 86,
        6 => 97,
        7 => 99,
        8 => 96,
        9 => 117,
        10 => 89,
        11 => 42,
        12 => 77,
        13 => 75,
        14 => 39,
        15 => 88,
        16 => 126,
        17 => 120,
        18 => 68,
        19 => 108,
        20 => 125,
        21 => 82,
        22 => 69,
        23 => 111,
        24 => 107,
        25 => 78,
        26 => 58,
        27 => 35,
        28 => 63,
        29 => 71,
        30 => 34,
        31 => 105,
        32 => 64,
        33 => 53,
        34 => 122,
        35 => 93,
        36 => 38,
        37 => 103,
        38 => 113,
        39 => 116,
        40 => 121,
        41 => 102,
        42 => 114,
        43 => 36,
        44 => 40,
        45 => 119,
        46 => 101,
        47 => 52,
        48 => 123,
        49 => 87,
        50 => 80,
        51 => 41,
        52 => 72,
        53 => 45,
        54 => 90,
        55 => 110,
        56 => 44,
        57 => 91,
        58 => 37,
        59 => 92,
        60 => 51,
        61 => 100,
        62 => 76,
        63 => 43,
        64 => 81,
        65 => 59,
        66 => 62,
        67 => 85,
        68 => 33,
        69 => 112,
        70 => 74,
        71 => 83,
        72 => 55,
        73 => 50,
        74 => 70,
        75 => 104,
        76 => 79,
        77 => 65,
        78 => 49,
        79 => 67,
        80 => 66,
        81 => 54,
        82 => 118,
        83 => 94,
        84 => 61,
        85 => 73,
        86 => 95,
        87 => 48,
        88 => 47,
        89 => 56,
        90 => 124,
        91 => 106,
        92 => 115,
        93 => 98,
         _ => panic!("Impossible remainder")
    };
    result
}

#[allow(dead_code)]
pub fn to_string(mut tryte: Tryte) -> String {
    let mut string: String = String::new();
    for _ in 0..TRITS {
        let reminder = tryte % 3;
        tryte /= 3;
        string.push(std::char::from_digit(reminder as u32, 10).unwrap());
    }
    string.chars().rev().collect::<String>()
}

#[allow(dead_code)]
pub fn from_str(s: &str) -> Option<Tryte> {
    let mut num = 0;
    let mut pow = 0;
    for c in s.chars().rev() {
        let digit = c.to_digit(10)?;
        num += digit as Tryte * Tryte::pow(3, pow);
        pow += 1;
    }
    Some(num)
}
