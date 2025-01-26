trait ToPagable {
	
    fn to_json(&self) {
		Json(&self)
	}
	
}