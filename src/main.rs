#![windows_subsystem = "windows"]
use eframe::egui;
use pdf_r2l_script_win::{convert, convert_in_place};
use std::path::PathBuf;

const LICENSES: &str = include_str!("../LICENSES.txt");

struct App {
    src: Option<PathBuf>,
    in_place: bool,
    msg: String,
    show_license: bool,
}

fn setup_fonts(ctx: &egui::Context) {
    for p in [
        "C:/Windows/Fonts/YuGothM.ttc",
        "C:/Windows/Fonts/meiryo.ttc",
        "C:/Windows/Fonts/msgothic.ttc",
    ] {
        if let Ok(bytes) = std::fs::read(p) {
            let mut f = egui::FontDefinitions::default();
            f.font_data.insert("jp".into(), egui::FontData::from_owned(bytes));
            for fam in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
                f.families.entry(fam).or_default().push("jp".into());
            }
            ctx.set_fonts(f);
            return;
        }
    }
}

impl App {
    fn run(&mut self) {
        let Some(src) = self.src.clone() else { return };
        if self.in_place {
            self.msg = match convert_in_place(&src, Some("bak")) {
                Ok(()) => format!("完了(上書き): {}", src.display()),
                Err(e) => format!("エラー: {e}"),
            };
            return;
        }
        let stem = src.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let mut dlg = rfd::FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .set_file_name(format!("{stem}-r2l.pdf"));
        if let Some(dir) = src.parent() {
            dlg = dlg.set_directory(dir);
        }
        let Some(dest) = dlg.save_file() else { return };
        self.msg = match convert(&src, &dest) {
            Ok(()) => format!("完了: {}", dest.display()),
            Err(e) => format!("エラー: {e}"),
        };
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.input(|i| {
            if let Some(f) = i.raw.dropped_files.first().and_then(|f| f.path.clone()) {
                self.src = Some(f);
            }
        });
        egui::Window::new("ライセンス / License")
            .open(&mut self.show_license)
            .default_size([560.0, 400.0])
            .show(ctx, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add(egui::Label::new(egui::RichText::new(LICENSES.trim_start_matches('\u{feff}')).monospace().size(11.0)).wrap_mode(egui::TextWrapMode::Extend));
                });
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PDF 右綴じ変換");
            ui.add_space(8.0);
            ui.label("PDFファイルをドラッグ&ドロップ、または選択:");
            ui.horizontal(|ui| {
                if ui.button("PDFを選択…").clicked() {
                    if let Some(p) = rfd::FileDialog::new().add_filter("PDF", &["pdf"]).pick_file() {
                        self.src = Some(p);
                    }
                }
                let name = self.src.as_ref().map(|p| p.display().to_string()).unwrap_or("(未選択)".into());
                ui.label(name);
            });
            ui.add_space(8.0);
            ui.label("右綴じ(R2L)・見開き右始まり(TwoColumnRight)にします");
            ui.checkbox(&mut self.in_place, "元ファイルを上書き(.bak にバックアップ)");
            ui.add_space(8.0);
            if ui.add_enabled(self.src.is_some(), egui::Button::new("変換して保存…")).clicked() {
                self.run();
            }
            ui.add_space(8.0);
            ui.label(&self.msg);
            ui.add_space(8.0);
            if ui.button("ライセンス / License").clicked() {
                self.show_license = true;
            }
        });
    }
}

fn main() -> eframe::Result {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([520.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "PDF R2L",
        opts,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(App { src: None, in_place: false, msg: String::new(), show_license: false }))
        }),
    )
}
