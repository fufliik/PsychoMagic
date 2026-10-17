use crate::gui::PlayStatus::{Play, Update};

mod gui;
mod minecraft;
mod dir;
mod auth;
mod update;
mod server;
mod mods;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = gui::App::new();
    eprintln!("start");


    //переделать и инфо  в гуи
    match tokio::task::spawn_blocking(|| {
        update::check();
    }).await
    {
        Ok(info) => {
            println!()
        }
        Err(e) => {
            println!("Ошибка обновление: {}", e)
        }
    }

    let user = auth::User::load().unwrap_or_default(); // нет файла или битый: пустой ник
    ui.username(&user.username);                       // поле ввода заполнится само

    gui::App::play_status(ui.ui_weak.clone(), Play, true);
    ui.bind_auth();

    ui.minecraft_run();
    ui.run()


}
