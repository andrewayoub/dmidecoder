pub mod dmidecoder {
    use std::collections::HashMap;
 
    #[derive(Debug)]
    pub struct Property {
        pub value: String,
        pub items: Vec<String>,
    }

    impl Property {
        pub fn new() -> Property {
            Property {
                value: String::new(),
                items: Vec::new(),
            }
        }
        pub fn is_empty(&self) -> bool {
            return self.value.is_empty() && self.items.is_empty()
        }
    }

    #[derive(Debug)]
    pub struct Section {
        pub handle_line: String,
        pub properties: HashMap<String, Property>,
    }

    impl Section {
        fn new() -> Section {
            Section {
                handle_line: String::new(),
                properties: HashMap::new(),
            }
        }
        fn is_empty(&self) -> bool {
            return self.handle_line.is_empty() && self.properties.is_empty()
        }
    }

    #[derive(Debug)]
    enum State {
        Section,
        Kv,
        List,
    }

    fn clean_str(line :&str) -> String {
        let mut i = 0;
        for c in line.chars() {
            if c.is_whitespace() || c == ':' {
                i = i + 1;
            } else {
                return line[i..].to_string();
            }
        }
        return String::from(line.to_string())
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

    fn get_state(state :State, line :&str, last_indentation :u8) -> State {
        let mut new_state = State::Section;
        let indentation = get_indentation(line);
        if indentation == last_indentation {
            return state;
        } else if indentation > last_indentation {
            match state {
                State::Section => new_state = State::Kv,
                State::Kv => new_state = State::List,
                State::List => (),
            }
        } else if indentation < last_indentation {
            match state {
                State::Section => (),
                State::Kv => new_state = State::Section,
                State::List => {
                    if indentation == 0 {
                        new_state = State::Section;
                    } else {
                        new_state = State::Kv;
                    }
                },
            }
        }
        return new_state;
    }

    pub fn parse(data :&str) -> HashMap<String, Section> {
        let mut sections = HashMap::new();
        let mut state = State::Section;
        let mut current_section: Section = Section::new();
        let mut current_section_name: String = String::new();
        let mut current_property: Property = Property::new();
        let mut current_property_name = String::new(); 
        let mut last_indentation = 0;

        for line in data.lines() {
            let indentation = get_indentation(line);
            state = get_state(state, line, last_indentation);
            last_indentation = indentation;
            match state {
                State::Section => {
                    if line.starts_with("Handle") {
                        if !current_section.is_empty() && !current_section_name.is_empty() {
                            if !current_property.is_empty() && !current_property_name.is_empty() {
                                current_section.properties.insert(current_property_name, current_property);
                                current_property = Property::new();
                                current_property_name = String::new();
                            }
                            sections.insert(current_section_name, current_section);
                            current_section = Section::new();
                            current_section_name = String::new();
                        }
                        current_section.handle_line = clean_str(line);
                    } else if !current_section.handle_line.is_empty() && !line.is_empty() {
                        current_section_name = clean_str(line);
                    }
                },
                State::Kv => {
                    if !current_property.is_empty() && !current_property_name.is_empty() {
                        current_section.properties.insert(current_property_name, current_property);
                        current_property = Property::new();
                        current_property_name = String::new();
                    }
                    let colon_index = line.find(':').unwrap_or(line.len());
                    if colon_index != line.len()
                    {
                        current_property_name = clean_str(&line[..colon_index]);
                        current_property.value = clean_str(&line[colon_index..]);
                    }
                },
                State::List => {
                   current_property.items.push(clean_str(line))
                }
            }
        }
        if !current_property.is_empty() && !current_property_name.is_empty() {
            current_section.properties.insert(current_property_name, current_property);
        }
        if !current_section.is_empty() && !current_section_name.is_empty(){
            sections.insert(current_section_name, current_section);
        }
        return sections;
    }
}
#[cfg(test)]
mod tests {
    use dmidecoder::parse;
    #[test]
    fn it_works() {
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
        parse(sample);
    }
}