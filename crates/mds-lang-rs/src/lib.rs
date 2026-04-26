pub const MODULE_BEGIN: &str = "// mds:begin generated modules";
pub const MODULE_END: &str = "// mds:end generated modules";

pub fn module_block(modules: &[String]) -> String {
    let mut out = String::from(MODULE_BEGIN);
    out.push('\n');
    for module in modules {
        out.push_str("pub mod ");
        out.push_str(module);
        out.push_str(";\n");
    }
    out.push_str(MODULE_END);
    out.push('\n');
    out
}
