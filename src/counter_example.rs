use std::{thread, time};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show, MoveTo},
    style::Print,
};
use std::io::{stdout, Write};

fn main() {
    let mut stdout = stdout();

    // Enter alternate screen
    stdout.execute(EnterAlternateScreen).unwrap();
    // Hide the cursor
    stdout.execute(Hide).unwrap();

    let mut count = 0;

    loop {
        // Clear the screen
        stdout.queue(Clear(ClearType::All)).unwrap();
        // Move cursor to top-left
        stdout.queue(MoveTo(count * 3, count * 3)).unwrap();
        // Print the count
        stdout.queue(Print(format!("Count: {}", count))).unwrap();
        // Flush the output to the screen
        stdout.flush().unwrap();

        // Increment count
        count += 1;

        // Wait for a second
        thread::sleep(time::Duration::from_secs(1));

        // Exit the loop after 10 iterations
        if count > 3 {
            break;
        }
    }

    // Show the cursor
    stdout.execute(Show).unwrap();
    // Leave alternate screen
    stdout.execute(LeaveAlternateScreen).unwrap();
}