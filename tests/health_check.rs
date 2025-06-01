#[test]
fn dummy_fail() {
    let result: Result<&str, &str> = Err("The site crashed due to an error");
    claims::assert_err!(result);
}
