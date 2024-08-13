

use std::{io::{stdout, Write}, thread, time::{self, Duration}};

use crossterm::{cursor::{Hide, MoveTo, Show}, event::{poll, read, Event}, style::Print, terminal::{size, Clear, EnterAlternateScreen, LeaveAlternateScreen}, QueueableCommand};

fn main() {
    let mut stdout = stdout();
    // let test = format!("test {}", "hehe");
    stdout.queue(EnterAlternateScreen).unwrap();
    let size = size();
    let (mut width, mut height) = match size {
        Ok((w, h)) => (w, h),
        Err(error) => panic!("Could not read terminal size: {error:?}"),
    };
    let mut size_msg;
    let mut size_msg_len: u16;
    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(w, h) => {
                    width = w;
                    height = h;
                },
                Event::Key(event) => todo!(),
                _ => {}
            }
        }
        stdout.queue(Clear(crossterm::terminal::ClearType::All)).unwrap();
        size_msg = format!("Terminal size is: {}x{}", width, height);
        size_msg_len = size_msg.len() as u16;
        stdout.queue(MoveTo(width / 2 - size_msg_len / 2, height / 2)).unwrap();
        stdout.queue(Print(&size_msg)).unwrap();
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(33));
    }


    // stdout().queue(EnterAlternateScreen).unwrap();
    // stdout().queue(Hide).unwrap();
    // stdout().queue(MoveTo(width / 2, height / 2)).unwrap();
    // println!("{} x {}", width, height);
    // thread::sleep(time::Duration::from_secs(3));
    // stdout().queue(Show).unwrap();
    // stdout().queue(LeaveAlternateScreen).unwrap();

}
