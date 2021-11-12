use pitr_util::pitrmap::PitrMap;
use pitr_util::errors::{FromFileError, FromStringError};

fn main() {
	let args = std::env::args();

	if args.len() < 2 {
		panic!("specify at least one argument");
	}

	args.skip(1).for_each(|filename| {
		println!("---info for map {}---", filename);

		let map = PitrMap::from_file(filename.as_str());

		match map {
			Ok(parsed_map) => {
				println!("Blocks: {}", parsed_map.Blocks.len());
				println!("Props: {}", parsed_map.Props.len());
				println!("Enemies: {}", parsed_map.Enemies.len());
			},
			Err(err) => {
				match err {
					FromFileError::FileError(reason) => {
						println!("cannot open file, {}", reason);
					},
					FromFileError::FromStringError(error) => {
						match error {
							FromStringError::UnsupportedSaveVersion(version) => {
								println!("unsupported save version ({})", version);
							},
							FromStringError::UnsupportedGameVersion(version) => {
								println!("unsupported game version ({})", version);
							},
							FromStringError::ParseError(_) => {
								println!("invalid map file");
							},
						}
					}
				}
			}
		}
	});
}
