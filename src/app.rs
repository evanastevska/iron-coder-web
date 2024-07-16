use egui::{Align, CentralPanel, Label, TextEdit, TopBottomPanel};

use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
enum Page {
    Home,
    AboutPage,
    Login,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    current_page: Page,

    #[serde(skip)]
    username: String,

    #[serde(skip)]
    password: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            //Valid info:
            current_page: Page::Login,
            username: String::new(),
            password: String::new(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts.

        // Load previous app state (if any).
        // Note that you must enable the persistence feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a SidePanel, TopBottomPanel, CentralPanel, Window or Area.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        } else if ui.button("Back to login").clicked() {
                            self.current_page = Page::Login;
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        //Central panel will display main contents
        CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                //First immediate page is Login page
                //Log in or Continue as a guest
                //Creating an account does not currently function
                Page::Login => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("Welcome!");
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.heading("Create an account");
                        ui.add_space(15.0);
                        ui.heading("OR");
                        ui.add_space(15.0);
                        ui.vertical(|ui| {
                            ui.heading("Log in");
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut self.username);
                        ui.label("Password:");
                        ui.add(TextEdit::singleline(&mut self.password).password(true));

                        if ui.button("Log in").clicked() {
                            // Still need to validate credentails
                            self.current_page = Page::Home;
                        }

                        });
                        ui.add_space(15.0);
                        ui.heading("OR");
                        ui.add_space(15.0);
                        ui.vertical(|ui| {
                            ui.heading("Continue as a Guest");
                        if ui.button("Guest").clicked() {
                            self.current_page = Page::Home;
                        }

                        });

                        });

                }//Home page contains main contents, user can manuvear to Github and About page
                Page::Home => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("Iron Coder");
                    });

                    ui.separator();

                    //About Iron Coder and GitHub link buttons
                    ui.horizontal(|ui| {
                        if ui.button("GitHub").clicked() {
                            open::that("https://github.com/shulltronics/iron-coder.git").expect("Failed to open URL");
                        }

                        ui.add_space(10.0);

                        if ui.button("About Iron Coder").clicked() {
                            self.current_page = Page::AboutPage;
                        }
                    });

                    ui.separator();

                    ui.add(egui::github_link_file!(
                        "https://github.com/emilk/eframe_template/blob/main/",
                        "Source code."
                    ));

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                        powered_by_egui_and_eframe(ui);
                        egui::warn_if_debug_build(ui);
                    });
                }
                //Contains description of Iron Coder
                Page::AboutPage => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("About Iron Coder");
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Description: ");
                        ui.add(Label::new("Iron Coder is an IDE designed to simplify embedded development in Rust. Inspired by modular hardware ecosystems such as Adafruit's Feather, Sparkfun's MicroMod, and more, Iron Coder generates project templates and code boilerplates based on hardware architecture descriptions. This approach provides new developers with an accessible platform to learn Rust, while enabling rapid prototyping of embedded hardware and firmware systems.").wrap());
                    });

                    ui.add_space(10.0);

                    if ui.button("Go back to Home Page").clicked() {
                        self.current_page = Page::Home;
                    }
                }
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
