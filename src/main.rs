use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (sender, receiver) = mpsc::channel(); // создаем mpsc канал

    // создаем поток который ожидает 5 секунд а затем завершает работу программы
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        process::exit(0);
    });
    // создаем поток который отправляет значения в канал с периодичностью в 1 секунду
    thread::spawn(move || loop {
        let message = String::from("message");

        sender.send(message).unwrap();
        thread::sleep(Duration::from_secs(1));
    });

    // итерируемся по значениям полученным из канала
    for received in receiver {
        println!("message from thread: {received}");
    }
}
