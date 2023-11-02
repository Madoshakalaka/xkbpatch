#![feature(fs_try_exists)]
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};

use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::fs::File;
use std::io::{Cursor, Write};

fn main() {
    let evdev = "/usr/share/X11/xkb/rules/evdev.xml";
    let mut reader = Reader::from_file(evdev).unwrap();
    // reader.trim_text(true);

    let mut buf = Vec::new();
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    let mut found_cutie = false;
    'outer: loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if !found_cutie && e.name().as_ref() == b"layout" => {
                writer
                    .write_event(Event::Start(BytesStart::new("layout")))
                    .unwrap();

                loop {
                    match reader.read_event_into(&mut buf) {
                        Ok(ref the_e @ Event::Text(ref t)) if !found_cutie => {
                            if unsafe { std::str::from_utf8_unchecked(t).contains("shiori") } {
                                found_cutie = true
                            }
                            writer.write_event(the_e).unwrap();
                        }
                        Ok(Event::End(e)) if !found_cutie && e.name().as_ref() == b"layoutList" => {
                            // <layout>
                            //   <configItem>
                            //     <name>shiori</name>
                            //     <!-- Keyboard indicator for the loveliest layout -->
                            //     <shortDescription>sh</shortDescription>
                            //     <description>English (Made with Love)</description>
                            //     <countryList>
                            //       <iso3166Id>US</iso3166Id>
                            //     </countryList>
                            //     <languageList>
                            //       <iso639Id>eng</iso639Id>
                            //     </languageList>
                            //   </configItem>
                            // </layout>

                            writer
                                .write_event(Event::Start(BytesStart::new("layout")))
                                .unwrap();
                            writer
                                .write_event(Event::Start(BytesStart::new("configItem")))
                                .unwrap();
                            writer
                                .write_event(Event::Start(BytesStart::new("name")))
                                .unwrap();
                            writer
                                .write_event(Event::Text(BytesText::new("shiori")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("name")))
                                .unwrap();

                            writer
                                .write_event(Event::Start(BytesStart::new("shortDescription")))
                                .unwrap();
                            writer
                                .write_event(Event::Text(BytesText::new("sh")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("shortDescription")))
                                .unwrap();

                            writer
                                .write_event(Event::Start(BytesStart::new("description")))
                                .unwrap();
                            writer
                                .write_event(Event::Text(BytesText::new("Shiori")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("description")))
                                .unwrap();

                            writer
                                .write_event(Event::Start(BytesStart::new("countryList")))
                                .unwrap();
                            writer
                                .write_event(Event::Start(BytesStart::new("iso3166Id")))
                                .unwrap();
                            writer
                                .write_event(Event::Text(BytesText::new("US")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("iso3166Id")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("countryList")))
                                .unwrap();

                            writer
                                .write_event(Event::Start(BytesStart::new("languageList")))
                                .unwrap();
                            writer
                                .write_event(Event::Start(BytesStart::new("iso639Id")))
                                .unwrap();
                            writer
                                .write_event(Event::Text(BytesText::new("eng")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("iso639Id")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("languageList")))
                                .unwrap();

                            writer
                                .write_event(Event::End(BytesEnd::new("configItem")))
                                .unwrap();
                            writer
                                .write_event(Event::End(BytesEnd::new("layout")))
                                .unwrap();

                            writer
                                .write_event(Event::End(BytesEnd::new("layoutList")))
                                .unwrap();
                        }
                        Ok(Event::Eof) => break 'outer,
                        Ok(the_e) => {
                            writer.write_event(the_e).unwrap();
                        }

                        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    }
                }
            }

            // we can either move or borrow the event to write, depending on your use-case
            Ok(e) => assert!(writer.write_event(e).is_ok()),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            // Err(e) => panic!("Error\n{}", std::str::from_utf8(&buf).unwrap()),
        }
    }

    let result = writer.into_inner().into_inner();

    let result = unsafe { std::str::from_utf8_unchecked(&result) };
    if found_cutie {
        println!("found cutie in evdev.xml")
    } else {
        println!("cutie not found in evdev.xml, written");
        let mut f = File::create(evdev).unwrap();
        f.write_all(result.as_bytes()).unwrap();
    }


    let shiori = "/usr/share/X11/xkb/symbols/shiori";
    if std::fs::try_exists(shiori).unwrap(){
        println!("symbols/shiori already exists, doing nothing");
    }else{
        println!("symbols/shiori doesn't exist, copying");
        let mut f = File::create(shiori).unwrap();
        f.write_all(include_bytes!("shiori")).unwrap();
    }
    

    #[cfg(debug_assertions)]
    println!("{result}");
}
