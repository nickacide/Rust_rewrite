#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}
impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Color {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
