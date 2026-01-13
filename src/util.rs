pub trait BoxStrOptionExt {
	fn none_if_empty(&self) -> Option<Box<str>>;
}

impl BoxStrOptionExt for Option<Box<str>> {
	fn none_if_empty(&self) -> Option<Box<str>> {
		self.as_ref().and_then(|a| if a.as_ref() == "" { None } else { Some(a.clone()) })
	}
}