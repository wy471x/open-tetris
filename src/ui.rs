use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Screen};
use crate::constants::*;
use crate::game::Game;
use crate::piece::TetrisColor;

fn to_color(c: TetrisColor) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

pub fn render(frame: &mut Frame, app: &App) {
    match &app.screen {
        Screen::Menu => render_menu(frame),
        Screen::Playing(game) => render_game(frame, game),
        Screen::Paused(game) => {
            render_game(frame, game);
            render_pause_overlay(frame);
        }
        Screen::GameOver { score } => render_game_over(frame, *score),
    }
}

fn render_menu(frame: &mut Frame) {
    let area = frame.area();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(block, area);

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(6),  // OPEN
            Constraint::Length(1),  // gap
            Constraint::Length(6),  // TETRIS
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .margin(2)
        .split(area);

    let open_lines = [
        " █████╗  █████╗  ███████╗ ███╗   ██╗",
        "██╔══██╗ ██╔══██╗ ██╔════╝ ████╗  ██║",
        "██║  ██║ ██████╔╝ █████╗   ██╔██╗ ██║",
        "██║  ██║ ██╔═══╝  ██╔══╝   ██║╚██╗██║",
        "╚█████╔╝ ██║      ███████╗ ██║ ╚████║",
        " ╚════╝  ╚═╝      ╚══════╝ ╚═╝  ╚═══╝",
    ];
    for (i, line) in open_lines.iter().enumerate() {
        let span = Span::styled(*line, Style::default().fg(Color::Yellow));
        let p = Paragraph::new(span).centered();
        let y = inner[1].y + i as u16;
        let r = Rect::new(inner[1].x, y, inner[1].width, 1);
        frame.render_widget(p, r);
    }

    let tetris_lines = [
        "████████╗ ███████╗ ████████╗ ██████╗   ██╗  ██████╗",
        "╚══██╔══╝ ██╔════╝ ╚══██╔══╝ ██╔══██╗  ██║ ██╔════╝",
        "   ██║    █████╗      ██║    ██████╔╝  ██║  █████╗ ",
        "   ██║    ██╔══╝      ██║    ██╔══██╗  ██║  ╚═══██╗",
        "   ██║    ███████╗    ██║    ██║  ██║  ██║ ██████╔╝",
        "   ╚═╝    ╚══════╝    ╚═╝    ╚═╝  ╚═╝  ╚═╝ ╚═════╝ ",
    ];
    for (i, line) in tetris_lines.iter().enumerate() {
        let span = Span::styled(*line, Style::default().fg(Color::Cyan));
        let p = Paragraph::new(span).centered();
        let y = inner[3].y + i as u16;
        let r = Rect::new(inner[3].x, y, inner[3].width, 1);
        frame.render_widget(p, r);
    }

    let subtitle = Paragraph::new(
        Line::from(vec![Span::styled(
            "A Rust TUI Game",
            Style::default().fg(Color::White),
        )])
        .centered(),
    );
    frame.render_widget(subtitle, inner[4]);

    let instructions = Paragraph::new(
        Line::from(vec![
            Span::styled("Press ENTER", Style::default().fg(Color::Green).bold()),
            Span::styled(" to start    ", Style::default().fg(Color::White)),
            Span::styled("Press Q", Style::default().fg(Color::Red).bold()),
            Span::styled(" to quit", Style::default().fg(Color::White)),
        ])
        .centered(),
    );
    frame.render_widget(instructions, inner[5]);
}

fn render_game(frame: &mut Frame, game: &Game) {
    let area = frame.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    let play_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[0]);

    render_board(frame, game, play_chunks[0]);
    render_side_panel(frame, game, play_chunks[1]);
    render_controls(frame, main_chunks[1]);
}

