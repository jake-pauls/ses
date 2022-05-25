pub enum LogType {
    Success,
    Error,
}

pub fn base(text: &str, level: LogType) {
    let icon: String;

    icon = match level {
        LogType::Success => "✔ [⚡] ".to_owned(),
        LogType::Error => "✘ [⚡] ".to_owned(),
    };

    let prompt: String = icon.clone() + &text;

    println!("{}", prompt);
}
