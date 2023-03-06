use pyo3::FromPyObject;


#[derive(FromPyObject, PartialEq, Debug)]
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

#[derive(FromPyObject, PartialEq, Debug)]
pub enum Ints{
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
}

#[derive(FromPyObject, PartialEq, Debug)]
pub enum Floats{
    Float32(f32),
    Float64(f64),
}

// Implement transformation of Floats to IntFloats
impl From<Floats> for IntFloats {
    fn from(f: Floats) -> Self {
        match f {
            Floats::Float32(f) => IntFloats::Float32(f),
            Floats::Float64(f) => IntFloats::Float64(f),
        }
    }
}

// Implement transformation of Ints to IntFloats
impl From<Ints> for IntFloats {
    fn from(i: Ints) -> Self {
        match i {
            Ints::USize(i) => IntFloats::USize(i),
            Ints::ISize(i) => IntFloats::ISize(i),
            Ints::UInt8(i) => IntFloats::UInt8(i),
            Ints::Int8(i) => IntFloats::Int8(i),
            Ints::UInt16(i) => IntFloats::UInt16(i),
            Ints::Int16(i) => IntFloats::Int16(i),
            Ints::UInt32(i) => IntFloats::UInt32(i),
            Ints::Int32(i) => IntFloats::Int32(i),
            Ints::UInt64(i) => IntFloats::UInt64(i),
            Ints::Int64(i) => IntFloats::Int64(i),
            Ints::UInt128(i) => IntFloats::UInt128(i),
            Ints::Int128(i) => IntFloats::Int128(i),
        }
    }
}

// Implement transformation of IntFloats to Ints
impl From<IntFloats> for Ints {
    fn from(i: IntFloats) -> Self {
        match i {
            IntFloats::USize(i) => Ints::USize(i),
            IntFloats::ISize(i) => Ints::ISize(i),
            IntFloats::UInt8(i) => Ints::UInt8(i),
            IntFloats::Int8(i) => Ints::Int8(i),
            IntFloats::UInt16(i) => Ints::UInt16(i),
            IntFloats::Int16(i) => Ints::Int16(i),
            IntFloats::UInt32(i) => Ints::UInt32(i),
            IntFloats::Int32(i) => Ints::Int32(i),
            IntFloats::UInt64(i) => Ints::UInt64(i),
            IntFloats::Int64(i) => Ints::Int64(i),
            IntFloats::UInt128(i) => Ints::UInt128(i),
            IntFloats::Int128(i) => Ints::Int128(i),
            IntFloats::Float32(_) => panic!("Cannot convert Float32 to Ints"),
            IntFloats::Float64(_) => panic!("Cannot convert Float64 to Ints"),
        }
    }
}
