#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd)]
pub struct BitBoard(pub u64);

impl core::ops::Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl core::ops::BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}
impl core::ops::BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}
impl core::ops::BitXor for BitBoard {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl core::ops::BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 & rhs.0)
    }
}
impl core::ops::BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 | rhs.0)
    }
}
impl core::ops::BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = BitBoard(self.0 ^ rhs.0)
    }
}
impl core::ops::Sub for BitBoard {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 - rhs.0)
    }
}

impl BitBoard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(!0);
    pub const fn new(index: usize) -> Self {
        BitBoard(1 << index)
    }
    pub const fn first(&self) -> usize {
        self.0.trailing_zeros() as usize
    }
    pub const fn count(&self) -> usize {
        self.0.count_ones() as usize
    }
    pub const fn u64(&self) -> u64 {
        self.0
    }
    pub const fn rank(self) -> i8 {
        (self.0.trailing_zeros() >> 3) as i8
    }
    pub const fn file(self) -> i8 {
        (self.0.trailing_zeros() & 7) as i8
    }
    pub const fn offset(self, r: i8, f: i8) -> BitBoard {
        let newrank = self.rank() + r;
        let newfile = self.file() + f;
        let newsq = (newrank as usize) << 3 | newfile as usize;
        BitBoard(1 << newsq)
    }
}
