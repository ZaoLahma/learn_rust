pub struct AStruct<'a>
{
    pub a_string: &'a str,
    pub four_bytes: u32,
}

impl<'a> AStruct<'a>
{
    pub fn to_string(&self) -> String
    {
        let str_object = self.a_string.to_owned() + ", " + &self.four_bytes.to_string();
        return str_object;
    }
}