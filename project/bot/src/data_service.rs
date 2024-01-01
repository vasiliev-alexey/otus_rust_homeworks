use log::info;
use redis::Commands;
use std::collections::{HashMap, HashSet};
use std::fs;

use crate::config_service::AppConfig;
use scraper::{Html, Selector};

const COURSES_KEY: &str = "COURSES_KEY";

#[derive(Clone)]
pub struct Service {
    client: redis::Client,
    source_file: String,
    course_pattern: String,
}

impl Service {
    pub(crate) fn new(config: &AppConfig) -> Service {
        info!("Redis url: {}", config.redis_url);
        info!("Source file: {}", config.source_file);
        Service {
            client: redis::Client::open(&*config.redis_url).unwrap(),
            source_file: config.source_file.clone(),
            course_pattern: config.course_pattern.clone(),
        }
    }

    pub(crate) fn load_sched(self) -> Result<(), Box<dyn std::error::Error>> {
        self.read_sched().unwrap();
        Ok(())
    }

    pub(crate) fn data(&self) -> Vec<String> {
        let mut con = self.client.get_connection().unwrap();

        let gr_str = con.get(COURSES_KEY).unwrap_or_else(|_error| "".to_string());
        serde_json::from_str::<Vec<String>>(&gr_str)
            .unwrap()
            .clone()
    }

    pub(crate) fn get_sched(&self, group: &String) -> Vec<String> {
        let mut con = self.client.get_connection().unwrap();
        let gr_str = con.get(group).unwrap_or_else(|_error| "".to_string());
        serde_json::from_str::<Vec<String>>(&gr_str)
            .unwrap()
            .clone()
    }

    fn read_sched(&self) -> Result<(), Box<dyn std::error::Error>> {
        const SELECTORS_TABLE: &str = "table.table_full";
        const SELECTORS_TR: &str = "tr.table__row";
        const SELECTORS_TD: &str = "td.table__col";
        let contents = fs::read_to_string(&self.source_file).unwrap();

        let mut courses: HashSet<String> = HashSet::new();

        let mut schedule: HashMap<String, Vec<String>> = HashMap::new();

        let selector = Selector::parse(SELECTORS_TABLE).unwrap();
        let selector_tr = Selector::parse(SELECTORS_TR).unwrap();
        let selector_td = Selector::parse(SELECTORS_TD).unwrap();
        let document = Html::parse_document(&contents);

        document.select(&selector).for_each(|node| {
            let t: Vec<_> = node.select(&selector_tr).collect();

            t.iter().for_each(|s| {
                let l = s
                    .select(&selector_td)
                    .flat_map(|el| el.text())
                    .filter(|s| s != &"\n" && !s.is_empty())
                    .map(|s| s.replace('\n', ""))
                    .collect::<Vec<_>>();
                if let Some(s) = l.get(2) {
                    if s.starts_with(&self.course_pattern) {
                        courses.insert(s.to_string());

                        let sched = match l.len() == 7 {
                            true => format!(
                                "🔖 {} {}: {} ",
                                &l.first().unwrap(),
                                &l.get(1).unwrap(),
                                &l.get(4).unwrap()
                            ),
                            _ => format!(
                                "🔖 {} {}: {} ({})",
                                &l.first().unwrap(),
                                &l.get(1).unwrap(),
                                &l.get(5).unwrap(),
                                &l.get(6).unwrap()
                            ),
                        };
                        info!("l   {}", &sched);
                        schedule.entry(s.to_string()).or_default().push(sched);
                    }
                }
            });
        });
        let mut con = self.client.get_connection().unwrap();
        let _r: Result<String, redis::RedisError> = con.set(
            COURSES_KEY,
            serde_json::to_string(
                &courses
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
            .unwrap(),
        );

        courses.iter().for_each(|s| {
            if schedule.contains_key(s) {
                let _: Result<String, redis::RedisError> =
                    con.set(s, serde_json::to_string(&schedule.get(s)).unwrap());
            }
        });

        Ok(())
    }
}
