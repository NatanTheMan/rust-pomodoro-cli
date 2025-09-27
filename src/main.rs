use std::process::Command;
use std::{thread, time::Duration};

fn main() {
    loop {
        let pomodoro: u16 = 15; // 25min
        thread::sleep(Duration::from_secs(pomodoro as u64));
        print_pomodoro_finished();

        let rest: u16 = 3; // 5min
        thread::sleep(Duration::from_secs(rest as u64));
        print_rest_finished();
    }
}

fn print_rest_finished() {
    Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("Pomodoro")
        .arg("Volte ao Trabalho")
        .spawn()
        .expect("error");
}

fn print_pomodoro_finished() {
    Command::new("notify-send")
        .arg("-u")
        .arg("normal")
        .arg("Pomodoro")
        .arg("Tempo de fazer uma pausa")
        .spawn()
        .expect("error");
}
