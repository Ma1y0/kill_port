use std::{net::TcpListener, process};

use kill_port::app::get_pid;

#[test]
fn test_get_pid() {
    let _listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pid = get_pid(8080).unwrap()[0];
    let pid_of_the_program = process::id();
    assert_eq!(pid_of_the_program, pid);
}
