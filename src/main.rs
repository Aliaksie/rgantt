mod app;
mod gantt;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), GenericError> {
    console_log::init_with_level(log::Level::Debug)
        .map_err(|err| GenericError::from(err.to_string()));

    yew::Renderer::<app::App>::new().render();
    Ok(())
}
