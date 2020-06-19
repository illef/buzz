fn main() {
    let mut app = match systray::Application::new() {
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

    app.wait_for_message().unwrap();
}
