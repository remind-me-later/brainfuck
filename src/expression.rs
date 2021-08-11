use std::convert;
use std::fmt;

pub enum Expression {
    Left(usize),
    Right(usize),
    Add(usize),
    Sub(usize),
    Input(usize),
    Output(usize),
    Open(usize),
    Close(usize),
}

impl Expression {
    pub fn modify_argument<F>(&mut self, f: F)
    where
        F: FnOnce(usize) -> usize,
    {
        match self {
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a)
            | Self::Open(a)
            | Self::Close(a) => *a = f(*a),
        }
    }

    pub fn combine(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            (Self::Left(a), Self::Left(b)) => Some(Self::Left(a + b)),
            (Self::Right(a), Self::Right(b)) => Some(Self::Right(a + b)),
            (Self::Left(l), Self::Right(r)) | (Self::Right(r), Self::Left(l)) => Some(if r > l {
                Self::Right(r - l)
            } else {
                Self::Left(l - r)
            }),

            (Self::Add(a), Self::Add(b)) => Some(Self::Add(a + b)),
            (Self::Sub(a), Self::Sub(b)) => Some(Self::Sub(a + b)),
            (Self::Add(a), Self::Sub(s)) | (Self::Sub(s), Self::Add(a)) => Some(if a > s {
                Self::Add(a - s)
            } else {
                Self::Sub(s - a)
            }),

            (Self::Input(a), Self::Input(b)) => Some(Self::Input(a + b)),
            (Self::Output(a), Self::Output(b)) => Some(Self::Output(a + b)),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn variant_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Left(_), Self::Left(_)) => true,
            (Self::Right(_), Self::Right(_)) => true,
            (Self::Add(_), Self::Add(_)) => true,
            (Self::Sub(_), Self::Sub(_)) => true,
            (Self::Input(_), Self::Input(_)) => true,
            (Self::Output(_), Self::Output(_)) => true,
            (Self::Open(_), Self::Open(_)) => true,
            (Self::Close(_), Self::Close(_)) => true,
            _ => false,
        }
    }

    pub fn argument(&self) -> usize {
        match self {
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a)
            | Self::Open(a)
            | Self::Close(a) => a.clone(),
        }
    }

    pub fn is_degenerate(&self) -> bool {
        match self {
            Self::Left(a)
            | Self::Right(a)
            | Self::Add(a)
            | Self::Sub(a)
            | Self::Input(a)
            | Self::Output(a) => a.clone() == 0,
            _ => false,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Right(_) => true,
            _ => false,
        }
    }

    pub fn is_add(&self) -> bool {
        match self {
            Self::Add(_) => true,
            _ => false,
        }
    }

    pub fn is_sub(&self) -> bool {
        match self {
            Self::Sub(_) => true,
            _ => false,
        }
    }

    pub fn is_open(&self) -> bool {
        match self {
            Self::Open(_) => true,
            _ => false,
        }
    }

    pub fn is_close(&self) -> bool {
        match self {
            Self::Close(_) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left(a) => write!(f, "Left({})", a),
            Self::Right(a) => write!(f, "Right({})", a),
            Self::Add(a) => write!(f, "Add({})", a),
            Self::Sub(a) => write!(f, "Sub({})", a),
            Self::Input(a) => write!(f, "Input({})", a),
            Self::Output(a) => write!(f, "Output({})", a),
            Self::Open(a) => write!(f, "Open({})", a),
            Self::Close(a) => write!(f, "Close({})", a),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Self::Left(_) => "<",
            Self::Right(_) => ">",
            Self::Add(_) => "+",
            Self::Sub(_) => "-",
            Self::Input(_) => ",",
            Self::Output(_) => ".",
            Self::Open(_) => "[",
            Self::Close(_) => "]",
        };

        let mut res = String::new();

        for _ in 0..self.argument() {
            res += symbol;
        }

        write!(f, "{}", res)
    }
}

impl convert::TryFrom<char> for Expression {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Left(1)),
            '>' => Ok(Self::Right(1)),
            '+' => Ok(Self::Add(1)),
            '-' => Ok(Self::Sub(1)),
            ',' => Ok(Self::Input(1)),
            '.' => Ok(Self::Output(1)),
            '[' => Ok(Self::Open(1)),
            ']' => Ok(Self::Close(1)),
            _ => Err(()),
        }
    }
}

impl convert::TryFrom<&char> for Expression {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        Self::try_from(c.clone())
    }
}

impl convert::TryFrom<u8> for Expression {
    type Error = ();

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        Self::try_from(b as char)
    }
}

impl convert::TryFrom<&u8> for Expression {
    type Error = ();

    fn try_from(b: &u8) -> Result<Self, Self::Error> {
        Self::try_from(b.clone())
    }
}
