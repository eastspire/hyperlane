use crate::*;

pub type BoxFunc = Box<dyn Func + Send + 'static>;
pub type VecBoxFunc = Vec<BoxFunc>;
pub type OptionVecBoxFunc = Option<VecBoxFunc>;
