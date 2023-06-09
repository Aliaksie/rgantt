use chrono::{Datelike, NaiveDateTime, Utc, NaiveDate, Timelike};
use yew::{html, Component, Context, Html};

use super::schemas::{
    BarTask, CalendarProps, GridProps, SvgProps, TaskGanttContentProps, TaskTypeInternal, ViewMode,
};

impl ViewMode {
    fn get_values(&self, props: CalendarProps) -> (Html, Html) {
        match self {
            ViewMode::Hour => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i != 0 && date.date() != dates.get(i - 1).unwrap().date())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}, {}", date.iso_week().week(), date.format("%d %m"));
                        let top_position = ((date.hour() - 24) / 2) as f64;
                        let x_text = props.column_width * (i + top_position);
                        let x_line = props.column_width * i;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%H"));
                        let key = format!("{}", date.timestamp());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::QuarterDay => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i != 0 && date.date() != dates.get(i - 1).unwrap().date())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}, {}", date.iso_week().week(), date.format("%d %m"));
                        let x_text = props.column_width * i + 4.0 * props.column_width * 0.5;
                        let x_line = props.column_width * i + 4.0 * props.column_width;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%H"));
                        let key = format!("{}", date.timestamp());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::HalfDay => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i != 0 && date.date() != dates.get(i - 1).unwrap().date())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}, {}", date.iso_week().week(), date.format("%d %m"));
                        let x_text = props.column_width * i + 2.0 * props.column_width * 0.5;
                        let x_line = props.column_width * i + 2.0 * props.column_width;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%H"));
                        let key = format!("{}", date.timestamp());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::Day => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| i + 1 != dates.len() && date.month() != dates.get(i + 1).unwrap().month())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}", date.format("%m"));
                        let year = if date.month() == 12 { date.year() + 1 } else { date.year() };
                        let date_num = NaiveDate::from_ymd_opt(year, date.month() + 1, 1).unwrap()
                            .signed_duration_since(NaiveDate::from_ymd_opt(year, date.month(), 1).unwrap()).num_days() as f64;

                        let x_text = props.column_width * (i + 1.0) - date_num * props.column_width * 0.5;
                        let x_line = props.column_width * (i + 1.0);
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%d"));
                        let key = format!("{}", date.timestamp());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::Week => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let mut week_count = 0.0; // todo ?
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i == 0 || date.month() != dates.get(i - 1).unwrap().month())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}", date.format("%Y"));
                        let x_text = props.column_width * i + week_count * props.column_width;
                        let x_line = props.column_width * i + week_count * props.column_width * 0.5;
                        week_count = 0.0;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("W{}", date.iso_week().week());
                        let key = format!("{}", date.format("%m, %Y"));
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::Month => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i == 0 || date.year() != dates.get(i - 1).unwrap().year())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}", date.format("%Y"));
                        let x_text = if props.rtl {
                            (6.0 + i + (date.month() as f64) + 1.0) * props.column_width
                        } else {
                            (6.0 + i - (date.month() as f64)) * props.column_width
                        };
                        let x_line = props.column_width * i;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%m"));
                        let key = format!("{}", date.format("%m, %Y"));
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::QuarterYear => {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i == 0 || date.year() != dates.get(i - 1).unwrap().year())
                    .map(|(i, date)| {
                        let i = i as f64;
                        // todo: customize ?
                        let value = format!("{}", date.format("%Y"));
                        let x_text = if props.rtl {
                            (6.0 + i + (date.month() as f64) + 1.0) * props.column_width
                        } else {
                            (6.0 + i - (date.month() as f64)) * props.column_width
                        };
                        let x_line = props.column_width * i;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("Q{}", ((date.month() + 3) / 3));
                        let key = format!("{}", date.timestamp_nanos());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
            ViewMode::Year =>  {
                let dates = props.date_setup.dates.clone().unwrap();
                let height = props.header_height * 0.5;
                let top: Html = dates
                    .iter()
                    .enumerate()
                    .filter(|(i, date)| *i == 0 || date.year() != dates.get(i - 1).unwrap().year())
                    .map(|(i, date)| {
                        // todo: customize ?
                        let i = i as f64;
                        let value = format!("{}", date.format("%Y"));
                        let x_text = if props.rtl {
                            (6.0 + i + (date.year() as f64) + 1.0) * props.column_width
                        } else {
                            (6.0 + i - (date.year() as f64)) * props.column_width
                        };
                        let x_line = props.column_width * i;
                        top_calendar(&props, x_line, height, x_text, value)
                    })
                    .collect();

                let bottom: Html = dates
                    .iter()
                    .enumerate()
                    .map(|(i, date)| { 
                        // todo: customize ?                    
                        let value = format!("{}", date.format("%Y"));
                        let key = format!("{}", date.timestamp_nanos());
                        bottom_calendar(&props, i as f64, value, key)
                    }).collect();

                (bottom, top)
            },
        }
    }
}

