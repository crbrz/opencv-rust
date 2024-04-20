use std::borrow::Cow;

use crate::function::Function;
use crate::type_ref::{ExternDir, Nullability};
use crate::{IteratorExt, NameStyle};

use super::element::RustElement;
use super::type_ref::{NullabilityExt, TypeRefExt};

impl RustElement for Function<'_, '_> {
	fn rust_module(&self) -> Cow<str> {
		"<unset>".into()
	}

	fn rust_name(&self, style: NameStyle) -> Cow<str> {
		let ret = self.return_type();
		let args = self.arguments();
		if let Some(userdata_idx) = args.iter().position(|a| a.is_user_data()) {
			let args = args
				.into_iter()
				.enumerate()
				.filter(|(idx, _)| *idx != userdata_idx)
				.map(|(_, arg)| arg.type_ref().rust_extern(ExternDir::Contained).into_owned())
				.join(", ");
			Nullability::Nullable.rust_wrap_nullable_decl(
				format!(
					"Box{fish}<dyn FnMut({args}) -> {ret} + Send + Sync + 'static>",
					fish = style.rust_turbo_fish_qual(),
					ret = ret.rust_extern(ExternDir::Contained),
				)
				.into(),
				style,
			)
		} else {
			self.rust_extern()
		}
	}

	fn rendered_doc_comment(&self, _comment_marker: &str, _opencv_version: &str) -> String {
		"".to_string()
	}
}

pub trait FunctionExt<'tu, 'ge> {
	fn rust_extern(&self) -> Cow<str>;
}

impl<'tu, 'ge> FunctionExt<'tu, 'ge> for Function<'tu, 'ge> {
	fn rust_extern(&self) -> Cow<str> {
		let args = self
			.arguments()
			.into_iter()
			.map(|a| a.type_ref().rust_extern(ExternDir::Contained).into_owned())
			.join(", ");
		let ret = self.return_type();
		Nullability::Nullable.rust_wrap_nullable_decl(
			format!(
				r#"unsafe extern "C" fn({args}) -> {ret}"#,
				args = args,
				ret = ret.rust_extern(ExternDir::Contained)
			)
			.into(),
			NameStyle::decl(),
		)
	}
}
