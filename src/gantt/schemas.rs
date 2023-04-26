use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use yew::Properties;

use super::macros;

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum BarMoveAction {
    #[serde(rename = "end")]
    End,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "start")]
    Start,
}
#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct BarTaskStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundSelectedColor")]
    pub background_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressColor")]
    pub progress_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressSelectedColor")]
    pub progress_selected_color: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct BarTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barChildren")]
    pub bar_children: Option<Vec<BarTask>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barCornerRadius")]
    pub bar_corner_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleWidth")]
    pub handle_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressWidth")]
    pub progress_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressX")]
    pub progress_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<BarTaskStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "typeInternal")]
    pub type_internal: Option<TaskTypeInternal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x1")]
    pub x_1: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x2")]
    pub x_2: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(flatten)]
    pub task: Task,
}

impl BarTask {
    macros::setters! {
     bar_children: Vec<BarTask> => Some(bar_children),
     bar_corner_radius: f64  => Some(bar_corner_radius),
     handle_width: f64 => Some(handle_width),
     height: f64 => Some(height),
     index: f64 => Some(index),
     progress_width: f64  => Some(progress_width),
     progress_x: f64 => Some(progress_x),
     styles: BarTaskStyles => Some(styles),
     type_internal: TaskTypeInternal => Some(type_internal),
     x_1: f64  => Some(x_1),
     x_2: f64 => Some(x_2),
     y: f64 => Some(y),
     task: Task => task,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct DateSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<NaiveDateTime>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewMode")]
    pub view_mode: Option<ViewMode>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct DisplayOption {
    #[doc = " Specifies the month name language. Able formats: ISO 639-2, Java Locale"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preStepsCount")]
    pub pre_steps_count: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtl: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewDate")]
    pub view_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "viewMode")]
    pub view_mode: Option<ViewMode>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct EventOption {
    #[doc = " Invokes on bar click."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onClick")]
    pub on_click: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on end and start time change. Chart undoes operation if method return false or "]
    #[doc = " error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onDateChange")]
    pub on_date_change: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on delete selected task. Chart undoes operation if method return false or error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onDelete")]
    pub on_delete: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on bar double click."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onDoubleClick")]
    pub on_double_click: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on expander on task list"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onExpanderClick")]
    pub on_expander_click: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on progress change. Chart undoes operation if method return false or error."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onProgressChange")]
    pub on_progress_change: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Invokes on bar select on unselect."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onSelect")]
    pub on_select: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = " Time step value for date changes."]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeStep")]
    pub time_step: Option<f64>,
}
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum GanttContentMoveAction {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "click")]
    Click,
    #[serde(rename = "dblclick")]
    Dblclick,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "end")]
    End,
    #[serde(rename = "mouseenter")]
    Mouseenter,
    #[serde(rename = "mouseleave")]
    Mouseleave,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "progress")]
    Progress,
    #[serde(rename = "select")]
    Select,
    #[serde(rename = "start")]
    Start,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct GanttEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<GanttContentMoveAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changedTask")]
    pub changed_task: Option<BarTask>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originalSelectedTask")]
    pub original_selected_task: Option<BarTask>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Properties)]
pub struct GanttProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    #[serde(flatten)]
    pub event_option: EventOption,
    #[serde(flatten)]
    pub display_option: DisplayOption,
    #[serde(flatten)]
    pub style_option: StylingOption,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PartialHeaderHeightNumberRowWidthStringFontFamilyStringFontSizeString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headerHeight")]
    pub header_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowWidth")]
    pub row_width: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, Properties)]
