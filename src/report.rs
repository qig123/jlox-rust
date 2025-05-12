pub fn error(line: usize, message: &str) {
    report(line, "", message);
}
fn report(line: usize, where_: &str, msg: &str) {
    //这里java版本会有一个全局变量hadError
    eprintln!("[{}] Error {} : {}", line, where_, msg);
}
