use crate::internal::prelude::*;
mod who;
mod hi;
mod what;

use who::*;
use hi::*;
use what::*;

#[group]
#[commands(hi, who, what)]
pub struct General;
