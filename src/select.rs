use crossterm::*;

#[derive(Clone, Debug)]
pub enum State {
	Untracked,
	Unmodified,
	Modified,
	Added,
	Deleted,
	Renamed,
	Copied,
	Unmerged,
	Missing,
	Ignored,
	Clean,
}

const RESET_COLOR: Attribute = Attribute::Reset;
const RESET_BG_COLOR: Attribute = Attribute::Reset;

const HELP_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 180, b: 100});

const UNTRACKED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 100, g: 180, b: 255});
const UNMODIFIED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 255, b: 255});
const MODIFIED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 200, b: 0});
const ADDED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 0, g: 255, b: 0});
const DELETED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 0, b: 0});
const RENAMED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 100, g: 100, b: 255});
const COPIED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 0, b: 255});
const UNMERGED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 180, b: 100});
const MISSING_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 0, b: 0});
const IGNORED_COLOR: Colored = Colored::Fg(Color::Rgb{r: 255, g: 180, b: 0});
const CLEAN_COLOR: Colored = Colored::Fg(Color::Rgb{r: 100, g: 180, b: 255});

impl State {
	fn color(&self) -> Colored {
		match self {
			State::Untracked => UNTRACKED_COLOR,
			State::Unmodified => UNMODIFIED_COLOR,
			State::Modified => MODIFIED_COLOR,
			State::Added => ADDED_COLOR,
			State::Deleted => DELETED_COLOR,
			State::Renamed => RENAMED_COLOR,
			State::Copied => COPIED_COLOR,
			State::Unmerged => UNMERGED_COLOR,
			State::Missing => MISSING_COLOR,
			State::Ignored => IGNORED_COLOR,
			State::Clean => CLEAN_COLOR,
		}
	}
}

#[derive(Clone)]
pub struct Entry {
	pub filename: String,
	pub selected: bool,
	pub state: State,
}

pub fn draw_select(
	input: &mut TerminalInput,
	entries: &mut Vec<Entry>,
	cursor_index: &mut usize,
) -> bool {
	if entries.len() == 0 {
		return false;
	}

	print!(
		"{}{}j/k{} move, {}space{} (de)select, {}a{} (de)select all, {}c{} continues\n\n",
		RESET_BG_COLOR,
		HELP_COLOR,
		RESET_COLOR,
		HELP_COLOR,
		RESET_COLOR,
		HELP_COLOR,
		RESET_COLOR,
		HELP_COLOR,
		RESET_COLOR,
	);

	let mut index = *cursor_index;

	for (i, e) in entries.iter().enumerate() {
		let cursor = if i == index { ">" } else { " " };
		let selection = if e.selected { "+" } else { " " };
		print!(
			"{}{} {} {}{:?}\t{}{}\n",
			RESET_COLOR,
			cursor,
			selection,
			e.state.color(),
			e.state,
			RESET_COLOR,
			e.filename
		);
	}

	match input.read_char() {
		Ok(key) => {
			match key {
				'\r' => return false,
				'c' => return false,
				'j' => index = (index + 1) % entries.len(),
				'k' => index = (index + entries.len() - 1) % entries.len(),
				' ' => entries[index].selected = !entries[index].selected,
				'a' => {
					if let Some(first) = entries.first().cloned() {
						for e in entries.iter_mut() {
							e.selected = !first.selected;
						}
					}
				}
				_ => (),
			};
		}
		Err(_) => {
			return false;
		}
	}

	*cursor_index = index;
	true
}
