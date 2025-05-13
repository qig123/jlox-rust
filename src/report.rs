pub fn error(line: usize, message: &str) {
    report(line, "", message);
}
pub fn report(line: usize, where_: &str, msg: &str) {
    eprintln!("[line {}] Error {} : {}", line, where_, msg);
}
