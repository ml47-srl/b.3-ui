use app::App;
use libsrl::navi::CellPath;
use libsrl::cell::Cell;
use libsrl::gen::*;
use app::get_height;
use app::MsgType;

extern crate ncurses;

impl App {
	pub fn render(&self) {
		ncurses::clear();
		for i in 0..self.db.count_rules() {
			let (prim, sec_vec) = self.filter_important(i as i32);
			let (prim_indices, sec_indices_vec) = determine_indices(prim, sec_vec);
			self.render_rule(self.db.get_rule(i), prim_indices, sec_indices_vec);
		}
		ncurses::mv(get_height() - 1, 0);
		ncurses::clrtoeol();
		match self.msg_type {
			MsgType::Error => { ncurses::attron(ncurses::COLOR_PAIR(4)); },
			MsgType::Input => { ncurses::attron(ncurses::COLOR_PAIR(5)); ncurses::printw("> "); },
			_ => {}
		}
		ncurses::printw(&self.msg);
		match self.msg_type {
			MsgType::Error => { ncurses::attroff(ncurses::COLOR_PAIR(4)); },
			MsgType::Input => { ncurses::attroff(ncurses::COLOR_PAIR(5)); },
			_ => {}
		}
		ncurses::refresh();
	}

	fn render_rule(&self, rule : Cell, prim_indices : Option<(i32, i32)>, sec_indices_vec : Vec<(i32, i32)>) {
		let string = rule.to_rule_string();
		for i in 0..string.len() {
			if let Some((x, _)) = prim_indices {
				if x == i as i32 {
					ncurses::attron(ncurses::COLOR_PAIR(2));
					ncurses::addch('<' as u64);
					ncurses::attroff(ncurses::COLOR_PAIR(2));
				}
			}
			for sec_indices in sec_indices_vec.clone() {
				let (x, y) = sec_indices;
				if x == i as i32 {
					ncurses::attron(ncurses::COLOR_PAIR(3));
					ncurses::addch('<' as u64);
					ncurses::attroff(ncurses::COLOR_PAIR(3));
				}
				if y == i as i32 {
					ncurses::attron(ncurses::COLOR_PAIR(3));
					ncurses::addch('>' as u64);
					ncurses::attroff(ncurses::COLOR_PAIR(3));
				}
			}
			if let Some((_, y)) = prim_indices {
				if y == i as i32 {
					ncurses::attron(ncurses::COLOR_PAIR(2));
					ncurses::addch('>' as u64);
					ncurses::attroff(ncurses::COLOR_PAIR(2));
				}
			}
			ncurses::addch(string.chars().nth(i).unwrap() as u64);
		}
		ncurses::addch('\n' as u64);
	}

	fn filter_important(&self, i : i32) -> (Option<CellPath>, Vec<CellPath>) {
		let prim = {
			if self.prim_marker.get_rule_id() as i32 == i {
				Some(self.prim_marker.get_path(&self.db.get_rules()).unwrap())
			} else {
				None
			}
		};
		let mut sec_vec = Vec::new();
		for sec in self.sec_markers.clone() {
			if sec.get_rule_id() as i32 == i {
				sec_vec.push(sec.get_path(&self.db.get_rules()).unwrap());
			}
		}
		(prim, sec_vec)
	}
}

fn determine_indices(prim : Option<CellPath>, sec_vec : Vec<CellPath>) -> (Option<(i32, i32)>, Vec<(i32, i32)>) {
	let prim_indices = match prim {
		Some(x) => Some(find_start_and_end(x)),
		None => None
	};
	let mut sec_indices_vec = Vec::new();
	for sec in sec_vec {
		sec_indices_vec.push(find_start_and_end(sec));
	}
	(prim_indices, sec_indices_vec)
}

fn find_start_and_end(path : CellPath) -> (i32, i32) {
	let x = find_inner_index(path.clone());
	if path.get_indices().is_empty() {
		return (x, x + path.get_cell().to_unwrapped_string().len() as i32);
	} else {
		return (x, x + path.get_cell().to_string().len() as i32);
	}
}

fn find_inner_index(path : CellPath) -> i32 {
	let x = simple("x".to_string());
	let y = simple("y".to_string());

	find_first_string_difference(path.replace_by(x).to_rule_string(), path.replace_by(y).to_rule_string())
}

fn find_first_string_difference(string1 : String, string2 : String) -> i32 {
	for i in 0..string1.len() {
		if string1.chars().nth(i).unwrap() != string2.chars().nth(i).unwrap() {
			return i as i32;
		}
	}
	panic!("find_first_string_difference: no difference")
}
