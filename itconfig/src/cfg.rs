config! {
    #![config(unwrap)]

    TEST => "hello",
    static MAIN => "main",
    TEST2 => "test",

    CONCAT < (
        TESTTTT => "hellooooooo",
        " ",
        "world",
    ),

    NAMESPACE {
        TEST: String => "test",

        NAMESPACE {
            TEST: &'static str => "test",

            #[env_prefix = "HELLO_"]
            NAMESPACE {
                TEST: &'static str => "test",

                #[cfg(not(target_os = "linux"))]
                #[env_prefix = "WORLD_"]
                NAMESPACE {
                    #[env_name = "TEST_TEST_TEST"]
                    TEST: &'static str => "test",
                }
            }
        }
    }
}
