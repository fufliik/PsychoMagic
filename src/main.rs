use crate::gui::PlayStatus;


mod gui;
mod minecraft;
mod dir;
mod auth;
mod update;
mod server;

mod settings;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = gui::App::new();
    let ui_weak = ui.ui_weak.clone();


    let update_ui_weak = ui_weak.clone();
    tokio::spawn(async move {
        let result = tokio::task::spawn_blocking(|| {
            update::check().map_err(|e| e.to_string())
        }).await;
        match result {
            Ok(Ok(true)) => {
                gui::App::app_update_enabled(update_ui_weak.clone(),false, PlayStatus::Update);

                let result = tokio::task::spawn_blocking(|| {
                    update::run().map_err(|e| e.to_string())
                }).await;

                match result {
                    Ok(Ok(())) => println!("Обновление завершено"),
                    Ok(Err(e)) => println!("Ошибка обновления: {e}"),
                    Err(e) => println!("Ошибка задачи: {e}"),
                }
            }
            Ok(Ok(false)) => {}
            Ok(Err(e)) => {
                println!("Ошибка проверки обновления: {e}");
            }
            Err(e) => {
                println!("Ошибка задачи: {e}");
            }
        }
    });
    gui::App::app_update_enabled(ui_weak.clone(),true, PlayStatus::Play);

    let server_ui_weak = ui_weak.clone();
    tokio::spawn(async move {
        loop {
            let status = server::check_online().await;

            let online = match status {
                Some((online, max)) => {
                    format!("{online}/{max}")
                }
                None => {
                    "None".to_string()
                }
            };
            gui::App::server_online(server_ui_weak.clone(), online);

            tokio::time::sleep(
                std::time::Duration::from_secs(5)
            ).await;
        }
    });

    let user = auth::User::load().unwrap_or_default(); // нет файла или битый: пустой ник
    ui.username(&user.username);                       // поле ввода заполнится само

    let memory = settings::Settings::load().unwrap_or_default();
    ui.memory(&memory.memory);

    gui::App::play_status(ui_weak.clone(), PlayStatus::Play, true);
    ui.bind_auth();
    ui.bind_settings();

    ui.minecraft_run();
    ui.run()


}
