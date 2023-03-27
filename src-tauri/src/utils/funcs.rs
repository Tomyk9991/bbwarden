pub fn assert_err(expression: bool) -> Option<()> {
    match expression {
        true => { None }
        false => { Some(()) }
    }
}