/* rust lib imports */

use std::io::{Write};
use std::thread;

/* 3rd-party imports */

use time;
use chrono;
use hyper::server::{Request};

/* ////////////////////////////////////////////////////////////////////////// */

/* LogEntry */

// Shamelessly taken from:
// https://github.com/tomaka/rouille/blob/68b9c65886f8aa75107f0ce3423a790f95ab675a/src/log.rs
/// RAII guard that ensures that a log entry corresponding to a request will be written.
///
/// # Example
///
/// ```no_run
/// rouille::start_server("localhost:80", move |request| {
///     let _entry = rouille::LogEntry::start(std::io::stdout(), request);
///
///     // process the request here
///
/// # panic!()
/// }); // <-- the log entry is written at the end of this block
/// ```
///
pub struct LogEntry<W>
    where W: Write
{
    line: String,
    output: W,
    start_time: u64,
}

impl<'a, W> LogEntry<W>
    where W: Write
{
    /// Starts a `LogEntry`.
    #[inline]
    pub fn start(output: W, rq: &Request) -> LogEntry<W> {
        LogEntry {
            line: format!("{} {}", rq.method, rq.uri),
            output: output,
            start_time: time::precise_time_ns(),
        }
    }
}

impl<W> Drop for LogEntry<W>
    where W: Write
{
    #[inline]
    fn drop(&mut self) {
        write!(self.output, "{} - ", self.line).unwrap();

        if thread::panicking() {
            write!(self.output, " - PANIC!").unwrap();
        } else {
            let elapsed = time::precise_time_ns() - self.start_time;
            format_time(self.output.by_ref(), elapsed);
        }

        let utc: chrono::DateTime<chrono::UTC> = chrono::UTC::now();
        // TODO: needs ms and ns granularity
        write!(self.output, " - {}", utc.to_rfc2822()).unwrap();

        writeln!(self.output, "").unwrap();
    }
}

#[inline]
fn format_time<W>(mut out: W, time: u64)
    where W: Write
{
    if time < 1_000 {
        write!(out, "{}ns", time).unwrap()
    } else if time < 1_000_000 {
        write!(out, "{:.1}us", time as f64 / 1_000.0).unwrap()
    } else if time < 1_000_000_000 {
        write!(out, "{:.1}ms", time as f64 / 1_000_000.0).unwrap()
    } else {
        write!(out, "{:.1}s", time as f64 / 1_000_000_000.0).unwrap()
    }
}
