// Clipboard operations

use std::fmt;

#[derive(Debug)]
pub enum ClipboardError {
    InitFailed(String),
    CopyFailed(String),
    NotAvailable,
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClipboardError::InitFailed(msg) => write!(f, "Failed to initialize clipboard: {}", msg),
            ClipboardError::CopyFailed(msg) => write!(f, "Failed to copy to clipboard: {}", msg),
            ClipboardError::NotAvailable => write!(f, "Clipboard is not available in this environment"),
        }
    }
}

impl std::error::Error for ClipboardError {}

pub fn copy_to_clipboard(text: &str) -> Result<(), ClipboardError> {
    use arboard::Clipboard;
    
    let mut clipboard = Clipboard::new()
        .map_err(|e| ClipboardError::InitFailed(e.to_string()))?;
    
    clipboard.set_text(text)
        .map_err(|e| ClipboardError::CopyFailed(e.to_string()))?;
    
    Ok(())
}
