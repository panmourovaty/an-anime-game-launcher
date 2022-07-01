use gtk4::{self as gtk, prelude::*};
use libadwaita::{self as adw, prelude::*};

use std::io::Error;

use super::{get_object, add_action};
use super::preferences::PreferencesStack;

use crate::lib::game;

#[derive(Clone)]
pub struct App {
    pub window: adw::ApplicationWindow,
    pub leaflet: adw::Leaflet,
    pub launch_game: adw::SplitButton,
    pub launch_game_debug: gtk::Button,
    pub open_preferences: gtk::Button,
    pub toast_overlay: adw::ToastOverlay,

    pub preferences_stack: PreferencesStack
}

impl App {
    pub fn new(app: &gtk::Application) -> Result<Self, String> {
        // Create builder from UI file
        let builder = gtk::Builder::from_string(include_str!("../../assets/ui/.dist/main.ui"));

        // Parse objects from builder
        let result = Self {
            window: get_object(&builder, "window")?,
            leaflet: get_object(&builder, "leaflet")?,
            launch_game: get_object(&builder, "launch_game")?,
            launch_game_debug: get_object(&builder, "launch_game_debug")?,
            open_preferences: get_object(&builder, "open_preferences")?,
            toast_overlay: get_object(&builder, "toast_overlay")?,

            preferences_stack: PreferencesStack::new()?
        };

        // Add preferences page to the leaflet
        let leaflet = result.leaflet.clone();

        result.leaflet.append(&result.preferences_stack.preferences).set_name(Some("preferences_page"));

        // Go back button for preferences page
        result.preferences_stack.preferences_go_back.connect_clicked(move |_| {
            leaflet.navigate(adw::NavigationDirection::Back);
        });

        // Launch game
        let app_copy = result.clone();
        
        result.launch_game.connect_clicked(move |_| {
            // Display toast message if the game is failed to run
            if let Err(err) = game::run(false) {
                app_copy.toast_error("Failed to run game", err);
            }
        });

        // Launch game in debug mode
        /*let app_copy = result.clone();
        
        result.launch_game_debug.connect_clicked(move |_| {
            // Display toast message if the game is failed to run
            if let Err(err) = game::run(true) {
                app_copy.toast_error("Failed to run game", err);
            }
        });*/

        // Bind app to the window
        result.window.set_application(Some(app));

        Ok(result)
    }

    /// Show toast with `toast` title and `See message` button
    /// 
    /// This button will show message dialog with error message
    pub fn toast_error(&self, toast: &str, err: std::io::Error) {
        let toast = adw::Toast::new(toast);

        toast.set_button_label(Some("See message"));
        toast.set_action_name(Some("see-message.see-message"));

        let window_copy = self.window.clone();

        // Show error message in a dialog window
        add_action(&self.toast_overlay, "see-message", move || {
            let dialog = gtk::MessageDialog::new(
                Some(&window_copy),
                gtk::DialogFlags::all(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Close,
                &err.to_string()
            );

            dialog.connect_response(move |dialog, _| {
                dialog.close();
            });

            dialog.show();
        });

        self.toast_overlay.add_toast(&toast);
    }

    pub fn open_preferences_page(&self) -> Result<(), Error> {
        self.preferences_stack.update()?;
        
        self.leaflet.set_visible_child_name("preferences_page");

        Ok(())
    }
}