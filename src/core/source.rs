//! Source code stream provider for the matrix background.

use std::fs;
use std::path::Path;

pub struct SourceStream {
    chars: Vec<char>,
}

impl SourceStream {
    /// Loads from a file, or falls back to built-in systems code if path is missing or fails.
    pub fn new(path: Option<&str>) -> Self {
        if let Some(p) = path {
            if let Ok(content) = fs::read_to_string(Path::new(p)) {
                let cleaned: Vec<char> = content
                    .chars()
                    .filter(|c| !c.is_control() || *c == ' ')
                    .collect();
                if !cleaned.is_empty() {
                    return Self { chars: cleaned };
                }
            }
        }

        Self::default_builtin()
    }

    /// Built-in dense code texture (real systems code snippet)
    pub fn default_builtin() -> Self {
        let code = r#"
fn schedule_task<T: Send + 'static>(task: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
    let (tx, rx) = oneshot::channel();
    let runner = async move {
        let result = task.await;
        let _ = tx.send(result);
    };
    Runtime::current().spawn(runner);
    JoinHandle { rx }
}

struct EnginePipeline {
    frame_idx: u64,
    delta_time: f32,
    buffers: DoubleBuffer,
    mask_cache: LruCache<HashKey, MaskMap>,
}

impl EnginePipeline {
    #[inline(always)]
    fn render_matrix(&mut self, grid: &Grid, palette: &Palette) {
        let cols = grid.cols as usize;
        let rows = grid.rows as usize;
        for y in 0..rows {
            let row_offset = (y * cols + (self.frame_idx as usize * 3)) % SOURCE_LEN;
            for x in 0..cols {
                let ch = SOURCE_CHARS[(row_offset + x) % SOURCE_LEN];
                self.buffers.set(x, y, ch);
            }
        }
    }
}

pub async fn connect_matrix_stream(uri: &str) -> Result<StreamClient, Error> {
    let stream = TcpStream::connect(uri).await?;
    let (mut reader, mut writer) = stream.into_split();
    tokio::spawn(async move {
        while let Some(msg) = reader.next_frame().await {
            dispatch_event(msg);
        }
    });
    Ok(StreamClient::new(writer))
}
"#;
        // Clean whitespace, collapse redundant spaces into single spaces
        let mut chars = Vec::with_capacity(code.len());
        let mut prev_space = false;
        for c in code.chars() {
            if c.is_whitespace() {
                if !prev_space {
                    chars.push(' ');
                    prev_space = true;
                }
            } else {
                chars.push(c);
                prev_space = false;
            }
        }

        Self { chars }
    }

    /// Total character count
    #[inline]
    pub fn len(&self) -> usize {
        self.chars.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.chars.is_empty()
    }

    /// Retrieves character at index modulo length
    #[inline]
    pub fn char_at(&self, idx: usize) -> char {
        self.chars[idx % self.chars.len()]
    }
}
