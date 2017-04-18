extern crate ncurses;
use libsrl::error::SRLError;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[derive(PartialEq)]
pub enum MsgType { Note, Error, Input }

pub struct App {
	pub db : Database,
	pub prim_marker : CellID,
	pub sec_markers : Vec<CellID>,
	pub msg : String,
	pub msg_type : MsgType
}

impl App {
	pub fn by_filename(string : &str) -> Result<App, SRLError> {
		let db = Database::by_filename(string)?;
		let cell_id = CellID::create(0, vec![]);
		Ok(App { db : db, prim_marker :  cell_id, sec_markers : Vec::new(), msg : String::new(), msg_type : MsgType::Note })
	}

	pub fn run(&mut self) {
		ncurses::initscr();
		ncurses::noecho();
		ncurses::start_color();
		ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
		ncurses::init_pair(1, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK); // default
		ncurses::init_pair(2, ncurses::COLOR_BLUE, ncurses::COLOR_BLACK); // primary marker
		ncurses::init_pair(3, ncurses::COLOR_GREEN, ncurses::COLOR_BLACK); // secondary markers
		ncurses::init_pair(4, ncurses::COLOR_RED, ncurses::COLOR_BLACK); // error messages
		ncurses::init_pair(5, ncurses::COLOR_YELLOW, ncurses::COLOR_BLACK); // input

		self.render();
		while self.handle_key(ncurses::getch()) {
			self.render();
			self.put_note("".to_string());
		}
		ncurses::endwin();
	}

	pub fn put_error(&mut self, msg : String) {
		self.msg = msg;
		self.msg_type = MsgType::Error;
	}

	pub fn put_note(&mut self, msg : String) {
		self.msg = msg;
		self.msg_type = MsgType::Note;
	}

	pub fn read_input(&mut self) -> String {
		self.msg_type = MsgType::Input;
		self.msg = String::new();
		self.render();
		loop {
			let chr = ncurses::getch();
			if chr == 127 { self.msg.pop(); }
			else if chr == '\n' as i32 {
				break;
			} else {
				self.msg.push(chr as u8 as char);
			}
			self.render();
		}
		return self.msg.clone();
	}
}

pub fn get_width() -> i32 {
	let mut x = 0; let mut y = 0;
	ncurses::getmaxyx(ncurses::stdscr(), &mut y, &mut x);
	x
}

pub fn get_height() -> i32 {
	let mut x = 0; let mut y = 0;
	ncurses::getmaxyx(ncurses::stdscr(), &mut y, &mut x);
	y
}
