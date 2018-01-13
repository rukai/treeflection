extern crate treeflection;

use treeflection::{Node, NodeRunner, NodeToken};
use std::fmt::Debug;
use std::{f64, f32};

fn assert_set<T>(mut node: T, set: &str, expected: T) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    assert_eq!(node.node_step(NodeRunner { tokens: tokens }), String::new());
    assert_eq!(expected, node);
}

fn assert_set_output<T>(mut node: T, set: &str, expected: T, expected_output: &str) where T: Node + Debug + PartialEq {
    let tokens = vec!(NodeToken::Set(String::from(set)));
    assert_eq!(node.node_step(NodeRunner { tokens: tokens }), expected_output);
    assert_eq!(expected, node);
}

fn assert_get<T: Node>(mut node: T, expected: &str) {
    let tokens = vec!(NodeToken::Get);
    let result = node.node_step(NodeRunner { tokens: tokens });
    assert_eq!(expected, String::from(result));
}

#[test]
fn int_set() {
    assert_set::<isize>(42, "13", 13);
    assert_set::<i64>(42, "13", 13);
    assert_set::<i32>(42, "13", 13);
    assert_set::<i16>(42, "13", 13);
    assert_set::<i8>(42, "13", 13);

    assert_set::<isize>(42, "-19", -19);
    assert_set::<i64>(42, "-19", -19);
    assert_set::<i32>(42, "-19", -19);
    assert_set::<i16>(42, "-19", -19);
    assert_set::<i8>(42, "-19", -19);

    assert_set::<usize>(42, "13", 13);
    assert_set::<u64>(42, "13", 13);
    assert_set::<u32>(42, "13", 13);
    assert_set::<u16>(42, "13", 13);
    assert_set::<u8>(42, "13", 13);

    assert_set_output::<isize>(42, "invalid", 42, "Invalid value for isize (needs to be: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)");
    assert_set_output::<i64>(42, "invalid", 42, "Invalid value for i64 (needs to be: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)");
    assert_set_output::<i32>(42, "invalid", 42, "Invalid value for i32 (needs to be: A number from –2,147,483,648 to 2,147,483,647)");
    assert_set_output::<i16>(42, "invalid", 42, "Invalid value for i16 (needs to be: A number from –32,768 to –32,767)");
    assert_set_output::<i8>(42, "invalid", 42, "Invalid value for i8 (needs to be: A number from -128 to 127)");

    assert_set_output::<usize>(42, "invalid", 42, "Invalid value for usize (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_set_output::<u64>(42, "invalid", 42, "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_set_output::<u32>(42, "invalid", 42, "Invalid value for u32 (needs to be: A number from 0 to 4,294,967,295)");
    assert_set_output::<u16>(42, "invalid", 42, "Invalid value for u16 (needs to be: A number from 0 to 65,535)");
    assert_set_output::<u8>(42, "invalid", 42, "Invalid value for u8 (needs to be: A number from 0 to 255)");
}

#[test]
fn int_get() {
    assert_get::<isize>(42, "42");
    assert_get::<i64>(42, "42");
    assert_get::<i32>(42, "42");
    assert_get::<i16>(42, "42");
    assert_get::<i8>(42, "42");

    assert_get::<isize>(-4, "-4");
    assert_get::<i64>(-4, "-4");
    assert_get::<i32>(-4, "-4");
    assert_get::<i16>(-4, "-4");
    assert_get::<i8>(-4, "-4");

    assert_get::<usize>(42, "42");
    assert_get::<u64>(42, "42");
    assert_get::<u32>(42, "42");
    assert_get::<u16>(42, "42");
    assert_get::<u8>(42, "42");
}

#[test]
fn copy_from_numeric() {
    // int
    let copy_token = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    let paste_token = NodeRunner { tokens: vec!(NodeToken::PasteTo) };

    let mut a: u8 = 250;
    let mut b: i8 = 13;
    let mut c: u8 = 13;
    let mut d: i64 = 13;
    let mut e: f32 = 13.0;

    assert_eq!(a.node_step(copy_token.clone()), "");
    assert_eq!(a, 250);
    assert_eq!(b.node_step(paste_token.clone()), "");
    assert_eq!(b, -6); // sadly it is too complicated to properly error on this case
    assert_eq!(c.node_step(paste_token.clone()), "");
    assert_eq!(c, 250);
    assert_eq!(d.node_step(paste_token.clone()), "");
    assert_eq!(c, 250);
    assert_eq!(e.node_step(paste_token.clone()), "");
    assert_eq!(e, 250.0);

    // float
    let copy_token = NodeRunner { tokens: vec!(NodeToken::CopyFrom) };
    let paste_token = NodeRunner { tokens: vec!(NodeToken::PasteTo) };

    let mut a: f64 = 13.37;
    let mut b: f32 = 99.9999;
    let mut c: i32 = 0;

    assert_eq!(b.node_step(copy_token.clone()), "");
    assert_eq!(b, 99.9999);
    assert_eq!(c.node_step(paste_token.clone()), "");
    assert_eq!(c, 99);

    assert_eq!(a.node_step(copy_token.clone()), "");
    assert_eq!(a, 13.37);
    assert_eq!(b.node_step(paste_token.clone()), "");
    assert_eq!(b, 13.37);
    assert_eq!(c.node_step(paste_token.clone()), "");
    assert_eq!(c, 13);
}

#[test]
fn numeric_invalid_custom() {
    let invalid_action = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("nothing"), vec!(String::from("4")))) };

    let mut a: u8  = 253;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(invalid_action.clone()), "u8 cannot 'nothing'");
    assert_eq!(a, 253);
    assert_eq!(b.node_step(invalid_action.clone()), "i8 cannot 'nothing'");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(invalid_action.clone()), "u64 cannot 'nothing'");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(invalid_action.clone()), "f32 cannot 'nothing'");
    assert_eq!(d, 13.0);
    assert_eq!(e.node_step(invalid_action.clone()), "f64 cannot 'nothing'");
    assert_eq!(e, 13.0);
}

