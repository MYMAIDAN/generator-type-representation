use rand::Rng;

#[derive(PartialEq, Eq)]
pub struct OtpCode(u16);

impl OtpCode{
    pub fn new() -> Self{
        Self(rand::thread_rng().gen_range(0000..9999))
    }
}


