#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi, egui::Align};
use std::process::Command;


struct MyApp {
    stdout: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
	    stdout: "".to_string(),
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        concat!("my command test ", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
	    if ui.button("Run").clicked() {
		let ret = run_program();
		self.stdout.push_str(ret.as_str());
	    }

	    egui::ScrollArea::vertical().show(ui, |ui| {
		ui.add_sized(ui.available_size(),
			     egui::TextEdit::multiline(&mut self.stdout));
		ui.scroll_to_cursor(Some(Align::BOTTOM));
	    });

	    
        });

	

    }
}

fn run_program() -> String
{
    let result = Command::new("ls")
	.args(["-l",
	      "/"])
	.output();
    match result {
	Ok(v) => {
	    let ret = String::from_utf8(v.stdout);
	    if let Ok(v) = ret {
		v
	    } else {
		"some binary\n".to_string()
	    }
	},
	Err(e) => {
	    e.to_string()
	}
    }
    
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}

