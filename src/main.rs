

use std::fs::File;
use std::io;
use std::io::prelude::*;
// Calling convention: R0 R1 R2 R3


fn process_opcode (r: &mut [u8],mem: &mut [u8], cflag: &mut bool, pc: usize) -> usize
{
    //println!("OPCODE: {}",mem[pc]);
    match (mem[pc])
    {
        // mov register, IMM
        0x10..=0x17 =>
        {
            let idx: usize = (mem[pc] & (0x0f)).into();
            let operand = mem[pc+1];
            r[idx] = operand;

            //println!("R{} set to {}",idx,operand);

            pc+2
        }

        // mov register, [mem]
        0x18..=0x1f =>
        {
            let idx: usize = (u8::from(mem[pc]) & (0x0f) - 0x8).into();
            let loc: usize = ( (u16::from(mem[pc+1]) << 8) + u16::from(mem[pc+2])).into();
            r[idx] = mem[loc];

            //println!("R[{}] set to {}",idx,r[idx]);


            pc+3
        }
        // mov [mem], register
        0x20..=0x27 =>
        {
            let loc: usize = ( (u16::from(mem[pc+1]) << 8) + u16::from(mem[pc+2])).into();
            let idx: usize = (mem[pc] & 0x0f).into();
            mem[loc] = r[idx];

            //println!("M[{}] set to {}",loc,mem[loc]);

            pc+3
        }
        // mov register1, register2
        0x30 =>
        {
            let idx1: usize = ( (mem[pc+1] & 0xf0) >> 4 ).into();
            let idx2: usize = (mem[pc+1] & 0x0f).into();

            r[idx1]  = r[idx2];
            pc+2
        }
        // nand register1, register2
        0x31 =>
        {
            let idx1: usize = ( (mem[pc+1] & 0xf0) >> 4 ).into();
            let idx2: usize = (mem[pc+1] & 0x0f).into();
            //println!("NAND of R[{}] and R[{}] = {}",idx1,idx2,!(r[idx1] & r[idx2]));
            r[idx1]  = !(r[idx1] & r[idx2]);
            pc+2
        }
        // cmp register1, register2
        0x32 =>
        {
            let idx1: usize = ( (mem[pc+1] & 0xf0) >> 4 ).into();
            let idx2: usize = (mem[pc+1] & 0x0f).into();
            //println!("NAND of R[{}] and R[{}] = {}",idx1,idx2,!(r[idx1] & r[idx2]));
            if (r[idx1] == r[idx2])
            {
                *cflag = true;
            }
            pc+2
        }
        // Right shift 
        0x40 =>
        {
            
            let idx: usize = ((mem[pc+1] & 0xf0) >> 4 ).into();
            let shift: u8 = mem[pc+1] & 0x0f;
            r[idx] =  r[idx] >> shift;
            pc+2
        }
        // Left shift
        0x41 =>
        {
            let idx: usize = ( (mem[pc+1] & 0xf0) >> 4 ).into();
            let shift: u8 = (mem[pc+1] & 0x0f);
            r[idx] =  r[idx] << shift;
            pc+2
        }
        // Jump zero
        0x50 =>
        {
            let loc: usize = ( (u16::from(mem[pc+1]) << 8) + u16::from(mem[pc+2])).into();
            if (*cflag == true)
            {
                //println!("JNZ passed!");
                *cflag = false;
                pc+3
            }
            else
            {
                //println!("JNZ failed!");
                loc
            }
        }
        // print 
        0x80 =>
        {
            let mut loc: usize = ( (u16::from(mem[pc+1]) << 8) + u16::from(mem[pc+2])).into();


            while mem[loc] != 0
            {
                print!("{}",mem[loc] as char);
                loc += 1;
            }
            println!("");
            pc+3
        }
        // input
        0x81 =>
        {
            let mut loc: usize = ( (u16::from(mem[pc+1]) << 8) + u16::from(mem[pc+2])).into();
            std::io::stdin().read(&mut mem[loc..loc+0x100]).unwrap();
            println!("");
            pc+3
        }
        // NOP
        0x90 => 
        {
            // println!("NOP");
            pc+1
        }
        0x0 =>
        {
            // println!("Program end reached!");
            0
        }
        _ => 
        {
            println!("UNRECOGNIZED OPCODE: {} {}!",mem[pc],mem[pc+1]);
            0
        }
    }
}

fn run_loop(r: &mut [u8],mem: &mut [u8])
{
    let mut pc: usize = 0x100;
    let mut instructions = 0;

    let mut cflag: bool = false;
    loop
    {

        pc = process_opcode(r,mem,&mut cflag,pc);
        instructions += 1;

        if pc == 0
        {
            //println!("We are done here! Instructions executed: {}",instructions);
            return

        }
    }
}
fn main() {
    let mut mem: [u8; 0x10000] = [0; 0x10000];
    let mut r: [u8; 8] = [0; 8];
    let mut f = File::open("encrypt").unwrap();
    f.read(&mut mem[0x100..=0x3000]).unwrap();
    run_loop(&mut r, &mut mem)
    // PC Loop
}
