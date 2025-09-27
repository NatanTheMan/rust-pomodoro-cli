use std::env::args;
use std::process::Command;
use std::{thread, time::Duration};

fn main() {
    // IN MINUTES
    let mut pomodoro_time: u16 = 25;
    let mut short_interval_time: u16 = 5;
    let mut long_interval_time: u16 = 15;

    let mut args = args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" | "--pomodoro" => {
                match args
                    .next()
                    .expect("error while reading pomodoro time")
                    .parse::<u16>()
                {
                    Ok(time) => {
                        pomodoro_time = time;
                        println!("pomodoro {}", time)
                    }
                    Err(_) => println!("invalid arg, using default value for Pomodoro."),
                }
            }
            "-s" | "--short-interval" => {
                match args
                    .next()
                    .expect("error while reading short time")
                    .parse::<u16>()
                {
                    Ok(time) => {
                        short_interval_time = time;
                        println!("short {}", time)
                    }
                    Err(_) => println!("invalid arg, using default value for Pomodoro."),
                }
            }
            "-l" | "--long-interval" => {
                match args
                    .next()
                    .expect("error while reading long time")
                    .parse::<u16>()
                {
                    Ok(time) => {
                        long_interval_time = time;
                        println!("long {}", time)
                    }
                    Err(_) => println!("invalid arg, using default value for Pomodoro."),
                }
            }
            _ => println!("sem args"),
        }
    }

    start_notification_daemon();

    run_pomodoro(pomodoro_time, short_interval_time, long_interval_time);
}

fn run_pomodoro(pomodoro_time: u16, short_interval_time: u16, long_interval_time: u16) {
    thread::sleep(Duration::from_secs(pomodoro_time as u64) / 20);
    spawn_notification("  Tempo de fazer uma pausa");

    thread::sleep(Duration::from_secs(short_interval_time as u64) / 5);
    spawn_notification("  Volte ao Trabalho");
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
        .arg("Pomodoro")
        .arg(message)
        .spawn()
        .expect("error while sending notification");
}
