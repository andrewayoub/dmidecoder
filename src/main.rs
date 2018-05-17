
#[derive(Debug)]
struct Property {
    value: Option<String>,
    items: Option<Vec<String>>,
}

#[derive(Debug)]
struct Section {
    title: Option<String>,
    handle_line: Option<String>,
    properties: Option<Vec<Property>>,
}

enum State {
    Section,
    Kv,
    List,
}

fn main() {
    let sample = r#"
# dmidecode 3.1
Getting SMBIOS data from sysfs.
SMBIOS 2.6 present.

Handle 0x0001, DMI type 1, 27 bytes
System Information
		Manufacturer: LENOVO
		Product Name: 20042
		Version: Lenovo G560
		Serial Number: 2677240001087
		UUID: CB3E6A50-A77B-E011-88E9-B870F4165734
		Wake-up Type: Power Switch
		SKU Number: Calpella_CRB
		Family: Intel_Mobile
    "#;
    let mut sections: Vec<Section> = Vec::new();
    let mut state = State::Section;
    let mut current_section: Option<Section> = None;
    let mut current_property: Option<Property> = None; 

    for c in sample.lines() {
        if c.starts_with("Handle"){
            match current_section {
                Some(s) => sections.push(s),
                None => (),
            }
            state = State::Section;
            current_section = Some(Section {
                title: None,
                handle_line: Some(String::from(c)),
                properties: None,
            });
            continue;
        }

        // should use match here 
        /*if state == State::Section {
            match current_section {
                Some(s) => {
                    s.title = c;
                    println!("{}", s.title)
                },
                None => ()// should panic here
            }
        }*/


    }
}
