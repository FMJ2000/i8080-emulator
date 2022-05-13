pub struct Opcode {
  pub index: usize,
  pub opcode: u8,
  pub name: String,
  pub len: usize,
  pub duration: usize,
  pub args: Vec<u8>,
}

impl Opcode {
  pub fn new(buffer: &[u8; 4], index: usize) -> Opcode {
      let opcode = buffer[0];
      let d = (opcode >> 3) & 0b111;
      let s = opcode & 0b111;
      let rp = (opcode >> 4) & 0b11;

      match opcode >> 6 {
          0b00 => {
              match opcode & 0b111 {
                  0b000 => Opcode { index, opcode, name: String::from("NOP"), len: 1, duration: 4, args: vec![] },
                  0b001 => {
                      match (opcode >> 3) & 0b1 {
                          0b0 => Opcode { index, opcode, name: format!("LXI {},#", Opcode::get_reg_pair(rp, false)), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
                          _ => Opcode { index, opcode, name: format!("DAD {}", Opcode::get_reg_pair(rp, false)), len: 1, duration: 10, args: buffer[index+1..index+3].to_vec() },
                      }
                  },
                  0b010 => {
                      match opcode >> 3 {
                          0b000 => Opcode { index, opcode, name: String::from("STAX BC"), len: 1, duration: 7, args: vec![] },
                          0b010 => Opcode { index, opcode, name: String::from("STAX DE"), len: 1, duration: 7, args: vec![] },
                          0b001 => Opcode { index, opcode, name: String::from("LDAX BC"), len: 1, duration: 7, args: vec![] },
                          0b011 => Opcode { index, opcode, name: String::from("LDAX DE"), len: 1, duration: 7, args: vec![] },
                          0b100 => Opcode { index, opcode, name: String::from("SHLD"), len: 3, duration: 16, args: buffer[index+1..index+3].to_vec() },
                          0b101 => Opcode { index, opcode, name: String::from("LHLD"), len: 3, duration: 16, args: buffer[index+1..index+3].to_vec() },
                          0b110 => Opcode { index, opcode, name: String::from("STA"), len: 3, duration: 13, args: buffer[index+1..index+3].to_vec() },
                          _ => Opcode { index, opcode, name: String::from("LDA"), len: 3, duration: 13, args: buffer[index+1..index+3].to_vec() },
                      }
                  },
                  0b011 => {
                      match (opcode >> 3) & 0b1 {
                          0b0 => Opcode { index, opcode, name: format!("INX {}", Opcode::get_reg_pair(rp, false)), len: 1, duration: 5, args: vec![] },
                          _ => Opcode { index, opcode, name: format!("DCX {}", Opcode::get_reg_pair(rp, false)), len: 1, duration: 5, args: vec![] },
                      }
                  },
                  0b100 => Opcode { index, opcode, name: format!("INR {}", Opcode::get_reg(d)), len: 1, duration: 5, args: vec![] },
                  0b101 => Opcode { index, opcode, name: format!("DCR {}", Opcode::get_reg(d)), len: 1, duration: 5, args: vec![] },
                  0b110 => Opcode { index, opcode, name: format!("MVI {},#", Opcode::get_reg(d)), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                  _ => {
                      match opcode >> 3 {
                          0b000 => Opcode { index, opcode, name: String::from("RLC"), len: 1, duration: 4, args: vec![] },
                          0b001 => Opcode { index, opcode, name: String::from("RRC"), len: 1, duration: 4, args: vec![] },
                          0b010 => Opcode { index, opcode, name: String::from("RAL"), len: 1, duration: 4, args: vec![] },
                          0b011 => Opcode { index, opcode, name: String::from("RAR"), len: 1, duration: 4, args: vec![] },
                          0b100 => Opcode { index, opcode, name: String::from("DAA"), len: 1, duration: 4, args: vec![] },
                          0b101 => Opcode { index, opcode, name: String::from("CMA"), len: 1, duration: 4, args: vec![] },
                          0b110 => Opcode { index, opcode, name: String::from("STC"), len: 1, duration: 4, args: vec![] },
                          _ => Opcode { index, opcode, name: String::from("CMC"), len: 1, duration: 4, args: vec![] },
                      }
                  }
              }
          },
          0b01 => {
              match opcode {
                  0b01110110 => Opcode { index, opcode, name: String::from("HLT"), len: 1, duration: 7, args: vec![] },
                  _ => Opcode { index, opcode, name: format!("MOV {},{}", Opcode::get_reg(d), Opcode::get_reg(s)), len: 1, duration: 5, args: vec![] },
              }
          },
          0b10 => {
              match (opcode >> 3) & 0b111 {
                  0b000 =>  Opcode { index, opcode, name: format!("ADD {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b001 => Opcode { index, opcode, name: format!("ADC {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b010 =>  Opcode { index, opcode, name: format!("SUB {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b011 => Opcode { index, opcode, name: format!("SBB {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b100 =>  Opcode { index, opcode, name: format!("ANA {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b101 => Opcode { index, opcode, name: format!("XRA {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  0b110 =>  Opcode { index, opcode, name: format!("ORA {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
                  _ => Opcode { index, opcode, name: format!("CMP {}", Opcode::get_reg(s)), len: 1, duration: 4, args: vec![] },
              }
          },
          _ => {
              match opcode & 0b111 {
                  0b000 =>  Opcode { index, opcode, name: format!("R{}", Opcode::get_cond(d)), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
                  0b001 => {
                      match (opcode >> 3) & 0b111 {
                          0b001 => Opcode { index, opcode, name: String::from("RET"), len: 1, duration: 10, args: vec![] },
                          0b101 => Opcode { index, opcode, name: String::from("PCHL"), len: 1, duration: 5, args: vec![] },
                          0b111 => Opcode { index, opcode, name: String::from("SPHL"), len: 1, duration: 5, args: vec![] },
                          _ => Opcode { index, opcode, name: format!("POP {}", Opcode::get_reg_pair(rp, true)), len: 1, duration: 10, args: vec![] },
                      }
                  }
                  0b010 => Opcode { index, opcode, name: format!("J{}", Opcode::get_cond(d)), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
                  0b011 => {
                      match (opcode >> 3) & 0b111 {
                          0b010 => Opcode { index, opcode, name: String::from("OUT"), len: 2, duration: 10, args: buffer[index+1..index+2].to_vec() },
                          0b011 => Opcode { index, opcode, name: String::from("IN"), len: 2, duration: 10, args: buffer[index+1..index+2].to_vec() },
                          0b100 => Opcode { index, opcode, name: String::from("XTHL"), len: 1, duration: 18, args: vec![] },
                          0b101 => Opcode { index, opcode, name: String::from("XCHG"), len: 1, duration: 5, args: vec![] },
                          0b110 => Opcode { index, opcode, name: String::from("DI"), len: 1, duration: 4, args: vec![] },
                          0b111 => Opcode { index, opcode, name: String::from("EI"), len: 1, duration: 4, args: vec![] },
                          _ => Opcode { index, opcode, name: String::from("JMP"), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
                      }
                  }
                  0b100 => Opcode { index, opcode, name: format!("C{}", Opcode::get_cond(d)), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
                  0b101 => {
                      match (opcode >> 3) & 0b1 {
                          0b0 => Opcode { index, opcode, name: format!("PUSH {}", Opcode::get_reg_pair(rp, true)), len: 1, duration: 11, args: vec![] },
                          _ => Opcode { index, opcode, name: String::from("CALL"), len: 3, duration: 17, args: buffer[index+1..index+3].to_vec() },
                      }
                  }
                  0b110 => {
                      match (opcode >> 3) & 0b111 {
                          0b000 => Opcode { index, opcode, name: String::from("ADI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b001 => Opcode { index, opcode, name: String::from("ACI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b010 => Opcode { index, opcode, name: String::from("SUI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b011 => Opcode { index, opcode, name: String::from("SBI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b100 => Opcode { index, opcode, name: String::from("ANI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b101 => Opcode { index, opcode, name: String::from("XRI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          0b110 => Opcode { index, opcode, name: String::from("ORI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                          _ => Opcode { index, opcode, name: String::from("CPI #"), len: 2, duration: 7, args: buffer[index+1..index+2].to_vec() },
                      }
                  }
                  _ => Opcode { index, opcode, name: format!("RST {:02X}", d), len: 3, duration: 10, args: buffer[index+1..index+3].to_vec() },
              }
          }
      }
  }

  fn get_reg(code: u8) -> char {
      match code {
          0b000 => 'B',
          0b001 => 'C',
          0b010 => 'D',
          0b011 => 'E',
          0b100 => 'H',
          0b101 => 'L',
          0b110 => 'M',
          0b111 => 'A',
          _ => 'X',
      }
  }

  fn get_reg_pair(code: u8, stack: bool) -> &'static str {
      match code {
          0b00 => "BC",
          0b01 => "DE",
          0b10 => "HL",
          0b11 => if stack { "PSW" } else { "SP" },
          _ => "XX",
      }
  }

  fn get_cond(code: u8) -> &'static str {
      match code {
          0b000 => "NZ",
          0b001 => "Z",
          0b010 => "NC",
          0b011 => "C",
          0b100 => "PO",
          0b101 => "PE",
          0b110 => "P",
          0b111 => "M",
          _ => "X",
      }
  }

  pub fn print(&self) {
      let mut args: String = self.args.iter().fold(String::new(), |line, val| format!("{:02X}{}", val, line));
      if args.len() > 0 {
          args = format!("${}", args);
      }
      println!("{:04X}\t{:>8}\t{}", self.index, self.name, args);
  }
}