fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = rtodo::get_app("rtodo.db", std::io::stdout());
    if let Err(e) = rtodo::run(&mut app, args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
