use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::{logger::Logger, server::cache::ServerCache};
use quick_xml::{events::Event, name::QName, Reader};

pub struct PlanParser {
    logger: Arc<Mutex<Logger>>,
    cache: Arc<Mutex<ServerCache>>,
}

impl PlanParser {
    pub fn new(logger: Arc<Mutex<Logger>>, cache: Arc<Mutex<ServerCache>>) -> Self {
        Self { logger, cache }
    }

    pub fn parse_plan(&self, response: &str, dest: &str) -> BTreeMap<String, String> {
        let mut reader = Reader::from_str(response);
        let mut in_ride = false;
        let mut time = String::new();
        let mut current_id = String::new();
        let mut train_times: BTreeMap<String, String> = BTreeMap::new();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => {
                    return train_times;
                }
                Ok(Event::Start(ref el)) if el.name() == QName(b"s") => {
                    let id = el.attributes().next().unwrap().unwrap();
                    current_id = String::from_utf8(id.value.to_ascii_lowercase())
                        .expect("Failed to cast the id of station into String");
                    time = PlanParser::extract_time_from_id(&current_id);
                    in_ride = true;
                }
                Ok(Event::Empty(ref el)) if el.name() == QName(b"dp") => {
                    if let Ok(ppth) = el.try_get_attribute(QName(b"ppth")) {
                        let ppth: String =
                            String::from_utf8(ppth.unwrap().value.to_ascii_lowercase())
                                .expect("Failed to convert ppth to string");
                        if ppth.split("|").find(|x| *x == dest).is_some() && in_ride {
                            train_times
                                .entry(current_id.clone())
                                .or_insert(time.clone());
                        }
                    }
                }
                Ok(Event::End(ref el)) if el.name() == QName(b"s") => {
                    in_ride = false;
                }
                _ => (),
            }
        }
    }

    fn extract_time_from_id(id: &String) -> String {
        let date: String = id.split("-").filter(|part| part.len() == 10).collect();
        date
    }
}
