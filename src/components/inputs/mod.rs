#![allow(unused_imports)]

mod label;
mod list;
mod nested_list;
mod switch;
mod text;
mod textarea;

pub use label::InputLabel;
pub use list::InputList;
pub use nested_list::InputNestedList;
pub use switch::InputSwitch;
pub use text::InputText;
pub use textarea::InputTextarea;

// TODO: Possible other inputs: Select/Dropdown, Checkbox Group, Radio Group
