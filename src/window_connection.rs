// Copyright Jeron A. Lau 2017-2018.
// Dual-licensed under either the MIT License or the Boost Software License,
// Version 1.0.  (See accompanying file LICENSE_1_0.txt or copy at
// https://www.boost.org/LICENSE_1_0.txt)

use c_void;

/// Connection is listed first, then window.
#[allow(unused)] #[derive(Clone)]
pub enum WindowConnection {
	/// XCB Window Handles
	Xcb(*mut c_void, u32),
	/// Wayland Window Handles
	Wayland,
	/// DirectFB Window Handles
	DirectFB,
	/// Windows Window Handles
	Windows(*mut c_void, *mut c_void),
	/// Android Window Handles
	Android,
	/// IOS Window Handles
	IOS,
	/// Aldaron's OS Window Handles
	AldaronsOS,
	/// Arduino Window Handles
	Arduino,
	/// Switch Window Handles
	Switch,
	/// Web Window Handles
	Web,
	/// No OS Window Handles
	NoOS,
}