#[test]
fn numeric_add() {
    let runner      = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("add"), vec!(String::from("4")))) };
    let runner_fail = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("add"), vec!(String::from("4a")))) };

    let mut a: u8  = 253;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(runner_fail.clone()), "Invalid value for u8 (needs to be: A number from 0 to 255)");
    assert_eq!(a, 253);
    assert_eq!(b.node_step(runner_fail.clone()), "Invalid value for i8 (needs to be: A number from -128 to 127)");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(runner_fail.clone()), "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(runner_fail.clone()), "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_eq!(d, 13.0);
    assert_eq!(e.node_step(runner_fail.clone()), "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
    assert_eq!(e, 13.0);

    assert_eq!(a.node_step(runner.clone()), "");
    assert_eq!(a, 255); // test clamp
    assert_eq!(b.node_step(runner.clone()), "");
    assert_eq!(b, 17);
    assert_eq!(c.node_step(runner.clone()), "");
    assert_eq!(c, 17);
    assert_eq!(d.node_step(runner.clone()), "");
    assert_eq!(d, 17.0);
    assert_eq!(e.node_step(runner.clone()), "");
    assert_eq!(e, 17.0);
}

#[test]
fn numeric_sub() {
    let runner      = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("subtract"), vec!(String::from("4")))) };
    let runner_fail = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("subtract"), vec!(String::from("4a")))) };

    let mut a: u8  = 2;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(runner_fail.clone()), "Invalid value for u8 (needs to be: A number from 0 to 255)");
    assert_eq!(a, 2);
    assert_eq!(b.node_step(runner_fail.clone()), "Invalid value for i8 (needs to be: A number from -128 to 127)");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(runner_fail.clone()), "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(runner_fail.clone()), "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_eq!(d, 13.0);
    assert_eq!(e.node_step(runner_fail.clone()), "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
    assert_eq!(e, 13.0);

    assert_eq!(a.node_step(runner.clone()), "");
    assert_eq!(a, 0); // test clamp
    assert_eq!(b.node_step(runner.clone()), "");
    assert_eq!(b, 9);
    assert_eq!(c.node_step(runner.clone()), "");
    assert_eq!(c, 9);
    assert_eq!(d.node_step(runner.clone()), "");
    assert_eq!(d, 9.0);
    assert_eq!(e.node_step(runner.clone()), "");
    assert_eq!(e, 9.0);
}

#[test]
fn numeric_multiply() {
    let runner      = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("multiply"), vec!(String::from("4")))) };
    let runner_fail = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("multiply"), vec!(String::from("4a")))) };

    let mut a: u8  = 100;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(runner_fail.clone()), "Invalid value for u8 (needs to be: A number from 0 to 255)");
    assert_eq!(a, 100);
    assert_eq!(b.node_step(runner_fail.clone()), "Invalid value for i8 (needs to be: A number from -128 to 127)");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(runner_fail.clone()), "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615)");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(runner_fail.clone()), "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_eq!(d, 13.0);
    assert_eq!(e.node_step(runner_fail.clone()), "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
    assert_eq!(e, 13.0);

    assert_eq!(a.node_step(runner.clone()), "");
    assert_eq!(a, 255); // test clamp
    assert_eq!(b.node_step(runner.clone()), "");
    assert_eq!(b, 52);
    assert_eq!(c.node_step(runner.clone()), "");
    assert_eq!(c, 52);
    assert_eq!(d.node_step(runner.clone()), "");
    assert_eq!(d, 52.0);
    assert_eq!(e.node_step(runner.clone()), "");
    assert_eq!(e, 52.0);
}

