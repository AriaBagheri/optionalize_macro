use optionalize_macro::Optionalize; // Import the procedural macro from the optionalize_macro crate

/// Test struct to derive `Optionalize`
#[derive(Optionalize, Debug, PartialEq)]
struct TestStruct {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[test]
fn test_optionalize_macro() {
    // Manually define the expected struct with optional fields
    #[derive(Debug, PartialEq)]
    struct TestStructOptional {
        pub id: Option<i32>,
        pub name: Option<String>,
        pub description: Option<String>,
    }

    // Create an instance of the original struct
    let original = TestStruct {
        id: 1,
        name: "example".to_string(),
        description: Some("description".to_string()),
    };

    // Create the expected "optionalized" struct manually
    let expected = TestStructOptional {
        id: Some(1),
        name: Some("example".to_string()),
        description: Some("description".to_string()),
    };

    // The generated struct will be `TestStructOptional`
    let optionalized = TestStructOptional {
        id: Some(original.id),
        name: Some(original.name.clone()),
        description: original.description.clone(),
    };

    // Verify that the generated optionalized struct matches the expected result
    assert_eq!(optionalized, expected);
}