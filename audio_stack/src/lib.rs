use audio_graph::{stack, AudioGraph, Sample};
use audio_modules::*;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

macro_rules! connect {
    ( $ops:ident, $class:ident, $($rest:tt)* ) => { $ops.push(stack::Op::Connect(Box::new($class::new($($rest)*)))) };
}

#[derive(Debug)]
pub enum ParseError {
    UnknownToken(String),
    NotEnoughParameters(String),
    WrongParameterType(String),
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    StackError(stack::Error),
}

pub fn parse_ops(s: &str, channels: u8, sample_rate: u32) -> Result<Vec<stack::Op>, ParseError> {
    let mut ops = Vec::new();
    let s = s.replace(|c| c == '[' || c == ']' || c == ',', " ");
    for token in s
        .split_terminator('\n')
        .flat_map(|s| s.splitn(2, "//").take(1).flat_map(|s| s.split_whitespace()))
    {
        match token {
            "*" => connect!(ops, Fn2, channels, pure::mul),
            "+" => connect!(ops, Fn2, channels, pure::add),
            "-" => connect!(ops, Fn2, channels, pure::sub),
            "/" => connect!(ops, Fn2, channels, pure::div),
            "\\" => connect!(ops, Fn1, channels, pure::recip),
            "^" | "pow" => connect!(ops, Fn2, channels, pure::pow),
            "cheb2" => connect!(ops, Fn1, channels, pure::cheb2),
            "cheb3" => connect!(ops, Fn1, channels, pure::cheb3),
            "cheb4" => connect!(ops, Fn1, channels, pure::cheb4),
            "cheb5" => connect!(ops, Fn1, channels, pure::cheb5),
            "cheb6" => connect!(ops, Fn1, channels, pure::cheb6),
            "cos" => connect!(ops, Fn1, channels, pure::cos),
            "dm" | "dmetro" => connect!(ops, DMetro, channels, sample_rate),
            "dmh" | "dmetro_hold" => connect!(ops, DMetroHold, channels, sample_rate),
            "h" | "bqhpf" => connect!(ops, BiQuad, channels, sample_rate, make_hpf_coefficients),
            "hpf" => connect!(ops, HPF, channels, sample_rate),
            "impulse" => {
                use envelopes::Impulse;
                connect!(ops, Impulse, channels, sample_rate)
            }
            "in" | "input" => connect!(ops, Input, channels),
            "l" | "bqlpf" => connect!(ops, BiQuad, channels, sample_rate, make_lpf_coefficients),
            "lpf" => connect!(ops, LPF, channels, sample_rate),
            "m" | "metro" => connect!(ops, Metro, channels, sample_rate),
            "m2f" | "midi2freq" => connect!(ops, Fn1, channels, pure::midi2freq),
            "mh" | "metro_hold" => connect!(ops, MetroHold, channels, sample_rate),
            "n" | "noise" => connect!(ops, WhiteNoise, channels),
            "p" | "pulse" => connect!(ops, Pulse, channels, sample_rate),
            "pan1" => connect!(ops, Pan1, channels),
            "pan2" => connect!(ops, Pan2, channels),
            "q" | "quantize" => connect!(ops, Fn2, channels, pure::quantize),
            "r" | "range" => connect!(ops, Fn3, channels, pure::range),
            "round" => connect!(ops, Fn1, channels, pure::round),
            "s" => connect!(ops, Osc, channels, sample_rate, pure::sine),
            "saw" => connect!(ops, Phasor0, channels, sample_rate),
            "sh" | "sample&hold" => connect!(ops, SampleAndHold, channels),
            "sin" => connect!(ops, Fn1, channels, pure::sin),
            "sine" => connect!(ops, OscPhase, channels, sample_rate, pure::sine),
            "spectral_shuffle" => {
                let mut rng = Box::new(SmallRng::from_entropy());
                connect!(
                    ops,
                    SpectralTransform,
                    channels,
                    2048,
                    64,
                    Box::new(move |freqs| freqs.shuffle(&mut rng)),
                )
            }
            "t" => connect!(ops, Osc, channels, sample_rate, pure::triangle),
            "tri" => connect!(ops, OscPhase, channels, sample_rate, pure::triangle),
            "unit" => connect!(ops, Fn1, channels, pure::unit),
            "w" => connect!(ops, Phasor, channels, sample_rate),
            // TODO parametrize
            "yin" => connect!(ops, Yin, channels, sample_rate, 1024, 64, 0.2),
            "zip" => connect!(ops, Zip, channels),
            "dup" => ops.push(stack::Op::Dup),
            "swap" => ops.push(stack::Op::Swap),
            "rot" => ops.push(stack::Op::Rot),
            "pop" => ops.push(stack::Op::Pop),
            _ => match token.parse::<Sample>() {
                Ok(x) => connect!(ops, Constant, channels, x),
                Err(_) => {
                    let subcmd = token.split(':').collect::<Vec<_>>();
                    match subcmd[0] {
                        "delay" => match subcmd.get(1) {
                            Some(x) => match x.parse::<f64>() {
                                Ok(max_delay) => {
                                    connect!(ops, Delay, channels, sample_rate, max_delay)
                                }
                                Err(_) => {
                                    return Err(ParseError::WrongParameterType(token.to_string()));
                                }
                            },
                            None => connect!(ops, Delay, channels, sample_rate, 60.0),
                        },
                        "fb" | "feedback" => match subcmd.get(1) {
                            Some(x) => match x.parse::<f64>() {
                                Ok(max_delay) => {
                                    connect!(ops, Feedback, channels, sample_rate, max_delay)
                                }
                                Err(_) => {
                                    return Err(ParseError::WrongParameterType(token.to_string()));
                                }
                            },
                            None => connect!(ops, Feedback, channels, sample_rate, 60.0),
                        },
                        "conv" => match subcmd.get(1) {
                            Some(x) => match x.parse::<usize>() {
                                Ok(window_size) => {
                                    connect!(ops, Convolution, channels, window_size)
                                }
                                Err(_) => {
                                    return Err(ParseError::WrongParameterType(token.to_string()));
                                }
                            },
                            None => return Err(ParseError::NotEnoughParameters(token.to_string())),
                        },
                        "convm" => match subcmd.get(1) {
                            Some(x) => match x.parse::<usize>() {
                                Ok(window_size) => {
                                    connect!(ops, ConvolutionM, channels, window_size)
                                }
                                Err(_) => {
                                    return Err(ParseError::WrongParameterType(token.to_string()));
                                }
                            },
                            None => return Err(ParseError::NotEnoughParameters(token.to_string())),
                        },
                        "param" => match subcmd.get(1) {
                            Some(x) => match x.parse::<u8>() {
                                Ok(index) => connect!(ops, Parameter, channels, index),
                                Err(_) => {
                                    return Err(ParseError::WrongParameterType(token.to_string()));
                                }
                            },
                            None => return Err(ParseError::NotEnoughParameters(token.to_string())),
                        },
                        _ => return Err(ParseError::UnknownToken(token.to_string())),
                    }
                }
            },
        }
    }
    Ok(ops)
}

pub fn parse_graph(
    s: &str,
    channels: u8,
    sample_rate: u32,
    inputs: u8,
) -> Result<AudioGraph, Error> {
    match parse_ops(s, channels, sample_rate) {
        Ok(ops) => stack::build_graph(ops, channels, inputs).or_else(|e| Err(Error::StackError(e))),
        Err(e) => Err(Error::ParseError(e)),
    }
}