#[test]
fn numeric_divide() {
    let runner      = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("divide"), vec!(String::from("4")))) };
    let runner_fail = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("divide"), vec!(String::from("4a")))) };
    let runner_0    = NodeRunner { tokens: vec!(NodeToken::Custom(String::from("divide"), vec!(String::from("0")))) };

    let mut a: u8  = 0;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(runner_fail.clone()), "Invalid value for u8 (needs to be: A number from 0 to 255, excluding 0)");
    assert_eq!(a, 0);
    assert_eq!(b.node_step(runner_fail.clone()), "Invalid value for i8 (needs to be: A number from -128 to 127, excluding 0)");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(runner_fail.clone()), "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615, excluding 0)");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(runner_fail.clone()), "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_eq!(d, 13.0);
    assert_eq!(e.node_step(runner_fail.clone()), "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
    assert_eq!(e, 13.0);

    assert_eq!(a.node_step(runner_0.clone()), "Invalid value for u8 (needs to be: A number from 0 to 255, excluding 0)");
    assert_eq!(a, 0);
    assert_eq!(b.node_step(runner_0.clone()), "Invalid value for i8 (needs to be: A number from -128 to 127, excluding 0)");
    assert_eq!(b, 13);
    assert_eq!(c.node_step(runner_0.clone()), "Invalid value for u64 (needs to be: A number from 0 to 18,446,744,073,709,551,615, excluding 0)");
    assert_eq!(c, 13);
    assert_eq!(d.node_step(runner_0.clone()), "");
    assert_eq!(d, f32::INFINITY);
    assert_eq!(e.node_step(runner_0.clone()), "");
    assert_eq!(e, f64::INFINITY);

    let mut a: u8  = 0;
    let mut b: i8  = 13;
    let mut c: u64 = 13;
    let mut d: f32 = 13.0;
    let mut e: f64 = 13.0;

    assert_eq!(a.node_step(runner.clone()), "");
    assert_eq!(a, 0);
    assert_eq!(b.node_step(runner.clone()), "");
    assert_eq!(b, 3);
    assert_eq!(c.node_step(runner.clone()), "");
    assert_eq!(c, 3);
    assert_eq!(d.node_step(runner.clone()), "");
    assert_eq!(d, 3.25);
    assert_eq!(e.node_step(runner.clone()), "");
    assert_eq!(e, 3.25);
}

#[test]
fn int_help() {
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };

    let output = r#"
u8 Help

Valid values: A number from 0 to 255

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: u8 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u16 Help

Valid values: A number from 0 to 65,535

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: u16 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u32 Help

Valid values: A number from 0 to 4,294,967,295

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: u32 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
u64 Help

Valid values: A number from 0 to 18,446,744,073,709,551,615

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: u64 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i8 Help

Valid values: A number from -128 to 127

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: i8 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i16 Help

Valid values: A number from –32,768 to –32,767

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: i16 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i32 Help

Valid values: A number from –2,147,483,648 to 2,147,483,647

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: i32 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
i64 Help

Valid values: A number from –9,223,372,036,854,775,808 to 9,223,372,036,854,775,807

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: i64 = 13;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);
}

#[test]
fn float_set() {
    assert_set::<f32>(49992.12345, "3.141592653589793", 3.141592653589793);
    assert_set::<f64>(49992.12345, "3.141592653589793", 3.141592653589793);

    assert_set_output::<f32>(49992.12345, "invalid", 49992.12345, "Invalid value for f32 (needs to be: A number with a decimal point)");
    assert_set_output::<f64>(49992.12345, "invalid", 49992.12345, "Invalid value for f64 (needs to be: A higher precision number with a decimal point)");
}

#[test]
fn float_get() {
    assert_get::<f32>(6.283185307179586, "6.2831855");
    assert_get::<f64>(6.283185307179586, "6.283185307179586");
}

#[test]
fn float_help() {
    let runner = NodeRunner { tokens: vec!( NodeToken::Help ) };

    let output = r#"
f32 Help

Valid values: A number with a decimal point

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: f32 = 13.37;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);

    let output = r#"
f64 Help

Valid values: A higher precision number with a decimal point

Commands:
*   help             - display this help
*   copy             - copy this value
*   paste            - paste the copied value here
*   get              - display value
*   set      $NUMBER - set to $NUMBER
*   add      $NUMBER - adds $NUMBER to this number
*   subtract $NUMBER - subtracts $NUMBER from this number
*   multiply $NUMBER - multiply this number with $NUMBER
*   divide   $NUMBER - divide this number by $NUMBER"#;
    let mut value: f64 = 13.37;
    assert_eq!(value.node_step(runner.clone()).as_str(), output);
}
