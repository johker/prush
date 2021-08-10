mod push;

use self::push::instructions::InstructionSet;
use self::push::parser::PushParser;
use self::push::state::PushState;

fn main() {
    let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";

    let mut push_state = PushState::new();
    let mut instruction_set = InstructionSet::new();
    instruction_set.load();

    PushParser::parse_program(&mut push_state, &instruction_set, &input);

    // Push P onto the EXEC stack
}