pub struct TableProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onExpanderClick")]
    pub on_expander_click: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowHeight")]
    pub row_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowWidth")]
    pub row_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectedTaskId")]
    pub selected_task_id: Option<String>,
    #[doc = " Sets selected task by id"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "setSelectedTask")]
    pub set_selected_task: Option<::std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct PartialTaskTaskFontSizeStringFontFamilyString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReactFCHeaderHeightNumberRowWidthStringFontFamilyStringFontSizeString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextTypes")]
    pub context_types: Option<ValidationMapAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultProps")]
    pub default_props:
        Option<PartialHeaderHeightNumberRowWidthStringFontFamilyStringFontSizeString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propTypes")]
    pub prop_types: Option<FontFamilyStringFontSizeString>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct OnExpanderClickTaskTaskVoid {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextTypes")]
    pub context_types: Option<ValidationMapAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultProps")]
    pub default_props: Option<TableProps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propTypes")]
    pub prop_types: Option<ExpanderClickTaskTaskVoid>,
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReactFCTaskTaskFontSizeStringFontFamilyString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contextTypes")]
    pub context_types: Option<ValidationMapAny>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "defaultProps")]
    pub default_props: Option<PartialTaskTaskFontSizeStringFontFamilyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propTypes")]
    pub prop_types: Option<ReactWeakValidationMapTaskTaskFontSizeStringFontFamilyString>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct FontFamilyStringFontSizeString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headerHeight")]
    pub header_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowWidth")]
    pub row_width: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ExpanderClickTaskTaskVoid {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onExpanderClick")]
    pub on_expander_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowHeight")]
    pub row_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowWidth")]
    pub row_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "selectedTaskId")]
    pub selected_task_id: Option<String>,
    #[doc = " Sets selected task by id"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "setSelectedTask")]
    pub set_selected_task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<String>>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct ReactWeakValidationMapTaskTaskFontSizeStringFontFamilyString {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct StylingOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TaskListHeader")]
    pub task_list_header:
        Option<ReactFCHeaderHeightNumberRowWidthStringFontFamilyStringFontSizeString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TaskListTable")]
    pub task_list_table: Option<OnExpanderClickTaskTaskVoid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "TooltipContent")]
    pub tooltip_content: Option<ReactFCTaskTaskFontSizeStringFontFamilyString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arrowColor")]
    pub arrow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arrowIndent")]
    pub arrow_indent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barBackgroundColor")]
    pub bar_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barBackgroundSelectedColor")]
    pub bar_background_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barCornerRadius")]
    pub bar_corner_radius: Option<f64>,
    #[doc = " How many of row width can be taken by task."]
    #[doc = " From 0 to 100"]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barFill")]
    pub bar_fill: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barProgressColor")]
    pub bar_progress_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barProgressSelectedColor")]
    pub bar_progress_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "columnWidth")]
    pub column_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ganttHeight")]
    pub gantt_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "handleWidth")]
    pub handle_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "headerHeight")]
    pub header_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "listCellWidth")]
    pub list_cell_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "milestoneBackgroundColor")]
    pub milestone_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "milestoneBackgroundSelectedColor")]
    pub milestone_background_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projectBackgroundColor")]
    pub project_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projectBackgroundSelectedColor")]
    pub project_background_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projectProgressColor")]
    pub project_progress_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projectProgressSelectedColor")]
    pub project_progress_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rowHeight")]
    pub row_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "todayColor")]
    pub today_color: Option<String>,
}
#[derive(Clone, PartialEq, Eq, Debug, Default, Deserialize, Serialize)]
pub struct TaskStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "backgroundSelectedColor")]
    pub background_selected_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressColor")]
    pub progress_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "progressSelectedColor")]
    pub progress_selected_color: Option<String>,
}
#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub struct Task {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "displayOrder")]
    pub display_order: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hideChildren")]
    pub hide_children: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "isDisabled")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = " From 0 to 100"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<TaskStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<TaskType>,
}
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TaskType {
    #[serde(rename = "milestone")]
    Milestone,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "task")]
    Task,
}
impl TaskType {
    pub fn get_internal(&self) -> TaskTypeInternal {
        match self {
            TaskType::Milestone => TaskTypeInternal::Milestone,
            TaskType::Project => TaskTypeInternal::Project,
            TaskType::Task => TaskTypeInternal::Task,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Default)]
pub enum TaskTypeInternal {
    #[serde(rename = "milestone")]
    Milestone,
    #[serde(rename = "project")]
    #[default]
    Project,
    #[serde(rename = "smalltask")]
    Smalltask,
    #[serde(rename = "task")]
    Task,
}
pub type ValidationMapAny = ::std::collections::BTreeMap<String, serde_json::Value>;
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize, Default)]
pub enum ViewMode {
    Day,
    #[serde(rename = "Half Day")]
    HalfDay,
    Hour,
    #[default]
    Month,
    #[serde(rename = "Quarter Day")]
    QuarterDay,
    QuarterYear,
    Week,
    Year,
}

