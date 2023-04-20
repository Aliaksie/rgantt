use yew::{html, Component, Context, Html};

use crate::gantt::{self, schemas::Task};

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let tasks_js = r###"[
            {
                "start": "2023-04-20 01:46:39",
                "end": "2023-06-20 01:46:39",
                "name": "Some Project",
                "id": "ProjectSample",
                "progress": 25,
                "type": "project",
                "hideChildren": false,
                "displayOrder": 1
            },
            {
                "start": "2023-04-01 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Idea",
                "id": "Task 0",
                "progress": 45,
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 2
            },
            {
                "start": "2023-04-20 01:46:39",
                "end": "2023-05-20 01:46:39",
                "name": "Research",
                "id": "Task 1",
                "progress": 25,
                "dependencies": [
                    "Task 0"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 3
            },
            {
                "start": "2023-04-01 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Discussion with team",
                "id": "Task 2",
                "progress": 10,
                "dependencies": [
                    "Task 1"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 4
            },
            {
                "start": "2023-04-13 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Developing",
                "id": "Task 3",
                "progress": 2,
                "dependencies": [
                    "Task 2"
                ],
                "type": "task",
                "project": "ProjectSample",
                "displayOrder": 5
            },
            {
                "start": "2023-04-05 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Review",
                "id": "Task 4",
                "type": "task",
                "progress": 70,
                "dependencies": [
                    "Task 2"
                ],
                "project": "ProjectSample",
                "displayOrder": 6
            },
            {
                "start": "2023-04-11 01:46:39",
                "end": "2023-04-20 01:46:39",
                "name": "Release",
                "id": "Task 6",
                "progress": 40,
                "type": "milestone",
                "dependencies": [
                    "Task 4"
                ],
                "project": "ProjectSample",
                "displayOrder": 7
            },
            {
                "start": "2023-05-10 01:46:39",
                "end": "2023-05-20 01:46:39",
                "name": "Party Time",
                "id": "Task 9",
                "progress": 0,
                "isDisabled": true,
                "type": "task"
            }
        ]"###;

        let tasks_: Vec<Task> = serde_json::from_str(tasks_js).unwrap();

        html! {
            <>
                <gantt::Gantt
                        event_option = {gantt::schemas::EventOption::default()}
                        display_option = {gantt::schemas::DisplayOption::default()}
                        style_option = {gantt::schemas::StylingOption::default()}
                        tasks = {tasks_}
                />
            </>
        }
    }
}
