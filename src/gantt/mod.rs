pub(crate) mod schemas;
mod svg_view;
mod table;

#[macro_use]
mod macros;

use yew::{html, Component, Context, Html};

#[derive(Default)]
pub struct Gantt {
    grid_props: schemas::GridProps,
    calendar_props: schemas::CalendarProps,
    bar_props: schemas::TaskGanttContentProps,
    gantt_height: f32,
    scroll_y: f32,
    scroll_x: f32,

    wrapper_ref: yew::NodeRef,
}

impl Component for Gantt {
    type Message = ();
    type Properties = schemas::GanttProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let props = _ctx.props();

        let a = schemas::GridProps::default().tasks(props.tasks.clone().unwrap());

        Gantt::default()
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
