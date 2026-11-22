//! Tokio: async byte streams, local TCP, framing, buffering, and files.
//!
//! Setup: cargo add tokio@1 --features full
//! Cargo.toml: tokio = { version="1", features=["full"] }
//! Run: cargo run --bin 04_async_io
//!
//! Review: read/write methods return futures. Awaiting a pending socket operation
//! lets the task suspend so other work can progress on the runtime thread.
//! Async does not make bytes arrive sooner; it avoids blocking a thread while waiting.
//!
//! The TCP demo binds ONLY 127.0.0.1 on an OS-selected free port and finishes
//! automatically. No internet, second terminal, or separately started server.
//! File examples create a NEW temporary directory, print its location, and leave
//! files there for inspection. They do not overwrite pre-existing user files.
//!
//! References:
//! https://tokio.rs/tokio/tutorial/io
//! https://docs.rs/tokio/latest/tokio/io/index.html
//! https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html
//! https://docs.rs/tokio/latest/tokio/fs/index.html

use std::error::Error;
use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::io::{
    duplex, AsyncRead, AsyncReadExt, AsyncBufReadExt,
    AsyncWrite, AsyncWriteExt, BufReader, BufWriter,
};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

#[tokio::main(flavor="current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_byte_stream().await?;
    println!("\n============================\n");

    demo_exact_reads().await?;
    println!("\n============================\n");

    // This outer budget also prevents an unexpected local connection failure
    // from leaving the tutorial waiting forever for the other endpoint.
    timeout(Duration::from_secs(5), demo_local_tcp()).await??;
    println!("\n============================\n");

    demo_buffered_lines().await?;
    println!("\n============================\n");

    demo_length_prefixed_frames().await?;
    println!("\n============================\n");

    demo_split_stream().await?;
    println!("\n============================\n");

    demo_files().await?;
    println!("\n============================\n");

    demo_cancellation_and_limits();

    Ok(())
}

// =====================================================
// 01. AsyncRead/AsyncWrite operate on bytes, not messages
// =====================================================

