use std::net::{TcpListener, TcpStream};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DebugEvent {
    DoneStep,
    Halted,
    Break,
    WatchWrite(u64),
    WatchRead(u64),
}

pub enum DebugExecMode {
    Step,
    Continue,
    RangeStep(u64, u64),
}
pub enum DebugRunEvent {
    IncomingData,
    Event(DebugEvent),
}
pub fn wait_for_tcp(port: u16) -> Result<TcpStream, Box<dyn std::error::Error>> {
    let sockaddr = format!("127.0.0.1:{}", port);
    eprintln!("Waiting for a GDB connection on {:?}...", sockaddr);

    let sock = TcpListener::bind(sockaddr)?;
    let (stream, addr) = sock.accept()?;
    eprintln!("Debugger connected from {}", addr);

    Ok(stream)
}
