use proptest::prelude::*;
use quant::register::Register;
use quant::operation;

#[test]
#[ignore = "Wait for feature confirmation"]
fn measure_on_zero_state_gives_false() {
    let mut register = Register::new([false]);
    let input = register.measure(0);
    let expected = false;

    assert_eq!(input, expected);
}

proptest!(
    #[test]
    fn measure_four_qubits_gives_same_result(
        state in any::<bool>(),
        state2 in any::<bool>(),
        state3 in any::<bool>(),
        state4 in any::<bool>()
    ) {
        let mut register = Register::new([state, state2, state3, state4]);
        //                                  q0     q1       q2     q3
        let input = [
            register.measure(0),
            register.measure(1),
            register.measure(2),
            register.measure(3),
        ];
        let expected = [state, state2, state3, state4];
        assert_eq!(input, expected);
    }

    #[test]
    fn measure_eight_qubits_gives_same_result(
        state in any::<bool>(),
        state2 in any::<bool>(),
        state3 in any::<bool>(),
        state4 in any::<bool>(),
        state5 in any::<bool>(),
        state6 in any::<bool>(),
        state7 in any::<bool>(),
        state8 in any::<bool>()
    ) {
        let mut register = Register::new([state, state2, state3, state4, state5, state6, state7, state8]);
        let input = [
            register.measure(0),
            register.measure(1),
            register.measure(2),
            register.measure(3),
            register.measure(4),
            register.measure(5),
            register.measure(6),
            register.measure(7),
        ];
        let expected = [state, state2, state3, state4, state5, state6, state7, state8];
        assert_eq!(input, expected);
    }

    //proptest that generates a abitratily long list of bools and checks if all are true
    #[test]
    fn measure_arbitrary_qubits_gives_same_result(
        states in proptest::collection::vec(any::<bool>(), 1..20)
    ) {
        // let mut register = Register::new(states.clone());
        // let mut input = Vec::new();
        // for i in 0..states.len() {
            // input.push(register.measure(i));
        // }
        let not_states = states.iter().map(|b| !b).collect::<Vec<bool>>();
        assert_eq!(not_states, states);
    }

    #[test]
    fn hadamard_hadamard_retains_original_state(i in 0..3) {
        let mut reg = Register::new([false,false,false]);
        let expected = reg.clone();

        let hadamard = operation::hadamard(i as usize);
        let input = reg.apply(&hadamard).apply(&hadamard);
        
        assert_eq!(*input, expected);
    }
);
