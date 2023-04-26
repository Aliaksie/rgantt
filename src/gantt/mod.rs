pub(crate) mod schemas;
mod svg_view;
mod table;

#[macro_use]
mod macros;

use chrono::{NaiveDateTime, Utc};
use yew::{html, Component, Context, Html};

use self::schemas::{BarTask, DateSetup, ViewMode};

#[derive(Default)]
pub struct Gantt {
    grid_props: schemas::GridProps,
    calendar_props: schemas::CalendarProps,
    bar_props: schemas::TaskGanttContentProps,
    gantt_height: f64,
    scroll_y: f64,
    scroll_x: f64,

    wrapper_ref: yew::NodeRef,
}

impl Component for Gantt {
    type Message = ();
    type Properties = schemas::GanttProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let props = _ctx.props();
        let fmt = props
            .display_option
            .locale
            .clone()
            .unwrap_or_else(|| "%Y-%m-%d %H:%M:%S".to_owned());
        let column_width = props.style_option.column_width.unwrap_or(30.0);
        let row_height = props.style_option.row_height.unwrap_or(50.0);
        let font_family = props.style_option.font_family.clone().unwrap_or_else(|| {
            "Arial, Roboto, Oxygen, Ubuntu, Cantarell, Fira Sans, Droid Sans, Helvetica Neue"
                .to_owned()
        });
        let font_size = props
            .style_option
            .font_size
            .clone()
            .unwrap_or_else(|| "14px".to_owned());
        let header_height = props.style_option.header_height.unwrap_or(50.0);
        let bar_fill = props.style_option.bar_fill.unwrap_or(60.0);
        let task_height = (row_height * bar_fill) / 100.0;
        let start_dates: NaiveDateTime = props
            .tasks
            .clone()
            .unwrap_or_default()
            .iter()
            .filter_map(|it| it.start.clone())
            .filter_map(|it| match NaiveDateTime::parse_from_str(&it, &fmt) {
                Ok(date) => Some(date),
                Err(err) => {
                    log::warn!("{}", err);
                    None
                }
            })
            .min()
            .unwrap_or_else(|| Utc::now().naive_utc());
        let end_dates: NaiveDateTime = props
            .tasks
            .clone()
            .unwrap_or_default()
            .iter()
            .filter_map(|it| it.end.clone())
            .filter_map(|it| match NaiveDateTime::parse_from_str(&it, &fmt) {
                Ok(date) => Some(date),
                Err(err) => {
                    log::warn!("{}", err);
                    None
                }
            })
            .max()
            .unwrap_or_else(|| Utc::now().naive_utc());
        let view_mode = props.display_option.view_mode.clone().unwrap_or_default();
        // todo: ?
        let dates_: Vec<NaiveDateTime> = seed_dates(start_dates, end_dates, view_mode.clone());

        let date_setup = DateSetup {
            view_mode: Some(view_mode.clone()),
            dates: Some(dates_.clone()),
        };

        let grid_props_ = schemas::GridProps::default()
            .tasks(props.tasks.clone().unwrap())
            .row_height(props.style_option.row_height.unwrap_or(50.0))
            .column_width(column_width)
            .dates(date_setup.dates.clone().unwrap())
            .rtl(false)
            .svg_width((dates_.len() as f64) * column_width)
            .today_color(
                props
                    .style_option
                    .today_color
                    .clone()
                    .unwrap_or_else(|| "rgba(252, 248, 227, 0.5)".to_owned()),
            );

        let calendar_props_ = schemas::CalendarProps::default()
            .column_width(column_width)
            .date_setup(date_setup)
            .font_family(font_family.clone())
            .font_size(font_size.clone())
            .header_height(header_height)
            .locale(fmt.clone()) // todo
            .rtl(false)
            .view_mode(view_mode);

        // todo!!
        let tasks = props.tasks.clone().unwrap_or_default();
        let bar_tasks: Vec<BarTask> = tasks
            .iter()
            .enumerate()
            .map(|(i, it)| {
                let childrens: Vec<BarTask> = tasks
                    .iter()
                    .filter_map(|el| {
                        el.dependencies
                            .clone()
                            .unwrap_or_default()
                            .iter()
                            .find(|id| &it.id.clone().unwrap() == *id)
                            .map(|_| el)
                    })
                    .map(|task| {
                        let index = tasks
                            .iter()
                            .position(|el| el.id.to_owned().unwrap() == task.id.to_owned().unwrap())
                            .unwrap();
                        bar_task(
                            task.clone(),
                            fmt.clone(),
                            &dates_,
                            column_width,
                            props,
                            task_height,
                            index,
                            row_height,
                        )
                    })
                    .collect();

                bar_task(
                    it.clone(),
                    fmt.clone(),
                    &dates_,
                    column_width,
                    props,
                    task_height,
                    i,
                    row_height,
                )
                .bar_children(childrens)
            })
            .collect();
        let bar_props_ = schemas::TaskGanttContentProps::default()
            .tasks(bar_tasks)
            .dates(dates_.clone())
            .gantt_event(schemas::GanttEvent::default())
            .selected_task(schemas::BarTask::default()) // todo!
            .row_height(row_height)
            .time_step(props.event_option.time_step.unwrap_or(300000.0))
            .svg(yew::NodeRef::default()) // todo!
            .svg_width((dates_.len() as f64) * column_width)
            .task_height(task_height)
            .arrow_color(
                props
                    .style_option
                    .arrow_color
                    .clone()
                    .unwrap_or_else(|| "grey".to_owned()),
            )
            .arrow_indent(props.style_option.arrow_indent.unwrap_or(20.0))
            .column_width(column_width)
            .rtl(false)
            .font_family(font_family)
            .font_size(font_size);

