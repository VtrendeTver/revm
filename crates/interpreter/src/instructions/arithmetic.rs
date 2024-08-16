use super::i256::{i256_div, i256_mod};
use crate::{
    gas,
    primitives::{Spec, U256},
    Host, Interpreter,
};

pub fn add<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.wrapping_add(*op2);
}

pub fn mul<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.wrapping_mul(*op2);
}

pub fn sub<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::VERYLOW);
    pop_top!(interpreter, op1, op2);
    *op2 = op1.wrapping_sub(*op2);
}

pub fn div<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, op1, op2);
    *op2 = if *op2 != U256::ZERO {
        op1.wrapping_div(*op2)
    } else {
        U256::MAX // Handle division by zero
    };
}

pub fn sdiv<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, op1, op2);
    *op2 = i256_div(op1, *op2);
}

pub fn rem<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, op1, op2);
    *op2 = if *op2 != U256::ZERO {
        op1.wrapping_rem(*op2)
    } else {
        U256::ZERO // Handle division by zero
    };
}

pub fn smod<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, op1, op2);
    *op2 = i256_mod(op1, *op2);
}

pub fn addmod<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::MID);
    pop_top!(interpreter, op1, op2, op3);
    *op3 = op1.add_mod(op2, *op3);
}

pub fn mulmod<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::MID);
    pop_top!(interpreter, op1, op2, op3);
    *op3 = op1.mul_mod(op2, *op3);
}

pub fn exp<H: Host + ?Sized, SPEC: Spec>(interpreter: &mut Interpreter, _host: &mut H) {
    pop_top!(interpreter, op1, op2);
    gas_or_fail!(interpreter, gas::exp_cost(SPEC::SPEC_ID, *op2));
    *op2 = op1.pow(*op2);
}

pub fn signextend<H: Host + ?Sized>(interpreter: &mut Interpreter, _host: &mut H) {
    gas!(interpreter, gas::LOW);
    pop_top!(interpreter, ext, x);
    if ext < U256::from(32) { // Corrected the condition to handle all cases
        let ext = ext.as_limbs()[0];
        let bit_index = (8 * ext + 7) as usize;
        let bit = x.bit(bit_index);
        let mask = (U256::from(1) << bit_index) - U256::from(1);
        *x = if bit { *x | !mask } else { *x & mask };
    }
}
