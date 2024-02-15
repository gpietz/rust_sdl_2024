use anyhow::Result;

//////////////////////////////////////////////////////////////////////////////
// - Bindable -
//////////////////////////////////////////////////////////////////////////////

pub trait Bindable {
    type Target;

    fn bind(&mut self) -> Result<&mut Self::Target>;
    fn unbind(&mut self) -> Result<&mut Self::Target>;
    fn is_bound(&self) -> bool;
}

//////////////////////////////////////////////////////////////////////////////
// - Deletable -
//////////////////////////////////////////////////////////////////////////////

pub trait Deletable {
    fn delete(&mut self) -> Result<()>;
}