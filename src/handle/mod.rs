use app::App;
use keys;
extern crate libsrl;
mod control;
mod law;

impl App {
	// returns whether to go on
	pub fn handle_key(&mut self, key : i32) -> bool {
		return match key {
			keys::QUIT => return false,
			keys::LEFT => self.handle_left(),
			keys::RIGHT => self.handle_right(),
			keys::IN => self.handle_in(),
			keys::OUT => self.handle_out(),
			keys::UP => self.handle_up(),
			keys::DOWN => self.handle_down(),
			keys::DELETE => self.handle_delete(),
			keys::EQUALS_LAW => self.handle_equals_law(),
			keys::EQUALS_LAW_IMPL => self.handle_equals_law_impl(),
			keys::INEQUAL_CONSTANTS => self.handle_inequal_constants(),
			keys::ADD_EQT => self.handle_add_eqt(),
			keys::RM_EQT => self.handle_rm_eqt(),
			keys::SCOPE_INSERTION => self.handle_scope_insertion(),
			keys::SCOPE_CREATION => self.handle_scope_creation(),
			keys::IMPL_DERIVATION => self.handle_impl_derivation(),
			keys::SCOPE_EXCHANGE => self.handle_scope_exchange(),
			keys::CASE_CREATION => self.handle_case_creation(),
			keys::DECLARATION => self.handle_declaration(),
			keys::CLEAR_SEC_MARKERS => self.handle_clear_sec_markers(),
			keys::TOGGLE_SEC_MARKER => self.handle_toggle_sec_marker(),
			_ => { self.put_note(format!("unknown key {}", key)); true }
		};
	}
}

