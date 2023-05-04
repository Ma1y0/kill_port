use kill_port::app::{get_port, get_pid, kill_port};

fn main() {
    // Get a port from arguments
    let port = get_port();
    let pid = match get_pid(port) {
        Some(pid) => pid,
        None => {
            eprintln!("There is nothing running on port :{}", port);
            return
        }
    };

    // Get a pid of the process
    let pid = if let Some(pid) = pid.get(0) {
        pid.to_owned()
    } else {
        eprintln!("Failed to get pid of process running on port :{}", port);
        return
    };

    // Try to kill the process
    if let Err(e) = kill_port(pid) {
        panic!("{}", e);
    } else {
        println!("Successfully kill process running on port :{} with pid {}", port, pid);
    }
}
