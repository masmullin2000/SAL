pub type Int = u16;
pub type StringType = smallstr::SmallString<[u8; 16]>;
const FRAMEWORK: &str = "Rust (actix";

#[derive(serde::Serialize)]
pub struct User {
    pub(crate) Id: Int,
    pub(crate) Age: Int,
    pub(crate) FirstName: StringType,
    pub(crate) LastName: StringType,
    pub(crate) Framework: &'static str, //String,
}

impl User {
    #[inline(always)]
    pub fn new(id: Int, age: Int, f_name: StringType, l_name: StringType) -> Self {
        User {
            Id: id,
            Age: age,
            FirstName: f_name,
            LastName: l_name,
            Framework: FRAMEWORK,
        }
    }
}
