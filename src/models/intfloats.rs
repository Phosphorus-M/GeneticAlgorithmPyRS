use pyo3::FromPyObject;


#[derive(FromPyObject)]
pub enum IntFloats {
    USize(usize),
    ISize(isize),
    UInt8(u8),
    Int8(i8),
    UInt16(u16),
    Int16(i16),
    UInt32(u32),
    Int32(i32),
    UInt64(u64),
    Int64(i64),
    UInt128(u128),
    Int128(i128),
    Float32(f32),
    Float64(f64),
}