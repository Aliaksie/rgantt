use yew::{html, Component, Context, Html};

#[derive(Default)]
pub struct Table {
    horizontal_container_ref: yew::NodeRef,
    task_list_ref: yew::NodeRef,
    table_props: super::schemas::TableProps,
}

impl Component for Table {
    type Message = ();
    type Properties = super::schemas::TableProps;

    fn create(ctx: &Context<Self>) -> Self {
        Table {
            table_props: ctx.props().clone(),
            ..Table::default()
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div ref={self.task_list_ref.clone()}>
                // <TaskListHeader {...headerProps} />
                <div
                    ref={self.horizontal_container_ref.clone()}
                    class="horizontal-container"
                    // style={ganttHeight ? { height: ganttHeight } : {}}
                >
                // <TaskListTable {...tableProps} />
                </div>
             </div>
        }
    }
}
