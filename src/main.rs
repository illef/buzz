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
    if let Err(e) = unseen_mail_count() {
        println!("Could not get unseen mail count, err : {}", e);
        return;
    }

    let app = match systray::Application::new() {
        Ok(app) => app,
        Err(e) => {
            println!("Could not create gtk application: {}", e);
            return;
        }
    };

    if let Err(e) = app
        .set_icon_from_file(&"/usr/share/icons/oxygen/base/32x32/status/mail-read.png".to_string())
    {
        println!("Could not set application icon: {}", e);
    }

    loop {
        match unseen_mail_count() {
            Ok(count) => {
                if count > 0 {
                    app.set_icon_from_file(
                        &"/usr/share/icons/oxygen/base/32x32/status/mail-unread-new.png"
                            .to_string(),
                    )
                    .unwrap();
                }
            }
            Err(e) => {
                println!("Something wrong!, err: {}", e);
                return;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
