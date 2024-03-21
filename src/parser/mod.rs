use std::io::BufRead;

use quick_xml::{events::Event, name::QName, Reader};

pub struct ResponseParser;

impl ResponseParser {
    pub fn parse(response: &str, dest: &str) -> Vec<String> {
        let mut reader = Reader::from_str(response);
        let mut in_ride = false;
        let mut time = String::new();
        let mut train_times: Vec<String> = Vec::new();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => {
                    train_times.sort();
                    return train_times;
                }
                Ok(Event::Start(el)) if el.name() == QName(b"s") => {
                    let id = el.attributes().next().unwrap().unwrap();
                    let id_as_string = String::from_utf8(id.value.to_ascii_lowercase())
                        .expect("Failed to cast the id of station into String");
                    time = ResponseParser::extract_time_from_id(id_as_string);
                    in_ride = true;
                }
                Ok(Event::Empty(el)) if el.name() == QName(b"dp") => {
                    if let Ok(ppth) = el.try_get_attribute(QName(b"ppth")) {
                        let ppth: String =
                            String::from_utf8(ppth.unwrap().value.to_ascii_lowercase())
                                .expect("Failed to convert ppth to string");
                        if ppth.split("|").find(|x| *x == dest).is_some() && in_ride {
                            train_times.push(time.clone());
                        }
                    }
                }
                Ok(Event::End(el)) if el.name() == QName(b"s") => {
                    in_ride = false;
                }
                _ => (),
            }
        }
    }

    fn extract_time_from_id(id: String) -> String {
        let date: String = id.split("-").filter(|part| part.len() == 10).collect();
        date
    }
}

mod tests {
    use super::ResponseParser;
    use std::fs;

    #[test]
    fn test_parse() -> std::io::Result<()> {
        let response = fs::read_to_string("tests/station_response.xml")?;
        let result = ResponseParser::parse(response.as_str(), "wyhlen");
        println!("Departure times: {result:?}");

        Ok(())
    }
}
