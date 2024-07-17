use egui::{CentralPanel, Label, TextEdit, TopBottomPanel, Align};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(PartialEq)]
enum Page {
    Home,
    About,
    Login,
    Register,
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

    #[serde(skip)]
    register_username: String,

    #[serde(skip)]
    register_password: String,

    #[serde(skip)]
    registration_message: String,
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
            register_username: String::new(),
            register_password: String::new(),
            registration_message: String::new(),
        }
    }
}

impl TemplateApp {
    // Called once before the first frame.
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

    // Registers a new user by appending their username and password to users.txt file
    fn register_user(&self) -> io::Result<()> {
        let path = "users.txt";
        // If users.txt file does not exist, creates
        if !Path::new(path).exists() {
            File::create(path)?;
        }

        // Check if the username already exists
        if self.username_exists(&self.register_username)? {
            // Return an error if already taken
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Username already exists"));
        }

        // Open file in append mode
        let mut file = OpenOptions::new()
            .append(true)
            .open(path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", e)))?;

        // Write username and password to file
        writeln!(file, "{} {}", self.register_username, self.register_password)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to write to file: {}", e)))?;

        println!("Debug: Registered new user: {}", self.register_username);
        Ok(())
    }

    // Checks if a username already exists in file
    fn username_exists(&self, username: &str) -> io::Result<bool> {
        let path = "users.txt";
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Debugging: Print all lines read from the file
        println!("Checking for existing usernames in users.txt...");

        // Go thru each line in the file
        for line in reader.lines() {
            let line = line?;
            println!("Read line: {}", line);
            let mut parts = line.split_whitespace();
            // Get username part and check if it matches input
            if let Some(existing_username) = parts.next() {
                println!("Checking username: {}", existing_username);
                if existing_username == username {
                    println!("Username {} already exists.", username);
                    return Ok(true);
                }
            }
        }

        println!("Username {} does not exist.", username);
        Ok(false)
    }

    // Checks if input username and password match existing user
    fn user_exists(&self) -> io::Result<bool> {
        let path = "users.txt";
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Go thru each line in file
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            // Get username and password parts, check if they match credentials
            if let (Some(username), Some(password)) = (parts.next(), parts.next()) {
                if username == self.username && password == self.password {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl eframe::App for TemplateApp {
    // Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // Called each time the UI needs repainting, which may be many times per second.
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

        // Central panel will display main contents
        CentralPanel::default().show(ctx, |ui| {
            match self.current_page {
                // First immediate page is Login page
                // Register OR Log in OR Continue as a guest
                Page::Login => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("Welcome!");
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        //Register Option
                        ui.vertical(|ui| {
                            ui.heading("Register for an account!");
                            if ui.button("Register").clicked() {
                                self.current_page = Page::Register;
                            }

                        });
                        ui.add_space(15.0);
                        ui.heading("OR");
                        ui.add_space(15.0);
                        //Log in Option
                        ui.vertical(|ui| {
                            ui.heading("Log in");
                            ui.label("Username:");
                            ui.text_edit_singleline(&mut self.username);
                            ui.label("Password:");
                            ui.add(TextEdit::singleline(&mut self.password).password(true));

                            // Handle login button click
                            if ui.button("Log in").clicked() {
                                // Check if the user exists
                                if let Ok(user_exists) = self.user_exists() {
                                    if user_exists {
                                        // If the user exists, go to the home page
                                        self.current_page = Page::Home;
                                    } else {
                                        ui.label("Invalid username or password");
                                    }
                                }
                            }
                        });
                        ui.add_space(15.0);
                        ui.heading("OR");
                        ui.add_space(15.0);
                        //Guest Option
                        ui.vertical(|ui| {
                            ui.heading("Continue as a Guest");
                            if ui.button("Guest").clicked() {
                                self.current_page = Page::Home;
                            }
                        });
                    });
                }
                Page::Register => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("Register");
                    });
                    ui.separator();
                    ui.label("Username:");
                    ui.text_edit_singleline(&mut self.register_username);
                    ui.label("Password:");
                    ui.add(TextEdit::singleline(&mut self.register_password).password(true));

                    // Handle registration button click
                    if ui.button("Create Account").clicked() {
                        match self.register_user() {
                            Ok(()) => {
                                // If registration is successful, display success message
                                self.registration_message = "Account created successfully!".to_owned();
                                self.current_page = Page::Login;
                            }
                            // If username already exists, display failure message
                            Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
                                self.registration_message = "Username already exists.".to_owned();
                            }
                            // If registration fails for diff reason, display failure message
                            Err(e) => {
                                self.registration_message = format!("Failed to create account: {}", e);
                            }
                        }
                    }

                    // Display registration message
                    ui.label(&self.registration_message);

                    // Switch back to login page
                    if ui.button("Back to Login").clicked() {
                        self.current_page = Page::Login;
                    }
                }
                // Home page contains main contents, user can maneuver to Github and About page
                Page::Home => {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        ui.heading("Iron Coder");
                    });

                    ui.separator();

                    // About Iron Coder and GitHub link buttons
                    ui.horizontal(|ui| {
                        ui.hyperlink_to("GitHub", "https://github.com/shulltronics/iron-coder.git");

                        ui.add_space(10.0);

                        if ui.button("About Iron Coder").clicked() {
                            self.current_page = Page::About;
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
                // Contains description of Iron Coder
                Page::About => {
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
