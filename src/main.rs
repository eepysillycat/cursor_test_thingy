mod app;
mod graphics;

use log::LevelFilter;
use winit::event_loop::EventLoop;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Warn)
        .filter_module(module_path!(), LevelFilter::Debug)
        .parse_default_env()
        .init();

    let event_loop = EventLoop::new()?;

    let mut app = App::new(&event_loop)?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
