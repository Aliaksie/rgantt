use yew::{html, Component, Context, Html};

use crate::gantt;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <gantt::Gantt
                        event_option = {gantt::schemas::EventOption::default()}
                        display_option = {gantt::schemas::DisplayOption::default()}
                        style_option = {gantt::schemas::StylingOption::default()}
                        tasks = {vec![]}
                />
            </>
        }
    }
}
