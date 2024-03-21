use quick_xml::{events::Event, name::QName, Reader};
use std::collections::BTreeMap;

pub struct ResponseParser;

impl ResponseParser {
    pub fn parse(response: &str) -> u8 {
        let mut reader = Reader::from_str(response);
        let mut in_ride = false;
        let mut children = 0;
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => return children,
                Ok(Event::Start(el)) if el.name() == QName(b"s") => {
                    let id = el.attributes().next().unwrap().unwrap();
                    let id_as_string = String::from_utf8(id.value.to_ascii_lowercase())
                        .expect("Failed to cast the id of station into String");
                    let time = ResponseParser::extract_time_from_id(id_as_string);
                    in_ride = true;
                }
                Ok(Event::End(el)) if el.name() == QName(b"s") => {
                    in_ride = false;
                    children += 1;
                }
                _ => (),
            }
        }
    }

    fn extract_time_from_id(id: String) -> String {
        let date: String = id.split("-").filter(|part| part.len() == 9).collect();
        date
    }
}

mod tests {
    use super::ResponseParser;
    use std::fs;

    #[test]
    fn test_parse() -> std::io::Result<()> {
        let response = fs::read_to_string("tests/station_response.xml")?;
        let result = ResponseParser::parse(response.as_str());
        assert_eq!(18, result);

        Ok(())
    }
}
