//! Excel (.xlsx) import. The parser can abort or exhaust memory on a crafted
//! file, so it runs in a child process, a copy of this program started with
//! [`CHILD_COMMAND`], that limits its own memory and CPU time. The server
//! only reads the child's JSON output.

use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::process::{ExitCode, Stdio};
use std::time::Duration;

use calamine::{Reader, Xlsx};
use rustix::process::{Resource, Rlimit, setrlimit};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::{ImportError, MAX_BYTES, MAX_ROWS, Sheet, rows_from_cells};

/// The hidden subcommand that runs the parser.
pub const CHILD_COMMAND: &str = "parse-xlsx";

/// The child's address-space limit.
const MEMORY_LIMIT_BYTES: u64 = 512 * 1024 * 1024;
/// The child's CPU-time limit.
const CPU_LIMIT_SECONDS: u64 = 10;
/// The most JSON the server reads back from the child.
const MAX_OUTPUT_BYTES: u64 = 16 * 1024 * 1024;
/// How long the server waits for the child.
const WAIT_LIMIT: Duration = Duration::from_secs(15);

/// Reads .xlsx files through the child process.
#[derive(Debug, Clone)]
pub struct XlsxReader {
    program: PathBuf,
}

impl XlsxReader {
    /// A reader that starts `program` (this program's own binary) as the child.
    pub fn new(program: PathBuf) -> Self {
        Self { program }
    }

    /// The first sheet's rows.
    pub async fn read(&self, bytes: &[u8]) -> Result<Sheet, ImportError> {
        if bytes.len() > MAX_BYTES {
            return Err(ImportError::TooLarge);
        }
        let unreadable = || ImportError::Unreadable("it isn't a readable Excel (.xlsx) file".to_owned());
        // No environment: a parser taken over by a crafted file finds no
        // database URL or API keys.
        let mut child = tokio::process::Command::new(&self.program)
            .arg(CHILD_COMMAND)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|err| ImportError::Unreadable(format!("the Excel reader didn't start: {err}")))?;
        let (stdin, stdout) = (child.stdin.take(), child.stdout.take());
        let talk = async {
            if let Some(mut stdin) = stdin {
                // A child that dies early closes the pipe; its exit status says why.
                let _ = stdin.write_all(bytes).await;
            }
            let mut output = Vec::new();
            if let Some(stdout) = stdout {
                stdout.take(MAX_OUTPUT_BYTES + 1).read_to_end(&mut output).await?;
            }
            if output.len() as u64 > MAX_OUTPUT_BYTES {
                // Dropping the child kills it.
                return Ok(None);
            }
            Ok::<_, std::io::Error>(Some((output, child.wait().await?)))
        };
        let (output, status) = tokio::time::timeout(WAIT_LIMIT, talk)
            .await
            .map_err(|_| ImportError::Unreadable("it took too long to read".to_owned()))?
            .map_err(|err| ImportError::Unreadable(err.to_string()))?
            .ok_or_else(unreadable)?;
        if !status.success() {
            return Err(unreadable());
        }
        let cells: Vec<Vec<String>> = serde_json::from_slice(&output).map_err(|_| unreadable())?;
        rows_from_cells(cells)
    }
}

/// The child: limits itself, reads a file on stdin and writes the first
/// sheet's cells to stdout as JSON.
pub fn run_child() -> ExitCode {
    for (resource, limit) in [(Resource::As, MEMORY_LIMIT_BYTES), (Resource::Cpu, CPU_LIMIT_SECONDS)] {
        // Some systems (macOS) refuse an address-space limit; the server's
        // wait limit still applies there.
        let _ = setrlimit(
            resource,
            Rlimit {
                current: Some(limit),
                maximum: Some(limit),
            },
        );
    }
    let mut bytes = Vec::new();
    let limit = u64::try_from(MAX_BYTES).unwrap_or(u64::MAX);
    if std::io::stdin().take(limit).read_to_end(&mut bytes).is_err() {
        return ExitCode::from(2);
    }
    let Some(cells) = cells_from_xlsx(bytes) else {
        return ExitCode::from(2);
    };
    match serde_json::to_writer(std::io::stdout().lock(), &cells) {
        Ok(()) => std::io::stdout()
            .flush()
            .map_or(ExitCode::from(2), |()| ExitCode::SUCCESS),
        Err(_) => ExitCode::from(2),
    }
}

/// The first sheet's rows from row 1, stopping once there are more
/// non-blank rows than an import allows.
fn cells_from_xlsx(bytes: Vec<u8>) -> Option<Vec<Vec<String>>> {
    let mut workbook = Xlsx::new(Cursor::new(bytes)).ok()?;
    let range = workbook.worksheet_range_at(0)?.ok()?;
    let first_row = range.start().map_or(0, |(row, _)| row);
    let mut cells: Vec<Vec<String>> = (0..first_row).map(|_| Vec::new()).collect();
    let mut non_blank = 0;
    for row in range.rows() {
        let mut texts: Vec<String> = row.iter().map(ToString::to_string).collect();
        while texts.last().is_some_and(String::is_empty) {
            texts.pop();
        }
        non_blank += usize::from(!texts.is_empty());
        cells.push(texts);
        // The header, the allowed rows, and one more to report "too many".
        if non_blank > MAX_ROWS + 1 {
            break;
        }
    }
    Some(cells)
}
