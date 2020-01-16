use eliza::Eliza;

#[test]
fn load_file_ok() {
    assert!(Eliza::from_file("scripts/doctor.json").is_ok());
}

#[test]
fn load_string_ok() {
    assert!(Eliza::from_str(r#"{"greetings": [], "farewells": [], "fallbacks": [], "transforms": [], "synonyms": [], "reflections": [], "keywords": []}"#).is_ok());
}

#[test]
fn load_file_err() {
    assert!(Eliza::from_file("scripts/not_a_script.json").is_err());
}

#[test]
fn convo_textbook() {
    //This is the classic conversation with ELIZA as outlined in the Article:
    //'ELIZA — A computer program for the study of natural language communication between man and
    // machine' by Joseph Weizenbaum 1966
    let mut e = Eliza::from_file("scripts/doctor.json").unwrap();

    assert_eq!("In what way?", e.respond("Men are all alike."));
    assert_eq!(
        "Can you think of a specific example?",
        e.respond("They're always bugging us about something or other.")
    );
    assert_eq!(
        "Your boyfriend made you come here?",
        e.respond("Well, my boyfriend made me come here.")
    );
    assert_eq!(
        "I am sorry to hear you are depressed.",
        e.respond("He says I'm depressed much of the time.")
    );
    assert_eq!(
        "Do you think coming here will help you not to be unhappy?",
        e.respond("It's true. I am unhappy.")
    );
    assert_eq!(
        "What would it mean to you if you got some help?",
        e.respond("I need some help, that much seems certain.")
    );
    assert_eq!(
        "Tell me more about your family.",
        e.respond("Perhaps I could learn to get along with my mother.")
    );
    assert_eq!(
        "Who else in your family takes care of you?",
        e.respond("My mother takes care of me.")
    );
    assert_eq!("Your father?", e.respond("My father."));
    assert_eq!(
        "What resemblance do you see?",
        e.respond("You are like my father in some ways.")
    );
    assert_eq!(
        "What makes you think I am not very aggressive?",
        e.respond("You are not very aggressive but I think you don't want me to notice that.")
    );
    assert_eq!(
        "Why do you think I dont argue with you?",
        e.respond("You don't argue with me.")
    );
    assert_eq!(
        "Does it please you to believe I am afraid of you?",
        e.respond("You are afraid of me.")
    );
    assert_eq!(
        "What else comes to mind when you think of your father?",
        e.respond("My father is afraid of everybody.")
    );
    assert_eq!(
        "Does that have anything to do with the fact that your boyfriend made you come here?",
        e.respond("Bullies.")
    );
}
