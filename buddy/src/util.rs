pub fn ilog_ceil(base: usize, value: usize) -> usize {
    let mut log = value.ilog(base) as usize;
    if value > base.pow(log as _) {
        log += 1;
    }

    log
}
