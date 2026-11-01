use crate::minecraft;
use slint::Weak;
use crate::auth::User;
use crate::settings::Settings;

slint::include_modules!();
pub struct App {
    pub ui: MainWindow,
    pub ui_weak: Weak<MainWindow>,
}
pub enum PlayStatus{
    Play,
    Starting,
    Update,
}

impl PlayStatus{
    pub fn data(self) -> (&'static str, slint::Color) {
        match self {
            PlayStatus::Play => (
                "Играть",
                slint::Color::from_rgb_u8(251, 130, 7),
            ),
            PlayStatus::Starting => (
                "Запуск..",
                slint::Color::from_rgb_u8(252, 181, 23),
            ),
            PlayStatus::Update => (
                "Обновить",
                slint::Color::from_rgb_u8(150, 150, 150),
            ),
        }
    }
}

impl App {
    pub fn new() -> Self {
        let ui = MainWindow::new().unwrap();
        let ui_weak = ui.as_weak();
        Self { ui, ui_weak }
    }
    //global Logic
    /*    pub fn set_button_play_status(&self, text: &str) {
                self.ui.global::<Logic>().set_button_play_status(text.into());
        }*/
    pub fn set_login_information_output(&self, text: &str) {
        self.ui.global::<Logic>().set_login_information_output(text.into());
    }
    pub fn play_status(ui_weak: Weak<MainWindow>, status: PlayStatus, enabled: bool) {
        let (output, color) = status.data();

        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_button_play_status(output.into());
                ui.global::<Color>().set_button_play(color.into());
                ui.global::<Enabled>().set_button_play(enabled.into());
            }
        });
    }

    pub fn username(&self, username: &str) {
        self.ui.global::<Logic>().set_username(username.into());
    }/*
    pub fn password(&self, password: &str){
        self.ui.global::<Logic>().set_password(password.into());
    }
    */

    pub fn memory(&self, memory: &i32) {
        self.ui.global::<Logic>().set_mamory(*memory);
    }
    pub fn bind_auth(&self) {
        let ui_weak = self.ui.as_weak();
        self.ui.global::<Logic>().on_auth_save(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let user = User { username: ui.global::<Logic>().get_username().to_string() };
            if let Err(e) = user.save() {
                eprintln!("не удалось сохранить: {e}");
            }
        });
    }

    pub fn bind_settings(&self) {
        let ui_weak = self.ui.as_weak();
        self.ui.global::<Logic>().on_settings_save(move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            let settings = Settings { memory: ui.global::<Logic>().get_mamory() };
            if let Err(e) = settings.save() {
                eprintln!("не удалось сохранить: {e}")
            }
        })
    }

    //выдод в интерфейс с процент газрузки
    pub fn emitter_mdp_minecraft(ui_weak: Weak<MainWindow>, text: String) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_progress_bar(text.into());
            }
        });
    }


    pub fn emitter_console_minecraft(ui_weak: Weak<MainWindow>, text: String) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_console_log(text.into());
            }
        });
    }

    pub fn hotbar_enabled(ui_weak: Weak<MainWindow>, res: bool) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Enabled>().set_hotbar(res.into());
            }
        });
    }

    pub fn hotbar_status(ui_weak: Weak<MainWindow>, text: String) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_status_bar(text.into());
            }
        });

    }
    pub fn button_play_enabled(ui_weak: Weak<MainWindow>, res: bool) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Enabled>().set_button_play(res.into());
            }
        });
    }
    pub fn app_update_enabled(ui_weak: Weak<MainWindow>, res: bool, status: PlayStatus) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                if !res{
                    ui.global::<Enabled>().set_global_button(res.into());
                    Self::play_status(ui_weak, status, res);
                    ui.global::<Enabled>().set_hotbar(true.into());
                    ui.global::<Logic>().set_status_bar("Обновление Лаунчера...".into());
                }
            }
        });
    }
    pub fn server_online(ui_weak: Weak<MainWindow>, online: String) {
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.global::<Logic>().set_server_online(online.into());
            }
        });
    }

    pub fn console_enabled(&self) {
        self.ui.global::<Enabled>().on_console_toggle(|enabled| {
            if enabled {
                crate::console::enable();
            } else {
                crate::console::disable();
            }
        });

    }

    //global Enabled

    pub fn minecraft_run(&self) {
        let ui_weak = self.ui.as_weak();
        self.ui.global::<Logic>().on_play_minecrat_process(move || {
            // блокируем кнопку в момент клика
            Self::play_status(ui_weak.clone(), PlayStatus::Play, true);

            let ui_weak = ui_weak.clone();
            tokio::spawn(async move {

                // Box<dyn Error> не Send, поэтому сразу превращаем в String
                let result = minecraft::minecraft_run(ui_weak.clone())
                    .await
                    .map_err(|e| e.to_string());

                if let Err(err) = result {
                    eprintln!("minecraft error: {err}");                              // в логи
                    Self::emitter_mdp_minecraft(ui_weak.clone(), format!("Ошибка: {err}")); // в хотбар скрыт текст выводит но не показывает
                }

                Self::play_status(ui_weak, PlayStatus::Play, true);
            });
        });
    }


    //runtime
    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.ui.run()
    }

}