impl ViewMode {
    pub fn get_mod_date(&self, date_time: NaiveDateTime, add: bool) -> NaiveDateTime {
        match self {
            ViewMode::Hour => {
                if add {
                    date_time.checked_add_signed(chrono::Duration::hours(1))
                } else {
                    date_time.checked_sub_signed(chrono::Duration::hours(1))
                }
            }
            ViewMode::QuarterDay => {
                if add {
                    date_time.checked_add_signed(chrono::Duration::hours(6))
                } else {
                    date_time.checked_sub_signed(chrono::Duration::hours(6))
                }
            }
            ViewMode::HalfDay => {
                if add {
                    date_time.checked_add_signed(chrono::Duration::hours(12))
                } else {
                    date_time.checked_sub_signed(chrono::Duration::hours(12))
                }
            }
            ViewMode::Day => {
                if add {
                    date_time.checked_add_days(chrono::Days::new(1))
                } else {
                    date_time.checked_sub_days(chrono::Days::new(1))
                }
            }
            ViewMode::Week => {
                if add {
                    date_time.checked_add_days(chrono::Days::new(7))
                } else {
                    date_time.checked_sub_days(chrono::Days::new(7))
                }
            }
            ViewMode::Month => {
                if add {
                    date_time.checked_add_months(chrono::Months::new(1))
                } else {
                    date_time.checked_sub_months(chrono::Months::new(1))
                }
            }
            ViewMode::QuarterYear => {
                if add {
                    date_time.checked_add_months(chrono::Months::new(3))
                } else {
                    date_time.checked_sub_months(chrono::Months::new(3))
                }
            }
            ViewMode::Year => {
                if add {
                    date_time.checked_add_months(chrono::Months::new(12))
                } else {
                    date_time.checked_sub_months(chrono::Months::new(12))
                }
            }
        }
        .unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Properties)]
pub struct GridProps {
    pub tasks: Vec<super::schemas::Task>,
    pub dates: Vec<chrono::NaiveDateTime>,
    pub svg_width: f64,
    pub row_height: f64,
    pub column_width: f64,
    pub today_color: String,
    pub rtl: bool,
}

impl GridProps {
    macros::setters! {
        tasks: Vec<super::schemas::Task> => tasks,
        dates: Vec<chrono::NaiveDateTime> => dates,
        svg_width: f64 => svg_width,
        row_height: f64 => row_height,
        column_width: f64 => column_width,
        today_color: String => today_color,
        rtl: bool => rtl,
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CalendarProps {
    pub date_setup: DateSetup,
    pub locale: String,
    pub view_mode: ViewMode,
    pub rtl: bool,
    pub header_height: f64,
    pub column_width: f64,
    pub font_family: String,
    pub font_size: String,
}

impl CalendarProps {
    macros::setters! {
        date_setup: DateSetup => date_setup,
        locale: String => locale,
        view_mode: ViewMode => view_mode,
        rtl: bool => rtl,
        header_height: f64 => header_height,
        column_width: f64 => column_width,
        font_family: String => font_family,
        font_size: String => font_size,
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct TaskGanttContentProps {
    pub tasks: Vec<BarTask>,
    pub dates: Vec<chrono::NaiveDateTime>,
    pub gantt_event: GanttEvent,
    pub selected_task: BarTask,
    pub row_height: f64,
    pub time_step: f64,
    pub svg: yew::NodeRef,
    pub svg_width: f64,
    pub task_height: f64,
    pub arrow_color: String,
    pub arrow_indent: f64,
    pub column_width: f64,
    pub font_size: String,
    pub font_family: String,
    pub rtl: bool,
}

impl TaskGanttContentProps {
    macros::setters! {
        tasks: Vec<BarTask> => tasks,
        dates: Vec<chrono::NaiveDateTime> => dates,
        gantt_event: GanttEvent => gantt_event,
        selected_task: BarTask => selected_task,
        row_height: f64 => row_height,
        time_step: f64 => time_step,
        svg: yew::NodeRef => svg,
        svg_width: f64 => svg_width,
        task_height: f64 => task_height,
        arrow_color: String => arrow_color,
        arrow_indent: f64 => arrow_indent,
        column_width: f64 => column_width,
        rtl: bool => rtl,
        font_family: String => font_family,
        font_size: String => font_size,
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct SvgProps {
    pub grid_props: GridProps,
    pub calendar_props: CalendarProps,
    pub bar_props: TaskGanttContentProps,
    pub gantt_height: f64,
    pub scroll_y: f64,
    pub scroll_x: f64,
}