fn render_board(frame: &mut Frame, game: &Game, area: Rect) {
    let block = Block::default()
        .title(" OPEN TETRIS ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let cell_w: u16 = 2;
    let grid_w = BOARD_COLS as u16 * cell_w;
    let grid_h = BOARD_ROWS as u16;
    let start_x = inner.x + (inner.width.saturating_sub(grid_w)) / 2;
    let start_y = inner.y + (inner.height.saturating_sub(grid_h)) / 2;

    let piece_cells = game.current_piece.cells();
    let piece_color = to_color(game.current_piece.color());

    let ghost_y = game.ghost_y();
    let ghost_cells = {
        let mut pc = game.current_piece.clone();
        pc.y = ghost_y;
        pc.cells()
    };

    for row in 0..BOARD_ROWS {
        for col in 0..BOARD_COLS {
            let x = start_x + col as u16 * cell_w;
            let y = start_y + row as u16;

            let cell = &game.board.grid[row][col];

            let bg = if piece_cells.contains(&(col as i32, row as i32)) {
                piece_color
            } else if cell.occupied {
                to_color(cell.color)
            } else if ghost_cells.contains(&(col as i32, row as i32)) {
                Color::DarkGray
            } else {
                Color::Black
            };

            let span = Span::styled("  ", Style::default().bg(bg));
            let r = Rect::new(x, y, cell_w, 1);
            frame.render_widget(Paragraph::new(span), r);
        }
    }
}

fn render_side_panel(frame: &mut Frame, game: &Game, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    render_next_piece(frame, game, chunks[0]);
    render_stat(frame, "SCORE", &game.score.to_string(), chunks[1]);
    render_stat(frame, "LEVEL", &game.level.to_string(), chunks[2]);
    render_stat(frame, "LINES", &game.lines.to_string(), chunks[3]);
}

fn render_next_piece(frame: &mut Frame, game: &Game, area: Rect) {
    let block = Block::default()
        .title(" NEXT ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let cells = game.next_piece.cells();
    let color = to_color(game.next_piece.color());

    let min_x = cells.iter().map(|(x, _)| *x).min().unwrap_or(0);
    let max_x = cells.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let min_y = cells.iter().map(|(_, y)| *y).min().unwrap_or(0);
    let max_y = cells.iter().map(|(_, y)| *y).max().unwrap_or(0);
    let pw = (max_x - min_x + 1) as u16 * 2;
    let ph = (max_y - min_y + 1) as u16;
    let offset_x = inner.x + (inner.width.saturating_sub(pw)) / 2;
    let offset_y = inner.y + (inner.height.saturating_sub(ph)) / 2;

    for &(cx, cy) in &cells {
        let x = offset_x + (cx - min_x) as u16 * 2;
        let y = offset_y + (cy - min_y) as u16;
        let span = Span::styled("  ", Style::default().bg(color));
        let r = Rect::new(x, y, 2, 1);
        frame.render_widget(Paragraph::new(span), r);
    }
}

fn render_stat(frame: &mut Frame, label: &str, value: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(inner);

    let label_p = Paragraph::new(
        Line::from(vec![Span::styled(
            label,
            Style::default().fg(Color::Gray).bold(),
        )])
        .centered(),
    );
    frame.render_widget(label_p, chunks[0]);

    let value_p = Paragraph::new(
        Line::from(vec![Span::styled(
            value,
            Style::default().fg(Color::White).bold(),
        )])
        .centered(),
    );
    frame.render_widget(value_p, chunks[1]);
}

fn render_controls(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let controls = vec![
        Span::styled("LEFT/RIGHT", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Move  ", Style::default().fg(Color::White)),
        Span::styled("UP", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Rotate CW  ", Style::default().fg(Color::White)),
        Span::styled("Z", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Rotate CCW  ", Style::default().fg(Color::White)),
        Span::styled("DOWN", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Soft Drop  ", Style::default().fg(Color::White)),
        Span::styled("SPACE", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Hard Drop  ", Style::default().fg(Color::White)),
        Span::styled("P", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Pause  ", Style::default().fg(Color::White)),
        Span::styled("Q", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" Quit", Style::default().fg(Color::White)),
    ];

    let line = Line::from(controls).centered();
    let p = Paragraph::new(line);
    let y = inner.y + inner.height.saturating_sub(1) / 2;
    let r = Rect::new(inner.x, y, inner.width, 1);
    frame.render_widget(p, r);
}

fn render_pause_overlay(frame: &mut Frame) {
    let area = frame.area();
    let overlay_w = 20;
    let overlay_h = 3;
    let x = area.x + (area.width.saturating_sub(overlay_w)) / 2;
    let y = area.y + (area.height.saturating_sub(overlay_h)) / 2;
    let r = Rect::new(x, y, overlay_w, overlay_h);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .style(Style::default().bg(Color::Black));
    let inner = block.inner(r);
    frame.render_widget(block, r);

    let text = Paragraph::new(
        Line::from(vec![Span::styled(
            "PAUSED",
            Style::default().fg(Color::Yellow).bold(),
        )])
        .centered(),
    );
    frame.render_widget(text, inner);
}

fn render_game_over(frame: &mut Frame, score: u32) {
    let area = frame.area();
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(Color::Red));
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .margin(4)
        .split(area);

    let game_over = Paragraph::new(
        Line::from(vec![Span::styled(
            "GAME OVER",
            Style::default().fg(Color::Red).bold(),
        )])
        .centered(),
    );
    frame.render_widget(game_over, chunks[1]);

    let score_text = Paragraph::new(
        Line::from(vec![
            Span::styled("Final Score: ", Style::default().fg(Color::White)),
            Span::styled(score.to_string(), Style::default().fg(Color::Yellow).bold()),
        ])
        .centered(),
    );
    frame.render_widget(score_text, chunks[3]);

    let instructions = Paragraph::new(
        Line::from(vec![
            Span::styled("Press ENTER", Style::default().fg(Color::Green).bold()),
            Span::styled(" to return to menu", Style::default().fg(Color::White)),
        ])
        .centered(),
    );
    frame.render_widget(instructions, chunks[5]);
}
