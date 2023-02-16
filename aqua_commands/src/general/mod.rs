use crate::imports::*;
mod who;
mod hi;

use who::*;
use hi::*;

#[group]
#[commands(who, hi)]
pub struct General;