impl TaskTypeInternal {
    fn get_task_item(&self, task: BarTask) -> Html {
        match self {
            TaskTypeInternal::Task => {
                let progress_x = task.progress_x.unwrap() + task.progress_width.unwrap();
                let task_y = task.y.unwrap();
                let task_height = task.height.unwrap();
                let  point = [progress_x - 5.0, task_y + task_height, progress_x + 5.0, task_y + task_height, progress_x, task_y + task_height - 8.66,].iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<String>>()
                    .join(",");

                html!{
                    <g class="bar-wrapper" tabIndex={0}>
                        <g /* TODO: onMouseDown={onMouseDown}*/>
                            <rect
                                x={task.x_1.unwrap().to_string()}
                                width={(task.x_2.unwrap() - task.x_1.unwrap()).to_string()}
                                y={task.y.unwrap().to_string()}
                                height={task.height.unwrap().to_string()}
                                ry={task.bar_corner_radius.unwrap().to_string()}
                                rx={task.bar_corner_radius.unwrap().to_string()}
                                fill={task.styles.clone().unwrap().background_color.unwrap()} // todo! isSelected
                                class="bar-back-ground"
                            />
                            <rect
                                x={task.progress_x.unwrap().to_string()}
                                width={task.progress_width.unwrap().to_string()}
                                y={task.y.unwrap().to_string()}
                                height={task.height.unwrap().to_string()}
                                ry={task.bar_corner_radius.unwrap().to_string()}
                                rx={task.bar_corner_radius.unwrap().to_string()}
                                fill={task.styles.clone().unwrap().progress_color.unwrap()} // todo! isSelected
                            />
                        </g>
                        <g class="handle-group">
                            // TODO! if isDateChangeable
                             <g>
                                /* left */
                                <rect
                                    x={(task.x_1.unwrap() + 1.0).to_string()}
                                    y={(task.y.unwrap() + 1.0).to_string()}
                                    width={task.handle_width.unwrap().to_string()}
                                    height={task.height.unwrap().to_string()}
                                    class="bar-handle"
                                    ry={task.bar_corner_radius.unwrap().to_string()}
                                    rx={task.bar_corner_radius.unwrap().to_string()}
                                    // TODO! onMouseDown={onMouseDown}
                                 />
                                /* right */
                                 <rect
                                    x={(task.x_2.unwrap() - task.handle_width.unwrap() - 1.0).to_string()}
                                    y={(task.y.unwrap() + 1.0).to_string()}
                                    width={task.handle_width.unwrap().to_string()}
                                    height={task.height.unwrap().to_string()}
                                    class="bar-handle"
                                    ry={task.bar_corner_radius.unwrap().to_string()}
                                    rx={task.bar_corner_radius.unwrap().to_string()}
                                    // TODO! onMouseDown={onMouseDown}
                                 />
                            </g>
                    
                            // TODO! if isProgressChangeable
                            <polygon
                                 class="bar-handle"
                                 points={point}
                                // TODO! onMouseDown={onMouseDown}
                            />            
                        </g>
                    </g>
                }
            },
            TaskTypeInternal::Project => {
                // TODO: isSelcted
                let mut styles = task.styles.clone().unwrap();
                let bar_color = if true {
                    styles.background_selected_color
                } else {
                    styles.background_color
                }
                .unwrap();
                styles = task.styles.clone().unwrap();
                let process_color = if true {
                    styles.progress_selected_color
                } else {
                    styles.progress_color
                }
                .unwrap();
                let project_with = task.x_2.unwrap() - task.x_1.unwrap();

                let project_left_triangle = [
                    task.x_1.unwrap(),
                    task.y.unwrap() + task.height.unwrap() / 2.0 - 1.0,
                    task.x_1.unwrap(),
                    task.y.unwrap() + task.height.unwrap(),
                    task.x_1.unwrap() + 15.0,
                    task.y.unwrap() + task.height.unwrap() / 2.0 - 1.0,
                ]
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(",");

                let project_right_triangle = [
                    task.x_2.unwrap(),
                    task.y.unwrap() + task.height.unwrap() / 2.0 - 1.0,
                    task.x_2.unwrap(),
                    task.y.unwrap() + task.height.unwrap(),
                    task.x_2.unwrap() - 15.0,
                    task.y.unwrap() + task.height.unwrap() / 2.0 - 1.0,
                ]
                .iter()
                .map(|it| it.to_string())
                .collect::<Vec<String>>()
                .join(",");

                let node = 
                html! {
                    <g tabIndex={0} class="project-wrapper">
                        <rect
                            fill={bar_color.clone()} // todo! isSelected
                            x={task.x_1.unwrap().to_string()}
                            width={project_with.to_string()}
                            y={task.y.unwrap().to_string()}
                            height={task.height.unwrap().to_string()}
                            rx={task.bar_corner_radius.unwrap().to_string()}
                            ry={task.bar_corner_radius.unwrap().to_string()}
                            class="project-background"
                        />
                        <rect
                            x={task.progress_x.unwrap().to_string()}
                            width={task.progress_width.unwrap().to_string()}
                            y={task.y.unwrap().to_string()}
                            height={task.height.unwrap().to_string()}
                            ry={task.bar_corner_radius.unwrap().to_string()}
                            rx={task.bar_corner_radius.unwrap().to_string()}
                            fill={process_color} // todo! isSelected
                        />
                        <rect
                            fill={bar_color.clone()}
                            x={task.x_1.unwrap().to_string()}
                            width={project_with.to_string()}
                            y={task.y.unwrap().to_string()}
                            height={(task.height.unwrap() / 2.0).to_string()}
                            rx={task.bar_corner_radius.unwrap().to_string()}
                            ry={task.bar_corner_radius.unwrap().to_string()}
                            class="project-top"
                        />
                        <polygon
                            class="project-top"
                            points={project_left_triangle}
                            fill={bar_color.clone()} // todo! isSelected
                        />
                        <polygon
                            class="project-top"
                            points={project_right_triangle}
                            fill={bar_color.clone()} // todo! isSelected
                        />
                    </g>
                };
                node
            }
            TaskTypeInternal::Milestone => {
                let transform = format!("rotate(45 {} {})", task.x_1.unwrap() + task.height.unwrap() * 0.356, task.y.unwrap() + task.height.unwrap() * 0.85);
                html!{
                    <g tabIndex={0} class="milestone-wrapper">
                        <rect
                            fill={task.styles.clone().unwrap().background_color.unwrap()} // todo! isSelected
                            x={task.x_1.unwrap().to_string()}
                            width={task.height.unwrap().to_string()}
                            y={task.y.unwrap().to_string()}
                            height={task.height.unwrap().to_string()}
                            rx={task.bar_corner_radius.unwrap().to_string()}
                            ry={task.bar_corner_radius.unwrap().to_string()}
                            transform={transform}
                            class="milestone-background"
                            //TODO:  onMouseDown=
                        />
                  </g>
                }
            },
            TaskTypeInternal::Smalltask => todo!(),
        }
    }
}

