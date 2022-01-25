use clap::Parser;
use todo::Container;
use todo::GetPath;
use crossterm::event;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use std::io::Error as IOError;
use std::io::stdout as get_stdout;
use std::path::PathBuf;
use std::thread::sleep as thread_sleep;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::widgets::Block;
use tui::widgets::Borders;
use tui::Terminal;
#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    list_path: String,
}
struct Ctx {
    args: Args,
    path: PathBuf,
}
impl Ctx {
    fn new(args: Args) -> Self {
        Self { args, path: PathBuf::new(), }
    }
    fn construct_path(&mut self) {
        let tmp_path = PathBuf::from(format!("{}", &self.args.list_path));
        match tmp_path.extension() {
            Some(ext) => {
                if !ext.eq("json") {
                    self.path.push(format!("{}.json", &self.args.list_path));
                } else {
                    self.path.push(format!("{}", &self.args.list_path));
                }
            },
            None => self.path.push(format!("{}.json", &self.args.list_path)),
        }
    }
}
impl GetPath for Ctx {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
    fn get_path_mut(&mut self) -> &mut PathBuf {
        &mut self.path
    }
}
fn main() -> Result<(), IOError> {
    let args = Args::parse();
    let mut ctx = Ctx::new(args);
    ctx.construct_path();
    let container = Container::load(&mut ctx);
    enable_raw_mode()?;
    let mut stdout = get_stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;
    thread_sleep(Duration::from_millis(5000));
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;
    Ok(())
}
