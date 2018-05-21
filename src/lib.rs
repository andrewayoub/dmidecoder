pub mod dmidecoder {
    use std::collections::HashMap;
    
    /// Property represents a single item inside a section
    /// it contains value and items as vector
    /// you should have at least on of these fields set
    #[derive(Debug)]
    pub struct Property {
        pub value: String,
        pub items: Vec<String>,
    }

    impl Property {
        /// Returns empty property
        /// # examples
        /// ```
        /// let property: Property = Property::new();
        /// ```
        pub fn new() -> Property {
            Property {
                value: String::new(),
                items: Vec::new(),
            }
        }
        /// Check if the property is empty
        /// # examples
        /// ```
        /// property.is_empty()
        /// ```
        pub fn is_empty(&self) -> bool {
            return self.value.is_empty() && self.items.is_empty()
        }
    }

    /// Section represents a full section of data
    #[derive(Debug)]
    pub struct Section {
        pub handle_line: String,
        pub properties: HashMap<String, Property>,
    }

    impl Section {
        /// Returns empty Section
        /// # examples
        /// ```
        /// let section: Section = Section::new();
        /// ```
        fn new() -> Section {
            Section {
                handle_line: String::new(),
                properties: HashMap::new(),
            }
        }
        /// Check if the section is empty
        /// # examples
        /// ```
        /// section.is_empty()
        /// ```
        fn is_empty(&self) -> bool {
            return self.handle_line.is_empty() && self.properties.is_empty()
        }
    }

    /// State will be used in parsing
    /// will indecate how to handle the current line
    #[derive(Debug)]
    enum State {
        Section,
        Kv,
        List,
    }
    /// clean string by removing white spaces from the begining
    fn clean_str(line :&str) -> String {
        let mut i = 0;
        for c in line.chars() {
            if c.is_whitespace() {
                i = i + 1;
            } else {
                return line[i..].to_string();
            }
        }
        return String::from(line.to_string())
    }
    /// get the current indentation level by counting the white spaces or taps
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
    /// decide the current state of the line
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
    // Parse data into sections hashmap
    pub fn parse(data :&str) -> HashMap<String, Section> {
        let mut sections = HashMap::new();
        let mut state = State::Section;
        let mut current_section: Section = Section::new();
        let mut current_section_name: String = String::new();
        let mut current_property: Property = Property::new();
        let mut current_property_name = String::new(); 
        let mut last_indentation = 0;

        for line in data.lines() {
            // decide state
            let indentation = get_indentation(line);
            state = get_state(state, line, last_indentation);
            last_indentation = indentation;
            // match the state to parse each line
            match state {
                State::Section => {
                    if line.starts_with("Handle") {
                        // when we find handle line this means it's the begining of a new section
                        // so we will push the last section if it's not empty
                        // and create another one with the found handle line
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
                        // if we have the handle line already we will read the follopwing line as the title
                    } else if !current_section.handle_line.is_empty() && !line.is_empty() {
                        current_section_name = clean_str(line);
                    }
                },
                State::Kv => {
                    // will check if we have current properety data to push before creating another one from the current line
                    if !current_property.is_empty() && !current_property_name.is_empty() {
                        current_section.properties.insert(current_property_name, current_property);
                        current_property = Property::new();
                    }
                    let kvdata : Vec<&str> = line.split(':').collect();
                    current_property_name = clean_str(kvdata[0]);
                    if kvdata.len() > 1 {
                        current_property.value = clean_str(kvdata[1]);
                    }
                },
                State::List => {
                   current_property.items.push(clean_str(line))
                }
            }
        }
        // finalize by pushing the last property and the current section if there is data00 
        if !current_property.is_empty() && !current_property_name.is_empty() {
            current_section.properties.insert(current_property_name, current_property);
        }
        if !current_section.is_empty() && !current_section_name.is_empty(){
            sections.insert(current_section_name, current_section);
        }
        return sections;
    }
}

mod tests;