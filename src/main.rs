use std::env::args;
use std::num::ParseIntError;
use std::process::Command;
use std::{thread, time::Duration};

fn main() {
    start_notification_daemon();

    let mut args = args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" => {
                if let Err(_) = args
                    .next()
                    .expect("error while reading arg after flag")
                    .parse::<i32>()
                {
                    println!("invalid arg, using default values.");
                } else {
                    println!("tudo certo");
                }
            }
            _ => (),
        }
    }

    loop {
        let pomodoro: u16 = 15; // 25min
        thread::sleep(Duration::from_secs(pomodoro as u64));
        spawn_notification("Tempo de fazer uma pausa");
        let rest: u16 = 3; // 5min
        thread::sleep(Duration::from_secs(rest as u64));
        spawn_notification("Volte ao Trabalho");
    }
}

fn start_notification_daemon() {
    let status = Command::new("pgrep").arg("dunst").status();
    match status {
        Ok(s) if s.code() == Some(1) => run_dunst(),
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn run_dunst() {
    Command::new("dunst").spawn().expect("cant run Dunst");
}

fn spawn_notification(message: &str) {
    Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("Pomodoro")
        .arg(message)
        .spawn()
        .expect("error while sending notification");
}
