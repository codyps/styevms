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

    pub fn interp(&mut self, bytecode: &[u8]) -> Result<(), Error> {
        self.ip = 0;
        loop {
            let insv = bytecode[self.ip];
            self.ip += 1;

            match num::FromPrimitive::from_u8(insv) {
                Some(Op::Inc) => self.accum += 1,
                Some(Op::Dec) => self.accum -= 1,
                Some(Op::AddI) => {
                    let arg = bytecode[self.ip];
                    self.ip += 1;
                    self.accum += arg as u64;
                }
                Some(Op::SubI) => {
                    let arg = bytecode[self.ip];
                    self.ip += 1;
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
