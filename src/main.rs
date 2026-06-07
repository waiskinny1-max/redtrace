fn main() {
    if let Err(error) = redtrace::cli::run() {
        eprintln!("error: {error:?}");
        std::process::exit(1);
    }
}
