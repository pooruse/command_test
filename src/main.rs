#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi, egui::Align};
use std::process::{Command, Child, Stdio};
use std::fs::File;
use std::io::Read;


#[cfg(unix)]
use std::os::unix::io::{FromRawFd, AsRawFd};

struct MyApp {
    stdout: String,
    process: Option<Child>,
    auto: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
	    stdout: "".to_string(),
	    process: None,
	    auto: false,
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        concat!("my command test ", env!("CARGO_PKG_VERSION"))
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
	
	let button_enabled = (!self.auto) && (self.process.is_none());
	
        egui::CentralPanel::default().show(ctx, |ui| {
	    ui.horizontal(|ui| {
		if ui.add_enabled(button_enabled, egui::Button::new("run"))
		    .clicked()
		{
		    if self.process.is_none() {
			self.process = run_program();
		    }
		}
		ui.checkbox(&mut self.auto, "auto");

	    });

	    egui::ScrollArea::vertical().show(ui, |ui| {
		ui.add_sized(ui.available_size(),
			     egui::TextEdit::multiline(&mut self.stdout));
		ui.scroll_to_cursor(Some(Align::BOTTOM));
	    });

	    // check the program and output the stdout
	    let mut stdio_bytes: Vec<u8> = Vec::<u8>::new();
	    match &self.process {
		Some(v) => {
		    match &v.stdout {
			Some(v) => {
			    let mut f;
			    unsafe {
				f = File::from_raw_fd(v.as_raw_fd());
			    }
			    match f.read(&mut stdio_bytes) {
				Ok(_) => {
				    match String::from_utf8(stdio_bytes) {
					Ok(v) => {
					    self.stdout.push_str(v.as_str());
					},
					Err(_) => {
					    self.stdout.push_str("<binary data>\n");
					}
				    }
				},
				_ => {
				    self.stdout.push_str("Can't read size\n");
				}
			    }
			    
			},
			_ => {
			    self.stdout.push_str("Can't read stdout\n");
			}
		    }
		},
		_ => {
		    // donothing
		}
	    };
        });

    }
}

fn run_program() -> Option<Child>
{
    let result = Command::new("ls")
	.args(["-l",
	       "/"])
	.stdout(Stdio::piped())
	.spawn();
    match result {
	Ok(v) => {
	    Some(v)
	},
	Err(_) => {
	    None
	}
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(MyApp::default()), options);
}

