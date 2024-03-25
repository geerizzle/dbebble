use std::collections::{BTreeMap, HashMap};

use quick_xml::{events::Event, name::QName, Reader};

pub struct EvaParser;

impl EvaParser {
    pub fn parse_eva(response: &str) -> HashMap<String, String> {
        let mut reader = Reader::from_str(response);
        let mut eva_map: HashMap<String, String> = HashMap::new();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => return eva_map,
                Ok(Event::Empty(ref e)) if e.name() == QName(b"station") => {
                    let eva = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key == QName(b"eva"));
                    let name = e
                        .attributes()
                        .find(|a| a.as_ref().unwrap().key == QName(b"name"));
                    let eva_as_str = String::from_utf8(
                        eva.unwrap().as_ref().unwrap().value.to_ascii_lowercase(),
                    )
                    .unwrap();

                    let name_as_str = String::from_utf8(
                        name.unwrap().as_ref().unwrap().value.to_ascii_lowercase(),
                    )
                    .unwrap();

                    eva_map.entry(eva_as_str).or_insert(name_as_str);
                }
                _ => {}
            }
        }
    }
}
