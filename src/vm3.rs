//! Note: this does not manage `ip` the same way as the article, and the `ip` could really be
//! dropped entirely.

const STACK_MAX: usize = 256;

pub struct Vm {
    ip: usize,

    stack: [u64;STACK_MAX],
    stack_top: usize,

    result: u64,
}

#[derive(Debug, PartialEq, Eq)]
#[derive(ToPrimitive,FromPrimitive)]
pub enum Op {
    Done = 0,
    PushI = 1,
    Add = 2,
    Sub = 3,
    Div = 4,
    Mul = 5,
    PopRes = 6,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode,
    DivByZero,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            ip: 0,
            stack: [0u64;256],
            stack_top: 0,
            result: 0,
        }
    }
}

impl Vm {
    pub fn reset(&mut self)
    {
        *self = Self::default();
    }

    fn stack_push(&mut self, v: u64)
    {
        self.stack[self.stack_top] = v;
        self.stack_top += 1;
    }

    fn stack_pop(&mut self) -> u64
    {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    fn next_i(&mut self, bytecode: &mut &[u8]) -> Option<u8>
    {
        if bytecode.len() == 0 {
            None
        } else {
            let a = bytecode[0];
            *bytecode = &bytecode[1..];
            self.ip += 1;
            Some(a)
        }
    }

    pub fn interp(&mut self, mut bytecode: &[u8])
        -> Result<(), Error>
    {
        self.reset();

        loop {
            let insv = self.next_i(&mut bytecode).unwrap();

            match num::FromPrimitive::from_u8(insv) {
                Some(Op::PushI) => {
                    let arg = self.next_i(&mut bytecode).unwrap();
                    self.stack_push(arg as u64); 
                }
                Some(Op::Add) => {
                    let ar = self.stack_pop();
                    let al = self.stack_pop();
                    let r = al + ar;
                    self.stack_push(r);
                }
                Some(Op::Sub) => {
                    let ar = self.stack_pop();
                    let al = self.stack_pop();
                    let r = al - ar;
                    self.stack_push(r);
                }
                Some(Op::Div) => {
                    let ar = self.stack_pop();
                    let al = self.stack_pop();
                    if ar == 0 {
                        return Err(Error::DivByZero);
                    }
                    let r = al / ar;
                    self.stack_push(r);
                }
                Some(Op::Mul) => {
                    let ar = self.stack_pop();
                    let al = self.stack_pop();
                    let r = al * ar;
                    self.stack_push(r);
                }
                Some(Op::PopRes) => {
                   self.result = self.stack_pop();
                }
                Some(Op::Done) => break,
                None => return Err(Error::UnknownOpcode),
            }
        }

        Ok(())
    }
}

#[test]
fn t1() {
    use num::ToPrimitive;
    let mut vm = Vm::default();

    assert_eq!(vm.interp(&[
        Op::PushI.to_u8().unwrap(), 40,
        Op::PushI.to_u8().unwrap(), 5,
        Op::Mul.to_u8().unwrap(),
        Op::PushI.to_u8().unwrap(), 4,
        Op::Div.to_u8().unwrap(),
        Op::PushI.to_u8().unwrap(), 90,
        Op::Add.to_u8().unwrap(),
        Op::PushI.to_u8().unwrap(), 30,
        Op::Sub.to_u8().unwrap(),
        Op::PopRes.to_u8().unwrap(),
        Op::Done.to_u8().unwrap(),
    ]), Ok(()));

    assert_eq!(vm.result, (((40 * 5) / 4) + 90 - 30));
}
