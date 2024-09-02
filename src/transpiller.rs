struct BlnString {
    content:String
}
pub enum BlnValue {
    String(BlnString),
    Array(Vec<BlnCode>),
    Object(Vec<(String, BlnCode)>),
}
struct BlnObject {
    name:string
}
pub enum BlnCode {
    Declare(BlnObject)
}