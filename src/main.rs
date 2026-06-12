use rtodo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut app = rtodo::get_app("rtodo.db", std::io::stdout());
    rtodo::run(&mut app, args);
}
