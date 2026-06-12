use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Spawns a background task to simulate capturing screenshots
/// every 300ms, pushing them directly into the ring buffer inside AppState.
pub async fn start_capture_thread(buffer: Arc<Mutex<VecDeque<String>>>) {
    // Mock capture loop
    let mut counter = 0;
    loop {
        // Wait 300ms between captures
        sleep(Duration::from_millis(300)).await;

        let mut buf_lock = buffer.lock().await;

        // Generate a mock base64 screenshot string
        let mock_base64 = format!("data:image/png;base64,mock_screenshot_{}", counter);
        
        // Push to back of queue
        buf_lock.push_back(mock_base64);

        // Maintain max 10 items
        if buf_lock.len() > 10 {
            buf_lock.pop_front();
        }

        counter += 1;
    }
}
