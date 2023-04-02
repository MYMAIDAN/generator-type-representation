#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;
mod otp_code;

pub enum SignInArgs{
    SEND_OTP(PhoneNumber),
    RESEND,
    VALIDATE(otp_code::OtpCode)
}

pub enum LoginResult{
    SUCCESS,
    WRONG_ARGS
}
#[derive(Clone)]
pub struct PhoneNumber(String);

pub fn send_otp(phone_number: &PhoneNumber, otp: &otp_code::OtpCode) -> bool{
    true
}


fn sign_in() -> impl Generator<SignInArgs, Yield = (), Return = LoginResult> + 'static{
    |args: SignInArgs| {
        let phone_number = match args {
            SignInArgs::SEND_OTP(phone_number) => phone_number.clone(),
            _ => return LoginResult::WRONG_ARGS
            
        }; 
        let mut otp = otp_code::OtpCode::new();
        send_otp(&phone_number, &otp);

        loop{
            let cmd = yield;
            match cmd{
               SignInArgs::RESEND => {
                otp = otp_code::OtpCode::new();
                send_otp(&phone_number, &otp);
               },
               SignInArgs::VALIDATE(recv_otp) => {
                    if recv_otp == otp{
                        return LoginResult::SUCCESS;
                    }else{
                        return LoginResult::WRONG_ARGS;
                    }
               },
               _ => return LoginResult::WRONG_ARGS
            }
        }
    }

}

fn main() {

    let phone_number = PhoneNumber("+380688857281".to_owned());
    let mut login = sign_in();

    match Pin::new(&mut login).resume(SignInArgs::SEND_OTP(phone_number.clone())) {
        GeneratorState::Yielded(()) => {}
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut login).resume(SignInArgs::RESEND) {
        GeneratorState::Complete(state) => {}
        _ => panic!("unexpected value from resume"),
    }
}