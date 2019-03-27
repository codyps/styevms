
#[derive(Debug,PartialEq,Eq)]
pub struct Vm {
    accum: u64,
    ip: usize,
}

#[derive(Debug,PartialEq,Eq,ToPrimitive,FromPrimitive)]
pub enum Op {
    Done = 0,
    Inc = 1,
    Dec = 2,
}

#[derive(Debug,PartialEq,Eq)]
pub enum Error {
    UnknownOpcode,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            accum: 0,
            ip: 0,
        }
    }
}

impl Vm {
    pub fn reset(&mut self)
    {
        *self = Self::default();
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
        self.ip = 0;
        loop {
            let insv = self.next_i(&mut bytecode).unwrap();

            match num::FromPrimitive::from_u8(insv) {
                Some(Op::Inc) => self.accum += 1,
                Some(Op::Dec) => self.accum -= 1,
                Some(Op::Done) => break,
                None => return Err(Error::UnknownOpcode),
            }
        }

        Ok(())
    }
}

#[test]
fn t1() {
    let mut vm = Vm::default();

    let v: Vec<u8> = [
        Op::Inc,
        Op::Dec,
        Op::Inc,
        Op::Done,
    ].into_iter().map(|x| num::ToPrimitive::to_u8(x).unwrap()).collect();
    vm.interp(&v[..]).unwrap();

    assert_eq!(vm.accum, 1 + 1 - 1);
}
