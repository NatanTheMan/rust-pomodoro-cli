use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::env::args;
use std::process::Command;
use std::{thread, time::Duration};

fn main() {
    // IN MINUTES
    let mut pomodoro_time: u16 = 25;
    let mut short_interval_time: u16 = 5;
    let mut long_interval_time: u16 = 15;

    set_args(
        &mut pomodoro_time,
        &mut short_interval_time,
        &mut long_interval_time,
    );

    start_notification_daemon();

    run_pomodoro(pomodoro_time, short_interval_time, long_interval_time);
}

fn set_args(pomodoro_time: &mut u16, short_interval_time: &mut u16, long_interval_time: &mut u16) {
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
                        *pomodoro_time = time;
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
                        *short_interval_time = time;
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
                        *long_interval_time = time;
                        println!("long {}", time)
                    }
                    Err(_) => println!("invalid arg, using default value for Pomodoro."),
                }
            }
            _ => println!("sem args"),
        }
    }
}

fn run_pomodoro(pomodoro_time: u16, short_interval_time: u16, long_interval_time: u16) {
    loop {
        sleep(pomodoro_time);

        spawn_notification("  Tempo de fazer uma pausa", "normal");
        sleep(short_interval_time);

        spawn_notification("  Volte ao Trabalho", "normal");
        sleep(pomodoro_time);

        spawn_notification("  Tempo de fazer uma pausa", "normal");
        sleep(short_interval_time);

        spawn_notification("  Volte ao Trabalho", "normal");
        sleep(pomodoro_time);

        spawn_notification("  Tempo de fazer uma pausa", "normal");
        sleep(short_interval_time);

        spawn_notification("  Volte ao Trabalho", "normal");
        sleep(pomodoro_time);

        spawn_notification("  Hora de uma pausa Longa", "critical");
        sleep(long_interval_time);
        spawn_notification("  Volte ao Trabalho", "normal");
    }
}

fn sleep(time: u16) {
    thread::sleep(Duration::from_secs((time as u64) * 60));
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

fn spawn_notification(message: &str, urgency: &str) {
    Command::new("notify-send")
        .arg("-u")
        .arg(urgency)
        .arg("Pomodoro")
        .arg(message)
        .spawn()
        .expect("error while sending notification");
    play_notification_sound();
}

fn play_notification_sound() {
    let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
    let sink = Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("./eagle_sound.mp3").unwrap();
    sink.append(Decoder::try_from(file).unwrap());

    sink.sleep_until_end();
}
