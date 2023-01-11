use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BestDay {
  pub created_at: String,
  pub date: String,
  pub id: String,
  pub modified_at: String,
  pub text: String,
  pub total_seconds: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Categories {
  pub decimal: String,
  pub digital: String,
  pub hours: f64,
  pub minutes: f64,
  pub name: String,
  pub percent: f64,
  pub text: String,
  pub total_seconds: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
  pub best_day: BestDay,
  pub categories: Vec<Categories>,
  pub created_at: String,
  pub daily_average: f64,
  pub daily_average_including_other_language: f64,
  pub days_including_holidays: f64,
  pub days_minus_holidays: f64,
  pub dependencies: Vec<Categories>,
  pub editors: Vec<Categories>,
  pub end: String,
  pub holidays: f64,
  pub human_readable_daily_average: String,
  pub human_readable_daily_average_including_other_language: String,
  pub human_readable_range: String,
  pub human_readable_total: String,
  pub human_readable_total_including_other_language: String,
  pub id: String,
  pub is_already_updating: bool,
  pub is_coding_activity_visible: bool,
  pub is_including_today: bool,
  pub is_other_usage_visible: bool,
  pub is_stuck: bool,
  pub is_up_to_date: bool,
  pub is_up_to_date_pending_future: bool,
  pub languages: Vec<Categories>,
  pub machines: Vec<Machines>,
  pub modified_at: String,
  pub operating_systems: Vec<Categories>,
  pub percent_calculated: f64,
  pub projects: Vec<Categories>,
  pub range: String,
  pub start: String,
  pub status: String,
  pub timeout: f64,
  pub timezone: String,
  pub total_seconds: f64,
  pub total_seconds_including_other_language: f64,
  pub user_id: String,
  pub username: Option<String>,
  pub writes_only: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Machines {
  pub decimal: String,
  pub digital: String,
  pub hours: f64,
  pub machine_name_id: String,
  pub minutes: f64,
  pub name: String,
  pub percent: f64,
  pub text: String,
  pub total_seconds: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
  pub data: Data,
}

