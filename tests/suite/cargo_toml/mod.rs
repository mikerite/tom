//mod manipulator;
mod cargo_toml;

//use self::manipulator::{CargoTomlManipulator, Dependency, DependencySource};
use testutils::assert_eq_text;
use self::cargo_toml::CargoToml;

#[test]
fn adding_dependency_to_table() {
    check_cargo_toml_edit(
        r#"
[package]
name = "tom"

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"
"#,
        r#"
[package]
name = "tom"

[dependencies]
lalrpop-util = "0.15"
regex = "0.2"
pest = "1.0"
"#,
        |toml| toml.add_dependency("pest", "1.0"),
    );
}

//#[test]
//fn adding_dependency_no_table() {
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//pest = "1.0"
//"#,
//        |toml| toml.add_dependency("pest", "1.0"),
//    );
//}
//
//#[test]
//fn adding_dependency_no_table_bin_section() {
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[bin]
//name = "baz"
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//pest = "1.0"
//
//[bin]
//name = "baz"
//"#,
//        |toml| toml.add_dependency("pest", "1.0"),
//    )
//}
//
//#[test]
//fn adding_two_dependencies() {
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//[dependencies]
//"#,
//        r#"
//[package]
//name = "tom"
//[dependencies]
//regex = "1.0"
//pest = "1.0"
//"#,
//        |toml| {
//            toml.add_dependency("regex", "1.0");
//            toml.add_dependency("pest", "1.0");
//        },
//    );
//}
//
//
//#[test]
//fn updating_dependency() {
//    check_cargo_toml_edit(
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//"#,
//        r#"
//[package]
//name = "tom"
//
//[dependencies]
//regex = "1.0"
//"#,
//        |toml| {
//            toml.update_dependency(Dependency {
//                name: "regex".to_string(),
//                source: DependencySource::Version("1.0".to_string()),
//                optional: false,
//            })
//        },
//    );
//}
//

fn check_cargo_toml_edit(
    before: &str,
    after: &str,
    edit: impl FnOnce(&mut CargoToml),
) {
    let mut cargo_toml = CargoToml::new(before);
    edit(&mut cargo_toml);
    let actual = cargo_toml.finish();
    assert_eq_text(after, &actual);
}

