mod greetd;

use chrono::Local;
use eframe::egui;
use greetd::GreetdClient;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "better-greet",
        options,
        Box::new(|_cc| Box::new(BetterGreetApp::default())),
    )
}

struct BetterGreetApp {
    username: String,
    password: String,
    error_msg: Option<String>,
}

impl Default for BetterGreetApp {
    fn default() -> Self {
        Self {
            username: "wastle".to_string(), // Измени на свой логин по умолчанию
            password: String::new(),
            error_msg: None,
        }
    }
}

impl eframe::App for BetterGreetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Фоновое окно
        egui::CentralPanel::default().show(ctx, |ui| {
            let screen_width = ui.available_width();
            let screen_height = ui.available_height();

            // Левая панель (28% ширины)
            let panel_width = screen_width * 0.28;

            egui::SidePanel::left("left_panel")
                .resizable(false)
                .exact_width(panel_width)
                .frame(egui::Frame::none().fill(egui::Color32::from_rgba_unmultiplied(30, 30, 46, 210)))
                .show_inside(ui, |ui| {
                    ui.add_space(screen_height * 0.25);
                    ui.vertical_centered(|ui| {
                        // Крупные часы
                        let time_str = Local::now().format("%H:%M").to_string();
                        ui.label(
                            egui::RichText::new(time_str)
                                .size(64.0)
                                .bold()
                                .color(egui::Color32::from_rgb(205, 214, 244)),
                        );

                        // Дата
                        let date_str = Local::now().format("%A, %d %B").to_string();
                        ui.label(
                            egui::RichText::new(date_str)
                                .size(18.0)
                                .color(egui::Color32::from_rgb(166, 173, 200)),
                        );

                        ui.add_space(40.0);

                        // Поля ввода
                        ui.scope(|ui| {
                            ui.set_max_width(panel_width * 0.8);
                            
                            ui.add(
                                egui::TextEdit::singleline(&mut self.username)
                                    .hint_text("Пользователь")
                                    .margin(egui::vec2(10.0, 10.0)),
                            );

                            ui.add_space(10.0);

                            let pass_response = ui.add(
                                egui::TextEdit::singleline(&mut self.password)
                                    .password(true)
                                    .hint_text("Пароль")
                                    .margin(egui::vec2(10.0, 10.0)),
                            );

                            ui.add_space(20.0);

                            // Кнопка логина или Enter
                            if (ui.button("Войти").clicked() || (pass_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) {
                                if let Ok(mut client) = GreetdClient::new() {
                                    if let Err(err) = client.login(&self.username, &self.password, vec!["Hyprland"]) {
                                        self.error_msg = Some(err);
                                    }
                                } else {
                                    self.error_msg = Some("Greetd IPC error".to_string());
                                }
                            }

                            if let Some(ref err) = self.error_msg {
                                ui.add_space(10.0);
                                ui.label(egui::RichText::new(err).color(egui::Color32::RED));
                            }
                        });
                    });
                });
        });

        // Запрашиваем перерисовку каждую секунду для часов
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}
