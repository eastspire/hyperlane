use super::*;

pub fn handle_error<T: ToString>(tmp: &Tmp, err: T) {
    tmp.get_log().error(err, common_log);
}
