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
    
    let ui_weak = ui.ui_weak.clone();
    tokio::spawn(async move {
        loop {
            let status = server::check_online().await;

            let online = match status {
                Some((online, max)) => {
                    format!("{online}/{max}")
                }
                None => {
                    "Сервер выключен".to_string()
                }
            };
            gui::App::server_online(ui_weak.clone(), online);

            tokio::time::sleep(
                std::time::Duration::from_secs(5)
            ).await;
        }
    });

    let user = auth::User::load().unwrap_or_default(); // нет файла или битый: пустой ник
    ui.username(&user.username);                       // поле ввода заполнится само

    gui::App::play_status(ui.ui_weak.clone(), Play, true);
    ui.bind_auth();

    ui.minecraft_run();
    ui.run()


}
