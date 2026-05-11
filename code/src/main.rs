fn main() -> eframe::Result{

    let native_opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0,300.0])
            .with_min_inner_size([300.0,200.0]),
            ..Default::default()
    };
    eframe::run_native("BCS", native_opts, Box::new(|cc| Ok(Box::new(code::App::new(cc)))),)
 
    } //Код не работает, ошибка пока и должна быть.
