#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epi, egui::Align};
use std::process::{Command, Child, Stdio, ChildStdout};
use std::io::{BufRead, BufReader};


struct MyApp {
    stdout: String,
    process: Option<Child>,
    process_stdout: Option<BufReader<ChildStdout>>,
    auto: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
	    stdout: "".to_string(),
	    process: None,
	    process_stdout: None,
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
			// spawn a process
			self.process = run_program();

			// check the process
			if self.process.is_none() {
			    return;
			}

			// check stdout
			let stdout = self.process.take().unwrap().stdout;
			if stdout.is_none() {
			    self.stdout.push_str("stdout is None\n");
			    return;
			}

			
			self.process_stdout =
			    Some(BufReader::
				 new(stdout.unwrap())
			    );

		    }
		}
		ui.checkbox(&mut self.auto, "auto");

	    });

	    egui::ScrollArea::vertical().show(ui, |ui| {
		ui.add_sized(ui.available_size(),
			     egui::TextEdit::multiline(&mut self.stdout));
		ui.scroll_to_cursor(Some(Align::BOTTOM));
	    });


	    if self.process_stdout.is_none() {
		return;
	    }


	    let mut reader = self.process_stdout.take().unwrap();
	    let mut line = String::new();
	    let ret = reader.read_line(&mut line);
	    match ret {
		Ok(_v) => {
		    self.stdout.push_str(line.as_str());
		},
		Err(_e) => {
		    self.stdout.push_str("<binary data catched>\n");
		}
	    }

	    self.process_stdout = Some(reader);
        });

    }
}

fn run_program() -> Option<Child>
{
    let result = Command::new("bash")
	.args(["test.sh"])
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

#[test]
fn test_process()
{
    let mut child = Command::new("ls")
	.args(["-l", "/"])
	.stdout(Stdio::piped())
	.spawn().unwrap();
    
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);

    let mut stdout_bytes = Vec::<u8>::new();
    reader.lines()
	.for_each(|l| {
	    let line = l.unwrap_or("123".to_string());
	    println!("{}", line);
	});
    assert!(false);
}
