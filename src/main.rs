use std::time::Duration;

fn main() {
    loop {
        eframe::run_native(
            "Gui Timer",
            eframe::NativeOptions::default(),
            Box::new(|cc| Ok(Box::new(MyApp))),
        )
        .expect("Unable to start GUI");

        // Previous window will not close until "run_native" is called again.
        std::thread::sleep(Duration::from_secs(2));
    }
}

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        //
    }
}