#[derive(Default, Clone)]
pub struct SvgView {
    grid_props: GridProps,
    calendar_props: CalendarProps,
    bar_props: TaskGanttContentProps,
    gantt_height: f64,
    scroll_y: f64,
    scroll_x: f64,

    vertical_gantt_container_ref: yew::NodeRef,
    horizontal_container_ref: yew::NodeRef,
    gantt_svg_ref: yew::NodeRef,
}

impl Component for SvgView {
    type Message = ();
    type Properties = SvgProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            grid_props: ctx.props().grid_props.clone(),
            calendar_props: ctx.props().calendar_props.clone(), 
            bar_props: ctx.props().bar_props.clone(), 
            gantt_height:ctx.props().gantt_height, 
            scroll_y: ctx.props().scroll_y, 
            scroll_x: ctx.props().scroll_x, 

            vertical_gantt_container_ref: yew::NodeRef::default(), 
            horizontal_container_ref: yew::NodeRef::default(), 
            gantt_svg_ref: yew::NodeRef::default()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let calendar_props = self.calendar_props.clone();
        let cal_values = calendar_props
            .date_setup
            .view_mode
            .unwrap()
            .get_values(self.calendar_props.clone());
        let mut y = 0.0;
        let grid_props = self.grid_props.clone();
        let grid_rows: Html = grid_props
            .tasks
            .iter()
            .map(|task| {
                let node = html! {
                    <rect
                        key={format!("Row{}", task.id.clone().unwrap())}
                        x="0"
                        y={y.to_string()}
                        width={grid_props.svg_width.to_string()}
                        height={grid_props.row_height.to_string()}
                        class="grid-row"
                  />
                };
                y += grid_props.row_height;
                node
            })
            .collect();

