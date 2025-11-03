//! Table rendering functionality

use std::io::stdout;
use termimad::crossterm::{
    style::Print,
    QueueableCommand,
};

/// Table rendering functionality
pub struct TableRenderer;

impl TableRenderer {
    /// Render a table with proper ASCII borders
    pub fn render_table(table_md: &str) {
        let lines: Vec<&str> = table_md.lines().collect();
        if lines.is_empty() {
            return;
        }

        // Parse table rows
        let mut rows: Vec<Vec<String>> = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let cells: Vec<String> = line.split('|').map(|s| s.trim().to_string()).collect();
            if !cells.is_empty() {
                rows.push(cells);
            }
        }

        if rows.is_empty() {
            return;
        }

        // Calculate column widths
        let mut col_widths: Vec<usize> = Vec::new();
        for row in &rows {
            for (i, cell) in row.iter().enumerate() {
                if i >= col_widths.len() {
                    col_widths.push(0);
                }
                col_widths[i] = col_widths[i].max(cell.len());
            }
        }

        // Render table with borders
        let mut stdout = stdout();

        // Top border
        let _ = stdout.queue(Print("┌"));
        for (i, &width) in col_widths.iter().enumerate() {
            if i > 0 {
                let _ = stdout.queue(Print("┬"));
            }
            let _ = stdout.queue(Print("─".repeat(width + 2)));
        }
        let _ = stdout.queue(Print("┐\n"));

        // Table rows
        for (row_idx, row) in rows.iter().enumerate() {
            // Data row
            let _ = stdout.queue(Print("│"));
            for (i, cell) in row.iter().enumerate() {
                if i > 0 {
                    let _ = stdout.queue(Print("│"));
                }
                let width = col_widths.get(i).copied().unwrap_or(0);
                let _ = stdout.queue(Print(format!(" {:<width$} ", cell, width = width)));
            }
            let _ = stdout.queue(Print("│\n"));

            // Separator row (after header or between data rows)
            if row_idx == 0 || row_idx < rows.len() - 1 {
                let _ = stdout.queue(Print("├"));
                for (i, &width) in col_widths.iter().enumerate() {
                    if i > 0 {
                        let _ = stdout.queue(Print("┼"));
                    }
                    let _ = stdout.queue(Print("─".repeat(width + 2)));
                }
                let _ = stdout.queue(Print("┤\n"));
            }
        }

        // Bottom border
        let _ = stdout.queue(Print("└"));
        for (i, &width) in col_widths.iter().enumerate() {
            if i > 0 {
                let _ = stdout.queue(Print("┴"));
            }
            let _ = stdout.queue(Print("─".repeat(width + 2)));
        }
        let _ = stdout.queue(Print("┘\n"));
    }
}