        Gantt {
            grid_props: grid_props_,
            calendar_props: calendar_props_,
            bar_props: bar_props_,
            gantt_height: props.style_option.gantt_height.unwrap_or(0.0),
            scroll_y: 0.0,
            scroll_x: -1.0,
            wrapper_ref: yew::NodeRef::default(), // todo!
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
          <div>
            <div
                class="wrapper"
                // onKeyDown={handleKeyDown} TODO:!!
                tabIndex={0}
                ref={self.wrapper_ref.clone()}
            >
              <table::Table />
              <svg_view::SvgView
                    grid_props={self.grid_props.clone()}
                    calendar_props={self.calendar_props.clone()}
                    bar_props={self.bar_props.clone()}
                    gantt_height={self.gantt_height}
                    scroll_y={self.scroll_y}
                    scroll_x={self.scroll_x}
              />
          // <Tooltip
          //     arrowIndent={arrowIndent}
          //     rowHeight={rowHeight}
          //     svgContainerHeight={svgContainerHeight}
          //     svgContainerWidth={svgContainerWidth}
          //     fontFamily={fontFamily}
          //     fontSize={fontSize}
          //     scrollX={scrollX}
          //     scrollY={scrollY}
          //     task={ganttEvent.changedTask}
          //     headerHeight={headerHeight}
          //     taskListWidth={taskListWidth}
          //     TooltipContent={TooltipContent}
          //     rtl={rtl}
          //     svgWidth={svgWidth}
          // />

          // <VerticalScroll
          //     ganttFullHeight={ganttFullHeight}
          //     ganttHeight={ganttHeight}
          //     headerHeight={headerHeight}
          //     scroll={scrollY}
          //     onScroll={handleScrollY}
          //     rtl={rtl}
          // />
              </div>
                    // <HorizontalScroll
                    //     svgWidth={svgWidth}
                    //     taskListWidth={taskListWidth}
                    //     scroll={scrollX}
                    //     rtl={rtl}
                    //     onScroll={handleScrollX}
                    // />
            </div>
        }
    }
}

fn bar_task(
    task: schemas::Task,
    fmt: String,
    dates_: &Vec<NaiveDateTime>,
    column_width: f64,
    props: &schemas::GanttProps,
    task_height: f64,
    i: usize,
    row_height: f64,
) -> BarTask {
    let start = NaiveDateTime::parse_from_str(&task.start.clone().unwrap(), &fmt).unwrap();
    let end = NaiveDateTime::parse_from_str(&task.end.clone().unwrap(), &fmt).unwrap();
    let x_1 = task_x_coordinate(start, dates_.to_owned(), column_width);
    let x_2 = task_x_coordinate(end, dates_.to_owned(), column_width);

    BarTask::default()
        // .bar_children(childrens)
        .bar_corner_radius(props.style_option.bar_corner_radius.unwrap_or(3.0))
        .handle_width(props.style_option.handle_width.unwrap_or(8.0))
        .height(task_height)
        .index(i as f64)
        .progress_width((x_2 - x_1) * task.progress.unwrap() * 0.01)
        .progress_x(x_1)
        .type_internal(task.type_.clone().unwrap().get_internal())
        .x_1(x_1)
        .x_2(x_2)
        .y((i as f64) * row_height + (row_height - task_height) / 2.0)
        .task(task)
        .styles(schemas::BarTaskStyles {
            background_color: Some(
                props
                    .style_option
                    .bar_background_color
                    .clone()
                    .unwrap_or_else(|| "#b8c2cc".to_owned()),
            ),
            background_selected_color: Some(
                props
                    .style_option
                    .bar_background_selected_color
                    .clone()
                    .unwrap_or_else(|| "#aeb8c2".to_owned()),
            ),
            progress_color: Some(
                props
                    .style_option
                    .bar_progress_color
                    .clone()
                    .unwrap_or_else(|| "#a3a3ff".to_owned()),
            ),
            progress_selected_color: Some(
                props
                    .style_option
                    .bar_progress_selected_color
                    .clone()
                    .unwrap_or_else(|| "#8282f5".to_owned()),
            ),
        })
}

fn task_x_coordinate(x_date: NaiveDateTime, dates: Vec<NaiveDateTime>, column_width: f64) -> f64 {
    let index = dates
        .iter()
        .position(|it| it.timestamp() >= x_date.timestamp())
        .unwrap()
        - 1;

    let remainder = x_date.timestamp_nanos() - dates[index].timestamp_nanos();
    let percent = remainder / (dates[index + 1].timestamp_nanos() - dates[index].timestamp_nanos());

    (index as f64) * column_width + (percent as f64) * (column_width as f64)
}

fn seed_dates(start: NaiveDateTime, end: NaiveDateTime, mode: ViewMode) -> Vec<NaiveDateTime> {
    let mut current = mode.get_mod_date(start, false);
    let mut dates: Vec<NaiveDateTime> = Vec::new();
    dates.push(current);
    while current < mode.get_mod_date(end, true) {
        current = mode.get_mod_date(current, true);
        dates.push(current);
    }

    dates
}
