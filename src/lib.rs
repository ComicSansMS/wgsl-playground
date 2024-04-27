use window::start_event_loop;

mod window;
mod state;

pub async fn run() {
    env_logger::init();

    let (window, event_loop) = window::create_window();
    let mut state = state::State::new(&window).await;
    start_event_loop(event_loop, &window, &mut state);
}
