# Presentaton

## Agenda
1. Generator type


## Generator Type
Generator is a "resumable function" that syntactically very close to the closure but compile to something complittly different.  The main feature of **generator** function that it can be suspeded during execution and resumed later from place where it was suspeneded. 
Generatoro usuly use keyword *yield* to suspend execution and resume happen just after *yield*.
*Code example of generator:*
```rust
fn login() -> impl Generator<SignInArgs, Yield = (), Return = LoginResult> + 'static{
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
```