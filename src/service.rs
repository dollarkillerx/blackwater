use super::*;

pub struct Core<'a> {
    param: &'a params::Params,
}

impl<'a> Core<'a> {
    pub async fn new(param: &'a params::Params) -> Core<'a> {
        Core {
            param
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        

        Ok(())
    }
}