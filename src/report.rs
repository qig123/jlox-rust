pub fn error(line: usize, message: &str) {
    report(line, "", message);
}
fn report(line: usize, where_: &str, msg: &str) {
    eprintln!("[{}] Error {} : {}", line, where_, msg);
}

