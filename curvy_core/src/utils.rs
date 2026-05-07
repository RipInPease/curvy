use std::io;

macro_rules! int_from_be_slice {
    ($ty:ty, $name1: ident, $name2: ident) => {
        pub fn $name1(slice: &[u8], start: usize) -> $ty {
            let mut bytes = [0; size_of::<$ty>()];
            bytes.copy_from_slice(&slice[start..start + size_of::<$ty>()]);
            <$ty>::from_be_bytes(bytes)
        }

        pub fn $name2(slice: &[u8], start: usize) -> $ty {
            let mut bytes = [0; size_of::<$ty>()];
            bytes.copy_from_slice(&slice[start..start + size_of::<$ty>()]);
            <$ty>::from_le_bytes(bytes)
        }
    };
}


int_from_be_slice!(u16, u16_from_be_slice, u16_from_le_slice);
int_from_be_slice!(u32, u32_from_be_slice, u32_from_le_slice);
int_from_be_slice!(u64, u64_from_be_slice, u64_from_le_slice);
int_from_be_slice!(u128, u128_from_be_slice, u128_from_le_slice);
int_from_be_slice!(i16, i16_from_be_slice, i16_from_le_slice);
int_from_be_slice!(i32, i32_from_be_slice, i32_from_le_slice);
int_from_be_slice!(i64, i64_from_be_slice, i64_from_le_slice);
int_from_be_slice!(i128, i128_from_be_slice, i128_from_le_slice);
int_from_be_slice!(f32, f32_from_be_slice, f32_from_le_slice);
int_from_be_slice!(f64, f64_from_be_slice, f64_from_le_slice);

#[inline]
pub fn invalid_format_error() -> io::Error {
    let kind = io::ErrorKind::InvalidData;
    io::Error::new(kind, "Invalid data format")
}