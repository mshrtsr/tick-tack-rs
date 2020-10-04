use chrono::Local;
use figlet_rs::FIGfont;

use termion::{clear, cursor};

fn main() {
    let standard_font = FIGfont::standand().unwrap();
    let delay = std::time::Duration::from_millis(1000);

    let heading = "Hello!! Would you like \"Tick Tack\"?";
    let tick = " ₍₍⁽⁽Tick!!₎₎⁾⁾\n";
    let tack = "\n         ₍₍⁽⁽Tack!!₎₎⁾⁾";
    let mut tick_tack_flag = true;

    loop {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        let local_date = Local::today().format("%Y/%m/%d").to_string();
        let local_time = Local::now().format("%H:%M:%S").to_string();
        let local_date = standard_font.convert(local_date.as_str());
        let local_time = standard_font.convert(local_time.as_str());

        println!();
        println!("{}", heading);
        print!("{}", local_date.unwrap());
        print!("{}", local_time.unwrap());
        if tick_tack_flag {
            println!("{}", tick);
        } else {
            println!("{}", tack);
        }

        tick_tack_flag = !tick_tack_flag;

        std::thread::sleep(delay);
    }
}
