
#[derive(Debug)]
struct Property {
    name: String,
    value: String,
    items: Vec<String>,
}

impl Property {
    fn new() -> Property {
        Property {
            name: String::new(),
            value: String::new(),
            items: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        return self.name.is_empty() && self.value.is_empty() && self.items.is_empty()
    }
}

#[derive(Debug)]
struct Section {
    title: String,
    handle_line: String,
    properties: Vec<Property>,
}

impl Section {
    fn new() -> Section {
        Section {
            title: String::new(),
            handle_line: String::new(),
            properties: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        return self.title.is_empty() && self.handle_line.is_empty() && self.properties.is_empty()
    }
}
#[derive(Debug)]
enum State {
    Section,
    Kv,
    List,
}

fn get_indentation(line :&str) -> u8 {
    let mut count = 0;
    for c in line.chars() {
        if c.is_whitespace() {
            count = count + 1;
        } else {
            return count;
        }
    }
    return count;
}
fn main() {
    let sample = r#"
Getting SMBIOS data from sysfs.
SMBIOS 2.7 present.

Handle 0x0000, DMI type 0, 24 bytes
BIOS Information
	Vendor: LENOVO
	Version: 9BCN26WW
	Release Date: 07/31/2014
	Address: 0xE0000
	Runtime Size: 128 kB
	ROM Size: 4096 kB
	Characteristics:
		PCI is supported
		BIOS is upgradeable
		BIOS shadowing is allowed
		Boot from CD is supported
		Selectable boot is supported
		EDD is supported
		Japanese floppy for NEC 9800 1.2 MB is supported (int 13h)
		Japanese floppy for Toshiba 1.2 MB is supported (int 13h)
		5.25"/360 kB floppy services are supported (int 13h)
		5.25"/1.2 MB floppy services are supported (int 13h)
		3.5"/720 kB floppy services are supported (int 13h)
		3.5"/2.88 MB floppy services are supported (int 13h)
		8042 keyboard services are supported (int 9h)
		CGA/mono video services are supported (int 10h)
		ACPI is supported
		USB legacy is supported
		BIOS boot specification is supported
		Targeted content distribution is supported
		UEFI is supported
	BIOS Revision: 0.26
	Firmware Revision: 0.26

Handle 0x0010, DMI type 13, 22 bytes
BIOS Language Information
	Language Description Format: Long
	Installable Languages: 4
		en|US|iso8859-1
		fr|CA|iso8859-1
		ja|JP|unicode
		zh|TW|unicode
	Currently Installed Language: en|US|iso8859-1

    "#;
    let mut sections: Vec<Section> = Vec::new();
    let mut state = State::Section;
    let mut current_section: Section = Section::new();
    let mut current_property: Property = Property::new(); 
    let mut last_indentation = 0;

    for line in sample.lines() {
        // decide state
        // if indentation didn't change state shouldn't change
        let indentation = get_indentation(line);
        if indentation > last_indentation {
            match state {
                State::Section => state = State::Kv,
                State::Kv => state = State::List,
                State::List => (),
            }
        } else if indentation < last_indentation {
            match state {
                State::Section => (),
                State::Kv => state = State::Section,
                State::List => {
                    if indentation == 0 {
                        state = State::Section;
                    } else {
                        state = State::Kv;
                    }
                },
            }
        }
        last_indentation = indentation;
        println!("{} : {:?}:\t{}", indentation, state, line);
        match state {
            State::Section => {
                println!(">>>>>>{:?}", current_section.title);

                if line.starts_with("Handle") {
                    if !current_section.is_empty() {
                        if !current_property.is_empty(){
                            current_section.properties.push(current_property);
                            current_property = Property::new();
                        }
                        sections.push(current_section);
                        current_section = Section::new();
                    }
                    current_section.handle_line = String::from(line);
                } else if !current_section.handle_line.is_empty() && !line.is_empty() {
                    println!("IN TITLEEEEE {}", line);
                    current_section.title = String::from(line);
                }
            },
            State::Kv => {
                if !current_property.is_empty() {
                    current_section.properties.push(current_property);
                    current_property = Property::new();
                }
                let colon_index = line.find(':').unwrap_or(line.len());
                if colon_index != line.len()
                {
                    current_property.name = String::from(&line[..colon_index]);
                    current_property.value = String::from(&line[colon_index..]);
                }
            },
            State::List => {
               current_property.items.push(String::from(line))
            }
        }
    }
    if !current_property.is_empty(){
        current_section.properties.push(current_property);
    }
    if !current_section.is_empty(){
        sections.push(current_section);
        println!("IN PUSH");

    }
    
    println!("{:?}", sections)
}
