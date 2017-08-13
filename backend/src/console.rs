use ::BackendMsg;
use std::io;
use std::sync::mpsc::SyncSender;


pub fn console_task(sender: SyncSender<BackendMsg>) {
    let run = true;

    while run {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let command = match input {
            "q" | "quit" => Some(BackendMsg::Quit),
            "" => Some(BackendMsg::Take),
            "a" => Some(BackendMsg::AutoTransition{secs: 0.25}),
            input => {
                match input.parse::<i32>() {
                    Ok(id) => Some(BackendMsg::SetPreview{id: id}),
                    Err(_) => None,
                }
            }
        };

        match command {
            Some(x) => sender.send(x).unwrap(),
            _ => {},
        }
    }
}
