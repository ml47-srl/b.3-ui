use app::App;
use libsrl::navi::CellID;

impl App {
	pub fn handle_left(&mut self) -> bool {
		match self.prim_marker.get_left_sibling() {
			Ok(x) => self.prim_marker = x,
			Err(_) => {}
		}
		return true;
	}

	pub fn handle_right(&mut self) -> bool {
		let new = match self.prim_marker.get_right_sibling() {
			Ok(x) => x,
			Err(_) => return true
		};
		if new.get_path(&self.db.get_rules()).is_ok() {
			self.prim_marker = new;
		}
		return true;
	}

	pub fn handle_in(&mut self) -> bool {
		let new = match self.prim_marker.get_child(0) {
			Ok(x) => x,
			Err(_) => return true
		};
		if new.get_path(&self.db.get_rules()).is_ok() {
			self.prim_marker = new;
		}
		return true;
	}

	pub fn handle_out(&mut self) -> bool {
		match self.prim_marker.get_parent() {
			Ok(x) => self.prim_marker = x,
			Err(_) => {}
		}
		return true;
	}


	pub fn handle_up(&mut self) -> bool {
		if self.prim_marker.get_rule_id() == 0 {
			return true;
		}
		self.prim_marker = CellID::create(self.prim_marker.get_rule_id() - 1, self.prim_marker.get_indices());
		while self.prim_marker.get_path(&self.db.get_rules()).is_err() {
			self.prim_marker = self.prim_marker.get_parent().unwrap();
		}
		return true;
	}

	pub fn handle_down(&mut self) -> bool {
		if self.prim_marker.get_rule_id() == self.db.count_rules() - 1 {
			return true;
		}
		self.prim_marker = CellID::create(self.prim_marker.get_rule_id() + 1, self.prim_marker.get_indices());
		while self.prim_marker.get_path(&self.db.get_rules()).is_err() {
			self.prim_marker = self.prim_marker.get_parent().unwrap();
		}
		return true;
	}

	pub fn handle_delete(&mut self) -> bool {
		let rule_id = self.prim_marker.get_rule_id();
		match self.db.delete_rule(rule_id) {
			Ok(_) => {
				let old_sec_markers = self.sec_markers.clone();
				self.sec_markers = Vec::new();
				for old_sec in old_sec_markers {
					let old_id = old_sec.get_rule_id();
					if old_id < rule_id {
						self.sec_markers.push(old_sec);
					} else if old_id > rule_id {
						let new = CellID::create(old_sec.get_rule_id() - 1, old_sec.get_indices());
						self.sec_markers.push(new);
					} // else: old_id == rule_id => sec has to be deleted (or just not added back again)
				}
				self.handle_up(); // for not having problems, when deleting rule at the bottom
			}
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_clear_sec_markers(&mut self) -> bool {
		self.sec_markers = Vec::new();
		return true;
	}

	pub fn handle_toggle_sec_marker(&mut self) -> bool {
		if self.sec_markers.contains(&self.prim_marker) {
			let prim = self.prim_marker.clone();
			self.sec_markers.retain(|x : &CellID| *x != prim);
		} else {
			self.sec_markers.push(self.prim_marker.clone());
		}
		return true;
	}
}