        y = 0.0;
        let row_lines: Html = grid_props
            .tasks
            .iter()
            .map(|task| {
                let node = html! {
                    <line
                        key={format!("Row{}", task.id.clone().unwrap())}
                        x="0"
                        y1={(y + grid_props.row_height).to_string()}
                        x2={grid_props.svg_width.to_string()}
                        y2={(y + grid_props.row_height).to_string()}
                        class="grid-row-line"
                  />
                };
                y += grid_props.row_height;
                node
            })
            .collect();

        let dates = grid_props.dates;
        let now = Utc::now().naive_utc();
        let mut today = html! {};
        let mut tick_x = 0.0;
        let ticks: Html = dates.iter().enumerate()
            .map(|(i, date)|{
                let node = html! {
                    <line
                        key={date.timestamp_nanos().to_string()}
                        x1={tick_x.to_string()}
                        y1={0}
                        x2={tick_x.to_string()}
                        y2={y.to_string()}
                        class="grid-tick"
                    />
                };
                
                
                if (i + 1 != dates.len() && date.timestamp_micros() < now.timestamp_micros() && dates[i + 1].timestamp_micros() >= now.timestamp_micros()) 
                    // if current date is last
                    || (i != 0 && i + 1 == dates.len() && date.timestamp_micros() < now.timestamp_micros() &&
                    NaiveDateTime::from_timestamp_micros(date.timestamp_micros() - dates[i - 1].timestamp_micros()).unwrap().timestamp_micros() >= now.timestamp_micros())
                {
                    today = html! {
                    <rect
                        x={tick_x.to_string()}
                        y={0}
                        width={grid_props.column_width.to_string()}
                        height={y.to_string()}
                        fill={grid_props.today_color.clone()}
                    />
                    };
                }

                tick_x += grid_props.column_width;
                node
            }).collect();

