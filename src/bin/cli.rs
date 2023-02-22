use std::{fmt::Display, vec};
use inquire::{error::InquireError, Select};

use quant::{operation::{identity, hadamard, not, cnot, phase, pauli_y, pauli_z, Operation, self}, register::{self, Register}};

enum Choice {
    Show,
    Apply,
}

impl Choice {
    fn choices() -> Vec<Choice> {
        vec![Choice::Show, Choice::Apply]
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Choice::Apply => write!(f, "Apply"),
            Choice::Show => write!(f, "Show"),
        }
    }
}

#[derive(Debug)]
enum OperationType {
    Unary,
    Binary,
}

impl OperationType {
    fn types() -> Vec<OperationType> {
        vec![OperationType::Unary, OperationType::Binary]
    }
}

impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Unary => write!(f, "Unary"),
            OperationType::Binary => write!(f, "Binary"),
        }
    }
}

enum UnaryOperation {
    Identity,
    Hadamard,
    Phase,
    NOT,
    PauliY,
    PauliZ,
}

impl UnaryOperation {
    fn operations() -> Vec<UnaryOperation> {
        vec![
            UnaryOperation::Identity,
            UnaryOperation::Hadamard,
            UnaryOperation::Phase,
            UnaryOperation::NOT,
            UnaryOperation::PauliY,
            UnaryOperation::PauliZ,
        ]
    }
}

impl Display for UnaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperation::Identity => write!(f, "Identity"),
            UnaryOperation::Hadamard => write!(f, "Hadamard"),
            UnaryOperation::Phase => write!(f, "Phase"),
            UnaryOperation::NOT => write!(f, "NOT"),
            UnaryOperation::PauliY => write!(f, "PauliY"),
            UnaryOperation::PauliZ => write!(f, "PauliZ"),
        }
    }
}

enum BinaryOperation {
    CNOT,
    Swap,
}

impl BinaryOperation {
    fn operations() -> Vec<BinaryOperation> {
        vec![BinaryOperation::CNOT, BinaryOperation::Swap]
    }
}

impl Display for BinaryOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperation::CNOT => write!(f, "CNOT"),
            BinaryOperation::Swap => write!(f, "Swap"),
        }
    }
}

/// Prompts the user for an initial choice.
///
/// Choices include:
/// - Applying an operation
/// - Showing the register state
fn initial_prompt() -> Result<Choice, InquireError> {
    let options = Choice::choices();
    Select::new("Select an option: ", options).prompt()
}

/// Prompts the user for an operation type.
///
/// Types include:
/// - Unary
/// - Binary
fn apply_prompt() -> Result<OperationType, InquireError> {
    let options = OperationType::types();
    Select::new("Select an operation type: ", options).prompt()
}

fn unary_prompt() -> Result<UnaryOperation, InquireError> {
    let options = UnaryOperation::operations();
    Select::new("Select an operation: ", options).prompt()
}

fn binary_prompt() -> Result<BinaryOperation, InquireError> {
    let options = BinaryOperation::operations();
    Select::new("Select an operation: ", options).prompt()
}

fn qubit_prompt(n: usize, size: usize) -> Result<Vec<usize>, InquireError> {
    assert!(n < size, "Cannot call operation on more qubits than register size! ({n} > {size}");
    let options: Vec<usize> = (0..size).collect();
    let mut targets: Vec<usize> = Vec::new();
    
    for _ in 0..n {
        let target = Select::new("Select a target index: ", options.clone()).prompt()?;
        targets.push(target);
    }
    
    Ok(targets)
}

fn handle_apply(reg: &mut Register<4>) {
    let op_type = match apply_prompt() {
        Ok(op_type) => op_type,
        Err(e) => panic!("Problem encountered during operation type selection: {:?}", e),
    };
    

    match op_type {
        OperationType::Unary => {
            let op = match unary_prompt() {
                Ok(op) => op,
                Err(e) => panic!("Problem encountered when selecting unary operation: {:?}", e),
            };
            let target = match qubit_prompt(1, 4) {
                Ok(ts) => ts[0],
                Err(e) => panic!("Problem encountered when selecting index: {:?}", e),
            };
            match op {
                UnaryOperation::Identity => reg.apply(&operation::identity(target)),
                UnaryOperation::Hadamard => reg.apply(&operation::hadamard(target)),
                UnaryOperation::Phase => reg.apply(&operation::phase(target)),
                UnaryOperation::NOT => reg.apply(&operation::not(target)),
                UnaryOperation::PauliY => reg.apply(&operation::pauli_y(target)),
                UnaryOperation::PauliZ => reg.apply(&operation::pauli_z(target)),
            };
        },
        OperationType::Binary => {
            let op = match binary_prompt() {
                Ok(op) => op,
                Err(e) => panic!("Problem encountered when selecting binary operation: {:?}", e),
            };
            let targets = match qubit_prompt(2, 4) {
                Ok(ts) => ts,
                Err(e) => panic!("Problem encountered when selecting index: {:?}", e),
            };
            match op {
                BinaryOperation::CNOT => reg.apply(&operation::cnot(targets[0], targets[1])),
                BinaryOperation::Swap => reg.apply(&operation::swap(targets[0], targets[1])),
            };
        }
    }
    
    
    
}

fn main() {
    let mut reg = Register::new([false; 4]);

    let initial = match initial_prompt() {
        Ok(choice) => choice,
        Err(e) => panic!("Problem selecting an option: {:?}", e),
    };

    let result = match initial {
        Choice::Show => reg.print_state(),
        Choice::Apply => handle_apply(&mut reg),
    };

    reg.print_state();
    
    
}
