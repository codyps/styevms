//! Note: this does not manage `ip` the same way as the article, and the `ip` could really be
//! dropped entirely.

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Vm {
    accum: u64,
    ip: usize,
}

#[derive(Debug, PartialEq, Eq, ToPrimitive, FromPrimitive)]
pub enum Op {
    Done = 0,
    Inc = 1,
    Dec = 2,
    AddI = 3,
    SubI = 4,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnknownOpcode,
}

impl Vm {
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn next_i(&mut self, bytecode: &mut &[u8]) -> Option<u8> {
        if bytecode.is_empty() {
            None
        } else {
            let a = bytecode[0];
            *bytecode = &bytecode[1..];
            self.ip += 1;
            Some(a)
        }
    }

    pub fn interp(&mut self, mut bytecode: &[u8]) -> Result<(), Error> {
        self.ip = 0;
        loop {
            let insv = self.next_i(&mut bytecode).unwrap();

            match num::FromPrimitive::from_u8(insv) {
                Some(Op::Inc) => self.accum += 1,
                Some(Op::Dec) => self.accum -= 1,
                Some(Op::AddI) => {
                    let arg = self.next_i(&mut bytecode).unwrap();
                    self.accum += arg as u64;
                }
                Some(Op::SubI) => {
                    let arg = self.next_i(&mut bytecode).unwrap();
                    self.accum -= arg as u64;
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

    assert_eq!(
        vm.interp(&[
            Op::Inc.to_u8().unwrap(),
            Op::AddI.to_u8().unwrap(),
            50,
            Op::SubI.to_u8().unwrap(),
            20,
            Op::Dec.to_u8().unwrap(),
            Op::Dec.to_u8().unwrap(),
            Op::Done.to_u8().unwrap(),
        ]),
        Ok(())
    );

    assert_eq!(vm.accum, 1 + 50 - 20 - 1 - 1);
}
