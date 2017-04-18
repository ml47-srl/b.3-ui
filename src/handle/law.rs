use app::App;
use libsrl::cell::Cell;

impl App {
	pub fn handle_equals_law(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 1 {
			self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
			return true;
		}
		let result = self.db.equals_law(self.prim_marker.clone(), self.sec_markers[0].clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_equals_law_impl(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 1 {
			self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
			return true;
		}
		let result = self.db.equals_law_impl(self.prim_marker.clone(), self.sec_markers[0].clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_inequal_constants(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.inequal_constants(self.prim_marker.clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_add_eqt(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.add_eqt(self.prim_marker.clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_rm_eqt(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.rm_eqt(self.prim_marker.clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_scope_insertion(&mut self) -> bool {
		let input = self.read_input();
		let cell = match Cell::by_string(&input) {
			Ok(x) => x,
			Err(srl_error) => {
				self.put_error(srl_error.to_string());
				return true;
			}
		};
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.scope_insertion(self.prim_marker.clone(), cell);
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_scope_creation(&mut self) -> bool {
		let mut vec = Vec::new();
		let prim_indices = self.prim_marker.get_indices();
		for sec in self.sec_markers.clone() {
			let mut sec_indices = sec.get_indices();
			for index in prim_indices.clone() {
				if index != sec_indices.remove(0) {
					self.put_error("sec_marker is out of prim_marker".to_string());
					return true;
				}
			}
			vec.push(sec_indices);
		}
		let result = self.db.scope_creation(self.prim_marker.clone(), vec);
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_impl_derivation(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 1 {
			self.put_error(format!("There are {} secondary markers, but only 1 is allowed", len));
			return true;
		}
		let result = self.db.implications_derivation(self.prim_marker.clone(), self.sec_markers[0].clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_scope_exchange(&mut self) -> bool {
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.scope_exchange(self.prim_marker.clone());
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_case_creation(&mut self) -> bool {
		let input = self.read_input();
		let cell = match Cell::by_string(&input) {
			Ok(x) => x,
			Err(srl_error) => {
				self.put_error(srl_error.to_string());
				return true;
			}
		};
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.case_creation(self.prim_marker.clone(), cell);
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}

	pub fn handle_declaration(&mut self) -> bool {
		let input = self.read_input();
		let len = self.sec_markers.len();
		if len != 0 {
			self.put_error(format!("There are {} secondary markers, but none allowed", len));
			return true;
		}
		let result = self.db.declaration(self.prim_marker.clone(), &input);
		match result {
			Ok(_) => {},
			Err(srl_error) => self.put_error(srl_error.to_string())
		}
		return true;
	}
}
