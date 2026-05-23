use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::{
    api::Post,
    app::{App, Focus},
};

pub fn draw(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(area);

    draw_search(frame, app, chunks[0]);
    draw_body(frame, app, chunks[1]);
    draw_status(frame, app, chunks[2]);

    if app.show_help {
        draw_help(frame, area);
    }
}

fn draw_search(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let style = if app.focus == Focus::Search {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };
    let title = format!(" Danbooru Search | page {} ", app.page);
    let search = Paragraph::new(app.query.as_str()).block(
        Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(style),
    );
    frame.render_widget(search, area);
}

fn draw_body(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
        .split(area);

    draw_results(frame, app, chunks[0]);
    draw_details(frame, app, chunks[1]);
}

fn draw_results(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let items = app.posts.iter().map(|post| {
        let title = format!(
            "#{:<8} {:>4} {:<1} {}",
            post.id,
            post.score,
            rating_marker(&post.rating),
            post.short_tags()
        );
        ListItem::new(Line::from(title))
    });

    let mut state = ListState::default();
    if !app.posts.is_empty() {
        state.select(Some(app.selected));
    }

    let border_style = if app.focus == Focus::Results {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Posts ")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut state);
}

fn draw_details(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let text = match app.selected_post() {
        Some(post) => post_details(post),
        None => Text::from("No post selected."),
    };

    let details = Paragraph::new(text)
        .block(Block::default().title(" Details ").borders(Borders::ALL))
        .wrap(Wrap { trim: false });
    frame.render_widget(details, area);
}

fn post_details(post: &Post) -> Text<'static> {
    let dimensions = match (post.image_width, post.image_height) {
        (Some(width), Some(height)) => format!("{width}x{height}"),
        _ => "unknown".to_string(),
    };

    let image_url = post.best_image_url().unwrap_or("none");
    let file_ext = post.file_ext.as_deref().unwrap_or("unknown");

    Text::from(vec![
        Line::from(vec![
            Span::styled("Post ", Style::default().fg(Color::Cyan)),
            Span::raw(post.id.to_string()),
        ]),
        Line::from(format!("Rating: {}   Score: {}", post.rating, post.score)),
        Line::from(format!("File: {file_ext}   Size: {dimensions}")),
        Line::from(""),
        Line::from(vec![
            Span::styled("Artist: ", Style::default().fg(Color::Magenta)),
            Span::raw(trim_or_dash(&post.tag_string_artist)),
        ]),
        Line::from(vec![
            Span::styled("Character: ", Style::default().fg(Color::LightBlue)),
            Span::raw(trim_or_dash(&post.tag_string_character)),
        ]),
        Line::from(vec![
            Span::styled("Copyright: ", Style::default().fg(Color::Green)),
            Span::raw(trim_or_dash(&post.tag_string_copyright)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Post URL: ", Style::default().fg(Color::Cyan)),
            Span::raw(post.post_url()),
        ]),
        Line::from(vec![
            Span::styled("Image URL: ", Style::default().fg(Color::Cyan)),
            Span::raw(image_url.to_string()),
        ]),
        Line::from(""),
        Line::from(Span::styled("Tags", Style::default().fg(Color::Yellow))),
        Line::from(post.tag_string.clone()),
    ])
}

fn draw_status(frame: &mut Frame<'_>, app: &App, area: Rect) {
    let controls = match app.focus {
        Focus::Search => "Enter search | Esc results | ? help | Ctrl-C quit",
        Focus::Results => {
            "/ search | j/k move | n/p page | o post | i image | r reload | q quit | ? help"
        }
    };
    let status = format!("{}  |  {controls}", app.status);

    let paragraph = Paragraph::new(status).block(Block::default().borders(Borders::ALL));
    frame.render_widget(paragraph, area);
}

fn draw_help(frame: &mut Frame<'_>, area: Rect) {
    let popup = centered_rect(68, 54, area);
    frame.render_widget(Clear, popup);

    let help = Text::from(vec![
        Line::from(Span::styled(
            "Booru Browser",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("/          edit search tags"),
        Line::from("Enter      run search or open selected post"),
        Line::from("j/k        move selection"),
        Line::from("n/p        next or previous result page"),
        Line::from("o          open selected Danbooru post"),
        Line::from("i          open selected image URL"),
        Line::from("r          reload current page"),
        Line::from("q          quit"),
        Line::from("? or Esc   close help"),
        Line::from(""),
        Line::from("Danbooru supports tag searches such as:"),
        Line::from("rating:safe landscape order:score"),
    ]);

    let block = Paragraph::new(help)
        .block(Block::default().title(" Help ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(block, popup);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn rating_marker(rating: &str) -> &'static str {
    match rating {
        "g" | "s" => "S",
        "q" => "Q",
        "e" => "E",
        _ => "?",
    }
}

fn trim_or_dash(value: &str) -> String {
    if value.trim().is_empty() {
        "-".to_string()
    } else {
        value.to_string()
    }
}
