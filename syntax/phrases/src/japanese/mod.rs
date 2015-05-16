
pub use self::greetings::hello; // brings it to this modue's scope
pub use self::farewells::goodbye; // allows us to do japanese::goodbye()

// alternatively we could say pub use self::greetings to put everything
// in greetings into public scope

// self can be seen as '.' ; super can be seen as '..'

// outside of use, paths are relative. foo::bar() refers to where we are now
//  if it's ::foo::bar - refers to an absolute path from your crate root
mod greetings;
mod farewells;