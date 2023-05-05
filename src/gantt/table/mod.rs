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

    fn view(&self, ctx: &Context<Self>) -> Html {
        let table_content: Html = self.table_props.tasks.clone().unwrap_or_default().iter()
            .map(|task|  {
                let expander = if task.hide_children.unwrap_or(true) { "▼" } else { "▶" };   
                html!{
                    <div
                        class="gantt-table-row"
                        style={format!("height: {}px;", self.table_props.row_height.clone().unwrap() - 2.0)}
                        key={format!("{}row", task.id.clone().unwrap())}
                    >
                        <div
                        class="gantt-table-cell"
                        style={format!("min-width: {}; max-width: {}",
                                self.table_props.row_width.clone().unwrap(),
                                self.table_props.row_width.clone().unwrap())
                         }
                        title={task.name.clone().unwrap()}
                        >
                        <div class="gantt-table-name-wrapper">
                            <div
                            class={if task.hide_children.unwrap_or(true) { "table-expander" } else { "table-empty-expander" } }
                            // onClick={() => onExpanderClick(t)}
                            >
                                {expander}
                            </div>
                            <div>{task.name.clone().unwrap()}</div>
                        </div>
                        </div>
                        <div
                            class="gantt-table-cell"
                            style={format!("min-width: {}; max-width: {}",
                                    self.table_props.row_width.clone().unwrap(),
                                    self.table_props.row_width.clone().unwrap())
                            }
                        >
                         {task.start.clone().unwrap()}
                        </div>
                        <div
                            class="gantt-table-cell"
                            style={format!("min-width: {}; max-width: {}",
                                    self.table_props.row_width.clone().unwrap(),
                                    self.table_props.row_width.clone().unwrap())
                            }
                        >
                         {task.end.clone().unwrap()}
                        </div>
                    </div>
                }
            }).collect();


        html! {
            <div ref={self.task_list_ref.clone()}>
                <div
                    class="gantt-table"
                    style={
                        format!("font-family:{}; font-size:{}", 
                            self.table_props.font_family.clone().unwrap(),
                            self.table_props.font_size.clone().unwrap())
                    }
                >
                    <div
                        class="gantt-table-header"
                        style={format!("height: {}px;", self.table_props.header_height - 2.0)}
                    >
                        <div
                            class="gantt-table-header-item"       
                            style={format!("min-width: {};", self.table_props.row_width.clone().unwrap())}
                        >
                            {"Name"}
                        </div>
                        <div
                            class="gantt-table-header-separator"
                            style={format!("height: {}px; margin-top: {}px",
                                 self.table_props.header_height * 0.5,
                                 self.table_props.header_height * 0.2)}
                        />
                        <div
                            class="gantt-table-header-item"
                            style={format!("min-width:{};", self.table_props.row_width.clone().unwrap())}
                        >
                            {"From"}
                        </div>
                        <div
                            class="gantt-table-header-separator"
                            style={format!("height: {}px; margin-top: {}px",
                                 self.table_props.header_height * 0.5,
                                 self.table_props.header_height * 0.25)}
                        />
                        <div
                            class="gantt-table-header-item"
                            style={format!("min-width:{};", self.table_props.row_width.clone().unwrap())}
                        >
                            {"To"}
                        </div>
                    </div>
                </div>
                <div
                    ref={self.horizontal_container_ref.clone()}
                    class="horizontal-container"
                    // style={format!("height:{};", self.table_props.gantt_height)}
                >
                    <div
                        class="gantt-table-wrapper"
                        style={
                            format!("font-family:{}; font-size:{}", 
                                self.table_props.font_family.clone().unwrap(),
                                self.table_props.font_size.clone().unwrap())
                        }
                    >
                        {table_content}
                    </div>
                </div>
            </div>
        }
    }
}