async fn demo_byte_stream() -> io::Result<()> {
    println!("01. AsyncRead/AsyncWrite operate on bytes, not messages");

    let (mut writer, mut reader) = duplex(8);
    let sending = async move {
        writer.write_all(b"hello ").await?;
        writer.write_all(b"Rust").await?;
        writer.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let receiving = async move {
        let mut buffer = [0_u8; 3];
        let mut received = Vec::new();
        loop {
            let count = reader.read(&mut buffer).await?;
            if count == 0 {
                println!("Reader reached EOF");
                break;
            }
            println!("Read {count} bytes: {:?}", &buffer[..count]);
            received.extend_from_slice(&buffer[..count]);
        }
        Ok::<Vec<u8>, io::Error>(received)
    };
    let (sent, received) = tokio::join!(sending, receiving);
    sent?;
    println!("Reassembled text: {}", String::from_utf8_lossy(&received?));
    // duplex is a bounded in-memory bidirectional byte stream, useful for learning
    // the same AsyncRead/AsyncWrite interfaces without networking first.
    // read returns 1..=buffer.len() bytes as available; one read need not correspond
    // to one write. With a nonempty buffer, Ok(0) means EOF here, not 'try later'.
    // Pending I/O is handled inside the future, not exposed as read returning zero.
    // Inspect ONLY buffer[..count]; the rest is not new data.
    // write can accept only part of a slice; write_all loops until all bytes are accepted.
    // The two writes may be combined/split by the reader. Actual chunk sizes vary.
    // Importing extension traits supplies these high-level methods on stream types.
    Ok(())
}

// =====================================================
// 02. read_exact knows a size; read_to_end waits for EOF
// =====================================================

async fn demo_exact_reads() -> io::Result<()> {
    println!("02. read_exact knows a size; read_to_end waits for EOF");

    let mut source: &[u8] = b"HEADpayload";
    let mut header = [0_u8; 4];
    source.read_exact(&mut header).await?;
    let mut remainder = Vec::new();
    source.read_to_end(&mut remainder).await?;
    println!("Fixed header: {:?}", String::from_utf8_lossy(&header));
    println!("Remainder: {:?}", String::from_utf8_lossy(&remainder));
    // &[u8] itself implements AsyncRead: these ready memory reads need no wait.
    // read_exact fills the requested buffer or returns an error.
    // read_to_end appends until EOF, NOT until one application's message is done.
    // A persistent TCP connection may stay open forever: do not read_to_end to
    // receive one request unless your protocol deliberately uses EOF as its boundary.

    println!();

    let mut short: &[u8] = b"ab";
    let mut expected = [0_u8; 4];
    match short.read_exact(&mut expected).await {
        Ok(_) => println!("Unexpected complete read"),
        Err(error) => println!("Expected truncated input: {:?}: {error}", error.kind()),
    }
    // Partial consumption may already have happened when read_exact fails.
    // Bytes are not always UTF-8. from_utf8_lossy is convenient for a demo;
    // use strict decoding and a protocol-specific error policy for real text.
    Ok(())
}

// =====================================================
// 03. One local TCP client and server in the same program
// =====================================================

async fn demo_local_tcp() -> io::Result<()> {
    println!("03. One local TCP client and server in the same program");

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    println!("Server listening at {address}");
    // bind completes before starting the client: no startup sleep/race is needed.
    // Port 0 asks the OS for a free port. 127.0.0.1 is local loopback only.

    let server = async move {
        let (mut connection, peer) = listener.accept().await?;
        println!("Server accepted {peer}");
        let mut request = Vec::new();
        connection.read_to_end(&mut request).await?;
        println!("Server received: {}", String::from_utf8_lossy(&request));
        connection.write_all(b"hello from server").await?;
        connection.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let client = async move {
        let mut connection = TcpStream::connect(address).await?;
        connection.write_all(b"hello from client").await?;
        connection.shutdown().await?; // finish sending, but KEEP receiving
        let mut reply = Vec::new();
        connection.read_to_end(&mut reply).await?;
        println!("Client received: {}", String::from_utf8_lossy(&reply));
        Ok::<(), io::Error>(())
    };
    let (server_result, client_result) = tokio::join!(server, client);
    server_result?;
    client_result?;
    // accept awaits a connection; connect awaits establishment. read/write may
    // suspend for readiness. Both endpoints progress on our one runtime thread.
    // shutdown on TCP shuts down the WRITE direction: this is a half-close.
    // Here EOF deliberately frames one request and one response. Without client
    // half-close, server waits for EOF while client waits for a reply -> deadlock.
    // TCP delivers an ordered byte stream, not application message boundaries.
    // Successful write_all means local acceptance, not that the peer processed it.
    // join! creates no detached tasks. The outer timeout in main drops both
    // endpoint futures/resources if this finite example cannot finish in time.
    Ok(())
}

// =====================================================
// 04. Buffered text: newline-delimited messages and flush
// =====================================================

async fn demo_buffered_lines() -> io::Result<()> {
    println!("04. Buffered text: newline-delimited messages and flush");

    let (writer, reader) = duplex(64);
    let sending = async move {
        let mut writer = BufWriter::new(writer);
        writer.write_all(b"alpha\n").await?;
        writer.write_all(b"beta\r\n\n").await?;
        writer.flush().await?;
        writer.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let receiving = async move {
        let mut lines = BufReader::new(reader).lines();
        while let Some(line) = lines.next_line().await? {
            println!("Complete line: {line:?}");
        }
        Ok::<(), io::Error>(())
    };
    let (sent, received) = tokio::join!(sending, receiving);
    sent?;
    received?;
    // lines().next_line() waits for another line or EOF and returns Result<Option<String>>.
    // It removes LF/CRLF endings. An empty line is Some(""); stream end is None.
    // Text must be UTF-8. EOF can also terminate a final line without a newline.
    // BufReader may read ahead; keep ONE reader around rather than recreating it
    // or bypassing it, which could lose access to buffered unread bytes.
    // BufWriter aggregates small writes; flush explicitly hands them onward.
    // Dropping Tokio BufWriter does NOT flush its buffered bytes automatically.
    // Flushing a socket does not acknowledge remote processing or create a message.
    // Simple lines() has no line-length limit: use bounded framing for untrusted peers.
    Ok(())
}

// =====================================================
// 05. Length-prefixed framing: know where each message ends
// =====================================================

async fn demo_length_prefixed_frames() -> io::Result<()> {
    println!("05. Length-prefixed framing: know where each message ends");

    let (mut writer, mut reader) = duplex(16);
    let sending = async move {
        write_frame(&mut writer, b"first").await?;
        write_frame(&mut writer, b"second message").await?;
        writer.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let receiving = async move {
        // This demo's protocol expects EXACTLY two messages, not an endless stream.
        for number in 1..=2 {
            let payload = read_frame(&mut reader).await?;
            println!("Frame {number}: {}", String::from_utf8_lossy(&payload));
        }
        Ok::<(), io::Error>(())
    };
    let (sent, received) = tokio::join!(sending, receiving);
    sent?;
    received?;
    // Wire format: four-byte unsigned big-endian payload length, then that many bytes.
    // The length counts BYTES, not Unicode characters. Empty payloads are valid.
    // Header/payload can arrive in arbitrary read chunks; read_exact reconstructs them.
    // Helpers validate a small maximum length BEFORE allocating the payload buffer.

    println!();

    let oversized_header = 1_025_u32.to_be_bytes();
    let mut source: &[u8] = &oversized_header;
    match read_frame(&mut source).await {
        Ok(payload) => println!("Unexpected accepted frame: {} bytes", payload.len()),
        Err(error) => println!("Expected oversized-frame error: {error}"),
    }
    // Helpers treat EOF before a full header/payload as UnexpectedEof.
    // A persistent protocol that permits clean EOF BETWEEN frames must distinguish
    // 'zero header bytes read' from a truncated header, rather than hiding both.
    // Production codecs (for example tokio-util) can encapsulate framing state.
    Ok(())
}

// =====================================================
// 06. Split a connection to read and write concurrently
// =====================================================

async fn demo_split_stream() -> io::Result<()> {
    println!("06. Split a connection to read and write concurrently");

    let (connection, mut peer) = duplex(32);
    let (mut read_half, mut write_half) = tokio::io::split(connection);
    let sending = async {
        write_half.write_all(b"ping").await?;
        write_half.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let receiving = async {
        let mut reply = [0_u8; 4];
        read_half.read_exact(&mut reply).await?;
        println!("Reply from peer: {}", String::from_utf8_lossy(&reply));
        Ok::<(), io::Error>(())
    };
    let peer_work = async {
        let mut request = [0_u8; 4];
        peer.read_exact(&mut request).await?;
        peer.write_all(b"pong").await?;
        peer.shutdown().await?;
        Ok::<(), io::Error>(())
    };
    let (sent, received, answered) = tokio::join!(sending, receiving, peer_work);
    sent?;
    received?;
    answered?;
    // One stream normally needs &mut self for both read and write calls, making
    // two simultaneous mutable borrows awkward. Split gives distinct direction handles.
    // tokio::io::split works generically. For TcpStream, split() borrows the socket;
    // into_split() produces owned halves suitable for separate spawned tasks.
    // Splitting does not add message boundaries, buffering, or more connections.
    // This in-memory example uses the same pattern without opening another TCP listener.
    Ok(())
}

// =====================================================
// 07. File helpers, buffered files, and explicit completion
// =====================================================

async fn demo_files() -> Result<(), Box<dyn Error>> {
    println!("07. File helpers, buffered files, and explicit completion");

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let directory = std::env::temp_dir()
        .join(format!("tokio_io_{}_{stamp}", std::process::id()));
    fs::create_dir(&directory).await?;
    println!("New output directory: {}", directory.display());

    let path = directory.join("notes.txt");
    fs::write(&path, "alpha\nbeta\n").await?;
    let text = fs::read_to_string(&path).await?;
    let bytes = fs::read(&path).await?;
    println!("Whole-file text: {text:?}");
    println!("Whole-file bytes: {}", bytes.len());
    // fs::write creates or truncates its destination. Safe here because path
    // is inside the new private demo directory; do not use it blindly elsewhere.
    // read_to_string validates UTF-8; read returns arbitrary bytes.
    // Both whole-file helpers allocate for the entire file; use incremental
    // reading/buffering when the input is large.

    let file = fs::File::open(&path).await?;
    let mut lines = BufReader::new(file).lines();
    while let Some(line) = lines.next_line().await? {
        println!("File line: {line}");
    }

    let file = fs::OpenOptions::new().write(true).create_new(true)
        .open(directory.join("results.txt")).await?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"result=42\n").await?;
    writer.flush().await?;
    writer.get_ref().sync_all().await?;
    drop(writer);
    println!("Buffered result file flushed and sync_all completed");
    // create_new refuses to overwrite an existing file. flush sends buffered
    // output and waits for Tokio file writes to complete; sync_all additionally
    // requests filesystem persistence. Crash guarantees still depend on the
    // filesystem/platform; this is not a transactional replacement protocol.
    // On stable/default Tokio, ordinary filesystem work uses blocking operations
    // on its blocking pool. Awaiting does not turn disk hardware into socket readiness.
    // Async file APIs keep runtime tasks responsive, but do not automatically
    // make a bulk file-processing job faster than synchronous I/O plus threads.
    // Dropping the async future may leave an underlying started file operation running.
    // Files remain in the printed directory for inspection.
    Ok(())
}

// =====================================================
// 08. Cancellation and limits belong to the protocol design
// =====================================================

fn demo_cancellation_and_limits() {
    println!("08. Cancellation and limits belong to the protocol design");

    println!("read: accepts available bytes; read_exact: fills a specified buffer.");
    println!("read_to_end: waits for EOF; framing: identifies one application message.");
    println!("flush: drains output buffers; shutdown: ends the write direction.");
    // A raw read is cancellation-safe for select: losing does not consume bytes.
    // read_exact and write_all are NOT cancellation-safe: partial reads/writes
    // can happen before their futures are dropped. Blindly restarting may corrupt
    // your interpretation of the byte stream or duplicate transmitted bytes.
    // read_line is not cancellation-safe; Lines::next_line is the preferred
    // cancellation-safe line API when selecting among events.
    // Even with safe next_line, dropping the whole Lines/BufReader discards its
    // internal partial/buffered state. Keep the reader alive across select calls.
    // Our length-prefixed helper is NOT resumable after arbitrary cancellation.
    // Retain parsing progress explicitly, or close/discard that connection on error.
    // A timeout stops local waiting; it does not undo bytes sent or remote side effects.
    // Bound message sizes and waiting times for external peers. A finite tutorial
    // exchanging known data is not a complete hardened network service.
    // Next: managing task groups, blocking work, Rayon integration, and shutdown.
}

// =====================================================
// Helpers: small length-prefixed protocol (maximum 1024 bytes)
// =====================================================

async fn write_frame<W: AsyncWrite + Unpin>(writer: &mut W, payload: &[u8]) -> io::Result<()> {
    if payload.len() > 1_024 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "frame exceeds 1024 bytes"));
    }
    writer.write_u32(payload.len() as u32).await?; // big-endian by default
    writer.write_all(payload).await?;
    Ok(())
}

async fn read_frame<R: AsyncRead + Unpin>(reader: &mut R) -> io::Result<Vec<u8>> {
    let length = reader.read_u32().await?;
    if length > 1_024 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame exceeds 1024 bytes"));
    }
    let mut payload = vec![0_u8; length as usize];
    reader.read_exact(&mut payload).await?;
    Ok(payload)
}
