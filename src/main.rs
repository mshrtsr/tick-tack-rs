use chrono::Local;
use figlet_rs::FIGfont;

use termion::{clear, cursor};

fn main() {
    let standard_font = FIGfont::standand().unwrap();
    let delay = std::time::Duration::from_millis(1000);

    let heading = "Hello!! Would you like \"Tick Tack\"?\n";
    let tick = " ₍₍⁽⁽Tick!!₎₎⁾⁾\n\n";
    let tack = "\n         ₍₍⁽⁽Tack!!₎₎⁾⁾\n";

    let mut tick_tack_flag = true;

    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        let local_date = Local::today().format("%Y/%m/%d").to_string();
        let local_time = Local::now().format("%H:%M:%S").to_string();
        let local_date = standard_font.convert(local_date.as_str()).unwrap();
        let local_time = standard_font.convert(local_time.as_str()).unwrap();

        let tick_or_tack = if tick_tack_flag { tick } else { tack };
        print!(
            "\n{}\n{}{}{}",
            heading, local_date, local_time, tick_or_tack
        );

        tick_tack_flag = !tick_tack_flag;

        std::thread::sleep(delay);
    }
}
