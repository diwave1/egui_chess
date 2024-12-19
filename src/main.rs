#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::time::{Duration, Instant};


fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "chess",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    pieces: Vec<(char, usize, usize)>, // (piece, row, col)
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pieces: vec![
                ('♜', 0, 0), ('♞', 0, 1), ('♝', 0, 2), ('♛', 0, 3), ('♚', 0, 4), ('♝', 0, 5), ('♞', 0, 6), ('♜', 0, 7),
                ('♟', 1, 0), ('♟', 1, 1), ('♟', 1, 2), ('♟', 1, 3), ('♟', 1, 4), ('♟', 1, 5), ('♟', 1, 6), ('♟', 1, 7),
                ('♙', 6, 0), ('♙', 6, 1), ('♙', 6, 2), ('♙', 6, 3), ('♙', 6, 4), ('♙', 6, 5), ('♙', 6, 6), ('♙', 6, 7),
                ('♖', 7, 0), ('♘', 7, 1), ('♗', 7, 2), ('♕', 7, 3), ('♔', 7, 4), ('♗', 7, 5), ('♘', 7, 6), ('♖', 7, 7),
            ],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let size = 800.0; // Size of the chessboard
            let square_size = size / 8.0; // Size of each square
            let (response, painter) = ui.allocate_painter(egui::Vec2::new(size, size), egui::Sense::hover());
            let rect = response.rect;

            for row in 0..8 {
                for col in 0..8 {
                    let x = rect.left() + col as f32 * square_size;
                    let y = rect.top() + row as f32 * square_size;
                    let square_rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), egui::Vec2::new(square_size, square_size));
                    let color = if (row + col) % 2 == 0 {
                        egui::Color32::from_rgb(240, 217, 181) // Light color
                    } else {
                        egui::Color32::from_rgb(181, 136, 99) // Dark color
                    };
                    painter.rect_filled(square_rect, 0.0, color);
                }
            }

            // Draw row labels (1-8)
            for row in 0..8 {
                let y = rect.top() + 0.0 + row as f32 * square_size + square_size / 2.0;
                painter.text(
                    egui::Pos2::new(rect.left() + 10.0, y),
                    egui::Align2::CENTER_CENTER,
                    (8 - row).to_string(),
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }

            // Draw column labels (A-H)
            for col in 0..8 {
                let x = rect.left() + 0.0 + col as f32 * square_size + square_size / 2.0;
                painter.text(
                    egui::Pos2::new(x, rect.top() + 10.0),
                    egui::Align2::CENTER_CENTER,
                    (b'A' + col as u8) as char,
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }

            // Draw the pieces
            for (piece, row, col) in &self.pieces {
                let x = rect.left() + 0.0 + *col as f32 * square_size + square_size / 2.0;
                let y = rect.top() + 0.0 + *row as f32 * square_size + square_size / 2.0;
                painter.text(
                    egui::Pos2::new(x, y),
                    egui::Align2::CENTER_CENTER,
                    piece.to_string(),
                    egui::FontId::proportional(64.0),
                    egui::Color32::BLACK,
                );
            }
        });
    }
}