// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{fs::File};

use gag::Redirect;
fn main() {
    let fs = File::create("./logfile.log").unwrap();
    let _redirect = Redirect::stdout(fs).expect("Failed to redirect stdout");
    let fs_stderr = File::create("./stderr.log").unwrap();
    let _redirect = Redirect::stderr(fs_stderr).expect("Failed to redirect stderr");
    tuari_template_lib::run()
}
