use core::ops::{Deref, DerefMut};

use iron::{drivers::generics::antenna::*, math::useful_functions::abs};
use ufmt::uDebug;

#[derive(Clone, SendWithAntenna)]
pub struct Command
{
    pub move_direction: [f32; 2]
}

impl uDebug for Command
{
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
        where W: ufmt::uWrite + ?Sized 
    {
        f.debug_struct("Command")?
            .field("move_direction", &[Debugf32(self.move_direction[0]), Debugf32(self.move_direction[1])])?
            .finish()
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Debugf32(pub f32);
impl Deref for Debugf32
{
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Debugf32
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl uDebug for Debugf32
{
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
        where W: ufmt::uWrite + ?Sized 
    {
        let mut current = self.0;
        if current.is_sign_negative() {
            f.write_char('-')?;
            current = -current;
        }

        if current < 1.0 {
            f.write_str("0.")?;
        }
        else {
            let integer_part = abs(current) as u32;
            integer_part.fmt(f)?;
        }
        
        for i in 0..6 {
            current *= 10.0;
            
            if let Some(digit) = char::from_digit(current as u32, 10) {
                f.write_char(digit)?;   
            }
            else {
                break;
            }
        }

        Ok(())
    }
}