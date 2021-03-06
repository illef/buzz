use std::process::*;

fn unseen_mail_count() -> Result<usize, String> {
    let _ = Command::new("bash")
        .arg("-c")
        .arg("notmuch new")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = Command::new("bash")
        .arg("-c")
        .arg("notmuch search 'tag:unread and folder:/.*INBOX/'")
        .output()
        .map_err(|e| e.to_string())?
        .stdout;

    Ok(std::str::from_utf8(&stdout)
        .map_err(|e| e.to_string())?
        .lines()
        .count())
}

fn main() {
    use std::fs::File;
    use std::io::prelude::*;

    File::create("/tmp/buzz-default.svg")
        .unwrap()
        .write_all(include_bytes!("../default.svg"))
        .unwrap();

    File::create("/tmp/buzz-unread.svg")
        .unwrap()
        .write_all(include_bytes!("../unread.svg"))
        .unwrap();

    let app = match systray::Application::new() {
        Ok(app) => app,
        Err(e) => {
            println!("Could not create gtk application: {}", e);
            return;
        }
    };

    loop {
        match unseen_mail_count() {
            Ok(count) => {
                if count > 0 {
                    app.set_icon_from_file(&"/tmp/buzz-unread.svg".to_string())
                        .unwrap();
                } else {
                    app.set_icon_from_file(&"/tmp/buzz-default.svg".to_string())
                        .unwrap();
                }
            }
            Err(e) => {
                println!("Something wrong!, err: {}", e);
                return;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