        let tasks = self.bar_props.tasks.clone();
        let arrow_tasks: Html = tasks
            .iter()
            .map(|task| {
                let nodes: Html = task
                    .bar_children
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|children| {
                        // todo: do we need rtl
                        let index_compare = if task.index.unwrap() > children.index.unwrap() {
                            -1.0
                        } else {
                            1.0
                        };
                        let task_to_end_position =
                            children.y.unwrap() + self.bar_props.task_height / 2.0;
                        let task_from_end_position =
                            task.x_2.unwrap() + self.bar_props.arrow_indent * 2.0;
                        let task_from_horizontal_offset_value =
                            if task_from_end_position < children.x_1.unwrap() {
                                "".to_owned()
                            } else {
                                format!("H {}", children.x_1.unwrap() - self.bar_props.arrow_indent)
                            };
                        let task_to_horizontal_offset_value = if task_from_end_position
                            > children.x_1.unwrap()
                        {
                            self.bar_props.arrow_indent
                        } else {
                            children.x_1.unwrap() - task.x_2.unwrap() - self.bar_props.arrow_indent
                        };

                        let path = format!(
                            "M {} {} h {} v {} {} V {} h {}",
                            task.x_2.unwrap(),
                            task.y.unwrap() + self.bar_props.task_height / 2.0,
                            self.bar_props.arrow_indent,
                            (index_compare * self.bar_props.row_height) / 2.0,
                            task_from_horizontal_offset_value,
                            task_to_end_position,
                            task_to_horizontal_offset_value
                        );

                        let triangle_points = format!(
                            "{},{} {},{} {},{}",
                            children.x_1.unwrap(),
                            task_to_end_position,
                            children.x_1.unwrap() - 5.0,
                            task_to_end_position - 5.0,
                            children.x_1.unwrap() - 5.0,
                            task_to_end_position + 5.0
                        );

                        html! {
                            <g class="arrow">
                                <path strok-width="1.5" d={path} fill="none" />
                                <polygon points={triangle_points} />
                            </g>
                        }
                    })
                    .collect();
                nodes
            })
            .collect();

        let bar_tasks: Html = tasks
            .iter()
            .map(|task| {
                let width = task.x_2.unwrap() - task.x_1.unwrap();
                let _has_child = !task.bar_children.clone().unwrap().is_empty();
                // todo! isTextInside
                let x = if true {
                    task.x_1.unwrap() + width * 0.5
                } else {
                    // todo rtl + ref
                    task.x_1.unwrap()
                        + width
                        + self.bar_props.arrow_indent
                        + self.bar_props.arrow_indent * 0.2
                };

                html! {
                    <g /*onKeyDown= TODO!! */ >
                        {task.type_internal.clone().unwrap().get_task_item(task.clone())}
                        <text
                            x={x.to_string()}
                            y={(task.y.unwrap() + self.bar_props.task_height * 0.5).to_string()}
                            // class={ if isTextInside
                            //     ? style.barLabel
                            //     : style.barLabel && style.barLabelOutside
                            // }
                            // ref={textRef}
                        >
                            {task.task.name.clone()}
                        </text>
                    </g>
                }
            })
            .collect();

        html! {
            <div
                class="gantt-vertical-container" // todo: add scss!!
                ref={self.vertical_gantt_container_ref.clone()} // todo: scroll_x render
                dir="ltr"
                >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width={self.grid_props.svg_width.to_string()}
                    height={self.calendar_props.header_height.to_string()}
                    font-family={self.bar_props.font_family.clone()}
                    >
                    <g class="calendar" font-size={self.calendar_props.font_size.clone()} font-family={self.calendar_props.font_family.clone()}>
                        <rect
                            x={0}
                            y={0}
                            width={(self.calendar_props.column_width * (self.calendar_props.date_setup.dates.clone().unwrap().len() as f64)).to_string()}
                            height={self.calendar_props.header_height.to_string()}
                            class="calendar-header"
                        />
                        {cal_values.0} 
                        {cal_values.1}
                    </g>
                </svg>

                <div
                    ref={self.horizontal_container_ref.clone()}
                    class="horizontal-container"
                    style={
                        if self.gantt_height > 0.0 { format!("height:{}; width:{}", self.gantt_height, self.grid_props.svg_width) }
                        else { format!("width:{}", self.grid_props.svg_width) }
                    }
                    >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={self.grid_props.svg_width.to_string()}
                        height={(self.bar_props.row_height * (self.bar_props.tasks.len() as f64)).to_string()}
                        font-family={self.bar_props.font_family.to_string()}
                        ref={self.gantt_svg_ref.clone()}
                    >
                        <g class="grid">
                            <g class="grid-body">
                                <g class="rows">{grid_rows}</g>
                                <g class="row-lines">{row_lines}</g>
                                <g class="ticks">{ticks}</g>
                                <g class="today">{today}</g>
                            </g>
                        </g>
                        <g class="content">
                            <g class="arrows" fill={self.bar_props.arrow_color.clone()} stroke={self.bar_props.arrow_color.clone()}>{arrow_tasks}</g>
                            <g class="bar" font-family={self.bar_props.font_family.clone()} font-size={self.bar_props.font_size.clone()}>{bar_tasks}</g>
                        </g>
                    </svg>
                </div>
            </div>
        }
    }
}

fn bottom_calendar(props: &CalendarProps, i: f64,  value: String, key: String ) -> yew::virtual_dom::VNode {
    html! {
        <text
            key={key}
            y={(props.header_height * 0.8).to_string()}
            x={(props.column_width * i + props.column_width * 0.5).to_string()}
            class="calendar-bottom-text"
        >
            {value}
        </text>
    }
}

fn top_calendar(props: &CalendarProps, x_line: f64, height: f64, x_text: f64, value: String) -> yew::virtual_dom::VNode {
    html! {
        <g class="calendar-top">
            <line
                x1={x_line.to_string()}
                y1={0}
                x2={x_line.to_string()}
                y2={height.to_string()}
                class="calendar-top-tick"
                key={format!("{}line", props.column_width)}
            />
            <text
                key={format!("{}text", value)}
                y={(height * 0.9).to_string()}
                x={x_text.to_string()}
                class="calendar-top-text"
            >
                {value}
            </text>
        </g>
    }
}
