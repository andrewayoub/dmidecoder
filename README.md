# Dmidecoder parser with rust

Convert Dmidecoder output into properly formated data (HashMap)

## installation

add dmidecoder parser to your dependencies
```toml
dmidecoder = {git="https://github.com/andrewayoub/dmidecoder.git"}
```

import as external crate
```rust
extern crate dmidecoder;
use dmidecoder::dmidecoder::parse;
```

pass dmidecoder raw data to parse method
```rust
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

    let sections = parse(sample);
    println!("{:?}", sections["BIOS Information"].properties["Characteristics"].items)
    
```

## Contribution
Feel free to contribute by any modifications or improvements.

## Tests
you can rus tests using cargo
```bash
cargo test
```

## Thanks
To [xmonader](https://github.com/xmonader) for his support.
