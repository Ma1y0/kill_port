use kill_port::app::{get_port, is_port_in_use};

fn main() {
    let port = get_port();
    println!("{}", port);
    let in_use = is_port_in_use(port);
    println!("{}", in_use);
}
