use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use hypermelon::{attr::PathCommand::*, build, prelude::*};
use serde::{Deserialize, Serialize};
use std::error::Error;

pub type GenericError = Box<dyn std::error::Error + Send + Sync>;

static MONTH_NAMES: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

pub struct GanttChartTool {}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemData {
    pub title: String,
    // pub color_group: String,
    pub duration: Option<i64>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(rename = "resource")]
    pub resource_name: Option<String>,
    pub open: Option<bool>,
    pub progress: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum ItemType {
    Project,
    Task,
    SubTask,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChartData {
    pub title: String,
    #[serde(rename = "markedDate")]
    pub marked_date: Option<NaiveDate>,
    pub resources: Vec<ChartDataRes>,
    pub items: Vec<ItemData>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChartDataRes {
    pub name: String,
    pub backgroundColor: String,
    pub backgroundSelectedColor: String,
    pub progressColor: String,
    pub progressSelectedColor: String,
}

#[derive(Debug)]
pub struct Gutter {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Gutter {
    pub fn height(&self) -> f32 {
        self.bottom + self.top
    }

    pub fn width(&self) -> f32 {
        self.right + self.left
    }
}

#[derive(Debug)]
struct RenderData {
    gutter: Gutter,
    row_gutter: Gutter,
    row_height: f32,
    resource_gutter: Gutter,
    resource_height: f32,
    marked_date_offset: f32,
    title_width: f32,
    max_month_width: f32,
    rect_corner_radius: f32,
    styles: Vec<String>,
    cols: Vec<ColumnRenderData>,
    rows: Vec<RowRenderData>,
    resources: Vec<RenderDataRes>,
}

#[derive(Debug)]
pub struct RenderDataRes {
    pub name: String,
    pub backgroundColor: String,
    pub backgroundSelectedColor: String,
    pub progressColor: String,
    pub progressSelectedColor: String,
}

#[derive(Debug)]
struct RowRenderData {
    title: String,
    resource_name: String,
    offset: f32,
    progress: f32,
    // If length not present then this is a milestone
    length: Option<f32>,
    open: bool,
}

#[derive(Debug)]
struct ColumnRenderData {
    width: f32,
    month_name: String,
}

impl GanttChartTool {
    pub fn new() -> Self {
        GanttChartTool {}
    }

    pub fn run(
        &mut self,
        chart_data: ChartData,
        title_width: f32,
        max_month_width: f32,
        add_resource_table: bool,
    ) -> Result<String, Box<dyn Error>> {
        let render_data = self.process_chart_data(title_width, max_month_width, &chart_data)?;
        let output = self.render_chart(add_resource_table, &render_data)?;

        Ok(output)
    }

    fn process_chart_data(
        &self,
        title_width: f32,
        max_month_width: f32,
        chart_data: &ChartData,
    ) -> Result<RenderData, Box<dyn Error>> {
        fn num_days_in_month(year: i32, month: u32) -> u32 {
            // the first day of the next month...
            let (y, m) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
            let d = NaiveDate::from_ymd(y, m, 1);

            // ...is preceded by the last day of the original month
            d.pred().day()
        }

        // Fail if only one task
        if chart_data.items.len() < 2 {
            return Err(GenericError::from("You must provide more than one task"));
        }

        let mut start_date = NaiveDate::MAX;
        let mut end_date = NaiveDate::MIN;
        let mut date = NaiveDate::MIN;
        let mut shadow_durations: Vec<Option<i64>> = Vec::with_capacity(chart_data.items.len());

        // Determine the project start & end dates
        for (i, item) in chart_data.items.iter().enumerate() {
            if let Some(item_start_date) = item.start_date {
                date = item_start_date;

                if item_start_date < start_date {
                    // Move the start if it falls on a weekend
                    start_date = match date.weekday() {
                        Weekday::Sat => date + Duration::days(2),
                        Weekday::Sun => date + Duration::days(1),
                        _ => date,
                    };
                }
            } else if i == 0 {
                return Err(From::from(
                    "First item must contain a start date".to_string(),
                ));
            }

            // Skip the weekends and update a shadow list of the _real_ durations
            if let Some(item_days) = item.duration {
                let duration = match (date + Duration::days(item_days)).weekday() {
                    Weekday::Sat => Duration::days(item_days + 2),
                    Weekday::Sun => Duration::days(item_days + 1),
                    _ => Duration::days(item_days),
                };

                date += duration;

                shadow_durations.push(Some(duration.num_days()));
            } else {
                shadow_durations.push(None);
            }

            if end_date < date {
                end_date = date;
            }
        }

        start_date = NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), 1).unwrap();
        end_date = NaiveDate::from_ymd_opt(
            end_date.year(),
            end_date.month(),
            num_days_in_month(end_date.year(), end_date.month()),
        )
        .unwrap();

        // Create all the column data
        let mut all_items_width: f32 = 0.0;
        let mut num_item_days: u32 = 0;
        let mut cols = vec![];

        date = start_date;

        while date <= end_date {
            let item_days = num_days_in_month(date.year(), date.month());
            let item_width = max_month_width * (item_days as f32) / 31.0;

            num_item_days += item_days;
            all_items_width += item_width;

            cols.push(ColumnRenderData {
                width: item_width,
                month_name: MONTH_NAMES[date.month() as usize - 1].to_string(),
            });

            date = NaiveDate::from_ymd_opt(
                date.year() + (if date.month() == 12 { 1 } else { 0 }),
                date.month() % 12 + 1,
                1,
            )
            .unwrap();
        }

        date = start_date;

        let gutter = Gutter {
            left: 0.0,
            top: 80.0, // todo!!!
            right: 0.0,
            bottom: 0.0,
        };
        let row_gutter = Gutter {
            left: 5.0,
            top: 5.0,
            right: 5.0,
            bottom: 5.0,
        };
        let row_height = row_gutter.height() + 20.0;
        let resource_gutter = Gutter {
            left: 10.0,
            top: 10.0,
            right: 10.0,
            bottom: 10.0,
        };
        let resource_height = resource_gutter.height() + 20.0;
        let mut rows = vec![];

        // Calculate the X offsets of all the bars and milestones
        for (i, item) in chart_data.items.iter().enumerate() {
            if let Some(item_start_date) = item.start_date {
                date = item_start_date;
            }

            let offset = title_width
                + gutter.left
                + ((date - start_date).num_days() as f32) / (num_item_days as f32)
                    * all_items_width;

            let mut length: Option<f32> = None;

            if let Some(item_days) = shadow_durations[i] {
                // Use the shadow duration instead of the actual duration as it accounts for weekends
                date += Duration::days(item_days);
                length = Some((item_days as f32) / (num_item_days as f32) * all_items_width);
            }

            let resource_index = if let Some(ref item_resource_index) = item.resource_name {
                item_resource_index.to_owned()
            } else {
                "undifiend".to_owned()
            };

            rows.push(RowRenderData {
                title: item.title.clone(),
                resource_name: resource_index.to_owned(),
                offset,
                length,
                open: item.open.unwrap_or(false),
                progress: item.progress,
            });
        }

        let marked_date_offset = title_width
            + gutter.left
            + ((Utc::now().naive_utc().date() - start_date).num_days() as f32)
                / (num_item_days as f32)
                * all_items_width;

        let mut styles = vec![
            ".outer-lines{stroke-width:3;stroke:#aaaaaa;}".to_owned(),
            ".inner-lines{stroke-width:2;stroke:#dddddd;}".to_owned(),
            ".item{font-family:Arial;font-size:12pt;dominant-baseline:middle;}".to_owned(),
            ".resource{font-family:Arial;font-size:12pt;text-anchor:end;dominant-baseline:middle;}".to_owned(),
            ".title{font-family:Arial;font-size:18pt;}".to_owned(),
            ".heading{font-family:Arial;font-size:16pt;dominant-baseline:middle;text-anchor:middle;}".to_owned(),
            ".task-heading{dominant-baseline:middle;text-anchor:start;}".to_owned(),
            ".milestone{fill:black;stroke-width:1;stroke:black;}".to_owned(),
            ".marker{stroke-width:2;stroke:#888888;stroke-dasharray:7;}".to_owned(),
            ".resource-undifiend{fill:#ff75fe;stroke-width:1;stroke:#ff75fe;}".to_owned(),
        ];

        for ele in &chart_data.resources {
            let name = ele.name.to_owned();
            let rgb = ele.backgroundColor.to_owned();
            styles.push(format!(
                ".resource-{name}{{fill:{rgb};stroke-width:1;stroke:{rgb};}}",
            ));
        }

        Ok(RenderData {
            gutter,
            row_gutter,
            row_height,
            resource_gutter,
            resource_height,
            styles,
            title_width,
            max_month_width,
            marked_date_offset,
            rect_corner_radius: 3.0,
            cols,
            rows,
            resources: chart_data
                .resources
                .iter()
                .map(|it| RenderDataRes {
                    name: it.name.to_owned(),
                    backgroundColor: it.backgroundColor.to_owned(),
                    backgroundSelectedColor: it.backgroundSelectedColor.to_owned(),
                    progressColor: it.progressColor.to_owned(),
                    progressSelectedColor: it.progressSelectedColor.to_owned(),
                })
                .collect(),
        })
    }

    fn render_chart(
        &self,
        add_resource_table: bool,
        rd: &RenderData,
    ) -> Result<String, Box<dyn Error>> {
        let width: f32 = rd.gutter.left
            + rd.title_width
            + rd.cols.iter().map(|col| col.width).sum::<f32>()
            + rd.gutter.right;
        let height = rd.gutter.top
            + (rd.rows.len() as f32 * rd.row_height)
            + (if add_resource_table {
                rd.resource_gutter.height() + rd.row_height
            } else {
                0.0
            })
            + rd.gutter.bottom;

        // todo!: rd.styles.iter().map(|it| it.as_str())
        let style =
            build::elem("style").append(build::from_iter(rd.styles.iter().map(|it| it.as_str())));

        let svg = build::elem("svg").with(attrs!(
            ("xmlns", "http://www.w3.org/2000/svg"),
            ("width", width),
            ("height", height),
            ("viewBox", format_move!("0 0 {} {}", width, height)),
            ("style", "background-color: white;")
        ));

        // Render all the chart rows
        let rows = build::elem("g").append(build::from_iter((0..=rd.rows.len()).map(|i| {
            build::from_closure(move |w| {
                let y = rd.gutter.top + (i as f32 * rd.row_height);
                let line;

                if i == 0 || i == rd.rows.len() {
                    line = build::single("line").with(attrs!(
                        ("class", "outer-lines"),
                        ("x1", rd.gutter.left),
                        ("y1", y),
                        ("x2", width - rd.gutter.right),
                        ("y2", y)
                    ));
                } else {
                    line = build::single("line").with(attrs!(
                        ("class", "inner-lines"),
                        ("x1", rd.gutter.left),
                        ("y1", y),
                        ("x2", width - rd.gutter.right),
                        ("y2", y)
                    ));
                }

                // Are we on one of the task rows?
                if i < rd.rows.len() {
                    let row: &RowRenderData = &rd.rows[i];
                    let _text = build::elem("text")
                        .with(attrs!(
                            ("class", "item"),
                            ("x", rd.gutter.left + rd.row_gutter.left),
                            ("y", y + rd.row_gutter.top + rd.row_height / 2.0)
                        ))
                        .append(row.title.as_str());

                    // Is this a task or a milestone?
                    if let Some(length) = row.length {
                        let bar = build::single("rect").with(attrs!(
                            ("class", format_move!("resource-{}", row.resource_name)),
                            ("x", row.offset),
                            ("y", y + rd.row_gutter.top,),
                            ("rx", rd.rect_corner_radius),
                            ("ry", rd.rect_corner_radius),
                            ("width", length),
                            ("height", rd.row_height - rd.row_gutter.height())
                        ));
                        let fill = match rd.resources.iter().find(|it| it.name == row.resource_name)
                        {
                            Some(it) => it.progressColor.clone(),
                            None => "".to_owned(),
                        };

                        let bar_fill = build::single("rect").with(attrs!(
                            ("x", row.offset),
                            ("y", y + rd.row_gutter.top,),
                            ("rx", rd.rect_corner_radius),
                            ("ry", rd.rect_corner_radius),
                            ("width", row.progress),
                            ("height", rd.row_height - rd.row_gutter.height()),
                            ("fill", fill)
                        ));

                        w.render(line.append(bar).append(bar_fill))
                    } else {
                        let n = (rd.row_height - rd.row_gutter.height()) / 2.0;

                        let milestone = build::single("path").with(attrs!(
                            ("class", "milestone"),
                            build::path([
                                M(row.offset - n, y + rd.row_gutter.top + n),
                                L_(n, -n),
                                L_(n, n),
                                L_(-n, n),
                                L_(-n, -n)
                            ])
                        ));

                        w.render(line.append(milestone))
                    }
                } else {
                    w.render(line)
                }
            })
        })));

        // Render all the charts columns
        let columns = build::elem("g").append(build::from_iter((0..=rd.cols.len()).map(|i| {
            build::from_closure(move |w| {
                let x: f32 = rd.gutter.left
                    + rd.title_width
                    + rd.cols.iter().take(i).map(|col| col.width).sum::<f32>();
                let line = build::single("line").with(attrs!(
                    ("class", "inner-lines"),
                    ("x1", x),
                    ("y1", rd.gutter.top),
                    ("x2", x),
                    (
                        "y2",
                        rd.gutter.top + ((rd.rows.len() as f32) * rd.row_height)
                    )
                ));

                if i < rd.cols.len() {
                    let text = build::elem("text")
                        .with(attrs!(
                            ("class", "heading"),
                            ("x", x + rd.max_month_width / 2.0),
                            (
                                "y",
                                rd.gutter.top - rd.row_gutter.bottom - rd.row_height / 2.0
                            )
                        ))
                        .append(rd.cols[i].month_name.as_str()); // todo: !format_move!("{}", &rd.cols[i].month_name)
                    log::debug!("{i} --- {:?}", rd.cols[i].month_name.as_str());
                    w.render(line.append(text))
                } else {
                    log::debug!("{:?}", i);
                    w.render(line)
                }
            })
        })));

        let marked = build::from_closure(move |w| {
            let marker = build::single("line").with(attrs!(
                ("class", "marker"),
                ("x1", rd.marked_date_offset),
                ("y1", rd.gutter.top - 5.0),
                ("x2", rd.marked_date_offset),
                (
                    "y2",
                    rd.gutter.top + ((rd.rows.len() as f32) * rd.row_height) + 5.0
                )
            ));

            w.render(marker)
        });

        let all = svg
            .append(style)
            .append(columns)
            // .append(tasks)
            .append(rows)
            .append(marked);
        // .append(resources);

        let mut output = String::new();
        hypermelon::render(all, &mut output)?;

        Ok(output)
    }
}
