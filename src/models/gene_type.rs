use super::intfloats::IntFloats;

pub enum GeneType {
    Tuple(Option<IntFloats>, Option<IntFloats>),
    List(Vec<GeneType>),
}
