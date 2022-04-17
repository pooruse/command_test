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

	    // check the process
	    if self.process.is_none() {
		return;
	    }

	    // check stdout
	    let stdout = self.process.as_ref().unwrap().stdout.as_ref();
	    if stdout.is_none() {
		self.stdout.push_str("stdout is None\n");
		return;
	    }

	    // get the raw fd
	    let stdout = stdout.unwrap();
	    let mut f;
	    unsafe {
		f = File::from_raw_fd(stdout.as_raw_fd());
	    }

	    let mut stdio_bytes: Vec<u8> = Vec::<u8>::new();
	    let size = f.read(&mut stdio_bytes);
	    if size.is_err() {
		self.stdout.push_str("can't read from stdio\n");
		self.process = None;
		return;
	    }

	    let size = size.unwrap();
	    let stdout = String::from_utf8(stdio_bytes);
	    if let Ok(v) = stdout {
		self.stdout.push_str(v.as_str());
	    } else {
		self.stdout.push_str(
		    format!("<binary data {} bytes>\n", size).as_str()
		);
	    }
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

