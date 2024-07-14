use std::borrow::ToOwned;
use egui::Pos2;
use egui::Shape::LineSegment;
use egui::TextStyle::Button;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)] // This how you opt-out of serialization of a field
    team_one_name: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    team_two_name: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    xs_turn: bool,
    #[serde(skip)] // This how you opt-out of serialization of a field
    board: [i32; 9],
}

fn get_winner(board: &[i32; 9]) -> i32 {
    for i in 0..3 {
        if board[3 * i] == board[3 * i + 1] && board[3 * i + 1] == board[3 * i + 2] && board[3 * i] != 0 {
            return board[3 * i];
        }
        if board[i] == board[3 + i] && board[3 + i] == board[6 + i] && board[i] != 0 {
            return board[i];
        }
    }
    for i in 0..2 {
        if board[2 * i] == board[4] && board[4] == board[8 - 2 * i] && board[4] != 0 {
            return board[4];
        }
    }
    for i in 0..9 {
        if board[i] == 0 {
            return 0;
        }
    }
    return -1;
}


impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            team_one_name: "Team One".to_owned(),
            team_two_name: "Team Two".to_owned(),
            value: 2.7,
            xs_turn: true,
            board: [0, 0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let index_to_symbol = [" - ".to_owned(), "X".to_owned(), "O".to_owned()];
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Time for Tic-Tac-Toe");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            for r in 0..3 {
                ui.horizontal(|ui| {
                    for c in 0..3 {
                        let button = egui::Button::new(index_to_symbol[self.board[r * 3 + c] as usize].to_owned());
                        if ui.add_sized([50., 50.], button).clicked() {
                            if self.board[r * 3 + c] != 0 {
                                //explode
                            } else {
                                self.board[r * 3 + c] = 1 + if self.xs_turn { 0 } else { 1 };
                                self.xs_turn = !self.xs_turn;
                            }
                        }
                    }
                });
                ui.separator();
            }
            let winner = get_winner(&self.board);
            if winner > 0 {
                ui.label(index_to_symbol[winner as usize].to_owned() + "'s have won the game!");
            } else if winner < 0 {
                ui.label("It's a fockin' draw lassie");
            }
            if ui.button("Reset").clicked() {
                self.board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
                self.xs_turn = true;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}