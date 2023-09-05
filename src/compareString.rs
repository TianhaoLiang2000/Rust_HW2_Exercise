pub fn compareString(x: &str, y: &str) -> bool {
    let mut char_x = x.chars();
    let mut char_y = y.chars();
    loop {
        match (char_x.next(), char_y.next()) {
            (Some(x_char), Some(y_char)) => {
                if x_char > y_char {
                    return true;
                } else {
                    return false;
                }
            }
            (Some(_), None) => {
                return true;
            }
            (None, Some(_)) => {
                return false;
            }
            (None, None) => {
                return false;
            }
        }
    }
}
